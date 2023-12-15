use minify_html::{minify, Cfg};

pub fn minify_response(html: String) -> String {
    return String::from_utf8(minify(&html.as_bytes(), &Cfg::new())).unwrap();
}
