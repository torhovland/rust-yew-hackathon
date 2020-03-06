use adminpage::AdminPage;
use frontpage::FrontPage;
use yew::prelude::*;
use yew_router::{prelude::*, Switch};

mod adminpage;
mod frontpage;

pub struct App {}

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/admin"]
    AdminPage,
    #[to = "/"]
    FrontPage,
}

pub enum AppMsg {}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
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
