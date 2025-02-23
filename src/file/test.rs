use crate::{
    DIR_NAME,
    file::*,
    DynError,
    getter_setter::FullConfig,
};

use std::{
    env,
    fs,
};

use chrono_tz::Tz;
use chrono::DateTime;

const PROF: &'static str = "test";

fn cleanup_dirs() -> Result<(), DynError> {
    let path = {
        let var = env::var("XDG_CONFIG_HOME")?;
        format!("{var}/{DIR_NAME}/{PROF}")
    };

    if !fs::exists(&path)? {
        println!("testing dir was already empty / deleted");
        return Ok(());
    };

    Ok(match fs::remove_dir_all(path) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("failed cleanup_dirs() test function");
            Err(e)?
        },
    })
}

#[test]
fn init_dirs_test() -> Result<(), DynError> {
    init_dirs(PROF)?;

    let path = {
        let var = env::var("XDG_CONFIG_HOME")?;
        format!("{var}/{DIR_NAME}")
    };

    assert!(
        fs::exists(&path)?,
        "{DIR_NAME} dir does not exist",
    );

    let prof_path = format!("{path}/{PROF}");

    assert!(
        fs::exists(&prof_path)?,
        "{PROF} dir does not exist"
    );

    Ok(cleanup_dirs()?)
}

#[test]
fn log_test() -> Result<(), DynError> {
    init_dirs(PROF)?;

    let cfg = FullConfig {
        relay_count: 10,
        relay_max: 12, 
        max_time: 14.0,
        gpio_pin: 8, 
        timezone: Tz::from_str_insensitive("America/New_York")?,
        multiplier: 5.0,
    };

    log(&cfg, 12.0, PROF)?;

    let path = {
        let var = env::var("XDG_CONFIG_HOME")?;
        format!("{var}/{DIR_NAME}/{PROF}/log")
    };

    let raw = fs::read_to_string(path)?;
    let (time_vec, log): (Vec<&str>, Vec<&str>) = raw
        .lines()
        .partition(
            |line| line.contains("Timestamp:")    
        )
    ;

    let timestamp = time_vec[0]
        .split(':')
        .skip(1)
        .collect::<Vec<&str>>()
        .join(":")
    ;

    DateTime::parse_from_rfc2822(&timestamp)?;

    // for some reason rust formats f64 numbers 
    // into strings which omit the trailing zero!
    assert_eq!(
        log.join("\n"),
        String::from(
r#"[ENTRY]
Seconds-Actuated:12
GpioPin:8
Multiplier:5
Max-Time:14"#
        ),
    );

    Ok(cleanup_dirs()?)
}

/// this tests the following fns:
/// sysd_entry
/// sysd_del_timer
/// sysd_cleanup_unused_service
/// verify_time
#[test]
fn sysd_fns_test() -> Result<(), DynError> {
    let bind = String::from(PROF);
    let not_now = { 
        let mut raw = Local::now()
            .to_rfc2822()
            .split(' ')
            .collect::<Vec<&str>>()[4]
            .split(':')
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
        ;

        let hr_num = raw[0].parse::<u8>()?;

        if hr_num > 20u8 {
            raw[0] = String::from("02");
        } else {
            raw[0] = (hr_num + 2).to_string();
        }

        raw.join(":")
    };

    println!("{}", not_now);

    sysd_entry(bind.clone(), 15.2, bind.clone(), not_now.clone())?; 

    let (service_path, timer_path) = {
        let base = format!("{SYSTEMD_DIR}/{IDENTIFIER}_{PROF}");
        (
            format!("{base}.service"),
            format!("{base}.timer"),
        )
    };

    let (service_cont, timer_cont) = (
        fs::read_to_string(&service_path)?,
        fs::read_to_string(&timer_path)?,
    );

    assert_eq!(
        format!(
r#"[Timer]
AccuracySec=1s
OnCalendar={not_now}
WakeSystem=true
Unit={IDENTIFIER}_{PROF}.service
[Unit]
Description=Times Actuations via gpio pins for the actuator program! 
[Install]
WantedBy=timers.target"#
        ),
        timer_cont,
    );

    assert_eq!(
        format!(
r#"[Service]
ExecStart=+/usr/local/bin/actuator --exec {PROF} 15.2
[Unit]
Description=Actuates gpio pins for the actautor program!"#
        ),
        service_cont,
    );

    sysd_del_timer(bind)?;
    sysd_cleanup_unused_service()?;

    let result = match fs::exists(&service_path)? {
        true => {
            eprintln!("failed the sysd_cleanup_unused_service fn");
            false
        }

        false => true,
    };

    match fs::exists(&timer_path)? {
        true => {
            eprintln!("failed the sysd_del_timer_function");
            if result == false {
                fs::remove_file(&service_path)?;
                fs::remove_file(&timer_path)?;
            } else {
                fs::remove_file(&timer_path)?;
            }
        }

        false => {
            if result == false {
                fs::remove_file(&service_path)?;
            }
        }
    }

    Ok(())
}
