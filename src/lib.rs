/*
    * Source for Tutorial: https://www.youtube.com/watch?v=0ywizYLPV00&list=PLtTT8p-gjGEdGzZ0ET2bwNnA6iP_mmmrv
*/

mod random;
mod minesweeper;

use std::cell::RefCell;
use minesweeper::*;
use wasm_bindgen::prelude::*;
use random::random_range;

thread_local! {
    static MINESWEEPER: RefCell<Minesweeper> = RefCell::new(Minesweeper::new(10, 10, random_range(5, 15)));
}

#[wasm_bindgen(js_name = getState)]
pub fn get_state() -> String {
    MINESWEEPER.with(|ms| ms.borrow().to_string())
}

#[wasm_bindgen(js_name = openField)]
pub fn open_fields(x: usize, y: usize) {
    MINESWEEPER.with(|ms| ms.borrow_mut().open((x, y)));
}

#[wasm_bindgen(js_name = toggleFlag)]
pub fn toggle_flag(x: usize, y: usize) {
    MINESWEEPER.with(|ms| ms.borrow_mut().toggle_flag((x, y)));
}

#[wasm_bindgen(js_name = isLost)]
pub fn is_lost() -> bool {
    return MINESWEEPER.with(|ms| ms.borrow().is_lost());
}

#[wasm_bindgen(js_name = isWon)]
pub fn is_won() -> bool {
    return MINESWEEPER.with(|ms| ms.borrow().is_won());
}


