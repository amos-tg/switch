use crate::{
    DynError,
    getter_setter::FullConfig,
};

use std::{
    thread,
    time,
};

use rppal::gpio::Gpio;

/// converts num ozs from config to num of seconds to remain on
pub fn amount_converter(amount: usize, multiplier: f64) -> time::Duration {
    let bind: f64 = amount as f64 * multiplier;
    time::Duration::from_secs(bind as u64)
}

pub fn actuator(
    duration: time::Duration,
    config: &FullConfig,
) -> Result<(), DynError> {
    let gpio = Gpio::new()?;
    let mut pin = gpio.get(config.gpio_pin)?.into_output();

    pin.set_high();

    thread::sleep(duration);

    Ok(pin.set_low())
}
