// the enum errors do not need an extra backtrace, they need the #[backtrace] thiserror attribute
// above the source field so that the backtrace is forwarded to the source error and not duplicated resulting in TWO BACKTRACES. 
//
// another option is the #[error(transparent)] attribute which forwards the source display and
// source methods straight through to an underlying errror without adding an additional message.

use thiserror::Error;

use std::{
    str,
    backtrace::Backtrace,
};

// ERROR TEMPLATE
//#[derive(Debug, Error)]
//#[error("{msg}\nBacktrace:\n{backtrace}")]
//pub struct  {
//    msg: &'static str,
//    backtrace: Backtrace,
//}
//
//impl  {
//    const MSG: &'static str = "";
//
//    pub fn new() -> Self {
//        Self { msg: Self::MSG, backtrace: Backtrace::capture() }
//    }
//}

#[derive(Debug, Error)]                                          
#[error("{msg}\nBacktrace:\n{backtrace}")]                       
pub struct OsStringToUtf8Error { 
    msg: &'static str,                                           
    backtrace: Backtrace,                                        
}                                                                

impl OsStringToUtf8Error { 
    const MSG: &'static str = "OsString did was not valid Utf8"; 
                                                                 
    pub fn new() -> Self {                                       
        Self { msg: Self::MSG, backtrace: Backtrace::capture() } 
    }                                                            
}                                                                

//#[derive(Debug, Error)]
//#[error("{msg}\nBacktrace:\n{backtrace}")]
//pub struct ValidityError {
//    msg: String,
//    backtrace: Backtrace,
//}
//
//impl ValidityError {
//    pub fn new(custom: &str) -> Self {
//        Self { 
//            msg: format!("ValidityError! failed validity check in config setter: {custom}"),
//            backtrace: Backtrace::capture(),
//        }
//    }
//}

#[derive(Debug, Error)]                                          
#[error("{msg}\nBacktrace:\n{backtrace}")]                       
pub struct MalformedConfigError {                                                    
    msg: &'static str,                                           
    backtrace: Backtrace,                                        
}                                                                

impl MalformedConfigError { 
    const MSG: &'static str = "The config file is malformed"; 
                                                                 
    pub fn new() -> Self { 
        Self { 
            msg: Self::MSG,
            backtrace: Backtrace::capture(),
        } 
    }                                                            
}                                                                

#[derive(Debug, Error)]
#[error("{msg}\nBacktrace:\n{backtrace}")]
pub struct MalformedLogError {
    msg: &'static str,
    backtrace: Backtrace,
}

impl MalformedLogError {
    const MSG: &'static str = "the log is malformed; it is missing information or incorrectly formatted. If you're a user, copy your log files contents to a safe place, check your total_relay_actuations, and then use the appropriate cmdline arg to add the old relay count to your NEW log file AFTER you delete the old one from the appropriate location. RELAY_ACTUATION_COUNT IS USED TO GUARENTEE YOUR SAFETY; YOUR RELAY WILL NOT LIVE FOREVER.";

    pub fn new() -> MalformedLogError {
        Self { msg: Self::MSG, backtrace: Backtrace::capture() }
    }
}

//#[derive(Debug, Error)]
//#[error("{msg}\nBacktrace:\n{backtrace}")]
//pub struct MaxOutputError {
//    msg: &'static str,
//    backtrace: Backtrace,
//}
//
//impl MaxOutputError {
//    const MSG: &'static str = "MaxOutputError: your max output limiter isn't set to anything, prevent eventual splashdown and set a limit (yes.. it's mandatory)!";
//
//    pub fn new() -> MaxOutputError {
//        Self { msg: Self::MSG, backtrace: Backtrace::capture() }
//    }
//}

//#[derive(Debug, Error)]
//#[error("{msg}\nBacktrace:\n{backtrace}")] 
//pub struct EmptyGpioError {
//    msg: &'static str, 
//    backtrace: Backtrace,
//}
//
//impl EmptyGpioError {
//    const MSG: &'static str = "EmptyGpio Error : the ~/.WaterPump/state-files/gpio.txt file does not contain a pin number to actuate";
//
//    pub fn new() -> EmptyGpioError {
//        Self { msg: Self::MSG, backtrace: Backtrace::capture() }
//    }
//}

//#[derive(Debug, Error)]
//#[error("{msg}\nBacktrace:\n{backtrace}")] 
//pub struct TimeFormatError {
//    msg: &'static str,
//    backtrace: Backtrace, 
//}
//
//impl TimeFormatError {
//    const MSG: &'static str = "TimeFormat Error : there is a time entry with an incorrect format"; 
//    
//    pub fn new() -> TimeFormatError {
//        Self { msg: Self::MSG, backtrace: Backtrace::capture() }
//    }
//}

//#[derive(Debug, Error)]
//#[error("{msg}\nBacktrace:\n{backtrace}")] 
//pub struct AmountFormatError {
//    msg: &'static str,
//    backtrace: Backtrace,
//}
//
//impl AmountFormatError {
//    const MSG: &'static str = "AmountFormat Error : an amount entry is formatted incorrectly!";
//
//    pub fn new() -> AmountFormatError {
//        Self { msg: Self::MSG, backtrace: Backtrace::capture() }
//    }
//} 

//#[derive(Debug, Error)]
//#[error("{msg}\nBacktrace:\n{backtrace}")] 
//pub struct TimeBuilderError {
//    msg: &'static str,
//    backtrace: Backtrace,
//}
//
//impl TimeBuilderError {
//    const MSG: &'static str = "TimeBuilder Error : a time operation from chrono module failed to resolve properly";
//
//    pub fn new() -> TimeBuilderError {
//        Self { msg: Self::MSG, backtrace: Backtrace::capture() }
//    }
//}

//#[derive(Debug, Error)]
//#[error("{msg}\nBacktrace:\n{backtrace}")] 
//pub struct ConfigFormatError { 
//    msg: &'static str,
//    backtrace: Backtrace,
//}
//
//impl ConfigFormatError {
//    const MSG: &'static str = "ConfigFormat Error : the configuration file contains formatting errors";
//
//    pub fn new() -> ConfigFormatError {
//        Self { msg: Self::MSG, backtrace: Backtrace::capture() }
//    }
//}

//#[derive(Debug, Error)]
//#[error("{msg}\nBacktrace:\n{backtrace}")] 
//pub struct EmptyConfigError {
//    msg: &'static str,
//    backtrace: Backtrace,
//}
//
//impl EmptyConfigError {
//    const MSG: &'static str = "EmptyConfig Error : the configuration file is empty."; 
//
//    pub fn new() -> EmptyConfigError {
//        Self { msg: Self::MSG, backtrace: Backtrace::capture() }
//    }
//} 

#[derive(Debug, Error)]
#[error("{msg}\nBacktrace:\n{backtrace}")] 
pub struct InvalidArgError {
    msg: &'static str,
    backtrace: Backtrace,
}

impl InvalidArgError {
    const MSG: &'static str = "InvalidArg Error : your inputed cmdline-argument is incorrect!";

    pub fn new() -> Self {
        Self { msg: Self::MSG, backtrace: Backtrace::capture() }
    }
} 

//#[derive(Debug, Error)]
//#[error("{msg}\nBacktrace:\n{backtrace}")] 
//pub struct EntryFormatError { 
//    msg: &'static str,
//    backtrace: Backtrace,
//}
//
//impl EntryFormatError {
//    const MSG: &'static str = "EntryFormat Error : an entry format error occurred check you entry formats.";
//
//    pub fn new() -> Self {
//        Self { msg: Self::MSG, backtrace: Backtrace::capture() }
//    }
//}

//#[derive(Debug, Error)]
//#[error("{msg}\nBacktrace:\n{backtrace}")] 
//pub struct MultiplierFormatError {
//    msg: &'static str,
//    backtrace: Backtrace,
//}
//
//impl MultiplierFormatError {
//    const MSG: &'static str = "MultiplierFormat Error : the multiplier format is incorrect!";
//
//    pub fn new() -> MultiplierFormatError {
//        Self { msg: Self::MSG, backtrace: Backtrace::capture() }
//    }
//}

#[derive(Debug, Error)]
#[error("{msg}\nBacktrace:\n{backtrace}")] 
pub struct MissingArgError {
    msg: &'static str,
    backtrace: Backtrace,
}

impl MissingArgError {
    const MSG: &'static str = "MissingArg Error : the cmdline-option is missing an argument!";

    pub fn new() -> MissingArgError {
        Self { msg: Self::MSG, backtrace: Backtrace::capture() }
    }
}

//#[derive(Debug, Error)]
//#[error("{msg}\nBacktrace:\n{backtrace}")] 
//pub struct EmptyMultiplierError {
//    msg: &'static str,
//    backtrace: Backtrace,
//}
//
//impl EmptyMultiplierError {
//    const MSG: &'static str = "EmptyMultiplier Error : the store file did not contain a multiplier. Please provide one!";
//
//    pub fn new() -> EmptyMultiplierError {
//        Self { msg: Self::MSG, backtrace: Backtrace::capture() }
//    }
//}

//#[derive(Debug, Error)]
//#[error("{msg}\nBacktrace:\n{backtrace}")] 
//pub struct EmptyTimeZoneError {
//    msg: &'static str,
//    backtrace: Backtrace,
//}
//
//impl EmptyTimeZoneError {
//    const MSG: &'static str = "EmptyTimeZone Error : the timezone.txt file in the $HOME/.WaterPump/state-files dir is empty, please provide one through the cmdline option API.";
//
//    pub fn new() -> EmptyTimeZoneError {
//        Self { msg: Self::MSG, backtrace: Backtrace::capture() }
//    }
//}
