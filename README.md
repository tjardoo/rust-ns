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

## Usage

### NS Web Service

Gets the information from the NS API and imports the data into the database and makes it accessible via multiple routes.

```bash
cd ns-web-service
cargo run --bin travel-information
```

#### List of departures

``/station/{station_code}/departures``.

#### Departure details

``/station/{station_code}/departures/{departure_id}``.

#### Download new departures

``/station/{station_code}/departures/download``.

### Web App

Server with multiple routes to display the information that it receives from the NS Web Service API.

```bash
cd web-app
cargo run --bin app
```

#### View all departures

``/departures``.

In the ``.env`` file you can change the station code that is used to display the list of departures.

### Extra

#### Example station codes

HN = Hoorn

ASD = Amsterdam Centraal

ASS = Amsterdam Sloterdijk

UT = Utrecht Centraal
