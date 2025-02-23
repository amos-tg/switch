pub const HELP: &'static str =
r#"actuator -- 
This is the cli interface for this application.


--help:
Displays this message.


--show:
Shows the configuration for every profile and any active timers.


--exec:
Executes the program based on the configuration of the given profile. This is not a reccomended public interface. This made for systemd timers to "actuate" via activating their corresponding profiles systemd service.

arg order = (1:profile-id) (2:f64-format-time-actuated-for)


--set-num-actuations:
Sets the number of actuations for the given profile. This exists because relays have a lifetime measured in number of actuations.

arg order = (1:profile-id) (2:u64-format-num-actuations-set)


--set-max-actuations:
Sets the number of max actuations for the given profile; This is based on the relay in use. This may be unimportant in certain applications.

arg order = (1:profile-id) (2:u64-format-max-num-actuations)


--set-max-time:
Sets the maximum amount of time the gpio pin can be actuated for for the given profile. Any value inputed above the max will default to the max value.

arg order =  (1:profile-id) (2:f64-format-max-time-actuated)


--set-gpio
Sets the gpio pin actuated for the given profile.

arg order = (1:prof-id) (2:u8-format-gpio-pin)


--set-timezone:
Takes an argument to set the timezone for the given profile; You can use any timezone in the IANA database provided by the chrono_tz crate. (accessible via docs.rs, chrono_tz crate, Tz enum)

arg order = (1:prof-id) (2:chrono_tz::Tz-format-timezone-string)


--set-multiplier:
Sets the multiplier for the given profile. (f64-formatting ex. 0.1, 1.1, 1.0, do not drop the leading or trailing and always include a zero)

arg order = (1:prof-id) (2:f64-format-time-actuated-multiplier)


--entry:
Makes an actuation occur at the given time based on the configuration of the given profile. Under the hood this creates a unique systemd timer which points to a profile wide systemd service. This program is systemd based on therefore will not work on non-systemd based systems.

arg order = (1:prof-id) (2:f64-format-time-actuated-for) (3:systemd-timer-file-name-prefix) (4:time-actuated-at)


--del-entry:
Deletes the entry for the timer file name prefix given. To gather a list of available entries for deletion use the --show argument. This function will automatically cleanup unused service files, you do not have to worry about manual systemd file management related to this program.

arg order = (1:systemd-file-name-prefix)"#;
