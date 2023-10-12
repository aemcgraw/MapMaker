pub mod algorithms;
pub mod map_data;
pub mod config;
pub mod utilities;
pub mod coloring;

use algorithms::ImageAlg;
use algorithms::diamond_square::DiamondSquare;
use algorithms::diamond_square_borderless::DiamondSquareBorderless;

use coloring::coloring::Coloring;

use clap::Parser;
use image::imageops;

#[derive(Parser, Debug)]
struct Cli {
    width: u32,
    height: u32,

    #[clap(short, long, 
        default_value = "DiamondSquare", 
        help = "Terrain generation algorithm to use:\n Options: \n\tDiamondSquare,\n\tDiamondSquareBorderless\n"
    )]
    algorithm: String,

    #[clap(short, long, 
        default_value = "bluegreen",
        help = "Process for coloring the resulting terrain:\n Options: \n\trainbow,\n\tbluegreen\n"
    )]
    coloring: String,

    #[clap(short, long, 
        default_value = "map.png",
        help = "Filename to save image to"
    )]
    path: String,

    #[clap(short = 'x', 
        long = "chaos",
        default_value = "0.5",
        help = "Maximum value for random variance at each step."
    )]
    chaos: f64,

    #[clap(short = 's',
        long = "seed",
        default_value = "0",
        help = "Value to seed the map generation."
    )]
    seed: u64,

    #[clap(short = 'v',
        long = "verbose",
        help = "Print verbose output"
    )]
    verbose: bool
}

fn initialize_algorithm(width: u32, height: u32, seed: u64, algorithm: &str) -> Box<dyn ImageAlg> {
    match algorithm {
        "DiamondSquare" | "ds" => Box::new(DiamondSquare::new(width, height, seed)),
        "DiamondSquareBorderless" | "dsb" => Box::new(DiamondSquareBorderless::new(width, height, seed)),
        _ => {
            println!("Provided algorithm {} not recognized. Defaulting to DiamondSquare", algorithm);
            Box::new(DiamondSquare::new(width, height, seed))
        }
    }
}

fn main() {
    let args = Cli::parse();

    let width = args.width;
    let height = args.height;
    let output = args.path;
    let coloring = args.coloring;
    let algo = args.algorithm;
    let chaos = args.chaos;
    let seed = args.seed;
    //let verbose = args.verbose;

    let mut dx = initialize_algorithm(width, height, seed, &algo);

    dx.run(chaos, 0.8, width);
    let mapdata = dx.get_data();

    let _maxima = mapdata.get_local_maxima();
    //println!("{:?}", maxima);

    let mut coloring = Coloring::new(mapdata, &algo);
    //coloring.data_to_blue_green(0.9);
    let image = coloring.get_image();

    let reimage = imageops::resize(image, width, height, imageops::FilterType::Nearest);
    dx.save(reimage, &output);

    ()
}

/*
fn test() {
    let mut dx = initialize_algorithm(128, 128, 1, "DiamondSquare");
    dx.run(chaos, 0.8, width);
}
*/
