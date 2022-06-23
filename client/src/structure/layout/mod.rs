use yew::prelude::*;

pub struct Layout;

impl Component for Layout {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div id="main-layout-grid"></div>
            </>
        }
    }
}
