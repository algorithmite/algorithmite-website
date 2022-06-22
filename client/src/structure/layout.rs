use super::background::Background;
use yew::prelude::*;

pub struct Layout {
    value: i64,
}

pub enum Msg {
    AddOne,
}

impl Component for Layout {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let body_element = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .parent_element()
            .unwrap();

        html! {
            <>
                <Background background_width={ body_element.client_width() } background_height={ body_element.client_height() } />
                <div id="main-layout-grid"></div>
                <button onclick={ctx.link().callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </>
        }
    }
}
