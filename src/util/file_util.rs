pub mod FileUtil {
    use crate::config;
    use image::RgbImage;
    use std::path::Path;

    pub fn save(image: RgbImage, name: &String) {
        let save_path = Path::new(config::OUTPUT_DIR).join(Path::new(name));
        image.save(save_path).expect("Couldn't save image");
    }
}
