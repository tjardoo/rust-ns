# Rust NS

This is a demo project to learn Rust.

The aim of this project is to develop a web app that can show the train information of the Dutch railways (called NS).

## Install

### Database and migration

To be able to run the migrations install the ``sqlx-cli`` with the Rust toolchain in the ``/ns-web-service`` directory.

```bash
#https://github.com/launchbadge/sqlx/tree/main/sqlx-cli

cargo install sqlx-cli

#sqlx database drop
sqlx database create

#sqlx migrate add <name>

sqlx migrate run
#sqlx migrate revert
```

Note: make sure to use InnoDB as default MySQL engine and not MyISAM. MyISAM does not support foreign key constrains.

## Usage

### NS Web Service

Gets the information from the NS API and imports the data into the database and makes it accessible via multiple routes.

```bash
cd ns-web-service
cargo run --bin travel-information
```

### API

- [Station Departure Overview](docs/api/station.md)
- [Platform Departures](docs/api/platform.md)
- [Departure Details](docs/api/departure.md)

#### Routes

``/station/{station_code}``

``/station/{station_code}/download``

``/station/{station_code}/platform/{platform_id}``

``/station/{station_code}/departure/{departure_id}``

### Web App

Server with multiple routes to display the information that it receives from the NS Web Service API.

```bash
npx tailwindcss -i ./static/input.css -o ./static/css/output.css --watch # for development
npx tailwindcss -i ./static/input.css -o ./static/css/output.css
```

```bash
cd web-app
cargo run --bin app
```

#### View all departures

``/departures``.

In the ``.env`` file you can change the station code that is used to display the list of departures.

## Support

### Example station codes

HN = Hoorn

ASD = Amsterdam Centraal

ASS = Amsterdam Sloterdijk

UT = Utrecht Centraal

## Resources

[https://apiportal.ns.nl/docs/services/reisinformatie-api/operations/getDepartures](https://apiportal.ns.nl/docs/services/reisinformatie-api/operations/getDepartures)

[https://www.ns.nl/platform/fundamentals/colours.html](https://www.ns.nl/platform/fundamentals/colours.html)

[https://www.ns.nl/platform/resources.html](https://www.ns.nl/platform/resources.html)

## Disclaimer

NS/NS Reizigers is part of Nederlandse Spoorwegen B.V. and/or partners and is not associated with this project. This project is an independent application that is developed with the aim of learning the programming language Rust. This project makes use of the NS API. In the tab [#resources](#resources) you can find links to their API, branding guide and resources.
