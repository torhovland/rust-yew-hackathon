use yew::prelude::*;
use yew_router::{prelude::*, Switch};
use frontpage::FrontPage;
use adminpage::AdminPage;

mod frontpage;
mod adminpage;

pub struct App {
}

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/"]
    FrontPage,
    #[to = "/admin"]
    AdminPage,
}

pub enum AppMsg {
}

impl Component for App { 
    type Message = AppMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::FrontPage => html!{<FrontPage/>},
                        AppRoute::AdminPage => html!{<AdminPage/>},
                    }
                })
            />
        }
    }
}
