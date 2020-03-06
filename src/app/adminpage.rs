use yew::prelude::*;

pub struct AdminPage {
    link: ComponentLink<Self>,
    title: String,
    question: String,
}

pub enum AdminMsg {
    Save,
}

impl Component for AdminPage {
    type Message = AdminMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        AdminPage {
            link,
            title: "".to_string(),
            question: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AdminMsg::Save => {
                info!("save!");
                false
            }
        }
    }

    fn view(&self) -> Html {
        let on_save = self.link.callback(|_| AdminMsg::Save);

        html! {
            <div>
                <h1>{ "Edit the survey" }</h1>

                <label>{ "Enter title" }</label>
                <br />
                <input type="text" value=&self.title />
                <br />
                <label>{ "Enter question" }</label>
                <br />
                <input type="text" value=&self.question />
                <br />
                <button onclick=on_save>{ "Save" }</button>
            </div>
        }
    }
}
