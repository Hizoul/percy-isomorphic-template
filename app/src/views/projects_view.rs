use crate::store::Store;
use crate::views::nav_bar_view::ActivePage;
use crate::views::nav_bar_view::NavBarView;
use crate::Msg;

use virtual_dom_rs::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct ProjectsView {
    store: Rc<RefCell<Store>>,
}

impl ProjectsView {
    pub fn new(store: Rc<RefCell<Store>>) -> ProjectsView {
        ProjectsView { store }
    }
}

impl View for ProjectsView {
    fn render(&self) -> VirtualNode {
        let nav_bar = NavBarView::new(ActivePage::Home).render();
        let store = Rc::clone(&self.store);
        html! {
        <div>
          { nav_bar }
          <span>projects view!</span>
        </div>
        }
    }
}
