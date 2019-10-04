
const HTML_PLACEHOLDER: &str = "#HTML_INSERTED_HERE_BY_SERVER#";
const STATE_PLACEHOLDER: &str = "#INITIAL_STATE_JSON#";
use isomorphic_app::App;
use html_minifier::HTMLMinifier;
use std::fs::write;
use virtual_dom_rs::VirtualNode;

const html: &str = include_str!("./index.html");

pub fn minify(to_minify: &str) -> String {
    let mut minifier = HTMLMinifier::new();
    minifier.digest(to_minify).unwrap();
    minifier.get_html()
}

pub fn prerender_app(path: &str) -> String {
    let app = App::new(path);
    let state = app.store.borrow();
    let mut insert = format!("{}", html);
    insert = insert.replacen(HTML_PLACEHOLDER, &app.render().to_string(), 1);
    insert = insert.replacen(STATE_PLACEHOLDER, &state.to_json(), 1);
    insert
}

pub fn prerender_app_to(path: &str, out_file: &str) -> Result<(), std::io::Error> {
    write(out_file, minify(prerender_app(path).as_str()))
}
pub fn prerender(node: VirtualNode) -> String {
    let mut insert = format!("{}", html);
    insert = insert.replacen(HTML_PLACEHOLDER, &node.to_string(), 1);
    insert
}

pub fn prerender_to(node: VirtualNode, out_file: &str) -> Result<(), std::io::Error> {
    write(out_file, minify(prerender(node).as_str()))
}
