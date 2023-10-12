pub mod file_util {
    use crate::config;
    //use image::{RgbImage, RgbaImage};
    use image::{ImageBuffer, Pixel, PixelWithColorType, EncodableLayout};
    use std::path::Path;
    use std::ops::Deref;

    pub fn save<P, C>(imagebuf: ImageBuffer<P, C>, name: &String) 
        where P: PixelWithColorType,
        C: Deref<Target = [P::Subpixel]>,
        [<P as Pixel>::Subpixel]: EncodableLayout
    {
        let save_path = Path::new(config::OUTPUT_DIR).join(Path::new(name));
        imagebuf.save(save_path).expect("Couldn't save image");
    }
}
