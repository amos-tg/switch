#![feature(error_generic_member_access)] 
#![deny(unused_must_use)]
#![warn(clippy::unwrap_used)]

pub mod arg;
pub mod errors;
pub mod getter_setter;
pub mod file;
pub mod gpio;
pub mod help;

use crate::{
    getter_setter::{
        FullConfig,
        Config,
        RelayCount,
    },
    file as fs,
};

use std::{
    env,
    str,
    time::Duration,
};

pub type DynError = Box<dyn std::error::Error>;

pub const DIR_NAME: &'static str = ".Actuators"; 

fn main() -> Result<(), DynError> {
    let mut args = env::args();
    let _ = args.next();

    Ok(while let Some(arg) = args.next() {
        arg::arg_matcher(arg, &mut args)?;
    })
}

impl FullConfig {
    pub fn execute(
        config: Self,
        time_on: f64,
        prof: &str,
    ) -> Result<(), DynError> {
        let filtered = time_on * config.multiplier
            .clamp(0.0, config.max_time)
        ; 

        gpio::actuator(Duration::from_secs_f64(filtered), &config)?;

        RelayCount::set(
            (&config.relay_count + 1).to_string(), 
            prof,
        )?;

        fs::log(&config, filtered, prof)?;

        Ok(())
    }
}
