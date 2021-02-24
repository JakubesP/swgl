use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// -----------------------------------------------------------------------------------------------------------

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

// -----------------------------------------------------------------------------------------------------------

pub struct KeyboardPressedStateBox {
    pub key_status_map: Rc<RefCell<HashMap<String, bool>>>,
}

impl KeyboardPressedStateBox {

    /// This function creates listeners for given vector of keys.
    pub fn new(keys: Vec<String>) -> KeyboardPressedStateBox {
        let ksbox = KeyboardPressedStateBox {
            key_status_map: Rc::new(RefCell::new(HashMap::new())),
        };

        let key_status_map_ref = ksbox.key_status_map.clone();

        let on_key_down = Closure::wrap(Box::new(
            enclose!((key_status_map_ref, keys) move |event: web_sys::KeyboardEvent| {
                for key in &keys {
                    if event.code() == key.to_string() {
                        key_status_map_ref.borrow_mut().insert(key.clone(), true);
                    }
                }
            }),
        ) as Box<dyn FnMut(_)>);

        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("keydown", on_key_down.as_ref().unchecked_ref())
            .unwrap();

        on_key_down.forget();

        // --------------------------------------------------------------------------------

        let on_key_up = Closure::wrap(Box::new(
            enclose!((key_status_map_ref, keys) move |event: web_sys::KeyboardEvent| {
                for key in &keys {
                    if event.code() == key.to_string() {
                        key_status_map_ref.borrow_mut().insert(key.clone(), false);
                    }
                }
            }),
        ) as Box<dyn FnMut(_)>);

        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("keyup", on_key_up.as_ref().unchecked_ref())
            .unwrap();

        on_key_up.forget();

        // --------------------------------------------------------------------------------

        let on_focus_out = Closure::wrap(Box::new(
            enclose!((key_status_map_ref, keys) move |_: web_sys::FocusEvent| {
                for key in &keys {
                    key_status_map_ref.borrow_mut().insert(key.clone(), false);
                }
            }),
        ) as Box<dyn FnMut(_)>);

        web_sys::window()
        .unwrap().add_event_listener_with_callback("blur", on_focus_out.as_ref().unchecked_ref()).unwrap();

        on_focus_out.forget();


        ksbox
    }

    pub fn is_key_pressed(&self, key: &str) -> bool {
        if let Some(state) = self.key_status_map.borrow().get(key) {
            return *state;
        }
        false
    }
}
