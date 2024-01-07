use wasm_bindgen::prelude::*;

use rand::Rng;

//Allows logging to the browser console with the console_log! macro
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct MapArgs {
    pub width: u32,
    pub height: u32,
    pub chaos: f64,
    pub damping: f64,
    pub blocksize: u32,
    pub seed: u32,
}

#[wasm_bindgen]
impl MapArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, chaos: f64, damping: f64, blocksize: u32, seed: u32) -> MapArgs {
        MapArgs { width, height, chaos, damping, blocksize, seed }
    }

    pub fn get_width(&self) -> u32 { return self.width }
    pub fn get_height(&self) -> u32 { return self.height }
    pub fn get_chaos(&self) -> f64 { return self.chaos }
    pub fn get_damping(&self) -> f64 { return self.damping }
    pub fn get_blocksize(&self) -> u32 { return self.blocksize }
    pub fn get_seed(&self) -> u64 {
        if self.seed == 0 {
            let mut rng = rand::thread_rng();
            return rng.gen_range(0..u64::MAX);
        } else {
            return self.seed.into();
        }   
    }
}

#[wasm_bindgen]
pub struct ColorArgs {
    pub coloring: u32,
    pub darklevel: f64,
    pub waterlevel: f64,
}

#[wasm_bindgen]
impl ColorArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(coloring: u32, darklevel: f64, waterlevel: f64) -> ColorArgs {
        ColorArgs { coloring, darklevel, waterlevel }
    }

    pub fn get_coloring(&self) -> u32 { return self.coloring }
    pub fn get_darklevel(&self) -> f64 { return self.darklevel }
    pub fn get_waterlevel(&self) -> f64 { return self.waterlevel }
}
