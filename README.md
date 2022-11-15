# Rust NS

This is a demo project to learn Rust.

The aim of this project is to develop a web app that can show the train information of the Dutch railways (called NS).

## Architecture

### /ns-web-service

Does the API calls to the NS API and prepares to data to display it in the ``web-app``.

Run the migration script ``scripts\departures.sql``.

```bash
cd ns-web-service
cargo run --bin travel-information
```

To view all departures for a certain station: ``/station/{station_code}/departures``.

To view the details of a departure: ``/station/{station_code}/departures/{departure_id}``.

### Some station codes to test

HN = Hoorn
ASD = Amsterdam Centraal
ASS = Amsterdam Sloterdijk
UT = Utrecht Centraal

### /web-app

Consists of a server with multiple routes to display the information prepared by the ``ns-web-service``.

```bash
cd web-app
cargo run --bin app
```

To view all departures: ``/departures``.

You can change the station_code in the ``.env`` file.
