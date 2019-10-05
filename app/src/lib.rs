#![feature(proc_macro_hygiene)]

pub use crate::state::*;
pub use crate::store::*;
use crate::views::*;
use router_rs::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use virtual_dom_rs::prelude::*;
pub use virtual_dom_rs::VirtualNode;
use wasm_bindgen;
use wasm_bindgen::prelude::*;

pub mod state;
pub mod store;
mod views;

pub struct App {
    pub store: Rc<RefCell<Store>>,
    pub router: Rc<Router>,
}

impl App {
    pub fn new(path: &str) -> App {
        let state = State::new();
        let store = Rc::new(RefCell::new(Store::new(state)));

        store.borrow_mut().msg(&Msg::SetPath(path.to_owned()));

        let router = make_router(Rc::clone(&store));

        store.borrow_mut().set_router(Rc::clone(&router));

        App { store, router }
    }

    pub fn from_state_json(json: &str) -> App {
        let state = State::from_json(json);
        let store = Rc::new(RefCell::new(Store::new(state)));

        let router = make_router(Rc::clone(&store));

        store.borrow_mut().set_router(Rc::clone(&router));

        let path = store.borrow().path().to_string();

        store.borrow_mut().msg(&Msg::SetPath(path));

        App { store, router }
    }
}

impl App {
    pub fn render(&self) -> VirtualNode {
        self.router.view(self.store.borrow().path()).unwrap()
    }
}

#[route(path = "/")]
pub fn home_route(store: Provided<Rc<RefCell<Store>>>) -> VirtualNode {
    HomeView::new(Rc::clone(&store)).render()
}
#[route(path = "/projects.html")]
pub fn projects_route(store: Provided<Rc<RefCell<Store>>>) -> VirtualNode {
    ProjectsView::new(Rc::clone(&store)).render()
}

fn make_router(store: Rc<RefCell<Store>>) -> Rc<Router> {
    let mut router = Router::default();

    router.provide(store);

    router.set_route_handlers(create_routes![home_route,projects_route]);

    Rc::new(router)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let app = App::new("/");
    }
}
