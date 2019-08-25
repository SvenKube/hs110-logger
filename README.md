# HS110 Logger

This is a simple application that uses the [hs110 Library](https://github.com/SvenKube/hs110-rs) to fetch realtime measurements from a TP-Link HS110 SmartPlug.
These measurements are stored in a influxdb.

## Confiuration

The application requires the following environment variables to be set:

- `HS110_IP`
- `HS110_INFLUX_HOST`
- `HS110_INFLUX_USERNAME`
- `HS110_INFLUX_PASSWORD`
- `HS110_INFLUX_DATABASE`


## Todo:
- [ ] Handle SmartPlug disconnect and reconnect
- [ ] Read configuration from arguments
- [ ] Read config file
- [ ] Checkout other influxdb libraries which use non-blocking http-requests
- [ ] Send daily and weekly statistics to influxdb
