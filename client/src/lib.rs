use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod structure;
use structure::layout::Layout;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<Layout>();

    Ok(())
}
