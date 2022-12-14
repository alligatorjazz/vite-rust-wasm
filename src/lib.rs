mod utils;
use wasm_bindgen::prelude::*;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;
use crate::utils::{set_panic_hook, Timer};
extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// TODO: find out why it demands UPPER_SNAKE_CASE
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello {}!", name));
}

// universe is a single "linear" array, but we use get_index to simulate multi-dimensionality
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        ((row * self.width) + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        // for row in the range of:
        // grid height - 1 (to compensate for 0-indexing)
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
		let _timer = Timer::new("Universe::tick");
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                next.set(idx, match (cell, live_neighbors) {
					(true, x) if x < 2 => false,
					(true, 2) | (true, 3) => true,
					(true, x) if x > 3 => false,
					(false, 3) => true,
					(otherwise, _) => otherwise
				});
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
		set_panic_hook();

        let width = 64;
        let height = 64;

		let size = (width * height) as usize;
		let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, rand::random());
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

	pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
		let value = self.cells[idx];
        self.cells.set(idx, !value);
    }
}

// removed #[wasm_bindgen] - not sent to js 
impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true)
        }
    }

	/// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
		self.width = width;
        self.cells = FixedBitSet::with_capacity((width * self.height) as usize);
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
		self.height = height;
        self.cells = FixedBitSet::with_capacity((self.width * height) as usize);
    }
}


impl Default for Universe {
    fn default() -> Self {
        Self::new()
    }
}