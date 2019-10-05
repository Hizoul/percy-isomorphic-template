use console_error_panic_hook;
use isomorphic_app;
use isomorphic_app::Msg;
use isomorphic_app::{App, Store};
use js_sys::Reflect;
use std::cell::RefCell;
use std::rc::Rc;
use virtual_dom_rs::prelude::*;
use wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;
use web_sys::Url;
use std::sync::{Mutex, Arc};
use router_rs::prelude::*;

pub struct Client {
    pub app: App,
    pub dom_updater: DomUpdater,
}

#[wasm_bindgen]
pub struct Renderer {
    client: Arc<Mutex<Client>>
}
#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(constructor)]
    pub fn new(initial_state: &str) -> Renderer {
        let client = Client::new(initial_state);
        let locked_client = Arc::new(Mutex::new(client));
        {
            let unlocked_client = locked_client.lock().unwrap();
            let sub = locked_client.clone();
            let mut borrowed_store = unlocked_client.app.store.borrow_mut();
            let mut a = Arc::new(Closure::wrap(Box::new(move || {
                let mut unlocked_client = sub.lock().unwrap();
                unlocked_client.render();
            }) as Box<dyn Fn()>));
            borrowed_store.subscribe(Box::new(move || {
                window().request_animation_frame((*a).as_ref().unchecked_ref());
            }));
        }
        {
            let mut unlocked_client = locked_client.lock().unwrap();
            unlocked_client.render();
        }
        Renderer {
            client: locked_client
        }
    }
}

impl Client {
    pub fn new(initial_state: &str) -> Client {
        console_error_panic_hook::set_once();

        let app = App::from_state_json(initial_state);

        app.store.borrow_mut().set_after_route(Box::new(|new_path| {
            history().push_state_with_url(&JsValue::null(), "Rust Web App", Some(new_path));
        }));

        let store = Rc::clone(&app.store);
        let on_popstate = move |_: web_sys::Event| {
            let location = location();
            let path = location.pathname().unwrap() + &location.search().unwrap();

            store.borrow_mut().msg(&Msg::SetPath(path))
        };
        let on_popstate = Box::new(on_popstate) as Box<FnMut(_)>;
        let mut on_popstate = Closure::wrap(on_popstate);
        window().set_onpopstate(Some(on_popstate.as_ref().unchecked_ref()));
        on_popstate.forget();

        let root_node = document()
            .get_element_by_id("isomorphic-rust-web-app")
            .unwrap();
        let dom_updater = DomUpdater::new_replace_mount(app.render(), root_node);

        let store = Rc::clone(&app.store);
        let router = Rc::clone(&app.router);
        intercept_relative_links(store, router);

        Client { app, dom_updater }
    }

    pub fn render(&mut self) {
        let vdom = self.app.render();
        self.dom_updater.update(vdom);
    }
}

// Ensure that anytime a link such as `<a href="/foo" />` is clicked we re-render the page locally
// instead of hitting the server to load a new page.
fn intercept_relative_links(store: Rc<RefCell<Store>>, router: Rc<Router>) {
    let on_anchor_click = move |event: web_sys::Event| {
        // Get the tag name of the element that was clicked
        let target = event
            .target()
            .unwrap()
            .dyn_into::<web_sys::Element>()
            .unwrap();
        let tag_name = target.tag_name();
        let tag_name = tag_name.as_str();

        // If the clicked element is an anchor tag, check if it points to the current website
        // (ex: '<a href="/some-page"></a>'
        if tag_name.to_lowercase() == "a" {
            let link = Reflect::get(&target, &"href".into());
            if link.is_ok() {
                let link_url_getter = Url::new(link.unwrap()
                    .as_string()
                    .unwrap().as_str());
                if link_url_getter.is_ok() {
                    let link_url = link_url_getter.unwrap();
                    // If this was indeed a relative URL, let our single page application router
                    // handle it
                    if link_url.hostname() == hostname() && link_url.port() == port() {
                        let pathname = link_url.pathname();
                        if router.matching_routerhandler(pathname.as_str()).is_some() {
                            event.prevent_default();
                            let msg = &Msg::SetPath(pathname);
                            store.borrow_mut().msg(msg);
                        }
                    }
                }
            }
        }
    };
    let on_anchor_click = Closure::wrap(Box::new(on_anchor_click) as Box<FnMut(_)>);

    window()
        .add_event_listener_with_callback("click", on_anchor_click.as_ref().unchecked_ref())
        .unwrap();
    on_anchor_click.forget();
}

fn window() -> web_sys::Window {
    web_sys::window().unwrap()
}

fn document() -> web_sys::Document {
    window().document().unwrap()
}

fn history() -> web_sys::History {
    window().history().unwrap()
}

fn location() -> web_sys::Location {
    document().location().unwrap()
}

fn hostname() -> String {
    location().hostname().unwrap()
}

fn port() -> String {
    location().port().unwrap()
}
