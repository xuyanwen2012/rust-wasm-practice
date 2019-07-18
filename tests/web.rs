//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use hello_wasm::Universe;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[cfg(test)]
pub fn input_spaceship() -> Universe {
    let mut universe = Universe::with_dimension(6, 6);
    universe.set_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
    universe
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    let mut universe = Universe::with_dimension(6, 6);
    universe.set_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);
    universe
}

#[wasm_bindgen_test]
pub fn test_tick() {
    let mut input_universe = input_spaceship();

    let expected_universe = expected_spaceship();

    input_universe.tick();

    let diff = input_universe
        .get_cells()
        .difference(expected_universe.get_cells());

    assert_eq!(diff.count(), 0);
}
