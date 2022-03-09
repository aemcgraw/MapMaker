pub mod algorithms;
pub mod map_data;
pub mod config;
pub mod util;

use algorithms::ImageAlg;
use algorithms::diamond_square::DiamondSquare;
use algorithms::diamond_square_borderless::DiamondSquareBorderless;

use clap::Parser;
use image::imageops;

#[derive(Parser)]
struct Cli {
    width: u32,
    height: u32,

    #[clap(short, long, 
        default_value = "DiamondSquare", 
        help = "Terrain generation algorithm to use:\n Options: \n\tDiamondSquare,\n\tDiamondSquareBorderless\n"
    )]
    algorithm: String,

    #[clap(short, long, 
        default_value = "rainbow",
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

    #[clap(short = 'v',
        long = "verbose",
        help = "Print verbose output"
    )]
    verbose: bool
}

fn initialize_algorithm(width: u32, height: u32, algorithm: &str) -> Box<dyn ImageAlg> {
    match algorithm {
        "DiamondSquare" | "ds" => Box::new(DiamondSquare::new(width, height)),
        "DiamondSquareBorderless" | "dsb" => Box::new(DiamondSquareBorderless::new(width, height)),
        _ => {
            println!("Provided algorithm {} not recognized. Defaulting to DiamondSquare", algorithm);
            Box::new(DiamondSquare::new(width, height))
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
    //let verbose = args.verbose;

    let mut dx = initialize_algorithm(width, height, &algo);

    dx.run(chaos);

    let image = dx.to_image(&coloring);
    let reimage = imageops::resize(&image, width, height, imageops::FilterType::Nearest);
    dx.save(reimage, &output);

    ()
}
