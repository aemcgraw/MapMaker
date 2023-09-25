use image::Rgb;

pub struct ColorUtil {
    pub stepsize : u8,
    pub vibrance : u8,
    pub cur_color : Vec<u8>,

    pivot : u8,
    sample : u8,
}

impl ColorUtil {
    pub fn new(vibrance: u8) -> ColorUtil {
        let new_vibrance = if vibrance > 256 { 256 } else { vibrance };
        return ColorUtil { stepsize : 64, vibrance : new_vibrance, cur_color : vec![0, 256, 128], pivot : 0, sample : 2 }
    }

    pub fn next(&self) -> Rgb {
        if pivot > 5 {

        }

        if 256 < ( self.stepsize + self.cur_color[sample] ) {
            
        } else {
            self.cur_color[sample] = self.cur_color[sample] + self.stepsize;
            return Rgb(self.cur_color)
        }
    }
}
