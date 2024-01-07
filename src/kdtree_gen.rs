pub mod algorithms;
pub mod config;
pub mod utilities;
pub mod coloring;
pub mod wasm_structs;
pub mod map_data;

use crate::wasm_structs::ColorArgs;
use coloring::coloring::Coloring;

use utilities::util::util;

use utilities::file_util::file_util;

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

fn main() {
    let args = Cli::parse();

    let width = args.width;
    let height = args.height;
    let points = args.points;
    let output = args.path;
    let _coloring = args.coloring;

    let kdt = util::generate_kdtree(points, width, height);
    //kdt.pretty_print();
    //let mapdata = MapData::mapdata_from_points(kdt.collect_points(true), width, height);
    let mapdata = kdt.to_mapdata(width, height);

    let mut coloring = Coloring::new(&mapdata, ColorArgs::new(2, 1.0, 0.0));
    let image = coloring.get_image();

    let reimage = imageops::resize(image, width, height, imageops::FilterType::Nearest);
    file_util::save(reimage, &output);

    ()
}
