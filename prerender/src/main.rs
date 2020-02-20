use isomorphic_prerender::prerender_app_to;

fn main() {
    let target = "../client/dist/";
    let routes = vec![
        ("/", "index.html"),
        ("/projects.html", "projects.html")
    ];
    for rout in routes {
        prerender_app_to(rout.0, format!("{}{}", target, rout.1).as_str()).unwrap();
        println!("rendered {:?}", rout);
    }
    println!("edit the routes vector in prerender/src/main.rs to add or remove routes");
}