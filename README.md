# rust_gpiozero on balena - Blink!

This project showcases how to use the [`rust_gpiozero`](https://crates.io/crates/rust_gpiozero) crate to control an LED attached to pin 17 on a raspberry pi. It uses the [`blink`](https://docs.rs/rust_gpiozero/0.2.0/rust_gpiozero/output_devices/struct.LED.html#method.blink) method.

The project also showcases the use of Service Variables in the balena dashboard.

By default, the LED blinking configuraton is as follows:

-   on_time : 2 seconds
-   off_time : 3 seconds

You should see the following output in the logs when the service runs

![defaults](img/defaults.png)

To configure the blinking settings, add the service variables for the device:

-   `ON_TIME`

-   `OFF_TIME`

![service variables](img/service_vars.png)

This automatically restarts the servive. You should see the outputs in the logs corresponding to the variables you set:

![service variables set](img/service_vars_set.png)

# Acknowledgement

This sample project is based on the [balena-rust-hello-world](https://github.com/balena-io-projects/balena-rust-hello-world) sample project from Balena. All credits go to the original authors!
