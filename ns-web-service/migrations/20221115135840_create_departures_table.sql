CREATE TABLE departures
(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    station_code VARCHAR(50) NOT NULL,
    direction VARCHAR(50) NOT NULL,
    train_name VARCHAR(50) NOT NULL,
    planned_date_time DATETIME NOT NULL,
    actual_date_time DATETIME NOT NULL,
    planned_track VARCHAR(50) NOT NULL,
    product_id INTEGER(10) NOT NULL,
    train_category VARCHAR(50) NOT NULL,
    is_cancelled INT(1) NOT NULL,
    departure_status VARCHAR(50) NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

ALTER TABLE departures ADD FOREIGN KEY (`product_id`) REFERENCES products(`id`);