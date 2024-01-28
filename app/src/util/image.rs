use crate::util::closure::closure_once;
use anyhow::{anyhow, Result};
use futures::channel::oneshot::channel;
use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlImageElement;

pub async fn load_image(source: &str) -> Result<HtmlImageElement> {
    let image = HtmlImageElement::new().unwrap();

    let (complete_tx, complete_rx) = channel::<Result<()>>();
    let success_tx = Rc::new(Mutex::new(Some(complete_tx)));
    let error_tx = Rc::clone(&success_tx);
    let success_callback = closure_once(move || {
        if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
            let _ = success_tx.send(Ok(()));
        }
    });
    let error_callback: Closure<dyn FnMut(JsValue)> = closure_once(move |err| {
        if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
            let _ = error_tx.send(Err(anyhow!("Error loading image: {:#?}", err)));
        }
    });
    image.set_onload(Some(success_callback.as_ref().unchecked_ref()));
    image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
    image.set_src(source);

    complete_rx.await??;
    Ok(image)
}
