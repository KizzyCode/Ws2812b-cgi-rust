[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/Ws2812b-cgi-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/Ws2812b-cgi-rust)


# `ws2812b-cgi`
Welcome to `ws2812b-cgi` 🎉
This application is a tiny (CGI) application which reads a JSON array with update commands from `stdin`, translates them
into [WS2812B driver commands](https://github.com/KizzyCode/WS2812BDriver-rust-rp2040), and sends them to the driver's
serial port.

## Configuration
Due to configuration limits in a CGI environment, the configuration is set during compilation via the following
environment variables:
- `WS2812B_CGI_SERIALDEVICE`: The path to the serial device (defaults to `/dev/ws2812b.serial`)
- `WS2812B_CGI_TIMEOUT`: The timeout for a CGI process in seconds, used for the internal watchdog (defaults to 10 
  seconds)

## Example
Change the 0th pixel of the 0th strip to white (RGBW 255,255,255,0):
```sh
echo '[{ "strip": 0, "pixel": 0, "rgbw": [255,255,255,0] }]' | ws2812b
```

## Docker
The docker image uses `/dev/ws2812b.serial` as serial device path - you can simply mount your real device onto this
path. Please note, that the `docker-compose.yml` is an example and must be adapted to your specific setup.
