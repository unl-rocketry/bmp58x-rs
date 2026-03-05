const BMP5_CHIP_ID: u8 = 0x50;

enum Address {
    Primary = 0x46,
    Secondary = 0x47
}

pub struct BMP5<I: I2c> {
    address: Address,
    i2c: I
}


impl<I, E> BMP5<I>{
    pub fn new(
        mut i2c: I,

    ) -> Self {
        
    }
}

pub struct Measurement{
    temperature: f32,
    pressure: Option<f32>,
}