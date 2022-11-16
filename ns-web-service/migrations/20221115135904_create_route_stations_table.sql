CREATE TABLE route_stations
(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    departure_id INT(10) UNSIGNED NOT NULL,
    station_id INT(10) UNSIGNED NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

ALTER TABLE route_stations ADD FOREIGN KEY (`departure_id`) REFERENCES departures(`id`);

ALTER TABLE route_stations ADD FOREIGN KEY (`station_id`) REFERENCES stations(`id`);

ALTER TABLE route_stations ADD CONSTRAINT `departure_station_unique_id` UNIQUE (`departure_id`, `station_id`);