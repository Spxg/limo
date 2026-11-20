use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or("expected a page document")?;
    let body = document.body().ok_or("expected a document body")?;
    let message_id = "wasm-message";
    let message = document.create_element("p")?;
    message.set_id(message_id);
    message.set_text_content(Some("Hello from Rust via Wasm!"));
    body.append_child(&message)?;

    let text = document
        .get_element_by_id(message_id)
        .and_then(|element| element.text_content())
        .ok_or("expected the paragraph in the document")?;
    web_sys::console::log_1(&JsValue::from_str(&text));
    Ok(())
}
