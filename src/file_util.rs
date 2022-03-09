//pub mod file_util {
use std::env as env;
use std::path::PathBuf;

pub fn get_cwd() {
    println!("{}", env::current_dir().expect("Couldn't get current directory").display());
    let cur_path = env::current_dir().expect("Couldn't get current directory");
    PathBuf::new().push(cur_path)
}
//}
