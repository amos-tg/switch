# switch
Dependencies:
    runtime:
      - systemd
      - systemd-anaylze
    build:
      - nightly rust
    
    About:

    This program is a simple systemd based timed gpio switch. This program only works for Rasberry Pis; See rppal rust crate on docs.rs for specifics. I plan on adding more configuration options as the program matures. Adding support for mathematical expressions on the multiplier seems important; It is not presently possible to accurately represent certain units which the user may want to express in their time actuated for Multiplier configuration option, for example, if you want to accurately represent fluid oz's based on how long a water pump is turned on for, you would not be able to do so with just float multiplication. I will likely invest time in making a GUI applet which can be run on linux and maybe android mobile devices for the purpose of controlling this program. Due to the reliance on systemd and the defaults which are currently in place this program requires superuser privledges. 

    Configuration:
        
    if XDG_CONFIG_HOME is not set the program will return a NotPresent error. Since the program runs as root when executing it is important to ensure that XDG_CONFIG_HOME is set. Example:

    echo $XDG_CONFIG_HOME 

    should output the directory you expect configuration to be present within
    since the program is running as root the files should only accessible with 
    root priviledges however I see this as a non-issue even if you were to  
    put it in an infected environment given that the inputs are tested as a part
    of the implentation and will produce errors instead of undefined behavior if
    they are incorrect or malicous in nature.

    you can set XDG_CONFIG_HOME with /etc/environment if it is unset or 
    whatever alternative you see fit.

    Building: 
    
    this program must be built with nightly rust because I am using a feature from thiserror only available in nightly (look up error_generic_member_access for more info).

    1: Install rust and nightly rust -- look it up if you don't know how

    2: build the project 
    cargo build --release

    3: move the binary from the target dir into /usr/local/bin/. 
