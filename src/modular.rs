

#derive[(PartialEq)]
pub struct modular {
    value: i32,
    modulus: i32,
}

impl modular {
    pub fn new(value: i32, modulus u32) {
        //TODO: is there a better way to do this when value >> modulus?

        let mut new_value = value;
        while new_value >= modulus {
            new_value = new_value - modulus;    
        }

        while new_value < 0 {
            new_value = new_value + modulus;
        }
        return modular { value: new_value, modulus: modulus }
    }

    pub fn add(&self, value1: &modular) -> modular {
        //TODO: handle error properly
        if self.modulus != value1.modulus {
            return -1;
        }

        let new_value = self.value + value1.value;
        if new_value > self.modulus {
            new_value = new_value 
        }
    } 
}
