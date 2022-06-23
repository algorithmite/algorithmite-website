use super::{background::Background, layout::Layout, menu::Menu};
use yew::prelude::*;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Background/>
                <Layout/>
                <Menu/>
            </>
        }
    }
}
