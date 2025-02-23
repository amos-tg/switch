use crate::{
    DIR_NAME,
    errors::MalformedConfigError,
    DynError,
};

use std::{
    str::Lines,
    env,
    fs,
    str,
};

#[cfg(test)]
mod test;

use chrono_tz::Tz;

#[derive(Debug, PartialEq)]
pub struct FullConfig { 
    pub relay_count: u64,
    pub relay_max: u64,                           
    pub max_time: f64,                          
    pub gpio_pin: u8,                             
    pub timezone: chrono_tz::Tz,                 
    pub multiplier: f64,                         
}

impl FullConfig {
    pub fn new(prof_id: &str) -> Result<Self, DynError> {
        let path = {
            let var = env::var("XDG_CONFIG_HOME")?;
            format!("{var}/{DIR_NAME}/{prof_id}/config")
        };

        let config_contents = fs::read_to_string(&path)?;

        if config_contents.is_empty() {
            return Err(MalformedConfigError::new().into()); 
        }

        let mut cont_lines = config_contents.lines();

        Ok(FullConfig {
            relay_count: RelayCount::get(&mut cont_lines)?.0,
            relay_max: RelayMax::get(&mut cont_lines)?.0,
            max_time: MaxTime::get(&mut cont_lines)?.0,
            gpio_pin: GpioPin::get(&mut cont_lines)?.0,
            timezone: Timezone::get(&mut cont_lines)?.0,
            multiplier: Multiplier::get(&mut cont_lines)?.0,
        })
    }
}

pub trait Config {
    const RAW: &'static str = 
r#"RelayCount:
RelayMax:
MaxTime:
GpioPin:
Timezone:
Multiplier:"# 
    ;

    const LINE: usize;

    fn verifier(set: &String) -> Result<(), DynError>;

    fn set(
        set: String,
        id: &str,
    ) -> Result<(), DynError> {
        Self::verifier(&set)?;

        let line_number = Self::LINE;
        let path = {
            let var = env::var("XDG_CONFIG_HOME")?;
            format!("/{var}/{DIR_NAME}/{id}/config")
        };

        if !fs::exists(&path)? {
            let mut lines = Self::RAW
                .lines()
                .collect::<Vec<&str>>()
            ;

            let split = lines[line_number]
                .split(':')
                .collect::<Vec<&str>>()
            ; 

            let bind = format!("{}: {}", split[0], set);
            lines[line_number] = bind.as_str();

            return Ok(fs::write(
                    &path,
                    lines.join("\n"))?,
            )
        } else {
            let config = fs::read_to_string(&path)?;

            let mut lines = config
                .lines()
                .collect::<Vec<&str>>()
            ;

            let split = lines[line_number]
                .split(':')
                .collect::<Vec<&str>>()
            ; 

            let bind = format!("{}: {}", split[0], set);
            lines[line_number] = bind.as_str();

            Ok(fs::write(&path, lines.join("\n"))?) 
        }
    }


    fn parser(got: &str) -> Result<Self, DynError> where Self: Sized;

    /// this function needs to be implemented in the get_all fn 
    /// in order of LINE's otherwise it will not work.
    #[inline]
    fn get(config_lines: &mut Lines) -> Result<Self, DynError> 
    where 
        Self: Sized,
    {
        let raw = config_lines
            .next()
            .ok_or(MalformedConfigError::new())?
        ;

        let val = {
            let split = raw
                .split(' ') 
                .collect::<Vec<&str>>()
            ; 

            if split.len() == 2 {
                Ok(split[1])
            } else {
                Err(MalformedConfigError::new())
            }
        }?;

        Ok(Self::parser(val)?)
    }
}

pub struct RelayCount(u64);

impl Config for RelayCount {
    const LINE: usize = 0;

    fn parser(got: &str) -> Result<Self, DynError> {
        Ok(Self(got.parse::<u64>()?))
    }
    
    fn verifier(set: &String) -> Result<(), DynError> {
        set.parse::<u64>()?;
        Ok(())
    }
}

pub struct RelayMax(u64);           

impl Config for RelayMax {
    const LINE: usize = 1;

    fn parser(got: &str) -> Result<Self, DynError> {
        Ok(Self(got.parse::<u64>()?))
    }

    fn verifier(set: &String) -> Result<(), DynError> {
        set.parse::<u64>()?;
        Ok(())
    }
}

pub struct MaxTime(f64);          

impl Config for MaxTime {
    const LINE: usize = 2;

    fn parser(got: &str) -> Result<Self, DynError> {
        Ok(Self(got.parse::<f64>()?))
    }

    fn verifier(set: &String) -> Result<(), DynError> {
        set.parse::<f64>()?;
        Ok(())
    }
}

pub struct GpioPin(u8);             

impl Config for GpioPin {
    const LINE: usize = 3;

    fn parser(got: &str) -> Result<Self, DynError> {
        Ok(Self(got.parse::<u8>()?))
    }

    fn verifier(set: &String) -> Result<(), DynError> {
        set.parse::<u8>()?;
        Ok(())
    }
}

pub struct Timezone(chrono_tz::Tz); 

impl Config for Timezone {
    const LINE: usize = 4;

    fn parser(got: &str) -> Result<Self, DynError> {
        Ok(Self(Tz::from_str_insensitive(got)?))
    }

    fn verifier(set: &String) -> Result<(), DynError> {
        Tz::from_str_insensitive(set)?;
        Ok(())
    }   
}

pub struct Multiplier(f64);         

impl Config for Multiplier {
    const LINE: usize = 5;

    fn parser(got: &str) -> Result<Self, DynError> {
        Ok(Self(got.parse::<f64>()?))
    }

    fn verifier(set: &String) -> Result<(), DynError> {
        set.parse::<f64>()?;
        Ok(())
    }
}
