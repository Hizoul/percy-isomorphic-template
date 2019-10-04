use css_rs_macro::css;
use virtual_dom_rs::prelude::*;

mod nav_bar_item_view;
use self::nav_bar_item_view::NavBarItemView;

pub struct NavBarView {
    active_page: ActivePage,
}

impl NavBarView {
    pub fn new(active_page: ActivePage) -> NavBarView {
        NavBarView { active_page }
    }
}

pub enum ActivePage {
    Home,
    Contributors,
}

impl View for NavBarView {
    fn render(&self) -> VirtualNode {
        let home = NavBarItemView::new("/", "Isomorphic Web App", "");
        html! {
            <div class=NAV_BAR_CSS>
                { home.render() }
            </div>
        }
    }
}

static NAV_BAR_CSS: &'static str = css! {""};
