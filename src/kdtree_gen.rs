pub mod algorithms;
pub mod map_data;
pub mod config;
pub mod util;
pub mod coloring;

use algorithms::ImageAlg;
use algorithms::diamond_square::DiamondSquare;
use algorithms::diamond_square_borderless::DiamondSquareBorderless;

use coloring::coloring::Coloring;

use map_data::MapData;
use util::util::Util;
use util::file_util::FileUtil;

use clap::Parser;
use image::imageops;

#[derive(Parser)]
struct Cli {
    width: u32,
    height: u32,

    #[clap(short = 'x', long, 
        default_value = "10", 
        help = "Number of nodes in the KDTree\n"
    )]
    points: u32,

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

    #[clap(short = 'v',
        long = "verbose",
        help = "Print verbose output"
    )]
    verbose: bool
}

//fn initialize_algorithm(width: u32, height: u32, algorithm: &str) -> Box<dyn ImageAlg> {
//    match algorithm {
//        "DiamondSquare" | "ds" => Box::new(DiamondSquare::new(width, height)),
//        "DiamondSquareBorderless" | "dsb" => Box::new(DiamondSquareBorderless::new(width, height)),
//        _ => {
//            println!("Provided algorithm {} not recognized. Defaulting to DiamondSquare", algorithm);
//            Box::new(DiamondSquare::new(width, height))
//        }
//    }
//}

fn main() {
    let args = Cli::parse();

    let width = args.width;
    let height = args.height;
    let points = args.points;
    let output = args.path;
    let coloring = args.coloring;
    //let verbose = args.verbose;

    let kdt = Util::generate_kdtree(points, width, height);
    let mapdata = MapData::mapdata_from_points(kdt.collect_points_below(), width, height);

    let coloring = Coloring::new(&mapdata);
    let image = coloring.data_to_binary();

    let reimage = imageops::resize(&image, width, height, imageops::FilterType::Nearest);
    FileUtil::save(reimage, &output);

    ()
}
