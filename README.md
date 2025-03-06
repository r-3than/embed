Car Headlight Security Device

This project implements a security system that controls the indicator LED of a car's headlight. The system is designed to activate and blink the LED in a random pattern when armed, serving as an indicator for security purposes. The device uses an ESP32 microcontroller, connects to a Wi-Fi network, and provides HTTP control endpoints for arming and disarming the system.

Features

- Wi-Fi Connectivity: Connects to a Wi-Fi network using credentials from a configuration file.
- GPIO Control: Controls a car's indicator LED by activating the corresponding GPIO pins on the ESP32.
- HTTP Server: Provides control endpoints over the network to arm or disarm the system.
  - /ping: A health check endpoint returning "pong".
  - /arm: Arms the system, enabling LED flashing with a random pattern.
  - /disarm: Disarms the system and stops the LED from flashing.

Requirements

- Rust: Make sure Rust is installed. Follow the official instructions: Install Rust (https://www.rust-lang.org/tools/install).
- ESP-IDF: Set up the ESP-IDF toolchain. For setup instructions, refer to the ESP-IDF Documentation (https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/).
- cargo-xbuild: Required to build the project for the ESP32 platform.

Setup

1. Clone the repository

Clone this repository to your local machine:

git clone https://github.com/your-username/car-headlight-security.git
cd car-headlight-security

2. Set up Wi-Fi credentials

In the project directory, create or edit the wifi_settings.txt file. This file should contain the SSID and password for your Wi-Fi network, each on a new line:

your_wifi_ssid
your_wifi_password

3. Build the project

Build the project using the following command:

cargo build --target=xtensa-esp32-none-elf

4. Flash the ESP32

Once the project is built, flash it to your ESP32 using this command (adjust the serial port as needed):

cargo espflash --port /dev/ttyUSB0

5. Monitor the serial output

To view logs from the ESP32, use the following command:

cargo monitor --port /dev/ttyUSB0

This will display logs that include Wi-Fi connection status, server activity, and GPIO control.

HTTP Endpoints

- GET /ping: A simple health check endpoint that returns "pong".

  Example request:
  curl http://<esp32-ip>/ping
  Response:
  pong

- GET /arm: Arms the system and begins flashing the car's indicator LED in a random pattern.

  Example request:
  curl http://<esp32-ip>/arm
  Response:
  okay

- GET /disarm: Disarms the system and stops the LED from flashing.

  Example request:
  curl http://<esp32-ip>/disarm
  Response:
  okay

How It Works

1. Wi-Fi Setup: The ESP32 connects to your Wi-Fi network using the credentials in the wifi_settings.txt file.
2. GPIO Control: The ESP32 controls a car's indicator LED by toggling the GPIO pins. GPIO1 is used for turning the LED on and off in a random flashing pattern when armed.
3. HTTP Server: The ESP32 runs a simple HTTP server, allowing remote control of the system to arm or disarm it via the /arm and /disarm endpoints.

Troubleshooting

- Ensure the Wi-Fi credentials in the wifi_settings.txt file are correct.
- Ensure the ESP32 is flashed correctly with the appropriate firmware.

Demonstration Video

Check out the demonstration of this security system in action:  
YouTube Link (insert_youtube_link_here)

License

This project is licensed under the MIT License - see the LICENSE file for details.

---

Stay secure with your car's headlight indicator LED!
