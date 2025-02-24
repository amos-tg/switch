use crate::{
    errors::{
        InvalidArgError,
        OsStringToUtf8Error,
    },
    DIR_NAME,
    getter_setter::FullConfig,
    DynError,
};

use std::{
    fs,
    env,
    str,
    process::Command,
};

use chrono::Local;

#[cfg(test)]
mod test;

pub fn init_dirs(prof: &str) -> Result<(), DynError> {
    let dir = {
        let var = env::var("XDG_CONFIG_HOME")?;
        format!("{var}/{DIR_NAME}/{prof}")
    };

    Ok(fs::create_dir_all(&dir)?)
}

pub fn log(
    config: &FullConfig,
    time_on: f64,
    id: &str,
) -> Result<(), DynError> {
    let timestamp = Local::now()
        .with_timezone(&config.timezone)
        .to_rfc2822()
    ;

    let log = format!(
r#"[ENTRY]
Seconds-Actuated:{time_on}
Timestamp:{timestamp}
GpioPin:{}
Multiplier:{}
Max-Time:{}"#,
        &config.gpio_pin,
        &config.multiplier,
        &config.max_time,
    );

    let path = {
        let var = env::var("XDG_CONFIG_HOME")?;
        format!("{var}/{DIR_NAME}/{id}/log")
    };

    Ok(if fs::exists(&path)? {
        let contents = fs::read_to_string(&path)?;
        fs::write(&path, format!("{contents}\n{log}"))?;
    } else {
        fs::write(&path, log)?;
    })
}

const SYSTEMD_DIR: &'static str = "/etc/systemd/system";
const IDENTIFIER: &'static str = "atg-actuator";

/// generates a timer file based on the inputed timestamp and 
/// a service file which takes a time actuated for and a profile
/// to look for configuration values within. 
///
/// There are lots of config values I could add to getter_setter 
/// here once the project is more mature. It would make sense 
/// to have defaults, which would be what I am using now because legacy
///
/// arg order = (1:prof-id) (2:f64-format-time-actuated-for) 
///             (3:systemd-timer-file-name-prefix) 
///             (4:time-actuated-at-in-systemd-format-(ex. = 00:00:00))
///
/// arg number 4 parsing is taken care of by systemd-analyze calendar cmd
pub fn sysd_entry(
    prof: String,
    time_on: f64,
    sysd_file_prefix: String,
    time: String,
) -> Result<(), DynError> {
    verify_time(&time)?;
    let timer_path = format!("{SYSTEMD_DIR}/{IDENTIFIER}_{sysd_file_prefix}.timer");
    let timer_cont = format!(
r#"[Timer]
AccuracySec=1s
OnCalendar={time}
WakeSystem=true
Unit={IDENTIFIER}_{prof}.service
[Unit]
Description=Times actuations via systemd for gpio pins for the switch program.
[Install]
WantedBy=timers.target"#
    );

    fs::write(timer_path, timer_cont)?;

    let service_path = format!("{SYSTEMD_DIR}/{IDENTIFIER}_{prof}.service");
    let service_cont = format!(
r#"[Service]
ExecStart=+/usr/local/bin/switch --exec {prof} {time_on}
[Unit]
Description=actuates gpio pins for the switch program"#
    );


    if fs::exists(&service_path)? {
        if fs::read_to_string(&service_path)? == service_cont {
            return Ok(());
        }
    } else {
        fs::write(&service_path, service_cont)?;
    }

    Ok(())
}

fn verify_time(time: &str) -> Result<(), DynError> {
    let out = Command::new("systemd-analyze")
        .args(["calendar", time])
        .output()?
    ;

    Ok(if 
        str::from_utf8(&out.stdout)?.contains("Failed")
    ||
        str::from_utf8(&out.stderr)?.contains("Failed")
    {
        return Err(InvalidArgError::new().into())
    })
}

/// check if there are any timers pointing to a prof.service
/// if there are no files pointing to the prof.service delete
/// the service
///
/// delete the inputed timer file based on the prefix-given
///
/// 
/// arg order = (1:systemd-timer-file-name-prefix)
pub fn sysd_del_timer(
    sysd_file_prefix: String,
) -> Result<(), DynError> {
    let timer_path = format!("{SYSTEMD_DIR}/{IDENTIFIER}_{sysd_file_prefix}.timer");
    fs::remove_file(&timer_path)?;
    Ok(())
} 

// cloning the ReadDir iterator is a pain in the ass
// thanks rust. reading the dir twice has limited benefits 
// and acceptable performance costs.
// 
/// check the timer files to see if any of them point at 
/// the profile.service file. If the service file is unused
/// delete the file.
pub fn sysd_cleanup_unused_service() -> Result<(), DynError> {
    let mut store: Vec<String> = Vec::new();
    let discrim = format!("{IDENTIFIER}_");

    let sysd_read_dir = fs::read_dir(SYSTEMD_DIR)?;

    for file in sysd_read_dir { 
        let file_os_str = file?.path(); 
        let file_path = file_os_str
            .to_str()                               
            .ok_or(OsStringToUtf8Error::new())?                   
        ;

        if 
            file_path.starts_with(IDENTIFIER)        
        &&                                          
            file_path.ends_with(".timer")         
        {                                           
            let contents = fs::read_to_string(&file_path)?;

            let prof = contents
                .split(&discrim)
                .collect::<Vec<&str>>()[1]
                .split(".")
                .collect::<Vec<&str>>()[0]
                .to_string()
            ;

            if !store.contains(&prof) {
                store.push(
                    prof
                );
            }
        }                                               
    }

    let sysd_read_dir = fs::read_dir(SYSTEMD_DIR)?;

    Ok(for file in sysd_read_dir {
        let file_os_str = file?.path();
        let file_path = file_os_str
            .to_str()
            .ok_or(OsStringToUtf8Error::new())?
        ;

        if 
            file_path.contains(IDENTIFIER) 
        &&
            file_path.ends_with(".service") 
        { 
            let prof = file_path
               .split(&discrim)
               .collect::<Vec<&str>>()[1]
               .split(".")
               .collect::<Vec<&str>>()[0]
               .to_string() 
            ;

            if !store.contains(&prof) {
                fs::remove_file(&file_path)?;
            }
        }
    })
}                                                    
