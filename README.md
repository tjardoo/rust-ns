# Rust NS

This project was created to learn the programming language Rust.

This project consists of 2 applications. `ns-web-service` communicates with the NS API (Dutch Railways), processes the departure data and saves it to the database. `web-app` fetches the departures from the database via the first application and displays it in the browser.

- `/station` displays all departures from the station set in the `.env` file.
- `/platform` displays the current/next departure from the given platform set in the `.env`.

Both applications run on their own port so they can communicate with each other (on the same machine). In the `.env` file you'll see we use `:7878` for the `ns-web-service` application and `:8888` for the `web-app`` application.

## Requirements

- Rust [https://rustup.rs/](https://rustup.rs/)
- MySQL

## Database setup

To be able to run the migrations you need to install the ``sqlx-cli`` with the Rust toolchain in the ``/ns-web-service`` directory. For more information visit [https://github.com/launchbadge/sqlx/tree/main/sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli)

```bash
cargo install sqlx-cli
sqlx database create
sqlx migrate run
```

Note: make sure you use `InnoDB` as default MySQL engine and not MyISAM. MyISAM does not support foreign key constrains.

## Usage

### NS Web Service

You'll first need to set some variables in the `.env` file.

```env
APP_URL=localhost:7878

DB_CONNECTION=mysql
DB_HOST=127.0.0.1
DB_PORT=3306
DB_DATABASE=rust_ns_test
DB_USERNAME=root
DB_PASSWORD=

NS_API_KEY=
NS_API_MAX_JOURNEYS=25

# required by sqlx (todo)
DATABASE_URL=mysql://username:password@127.0.0.1:3306/rust_ns_test
```

```bash
cd ns-web-service
cargo run --bin travel-information
```

#### NS Web Service routes

- `/station/{station_code}/download`
- `/station/{station_code}` [JSON response example](docs/api/station.md)
- `/station/{station_code}/platform/{platform_code}` [JSON response example](docs/api/platform.md)

### Web App

You'll first need to set some variables in the `.env` file.

```env
APP_URL=localhost:8888

TARGET_URL=http://localhost:7878 // this is the URL of the ns-web-service

STATION_CODE=ASS
PLATFORM_CODE=3
SCREEN_ROW_SIZE=10
PAGE_RELOAD_IN_SEC=30
```

```bash
cd web-app
cargo run --bin app
```

### Station

![Screenshot station](./docs/images/screenshot-station.png?raw=true "Screenshot station")

### Platform

![Screenshot platform](./docs/images/screenshot-platform.png?raw=true "Screenshot platform")

#### Web App routes

- `/station`
- `/platform`

Both pages automatically refresh after 30 seconds.

## Support

### Example station codes

- `ASD` = Amsterdam Centraal
- `ASS` = Amsterdam Sloterdijk
- `SHL` = Schiphol Airport
- `GVC` = Den Haag Centraal
- `UT` = Utrecht Centraal
- `HN` = Hoorn

### TailwindCSS CLI build process

You'll need to run the following command if you make changes the the html templates. This checks whether TailwindCSS classes have been added/removed and updates the `output.css` file.

```bash
npx tailwindcss -i ./static/input.css -o ./static/css/output.css --watch # for development
npx tailwindcss -i ./static/input.css -o ./static/css/output.css
```

## Resources

[https://apiportal.ns.nl/docs/services/reisinformatie-api/operations/getDepartures](https://apiportal.ns.nl/docs/services/reisinformatie-api/operations/getDepartures)

[https://www.ns.nl/platform/fundamentals/colours.html](https://www.ns.nl/platform/fundamentals/colours.html)

[https://www.ns.nl/platform/resources.html](https://www.ns.nl/platform/resources.html)

## Disclaimer

NS/NS Reizigers is part of Nederlandse Spoorwegen B.V. and/or partners and is NOT associated with this project. This project is an independent application that is developed with the aim of learning the programming language Rust.
