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
