use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use embedded_hal::digital::v2::OutputPin;
use esp_idf_hal::peripherals::Peripherals;

fn main() -> anyhow::Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    println!("Hello, world!");

    test_blinky()?;

    Ok(())
}

fn test_blinky() -> anyhow::Result<()> {
    use std::thread;
    use std::time::Duration;

    let peripherals = Peripherals::take().unwrap();
    let mut led = peripherals.pins.gpio4.into_output()?;

    loop {
        led.set_high()?;
        thread::sleep(Duration::from_secs(1));
        led.set_low()?;
        thread::sleep(Duration::from_secs(1));
    }
}
