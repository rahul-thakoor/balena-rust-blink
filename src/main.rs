
use rust_gpiozero::*;
use std::env;
fn main() {
    // blinking configurations
    struct BlinkConfig {
        on_time: f32,
        off_time: f32,
    }
    // default blinking configurations
    let mut config = BlinkConfig {
        on_time: 2.0,
        off_time: 3.0,
    };

    // Create a new LED attached to Pin 17
    let mut led = LED::new(17);

    // if `ON_TIME` variable is set, change blinking on_time setting
    match env::var("ON_TIME") {
        Ok(on_time_var) => {
            match on_time_var.parse::<f32>(){
                Ok(on_time) => {
                    println!("on_time set: {}",on_time);
                    config.on_time = on_time;
                },
                Err(_) => println!("Could not set on_time")
            }
        },
        Err(_) => println!("Using default on_time: {}",config.on_time),
    }

    // if `OFF_TIME` variable is set, change blinking off_time setting
    match env::var("OFF_TIME") {
        Ok(off_time_var) => {
            match off_time_var.parse::<f32>(){
                Ok(off_time) => {
                    println!("off_time set: {}",off_time);
                    config.off_time = off_time;
                },
                Err(_) => println!("Could not set off_time")
            }
        },
        Err(_) => println!("Using default off_time: {}",config.off_time),
    }

    // blink the LED
    led.blink(config.on_time, config.off_time);

    // prevent program from exiting immediately
    led.wait();

}
