use crate::{
    getter_setter::*,
    DynError,
    file,
};

const TEST: &'static str = "test";

/// returns the profs raw Vec<u8> of data from 
/// le profile
fn prof_raw() -> Result<String, DynError> {
    let path = {
        let var = env::var("XDG_CONFIG_HOME")?;
        format!("{var}/{DIR_NAME}/{TEST}/config")
    };

    let raw = fs::read_to_string(&path)?;

    Ok(raw)
}

fn unlazy_skip(
    num_nexts: usize, 
    skipped_upon: &mut impl Iterator,
) {
    for _ in 0..num_nexts {
        let _ = skipped_upon.next();
    }
}

#[test]
fn full_config_new() -> Result<(), DynError> {
    file::init_dirs(TEST)?;

    dbg!("made it to Self::set");

    RelayCount::set(String::from("120000"), TEST)?;
    RelayMax::set(String::from("140000"), TEST)?;
    MaxTime::set(String::from("50"), TEST)?;
    GpioPin::set(String::from("8"), TEST)?;
    Timezone::set(String::from("America/New_York"), TEST)?;
    Multiplier::set(String::from("2.31"), TEST)?;

    dbg!("made it past the set fns");

    assert_eq!(
        FullConfig::new(TEST)?,
        FullConfig { 
            relay_count: 120000,
            relay_max: 140000,                           
            max_time: 50.0, 
            gpio_pin: 8,                             
            timezone: Tz::America__New_York,                 
            multiplier: 2.31,                         
        },                  
        "Failed FullConfig::new test",
    );

    let path = {
        let var = env::var("XDG_CONFIG_HOME")?;
        format!("{var}/{DIR_NAME}/{TEST}")
    };

    Ok(fs::remove_dir_all(path)?)
}

#[test]
fn relay_count() -> Result<(), DynError> {
    file::init_dirs(TEST)?;

    let test_val = String::from("140000");
    RelayCount::set(test_val.clone(), TEST)?;

    let raw = prof_raw()?;
    let mut lines = raw.lines();

    let tested = RelayCount::get(&mut lines)?;

    assert_eq!(
            test_val.parse::<u64>()?,
            tested.0,
            "Failed RelayCount::get/set test",
    );

    let path = {                                
        let var = env::var("XDG_CONFIG_HOME")?; 
        format!("{var}/{DIR_NAME}/{TEST}")      
    };                                          

    Ok(fs::remove_dir_all(path)?)
}                                              

#[test]
fn relay_max() -> Result<(), DynError> {
    file::init_dirs(TEST)?;

    let test_val = String::from("120000");
    RelayMax::set(test_val.clone(), TEST)?;

    let raw = prof_raw()?;
    let mut lines = raw.lines();
    unlazy_skip(1, &mut lines);
    
    let tested = RelayMax::get(&mut lines)?;

    assert_eq!(
        test_val.parse::<u64>()?,
        tested.0, 
        "Failed RelayMax::get/set test",
    );

    let path = {                                
        let var = env::var("XDG_CONFIG_HOME")?; 
        format!("{var}/{DIR_NAME}/{TEST}")      
    };                                          

    Ok(fs::remove_dir_all(path)?)
}

#[test]
fn max_time() -> Result<(), DynError> {
    file::init_dirs(TEST)?;

    let test_val = String::from("140000");
    MaxTime::set(test_val.clone(), TEST)?;

    let raw = prof_raw()?;
    let mut lines = raw.lines();
    unlazy_skip(2, &mut lines);

    let tested = RelayMax::get(&mut lines)?;

    assert_eq!(
        test_val.parse::<u64>()?,
        tested.0,
        "Failed MaxTime::get/set test",
    );

    let path = {                                
        let var = env::var("XDG_CONFIG_HOME")?; 
        format!("{var}/{DIR_NAME}/{TEST}")      
    };                                          

    Ok(fs::remove_dir_all(path)?)
}

#[test]
fn gpio_pin() -> Result<(), DynError> {
    file::init_dirs(TEST)?;

    let test_val = String::from("32");
    GpioPin::set(test_val.clone(), TEST)?;

    let raw = prof_raw()?;
    let mut lines = raw.lines();
    unlazy_skip(3, &mut lines);

    let tested = GpioPin::get(&mut lines)?;

    assert_eq!(
        test_val.parse::<u8>()?,
        tested.0,
        "Failed GpioPin::get/set test",
    );

    let path = {                                
        let var = env::var("XDG_CONFIG_HOME")?; 
        format!("{var}/{DIR_NAME}/{TEST}")      
    };                                          

    Ok(fs::remove_dir_all(path)?)
}

#[test]
fn timezone_test() -> Result<(), DynError> {
    file::init_dirs(TEST)?;

    let test_val = String::from("America/New_York");
    Timezone::set(test_val.clone(), TEST)?;

    let raw = prof_raw()?;
    let mut lines = raw.lines();
    unlazy_skip(4, &mut lines);

    let tested = Timezone::get(&mut lines)?;

    assert_eq!(
            Tz::from_str_insensitive(&test_val)?,
            tested.0,
            "Failed Timezone::get/set test",
    );

    let path = {                                
        let var = env::var("XDG_CONFIG_HOME")?; 
        format!("{var}/{DIR_NAME}/{TEST}")      
    };                                          

    Ok(fs::remove_dir_all(path)?)
}

#[test]
fn multiplier() -> Result<(), DynError> {
    file::init_dirs(TEST)?;

    let test_val = String::from("3.21");
    Multiplier::set(test_val.clone(), TEST)?;

    let raw = prof_raw()?;
    let mut lines = raw.lines();
    unlazy_skip(5, &mut lines);

    let tested = Multiplier::get(&mut lines)?;

    assert_eq!(
        test_val.parse::<f64>()?,    
        tested.0,
        "Failed Multiplier::get/set test",
    );
    
    let path = {                                
        let var = env::var("XDG_CONFIG_HOME")?; 
        format!("{var}/{DIR_NAME}/{TEST}")      
    };                                          

    Ok(fs::remove_dir_all(path)?)
}
