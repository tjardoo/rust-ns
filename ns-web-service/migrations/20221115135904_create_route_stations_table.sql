CREATE TABLE route_stations
(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    departure_id INT UNSIGNED NOT NULL,
    station_id INT UNSIGNED NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (`departure_id`) REFERENCES departures(`id`) ON DELETE CASCADE,
    FOREIGN KEY (`station_id`) REFERENCES stations(`id`) ON DELETE CASCADE,
    CONSTRAINT `departure_station_unique_id` UNIQUE (`departure_id`, `station_id`)
);