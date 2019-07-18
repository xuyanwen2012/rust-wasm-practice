mod utils;

extern crate fixedbitset;
extern crate web_sys;

use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

trait Builder {
    fn with(self) -> Self;
    fn build(self) -> Universe;
}

struct UniverseBuilder {
    built: bool,
}

impl Builder for UniverseBuilder {
    #[inline]
    fn with(self) -> Self {
        self
    }

    fn build(self) -> Universe {
        unimplemented!()
    }
}

impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_col == 0 && delta_row == 0 {
                    continue;
                }

                let xs = (row + delta_row) % self.height;
                let xy = (col + delta_col) % self.width;
                let index = self.get_index(xs, xy);
                count += self.cells[index] as u8;
            }
        }
        count
    }

    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (rol, col) in cells.iter().cloned() {
            let index = self.get_index(rol, col);
            self.cells.put(index);
        }
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self {
        utils::set_panic_hook();
        Self::with_dimension(64, 64)
    }

    pub fn with_dimension(width: u32, height: u32) -> Self {
        utils::set_panic_hook();
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, i % 2 == 0 || i % 7 == 0)
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let cell = self.cells[index];
                let live_neighbors = self.live_neighbor_count(row, col);

                //                log!(
                //                    "Cell [{}, {}] is initially {:?} and has {} living neighbors",
                //                    row,
                //                    col,
                //                    cell,
                //                    live_neighbors
                //                );

                next.set(
                    index,
                    match (cell, live_neighbors) {
                        (true, x) if x < 2 => false,
                        (true, 2) | (true, 3) => true,
                        (true, x) if x > 3 => false,
                        (false, 3) => true,
                        (otherwise, _) => otherwise,
                    },
                );

                //                log!("    it becomes {:?}", next.contains(index));
            }
        }

        self.cells = next;
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let index = self.get_index(row, col);
        let bit = self.cells.contains(index);
        self.cells.set(index, !bit);
    }
}
