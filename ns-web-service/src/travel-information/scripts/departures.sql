CREATE TABLE departures
(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    station_code VARCHAR(50) NOT NULL,
    direction VARCHAR(50) NOT NULL,
    train_name VARCHAR(50) NOT NULL,
    planned_date_time VARCHAR(50) NOT NULL,
    planned_time_zone_offset INTEGER(10) NOT NULL,
    actual_date_time VARCHAR(50) NOT NULL,
    actual_time_zone_offset INTEGER(10) NOT NULL,
    planned_track VARCHAR(50) NOT NULL,
    product_id INTEGER(10) NOT NULL,
    train_category VARCHAR(50) NOT NULL,
    is_cancelled INT(1) NOT NULL,
    departure_status VARCHAR(50) NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE products
(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    product_number VARCHAR(50) NOT NULL,
    category_code VARCHAR(50) NOT NULL,
    short_category_code VARCHAR(50) NOT NULL,
    long_category_code VARCHAR(50) NOT NULL,
    operator_code VARCHAR(50) NOT NULL,
    operator_name VARCHAR(50) NOT NULL,
    product_type VARCHAR(50) NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE route_stations
(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    departure_id INT(10) UNSIGNED NOT NULL,
    uic_code VARCHAR(50) NOT NULL,
    medium_name VARCHAR(255) NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE messages
(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    departure_id INT(10) UNSIGNED NOT NULL,
    content VARCHAR(255) NOT NULL,
    style VARCHAR(50) NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

ALTER TABLE departures ADD FOREIGN KEY (`product_id`) REFERENCES products(`id`);

ALTER TABLE route_stations ADD FOREIGN KEY (`departure_id`) REFERENCES departures(`id`);

ALTER TABLE messages ADD FOREIGN KEY (`departure_id`) REFERENCES departures(`id`);

INSERT INTO messages(id, departure_id, content, style)
    VALUES(1, 1, 'Stopt niet in Zaandam', 'INFO');

INSERT INTO route_stations(id, departure_id, uic_code, medium_name)
    VALUES(1, 1, '8400059', 'Sloterdijk');

INSERT INTO products(id, product_number, category_code, short_category_code, long_category_code, operator_code, operator_name, product_type)
    VALUES(1, '3939', 'IC', 'NS Intercity', 'Intercity', 'NS', 'NS', 'TRAIN');

INSERT INTO
    departures(
        id,
        station_code,
        direction,
        train_name,
        planned_date_time,
        planned_time_zone_offset,
        actual_date_time,
        actual_time_zone_offset,
        planned_track,
        product_id,
        train_category,
        is_cancelled,
        departure_status
    )
VALUES(
        1,
        "HN",
        'Amsterdam Centraal',
        'NS 3939',
        '2022-11-03T10:40:00+0100',
        60,
        '2022-11-03T10:40:00+0100',
        60,
        '2',
        1,
        'IC',
        0,
        'ON_STATION'
    );

INSERT INTO route_stations(id, departure_id, uic_code, medium_name)
    VALUES(2, 2, '8400508', 'Purmerend');

INSERT INTO route_stations(id, departure_id, uic_code, medium_name)
    VALUES(3, 2, '8400731', 'Zaandam');

INSERT INTO route_stations(id, departure_id, uic_code, medium_name)
    VALUES(4, 2, '8400059', 'Sloterdijk');

INSERT INTO route_stations(id, departure_id, uic_code, medium_name)
    VALUES(5, 2, '8400561', 'Schiphol Airport');

INSERT INTO products(id, product_number, category_code, short_category_code, long_category_code, operator_code, operator_name, product_type)
    VALUES(2, '3343', 'SPR', 'NS Sprinter', 'Sprinter', 'NS', 'NS', 'TRAIN');

INSERT INTO
    departures(
        id,
        station_code,
        direction,
        train_name,
        planned_date_time,
        planned_time_zone_offset,
        actual_date_time,
        actual_time_zone_offset,
        planned_track,
        product_id,
        train_category,
        is_cancelled,
        departure_status
    )
VALUES(
        2,
        "HN",
        'Den Haag Centraal',
        'NS 3343',
        '2022-11-03T11:19:00+0100',
        60,
        '2022-11-03T11:19:00+0100',
        60,
        '2',
        2,
        'SPR',
        0,
        'INCOMING'
    );