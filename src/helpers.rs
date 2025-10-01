use crate::error::ServerError;
use kuchiki::NodeRef;
use kuchiki::parse_html;
use kuchiki::traits::*;
use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

static WIKI_WORD_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"([A-Z][a-z0-9]+(?:[A-Z][a-z0-9]*)+)").unwrap());

const PREVIEW_LENGTH: usize = 256;

pub fn sanitize_html(dirty_html: &str) -> String {
    use ammonia::Builder;
    let mut builder = Builder::default();
    builder.add_tags(&["video", "source"]);
    builder.add_generic_attributes(&["class", "id"]);
    builder.add_url_schemes(&["mailto"]);
    // force rel="noopener noreferrer" on anchors
    builder.url_relative(ammonia::UrlRelative::PassThrough);
    builder.link_rel(Some("noopener noreferrer"));
    builder.clean(dirty_html).to_string()
}

pub fn html_to_text(document: &NodeRef) -> String {
    fn walk(node: &NodeRef, out: &mut String) {
        match node.as_element().map(|el| el.name.local.as_ref()) {
            Some("p") => {
                for child in node.children() {
                    walk(&child, out);
                }
                out.push_str("\n\n"); // separate paragraphs
            }
            Some("br") => {
                out.push('\n');
            }
            Some("li") => {
                out.push_str("• "); // bullet point
                for child in node.children() {
                    walk(&child, out);
                }
                out.push('\n');
            }
            Some("ul") | Some("ol") => {
                for child in node.children() {
                    walk(&child, out);
                }
                out.push('\n');
            }
            Some("h1") | Some("h2") | Some("h3") | Some("h4") | Some("h5") | Some("h6") => {
                for child in node.children() {
                    walk(&child, out);
                }
                out.push_str("\n\n");
            }
            _ => {
                if let Some(text) = node.as_text() {
                    out.push_str(text.borrow().as_ref());
                } else {
                    for child in node.children() {
                        walk(&child, out);
                    }
                }
            }
        }
    }

    let mut result = String::new();
    walk(document, &mut result);

    // Normalize excessive whitespace
    let normalized = result
        .lines()
        .map(|l| l.trim_end())
        .collect::<Vec<_>>()
        .join("\n");

    normalized.trim().to_string()
}

pub fn trim_preview_text(input: &str) -> &str {
    if input.len() < PREVIEW_LENGTH {
        input
    } else {
        &input[..PREVIEW_LENGTH]
    }
}

pub fn transform_page(input: &str) -> Result<(NodeRef, Option<String>), ServerError> {
    let mut document = parse_html().from_utf8().read_from(&mut input.as_bytes())?;
    rewrite_wiki_links(&mut document)?;
    let toc = assign_ids_and_generate_toc(&document);
    Ok((document, toc))
}

pub fn stringify_doc(document: &NodeRef) -> Result<String, ServerError> {
    let mut output = Vec::new();
    document.serialize(&mut output)?;
    Ok(String::from_utf8(output)?)
}

fn rewrite_wiki_links(document: &mut NodeRef) -> Result<(), ServerError> {
    for css_match in document.descendants().text_nodes() {
        let parent_is_anchor = css_match.as_node().ancestors().any(|a| {
            a.as_element()
                .map(|e| e.name.local.as_ref() == "a")
                .unwrap_or(false)
        });
        if !parent_is_anchor {
            let text = css_match.borrow().clone();
            let replaced = WIKI_WORD_REGEX.replace_all(text.as_ref(), |caps: &regex::Captures| {
                let wiki_word = &caps[1];
                let full_match = caps.get(0).unwrap();
                let start = full_match.start();
                if start > 0 && text.as_bytes()[start - 1] == b'!' {
                    wiki_word.to_string()
                } else {
                    format!("<a href=\"/wiki/{}\">{}</a>", wiki_word, wiki_word)
                }
            });

            if replaced != text.as_ref() as &str {
                let fragment = parse_html().one(replaced.into_owned());
                css_match.as_node().insert_after(fragment);
                css_match.as_node().detach();
            }
        }
    }

    Ok(())
}

pub fn generate_toc(document: &NodeRef) -> Option<String> {
    generate_toc_with_links(document, None)
}

fn generate_toc_with_links(document: &NodeRef, text_to_slug: Option<&HashMap<String, String>>) -> Option<String> {
    let mut items = Vec::new();
    let selector = document.select("h1, h2, h3, h4, h5, h6").unwrap();
    for css_match in selector {
        let node = css_match.as_node().clone();
        let element = node.as_element().unwrap();
        let level = element.name.local[1..].parse::<usize>().unwrap();
        let text = css_match.text_contents().trim().to_string();
        if !text.is_empty() {
            items.push((level, text));
        }
    }

    if !items.is_empty() {
        let mut out = String::new();
        let mut stack: Vec<usize> = Vec::new();

        for (level, text) in items {
            let content = if let Some(slug_map) = text_to_slug {
                if let Some(slug) = slug_map.get(&text) {
                    format!("<a href=\"#{}\">{}</a>", slug, text)
                } else {
                    text
                }
            } else {
                text
            };

            if stack.is_empty() {
                out.push_str("<ul><li>");
                out.push_str(&content);
                stack.push(level);
                continue;
            }

            let current = *stack.last().unwrap();

            if level > current {
                // Create only ONE <ul> for the new nesting level, regardless of how many levels we skip
                out.push_str("<ul><li>");
                out.push_str(&content);
                stack.push(level);
            } else if level == current {
                out.push_str("</li><li>");
                out.push_str(&content);
            } else {
                // Unwind stack to target level
                while !stack.is_empty() && *stack.last().unwrap() > level {
                    out.push_str("</li></ul>");
                    stack.pop();
                }
                out.push_str("</li><li>");
                out.push_str(&content);
                // Update the stack to reflect the current level
                if !stack.is_empty() {
                    stack.pop();
                }
                stack.push(level);
            }
        }

        while !stack.is_empty() {
            out.push_str("</li></ul>");
            stack.pop();
        }

        Some(out)
    } else {
        None
    }
}

/// Generate a base slug from heading text
fn slugify(text: &str) -> String {
    let re = Regex::new(r"[^a-z0-9\-]+").unwrap();
    let mut slug = text
        .to_lowercase()
        .replace(|c: char| c.is_whitespace(), "-");
    slug = re.replace_all(&slug, "").into_owned();
    slug.trim_matches('-').to_string()
}

/// Generate unique slugs for headings, ensuring duplicates get numeric suffixes
fn generate_unique_slugs(document: &NodeRef) -> HashMap<String, String> {
    let mut slug_counts: HashMap<String, usize> = HashMap::new();
    let mut text_to_slug: HashMap<String, String> = HashMap::new();

    for css_match in document.select("h1, h2, h3, h4, h5, h6").unwrap() {
        let as_node = css_match.as_node();
        if let Some(_element) = as_node.as_element() {
            let text = as_node.text_contents();
            let trimmed_text = text.trim();
            if trimmed_text.is_empty() {
                continue;
            }

            let mut slug = slugify(trimmed_text);
            let count = slug_counts.entry(slug.clone()).or_insert(0);
            *count += 1;
            if *count > 1 {
                slug = format!("{}-{}", slug, count);
            }

            text_to_slug.insert(trimmed_text.to_string(), slug);
        }
    }

    text_to_slug
}

/// Assign unique slugs as `id` attributes to all headings h1–h6 in the document
fn assign_heading_ids(document: &NodeRef) {
    let text_to_slug = generate_unique_slugs(document);

    for css_match in document.select("h1, h2, h3, h4, h5, h6").unwrap() {
        let as_node = css_match.as_node();
        if let Some(element) = as_node.as_element() {
            let text = as_node.text_contents();
            let trimmed_text = text.trim();
            if trimmed_text.is_empty() {
                continue;
            }

            if let Some(slug) = text_to_slug.get(trimmed_text) {
                let mut attributes = element.attributes.borrow_mut();
                attributes.insert("id", slug.clone());
            }
        }
    }
}

fn assign_ids_and_generate_toc(document: &NodeRef) -> Option<String> {
    let text_to_slug = generate_unique_slugs(document);

    for css_match in document.select("h1, h2, h3, h4, h5, h6").unwrap() {
        let as_node = css_match.as_node();
        if let Some(element) = as_node.as_element() {
            let text = as_node.text_contents();
            let trimmed_text = text.trim();
            if trimmed_text.is_empty() {
                continue;
            }

            if let Some(slug) = text_to_slug.get(trimmed_text) {
                let mut attributes = element.attributes.borrow_mut();
                attributes.insert("id", slug.clone());
            }
        }
    }

    generate_toc_with_links(document, Some(&text_to_slug))
}

#[cfg(test)]
mod test {
    use crate::helpers::{stringify_doc, transform_page};
    use anyhow::Result;
    use kuchiki::parse_html;
    use kuchiki::traits::*;

    #[test]
    fn html_wiki_links() -> Result<()> {
        let html = r#"<p>This is a WikiPage with SomeOtherPage inside a paragraph.</p>"#;
        let html = stringify_doc(&transform_page(html)?.0)?;
        assert!(html.contains(r#"<a href="/wiki/WikiPage">WikiPage</a>"#));
        assert!(html.contains(r#"<a href="/wiki/SomeOtherPage">SomeOtherPage</a>"#));
        Ok(())
    }

    #[test]
    fn ignores_wiki_links_in_anchors() -> Result<()> {
        let html = r#"<p>This <a href="/existing">WikiPage and AnotherPage</a> should not be converted.</p>"#;
        let html = stringify_doc(&transform_page(html)?.0)?;
        // Should not contain new wiki links inside the existing anchor
        assert!(!html.contains(r#"<a href="/wiki/WikiPage">"#));
        assert!(!html.contains(r#"<a href="/wiki/AnotherPage">"#));
        // Should still contain the original anchor
        assert!(html.contains(r#"<a href="/existing">WikiPage and AnotherPage</a>"#));
        Ok(())
    }

    #[test]
    fn ignores_prefixed_wiki_links() -> Result<()> {
        let html = r#"<p>This !WikiPage should not be converted, but ThisPage should be.</p>"#;
        let html = stringify_doc(&transform_page(html)?.0)?;
        // Should not convert !WikiPage
        assert!(!html.contains(r#"<a href="/wiki/WikiPage">"#));
        assert!(html.contains("!WikiPage"));
        // Should convert ThisPage
        assert!(html.contains(r#"<a href="/wiki/ThisPage">ThisPage</a>"#));
        Ok(())
    }

    #[test]
    fn handles_multiple_wiki_words() -> Result<()> {
        let html = r#"<p>Check out WikiPage and also SomeOtherPage for more info.</p>"#;
        let html = stringify_doc(&transform_page(html)?.0)?;
        assert!(html.contains(r#"<a href="/wiki/WikiPage">WikiPage</a>"#));
        assert!(html.contains(r#"<a href="/wiki/SomeOtherPage">SomeOtherPage</a>"#));
        Ok(())
    }

    #[test]
    fn preserves_non_wiki_words() -> Result<()> {
        let html = r#"<p>This iPhone and HTML are not WikiWords.</p>"#;
        let html = stringify_doc(&transform_page(html)?.0)?;
        // These should not be converted as they don't match the WikiWord pattern
        assert!(!html.contains(r#"<a href="/wiki/iPhone">"#));
        assert!(!html.contains(r#"<a href="/wiki/HTML">"#));
        assert!(html.contains("iPhone"));
        assert!(html.contains("HTML"));
        Ok(())
    }

    #[test]
    fn single_level() -> Result<()> {
        let html = "<h1>One</h1><h1>Two</h1>";
        // let html = parse_html().from_utf8().read_from(&mut html.as_bytes())?;
        let toc = transform_page(&html)?.1;
        assert_eq!(toc, Some("<ul><li><a href=\"#one\">One</a></li><li><a href=\"#two\">Two</a></li></ul>".to_owned()));
        Ok(())
    }

    #[test]
    fn nested_levels() -> Result<()> {
        let html = "<h1>One</h1><h2>Sub</h2><h1>Two</h1>";
        // let html = parse_html().from_utf8().read_from(&mut html.as_bytes())?;
        let toc = transform_page(&html)?.1;
        assert_eq!(
            toc,
            Some("<ul><li><a href=\"#one\">One</a><ul><li><a href=\"#sub\">Sub</a></li></ul></li><li><a href=\"#two\">Two</a></li></ul>".to_owned())
        );
        Ok(())
    }

    #[test]
    fn deeper_nesting() -> Result<()> {
        let html = "<h1>One</h1><h2>A</h2><h3>B</h3><h2>C</h2><h1>Two</h1>";
        // let html = parse_html().from_utf8().read_from(&mut html.as_bytes())?;
        let toc = transform_page(&html)?.1;
        assert_eq!(
            toc,
            Some(
                "<ul><li><a href=\"#one\">One</a><ul><li><a href=\"#a\">A</a><ul><li><a href=\"#b\">B</a></li></ul></li><li><a href=\"#c\">C</a></li></ul></li><li><a href=\"#two\">Two</a></li></ul>"
                    .to_owned()
            )
        );
        Ok(())
    }

    #[test]
    fn nonsequential_levels() -> Result<()> {
        let html = "<h1>One</h1><h3>Deep</h3><h2>Back</h2>";
        // let html = parse_html().from_utf8().read_from(&mut html.as_bytes())?;
        let toc = transform_page(&html)?.1;
        assert_eq!(
            toc,
            Some(
                "<ul><li><a href=\"#one\">One</a><ul><li><a href=\"#deep\">Deep</a></li></ul></li><li><a href=\"#back\">Back</a></li></ul>"
                    .to_owned()
            )
        );
        Ok(())
    }
}
