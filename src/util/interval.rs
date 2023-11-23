use crate::core::traits::io::device::timer::TimerEvent;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
    fn clearInterval(id: i32);
}

pub fn set_interval<T>(timer: Rc<RefCell<T>>, mills: u32) -> i32
where
    T: 'static + TimerEvent,
{
    let closure = Closure::new(move || {
        if timer.borrow().is_continue() {
            timer.borrow_mut().routine();
        }
    });
    let interval_id = setInterval(&closure, mills);
    closure.forget();
    interval_id
}

pub fn clear_interval(id: i32) {
    clearInterval(id);
}
