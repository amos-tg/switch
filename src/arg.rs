use crate::{
    errors::{
        OsStringToUtf8Error,
        MissingArgError,
        InvalidArgError,
    },
    getter_setter::{
        FullConfig,
        Config,
        RelayCount,
        RelayMax,
        MaxTime,
        GpioPin, 
        Timezone,
        Multiplier,
    },
    DIR_NAME,
    file,
    DynError,
    help::HELP,
};

use std::{
    env,
    env::Args,
    fs,
};

use chrono_tz::Tz;

pub fn arg_matcher(
    arg: String,
    args: &mut Args
) -> Result<(), DynError> {
    Ok(match arg.as_str() {
        //////////////////////////
        // Informational 
        "--help" => {
            println!("{HELP}");
        }

        "--show" => {
            show()?;
        }
        //////////////////////////
        // Execution
        "--exec" => {
            exec(args)?;   
        }
        //////////////////////////
        // Configuration (in order)
        "--set-num-actuations" => {
            set_actuations(args)?; 
        }

        "--set-max-actuations" => {
            set_max_actuations(args)?;
        }

        "--set-max-time" => {
            set_max_time(args)?;
        }

        "--set-gpio" => {
            set_gpio(args)?;
        }
        
        "--set-timezone" => {
            set_timezone(args)?;
        }

        "--set-multiplier" => { 
            set_multiplier(args)?;
        }
        //////////////////////////
        // Systemd Actuation
        "--entry" => {
            sysd_entry(args)?;
        }

        "--del-entry" => {
            sysd_del_entry(args)?;
        }

        _ => return Err(InvalidArgError::new().into()),
    })
}

fn show() -> Result<(), DynError> {
    let dir_iter = {
        let var = env::var("XDG_CONFIG_HOME")?;
        fs::read_dir(format!("{var}/{DIR_NAME}").as_str())?
    };
    
    Ok(for res_prof in dir_iter {
        let dentry = res_prof?;
        let dpath_raw = dentry.path();
        let dprof_raw = dentry.file_name();

        let path = dpath_raw
            .to_str()
            .ok_or(OsStringToUtf8Error::new())?
        ;

        let prof = dprof_raw
            .to_str()
            .ok_or(OsStringToUtf8Error::new())?
        ;

        let config = fs::read_to_string(format!("{path}/config"))?;

        println!("{}", format!("Profile = {prof}:\n{config}\n\n"));
    })
}

/// arg order = (1:profile-id) (2:f64-format-time-actuated-for)
fn exec(args: &mut Args) -> Result<(), DynError> {
    let prof = args                               
        .next()                                
        .ok_or(MissingArgError::new())?        
    ;                                          

    file::init_dirs(&prof.as_str())?;         

    let time_on = args 
        .next()                                
        .ok_or(MissingArgError::new())?        
        .parse::<f64>()?                       
    ;

    let config = FullConfig::new(&prof)?;      
    Ok(FullConfig::execute(config, time_on, &prof)?)
}

/// arg order = (1:profile-id) (2:u64-format-num-actuations-set)
fn set_actuations(args: &mut Args) -> Result<(), DynError> {
    let prof = args
        .next()
        .ok_or(MissingArgError::new())?
    ;

    file::init_dirs(&prof)?;

    let set = args
        .next()
        .ok_or(MissingArgError::new())?
    ;

    let _ = &set.parse::<u64>()?;

    Ok(RelayCount::set(set, &prof)?)
}

/// arg order = (1:profile-id) (2:u64-format-max-num-actuations)
fn set_max_actuations(args: &mut Args) -> Result<(), DynError> {
    let prof = args 
        .next()
        .ok_or(MissingArgError::new())?
    ;

    file::init_dirs(&prof)?;

    let max = args
        .next()
        .ok_or(MissingArgError::new())?
    ;

    let _ = &max.parse::<u64>()?;

    Ok(RelayMax::set(max, &prof)?)
}

/// arg order =  (1:profile-id) (2:f64-format-max-time-actuated) 
fn set_max_time(args: &mut Args) -> Result<(), DynError> {
    let prof = args 
        .next()
        .ok_or(MissingArgError::new())?
    ;

    file::init_dirs(&prof)?;

    let max = args
        .next()
        .ok_or(MissingArgError::new())?
    ;              

    let _ = &max.parse::<f64>()?;

    Ok(MaxTime::set(max, &prof)?)
}

/// arg order = (1:prof-id) (2:u8-format-gpio-pin)
fn set_gpio(args: &mut Args) -> Result<(), DynError> {
    let prof = args 
        .next()
        .ok_or(MissingArgError::new())?
    ;

    file::init_dirs(&prof)?;

    let pin = args
        .next()
        .ok_or(MissingArgError::new())?
    ;

    let _ = &pin.parse::<u8>()?;

    Ok(GpioPin::set(pin, &prof)?)
}

/// arg order = (1:prof-id) (2:chrono_tz::Tz-format-timezone-string)
fn set_timezone(args: &mut Args) -> Result<(), DynError> {
    let prof = args 
        .next()
        .ok_or(MissingArgError::new())?
    ;

    file::init_dirs(&prof)?;

    let tz = args
        .next()
        .ok_or(MissingArgError::new())?
    ;

    let _ = Tz::from_str_insensitive(&tz)?;

    Ok(Timezone::set(tz, &prof)?)
}

/// arg order = (1:prof-id) (2:f64-format-time-actuated-multiplier)
fn set_multiplier(args: &mut Args) -> Result<(), DynError> {
    let prof = args 
        .next()
        .ok_or(MissingArgError::new())?
    ;

    file::init_dirs(&prof)?;

    let mult = args
        .next()
        .ok_or(MissingArgError::new())?
    ;

    let _ = &mult.parse::<f64>()?;

    Ok(Multiplier::set(mult, &prof)?)
}

/// arg order = (1:prof-id) (2:f64-format-time-actuated-for) 
///             (3:systemd-timer-file-name-prefix) 
///             (4:time-actuated-at)
fn sysd_entry(args: &mut Args) -> Result<(), DynError> {
    let prof = args 
        .next()
        .ok_or(MissingArgError::new())?
    ;

    file::init_dirs(&prof)?;

    let time_on = args
        .next()
        .ok_or(MissingArgError::new())?
        .parse::<f64>()?
    ;

    let sysd_prefix = args
        .next()
        .ok_or(MissingArgError::new())?
    ;

    let time = args
        .next()
        .ok_or(MissingArgError::new())?
    ;

    Ok(file::sysd_entry(prof, time_on, sysd_prefix, time)?)
}

/// arg order = (1:systemd-file-name-prefix)
fn sysd_del_entry(args: &mut Args) -> Result<(), DynError> {
    let sysd_prefix = args
        .next()
        .ok_or(MissingArgError::new())?
    ; 

    file::sysd_del_timer(sysd_prefix)?;
    Ok(file::sysd_cleanup_unused_service()?)
}
