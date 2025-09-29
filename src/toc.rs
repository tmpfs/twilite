use kuchiki::traits::*;

pub fn generate_toc(html: &str) -> String {
    let document = kuchiki::parse_html().one(html);
    let mut items = Vec::new();
    for css_match in document.select("h1, h2, h3, h4, h5, h6").unwrap() {
        let node = css_match.as_node().clone();
        let element = node.as_element().unwrap();
        let level = element.name.local[1..].parse::<usize>().unwrap();
        let text = css_match.text_contents().trim().to_string();
        if !text.is_empty() {
            items.push((level, text));
        }
    }

    let mut out = String::new();
    let mut stack: Vec<usize> = Vec::new();

    for (level, text) in items {
        if stack.is_empty() {
            out.push_str("<ul><li>");
            out.push_str(&text);
            stack.push(level);
            continue;
        }

        let current = *stack.last().unwrap();

        if level > current {
            for l in (current + 1)..=level {
                out.push_str("<ul><li>");
                if l == level {
                    out.push_str(&text);
                }
                stack.push(l);
            }
        } else if level == current {
            out.push_str("</li><li>");
            out.push_str(&text);
        } else {
            while !stack.is_empty() && *stack.last().unwrap() > level {
                out.push_str("</li></ul>");
                stack.pop();
            }
            out.push_str("</li><li>");
            out.push_str(&text);
        }
    }

    while !stack.is_empty() {
        out.push_str("</li></ul>");
        stack.pop();
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_level() {
        let html = "<h1>One</h1><h1>Two</h1>";
        let toc = generate_toc(html);
        assert_eq!(toc, "<ul><li>One</li><li>Two</li></ul>");
    }

    #[test]
    fn nested_levels() {
        let html = "<h1>One</h1><h2>Sub</h2><h1>Two</h1>";
        let toc = generate_toc(html);
        assert_eq!(
            toc,
            "<ul><li>One<ul><li>Sub</li></ul></li><li>Two</li></ul>"
        );
    }

    #[test]
    fn deeper_nesting() {
        let html = "<h1>One</h1><h2>A</h2><h3>B</h3><h2>C</h2><h1>Two</h1>";
        let toc = generate_toc(html);
        assert_eq!(
            toc,
            "<ul><li>One<ul><li>A<ul><li>B</li></ul></li><li>C</li></ul></li><li>Two</li></ul>"
        );
    }

    #[test]
    fn nonsequential_levels() {
        let html = "<h1>One</h1><h3>Deep</h3><h2>Back</h2>";
        let toc = generate_toc(html);
        assert_eq!(
            toc,
            "<ul><li>One<ul><li><ul><li>Deep</li></ul></li><li>Back</li></ul></li></ul>"
        );
    }
}
