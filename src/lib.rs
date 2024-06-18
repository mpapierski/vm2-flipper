#![cfg_attr(target_arch = "wasm32", no_main)]
#![cfg_attr(target_arch = "wasm32", no_std)]

use casper_macros::casper;
use casper_sdk::log;

/// This contract implements a simple flipper.
#[casper(contract_state)]
pub struct Flipper {
    /// The current state of the flipper.
    value: bool,
}

impl Default for Flipper {
    fn default() -> Self {
        panic!("Unable to instantiate contract without a constructor");
    }
}

#[casper]
impl Flipper {
    #[casper(constructor)]
    pub fn new(init_value: bool) -> Self {
        log!("Instantiating flipper contract init={:?}", init_value);
        Self { value: init_value }
    }

    #[casper(constructor)]
    pub fn default() -> Self {
        log!("Instantiating flipper contract with default init");
        Self::new(Default::default())
    }

    pub fn flip(&mut self) {
        let old_value = self.value;
        self.value = !self.value;
        log!("Flipping old_value={old_value:?} new_value={}", self.value);
    }

    pub fn get(&self) -> bool {
        log!("Getting flipper value {}", self.value);
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flipper() {
        let mut flipper = Flipper::new(false);
        assert_eq!(flipper.get(), false);
        flipper.flip();
        assert_eq!(flipper.get(), true);
        flipper.flip();
        assert_eq!(flipper.get(), false);
    }
}
