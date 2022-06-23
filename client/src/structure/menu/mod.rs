use yew::prelude::*;

pub struct Menu;

impl Component for Menu {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <nav id="main-layout-menu"></nav>
            </>
        }
    }
}
