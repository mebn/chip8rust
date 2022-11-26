use yew::prelude::*;
use wasm_bindgen::JsValue;

mod screen;
mod controls;
mod chip8;

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>

            <canvas id="canvas" style="border: 1px solid red"></canvas>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = web_sys::HtmlCanvasElement::from(JsValue::from(document.get_element_by_id("canvas").unwrap()));

    canvas.set_width(1500);
    canvas.set_height(1500);

    let ctx = web_sys::CanvasRenderingContext2d::from(JsValue::from(canvas.get_context("2d").unwrap().unwrap()));

    ctx.set_fill_style(&JsValue::from_str("green"));
    ctx.fill_rect(10., 10., 50., 50.);
}
