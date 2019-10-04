use isomorphic_app::App;
use html_minifier::HTMLMinifier;
use std::fs::write;
use isomorphic_prerender::prerender_app_to;

fn main() {
    prerender_app_to("/", "../client/dist/index.html").unwrap();
    println!("prerendered to ../client/dist/index.html. edit this to prerender more.");
}
