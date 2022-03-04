mod utils;

use num::complex::Complex;

use wasm_bindgen::prelude::*;

use web_sys;


// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct MandelbrotCanvas {
    width: u32,
    height: u32,
    zoom: f64,
    center: (f64, f64),
    escape_times: Vec<u8>,
    pixels: Vec<u8>,
}

#[wasm_bindgen]
impl MandelbrotCanvas {
    pub fn new(width: u32, height: u32) -> MandelbrotCanvas {
        MandelbrotCanvas {
            width,
            height,
            zoom: 1./width as f64,
            center: (-1., 0.),
            escape_times: vec![0; (width*height) as usize],
            pixels: vec![0; (4*width*height) as usize],
        }
    }

    pub fn center_on_pixel(&mut self, i: u32, j: u32) {
        let (x, y) = self.pixel_to_position(i, j);
        self.center = (x, y);
    }

    pub fn mandelbrot(&mut self) {
        for j in 0..self.height {
            for i in 0..self.width {
                let idx = (j*self.width + i) as usize;

                let (x, y) = self.pixel_to_position(i, j);
                let p = Complex::new(x, y);

                self.escape_times[idx] = MandelbrotCanvas::time_to_diverge(p);
            }
        }

        self.update_pixels();
    }

    pub fn pixels(&self) -> *const u8 {
        self.pixels.as_ptr()
    }

    pub fn zooming(&mut self, amount: isize) {
        if amount > 0 {
            self.zoom *= 1.1;
        } else {
            self.zoom /= 1.1;
        }
    }

    fn pixel_to_position(&self, i: u32, j: u32) -> (f64, f64) {
        let (cx, cy) = self.center;

        let x = (i as f64 - self.width as f64 / 2.) * self.zoom + cx;
        let y = (j as f64 - self.height as f64 / 2.) * self.zoom + cy;

        (x, y)
    }

    fn update_pixels(&mut self) {
        for j in 0..self.height {
            for i in 0..self.width {
                let idx = (j*self.width + i) as usize;

                self.pixels[4*idx] = 255 - self.escape_times[idx];
                self.pixels[4*idx+1] = 255 - self.escape_times[idx];
                self.pixels[4*idx+2] = 255 - self.escape_times[idx];
                self.pixels[4*idx+3] = 255;
            }
        }
    }

    fn time_to_diverge(mut state: Complex<f64>) -> u8 {
        // threshold is 2^2, since we compare to the square of the norm
        // as soon as the norm is >= 2 it is sure to diverge
        let threshold = 4.;

        // abort after 255 iterations
        let max_count = 255;

        let c = state;

        let mut ctr = 0;
        while {
            state = state * state + c;
            ctr += 1;

            state.norm_sqr() < threshold && ctr < max_count
        } {}
        ctr
    }
}
