pub fn parse_markdown(input: &str) -> String {
    let parser = pulldown_cmark::Parser::new(input);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}
