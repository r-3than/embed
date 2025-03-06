use anyhow::{bail, Result};
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::hal::gpio::{PinDriver};

use esp_idf_svc::http::Method;
use esp_idf_svc::sys::rand;
use log::info;

use esp_idf_svc::{
    wifi::EspWifi,
    nvs::EspDefaultNvsPartition,
    eventloop::EspSystemEventLoop,
};

use embedded_svc::wifi::{ClientConfiguration, Wifi, Configuration};
use rand::Rng;
use std::sync::{Arc, Mutex};
use esp_idf_svc::http::server::EspHttpServer;
use esp_idf_svc::io::Write;
fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    info!("Hello, world!");

    // Initialize GPIO1 as an output pin
    let mut gpio1 = PinDriver::output(peripherals.pins.gpio1)?;
    gpio1.set_high().unwrap();
    let mut gpio0 = PinDriver::input(peripherals.pins.gpio0)?;

    let mut wifi_driver = Arc::new(Mutex::new(
        EspWifi::new(peripherals.modem, sys_loop, Some(nvs)).unwrap(),
    ));

    let wifi_settings = include_str!("../src/wifi_settings.txt"); // Adjust path if needed

    let mut lines = wifi_settings.lines();

    let ssid_text = lines.next().unwrap();
    let password_text = lines.next().unwrap();

    let mut ssid = heapless::String::<32>::new();
    ssid.push_str(&ssid_text).unwrap();

    let mut password = heapless::String::<64>::new();
    password.push_str(&password_text).unwrap();

    wifi_driver.lock().unwrap().set_configuration(&Configuration::Client(ClientConfiguration{
        ssid: ssid,
        password: password,
        ..Default::default()
    })).unwrap();

    wifi_driver.lock().unwrap().start().unwrap();
    wifi_driver.lock().unwrap().connect().unwrap();
    while !wifi_driver.lock().unwrap().is_connected().unwrap(){
        let config = wifi_driver.lock().unwrap().get_configuration().unwrap();
        println!("Waiting for station {:?}", config);
    }
    println!("Should be connected now");
    
    
    let mut server = EspHttpServer::new(&esp_idf_svc::http::server::Configuration::default()).unwrap();

    let armed = Arc::new(Mutex::new(false));
    

    gpio1.set_high().unwrap();
    

    server.fn_handler("/ping", Method::Get, move |mut req| {
        //info!("GPIO1 set high");
        req.into_ok_response()?
            .write_all(b"pong")
            .map(|_| ())
    })?;

    let armed_clone = Arc::clone(&armed);
    server.fn_handler("/arm", Method::Get, move |mut req| {
        let mut armed = armed_clone.lock().unwrap();
        *armed = true;
        req.into_ok_response()?
            .write_all(b"okay")
            .map(|_| ())
    })?;
    let armed_clone = Arc::clone(&armed);
    server.fn_handler("/disarm", Method::Get, move |mut req| {
        let mut armed = armed_clone.lock().unwrap();
        *armed = false;
        req.into_ok_response()?
            .write_all(b"okay")
            .map(|_| ())
    })?;

    loop{
        gpio1.set_high().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(250));
        let armed2 = armed.lock().unwrap();
        println!("{:?}",armed2);
        
        match *armed2{
            true => {
                println!("Armed");
                if gpio0.is_high(){
                    
                let x = rand::rng().random_range(5..200);
                
                gpio1.set_low().unwrap();
                std::thread::sleep(std::time::Duration::from_millis(50));
                gpio1.set_high().unwrap();
                std::thread::sleep(std::time::Duration::from_millis(25));
                gpio1.set_low().unwrap();
                std::thread::sleep(std::time::Duration::from_millis(x));
                gpio1.set_high().unwrap();
                std::thread::sleep(std::time::Duration::from_millis(x));
                gpio1.set_low().unwrap();
                std::thread::sleep(std::time::Duration::from_millis(25));
                gpio1.set_high().unwrap();
                std::thread::sleep(std::time::Duration::from_millis(rand::rng().random_range(200..2000)));}},
            false => {println!("not armed {}",wifi_driver.lock().unwrap().sta_netif().get_ip_info().unwrap().ip);
            gpio1.set_high().unwrap();}
        }
        
        
    }
}