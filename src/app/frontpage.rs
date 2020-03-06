use yew::prelude::*;
use yew_router::prelude::*;
use super::AppRoute;

pub struct FrontPage {
    clicked: bool,
    width: String,
    height: String,
    link: ComponentLink<Self>,
}

pub enum FrontPageMsg {
    Click,
    WidthChange(String),
    HeightChange(String),
}

impl Component for FrontPage { 
    type Message = FrontPageMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        FrontPage {
            link,
            clicked: false,
            width: "200".to_string(),
            height: "100".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            FrontPageMsg::Click => {
                info!("click!");
                self.clicked = true;
                true // Indicate that the Component should re-render
            }
            FrontPageMsg::WidthChange(width) => {
                info!("width changed: {}", width);
                self.width = width;
                true // Indicate that the Component should re-render
            }
            FrontPageMsg::HeightChange(height) => {
                info!("height changed: {}", height);
                self.height = height;
                true // Indicate that the Component should re-render
            }
        }
    }

    fn view(&self) -> Html {
        let on_click = self.link.callback(|_| FrontPageMsg::Click);
        let on_width_change = self.link.callback(|e: InputData| FrontPageMsg::WidthChange(e.value));
        let on_height_change = self.link.callback(|e: InputData| FrontPageMsg::HeightChange(e.value));

        let button_text = if self.clicked { "Clicked!" } else { "Click me!" };
        let url = format!("https://loremflickr.com/{}/{}", self.width, self.height);

        html! {
            <div>
                <h1>{ "Rust + Yew Hackathon" }</h1>

                <button onclick=on_click>{ button_text }</button>
                <br />

                <label>{ format!("Choose width: {}", self.width) }</label>
                <br />
                <input type="range" min="10" max="640" value=&self.width class="slider" id="widthRange" oninput=on_width_change />                
                <br />
                <label>{ format!("Choose height: {}", self.height) }</label>
                <br />
                <input type="range" min="10" max="480" value=&self.height class="slider" id="heightRange" oninput=on_height_change />
                <br />

                <img src=url />

                <RouterButton<AppRoute> route=AppRoute::AdminPage> { "Opprett spørreskjema" } </RouterButton<AppRoute>>
            </div>
        }
    }
}
