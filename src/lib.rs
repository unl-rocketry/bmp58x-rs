use embedded_hal_async::delay::DelayNs;

const BMP5_CHIP_ID: u8 = 0x50;

enum Address {
    Primary = 0x46,
    Secondary = 0x47
}

pub struct BMP5<I: I2c> {
    address: Address,
    i2c: I

}
pub enum Error<E> {
    I2c(E),
    WrongChip(u8),
    Fatal,
    Command,
    Configuration,
}


impl<I, E> BMP5<I>
where
    I: I2c<Error = E>,
{
    pub async fn try_new<D: DelayNs>(
    mut i2c: I,
    address: Address,
    mut delay: D,
    config: &Configuration,
    ) -> Result<Self, Error<E>>
        
}


pub struct Measurement{
    temperature: f32,
    pressure: Option<f32>,
}