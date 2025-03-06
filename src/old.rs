use anyhow::{bail, Result};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::adc::oneshot::config::AdcChannelConfig;
use esp_idf_svc::hal::adc::oneshot::{AdcChannelDriver, AdcDriver};
use esp_idf_svc::hal::adc::Resolution;
use esp_idf_svc::hal::ledc::{LedcDriver, LedcTimer, TIMER1};
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::hal::gpio::{PinDriver};

use log::info;

use esp_idf_svc::wifi;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    info!("Hello, world!");

    // Initialize GPIO1 as an output pin
    let mut gpio1 = PinDriver::output(peripherals.pins.gpio1)?;
    


    // Initialize GPIO1 as a PWM output
    //let mut pwm = LedcDriver::new(Led,peripherals.pins.gpio1, TIMER1)?;
    let gpio0 = PinDriver::input(peripherals.pins.gpio0)?;

    // Initialize ADC driver
    let mut adc = AdcDriver::new(peripherals.adc1)?;

    // Configure GPIO4 as an ADC input (example pin)
    let mut adc_channel = AdcChannelDriver::new(&adc,peripherals.pins.gpio0,&AdcChannelConfig::new())?;
    let mut val: u16 = 0;

    loop {

        let adc_value: u16 = adc.read(&mut adc_channel)?;
        val = (val + adc_value)/2;
        info!("Analog value: {} smooth {}", adc_value,val);
        if val > 50{
            gpio1.set_high()?;
        //info!("GPIO1 set high");
        std::thread::sleep(std::time::Duration::from_millis(50));
        }
        else{
            gpio1.set_low()?;
        //info!("GPIO1 set low");
        std::thread::sleep(std::time::Duration::from_millis(50));
        }
        // Set GPIO1 high
        

        // Set GPIO1 low
        
    }
}