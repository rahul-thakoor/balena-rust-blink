# rust_gpiozero on balena - Blink!

This project showcases how to use the `rust_gpiozero` crate to control an LED attached to pin 17 on a raspberry pi.

The project also showcases the use of Service Variables in the balena dashboard.

By default, the LED blinking configuraton is as follows:

-   on_time : 2 seconds
-   off_time : 3 seconds

To configure the blinking settings, add the service variables for the device:

-   `ON_TIME`

-   `OFF_TIME`

# Acknowledgement

This sample project is based on the ![balena-rust-hello-world](https://github.com/balena-io-projects/balena-rust-hello-world) sample project from Balena. All credits go to the original authors!
