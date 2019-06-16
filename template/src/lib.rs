extern crate hex;
extern crate sha1;
extern crate wasm_bindgen;

use sha1::{Digest, Sha1};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocato
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Sha1sum {
    hasher: RefCell<Sha1>,
}

#[wasm_bindgen]
impl Sha1sum {
    pub fn new() -> Self {
        Sha1sum {
            hasher: RefCell::new(Sha1::new()),
        }
    }

    pub fn update(&mut self, input: &[u8]) {
        self.hasher.borrow_mut().input(input);
    }

    pub fn result(&self) -> String {
        format!("{:x}", self.hasher.borrow_mut().result_reset())
    }

    pub fn reset(&self) {
        self.hasher.borrow_mut().reset();
    }
}

#[wasm_bindgen(start)]
pub fn initialize() {
    console_error_panic_hook::set_once();
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn sha1_test() {
        let input = vec![
            97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 10,
        ];
        let expected = "d7c365d62cab86e98466e6b0431dc0d4529ef1dc";
        let mut sha1 = Sha1sum::new();
        sha1.update(&input);
        assert_eq!(sha1.result(), expected);
    }

    #[wasm_bindgen_test]
    fn sha1_update_test() {
        let input = vec![
            97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 10,
        ];
        let expected = "d7c365d62cab86e98466e6b0431dc0d4529ef1dc";
        let mut sha1 = Sha1sum::new();
        sha1.update(&input[0..2]);
        sha1.update(&input[2..10]);
        sha1.update(&input[10..]);
        assert_eq!(sha1.result(), expected);
    }
}
