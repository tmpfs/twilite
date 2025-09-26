use crate::error::ServerError;
use kuchiki::{parse_html, traits::*};
use regex::Regex;
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

pub fn html_to_text(html_input: &str) -> String {
    use scraper::Html;
    let fragment = Html::parse_fragment(html_input);
    let root_element = fragment.root_element();
    root_element.text().collect::<Vec<_>>().join(" ")
}

pub fn trim_preview_text<'a>(input: &'a str) -> &str {
    if input.len() < PREVIEW_LENGTH {
        input
    } else {
        &input[..PREVIEW_LENGTH]
    }
}

pub fn rewrite_wiki_links(input: &str) -> Result<String, ServerError> {
    let document = parse_html().from_utf8().read_from(&mut input.as_bytes())?;

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

    let mut output = Vec::new();
    document.serialize(&mut output)?;
    Ok(String::from_utf8(output)?)
}

#[cfg(test)]
mod test {
    use super::rewrite_wiki_links;
    #[test]
    fn html_wiki_links() {
        let html = r#"<p>This is a WikiPage with SomeOtherPage inside a paragraph.</p>"#;

        match rewrite_wiki_links(html) {
            Ok(rewritten_html) => {
                assert!(rewritten_html.contains(r#"<a href="/wiki/WikiPage">WikiPage</a>"#));
                assert!(
                    rewritten_html.contains(r#"<a href="/wiki/SomeOtherPage">SomeOtherPage</a>"#)
                );
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn ignores_wiki_links_in_anchors() {
        let html = r#"<p>This <a href="/existing">WikiPage and AnotherPage</a> should not be converted.</p>"#;

        match rewrite_wiki_links(html) {
            Ok(rewritten_html) => {
                // Should not contain new wiki links inside the existing anchor
                assert!(!rewritten_html.contains(r#"<a href="/wiki/WikiPage">"#));
                assert!(!rewritten_html.contains(r#"<a href="/wiki/AnotherPage">"#));
                // Should still contain the original anchor
                assert!(
                    rewritten_html.contains(r#"<a href="/existing">WikiPage and AnotherPage</a>"#)
                );
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn ignores_prefixed_wiki_links() {
        let html = r#"<p>This !WikiPage should not be converted, but ThisPage should be.</p>"#;

        match rewrite_wiki_links(html) {
            Ok(rewritten_html) => {
                // Should not convert !WikiPage
                assert!(!rewritten_html.contains(r#"<a href="/wiki/WikiPage">"#));
                assert!(rewritten_html.contains("!WikiPage"));
                // Should convert ThisPage
                assert!(rewritten_html.contains(r#"<a href="/wiki/ThisPage">ThisPage</a>"#));
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn handles_multiple_wiki_words() {
        let html = r#"<p>Check out WikiPage and also SomeOtherPage for more info.</p>"#;

        match rewrite_wiki_links(html) {
            Ok(rewritten_html) => {
                assert!(rewritten_html.contains(r#"<a href="/wiki/WikiPage">WikiPage</a>"#));
                assert!(
                    rewritten_html.contains(r#"<a href="/wiki/SomeOtherPage">SomeOtherPage</a>"#)
                );
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn preserves_non_wiki_words() {
        let html = r#"<p>This iPhone and HTML are not WikiWords.</p>"#;

        match rewrite_wiki_links(html) {
            Ok(rewritten_html) => {
                // These should not be converted as they don't match the WikiWord pattern
                assert!(!rewritten_html.contains(r#"<a href="/wiki/iPhone">"#));
                assert!(!rewritten_html.contains(r#"<a href="/wiki/HTML">"#));
                assert!(rewritten_html.contains("iPhone"));
                assert!(rewritten_html.contains("HTML"));
            }
            Err(e) => panic!("Error: {}", e),
        }
    }
}
