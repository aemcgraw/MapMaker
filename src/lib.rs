pub mod algorithms;
pub mod map_data;
pub mod config;
pub mod util;
pub mod coloring;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

use algorithms::Run;
use algorithms::ImageAlg;
use algorithms::diamond_square::DiamondSquare;
use algorithms::GetData;

use coloring::coloring::Coloring;

//use image::imageops;

//fn initialize_algorithm(width: u32, height: u32, algorithm: &str) -> Box<dyn ImageAlg> {
//    match algorithm {
//        "DiamondSquare" | "ds" => Box::new(DiamondSquare::new(width, height)),
        //"DiamondSquareBorderless" | "dsb" => Box::new(DiamondSquareBorderless::new(width, height)),
//        _ => {
//            println!("Provided algorithm {} not recognized. Defaulting to DiamondSquare", algorithm);
//            Box::new(DiamondSquare::new(width, height))
//        }
//    }
//}

#[wasm_bindgen]
pub fn makeimage(ctx: &CanvasRenderingContext2d, width: u32, height: u32, algorithm: &str, chaos: f64) -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    //Todo: Disallow very large sizes

    //let algo = "DiamondSquare";
    let chaos: f64 = 0.5;

    let mut dx = DiamondSquare::new(width, height);
    dx.run(chaos);

    let mapdata = dx.get_data();
    let coloring = Coloring::new(mapdata);
    let mut imagevec = coloring.data_to_blue_green_vec(0.7);

    let imagedata = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut imagevec), dx.dim, dx.dim)?;
    ctx.put_image_data(&imagedata, 0.0, 0.0)
}         
