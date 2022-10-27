CREATE TABLE departures
(
    id INT PRIMARY KEY,
    direction VARCHAR(50) NOT NULL,
    train_name VARCHAR(50) NOT NULL,
    planned_track VARCHAR(10) NOT NULL,
    train_category VARCHAR(10) NOT NULL
);

INSERT INTO departures(id, direction, train_name, planned_track, train_category)
    VALUES(1, 'Den Haag Centraal', 'NS 3357', "2", "spr");

INSERT INTO departures(id, direction, train_name, planned_track, train_category)
    VALUES(2, 'Enkhuizen', 'NS 3944', "1", "ic");

INSERT INTO departures(id, direction, train_name, planned_track, train_category)
    VALUES(3, 'Amsterdam Centraal', 'NS 4846', "3", "spr");

INSERT INTO departures(id, direction, train_name, planned_track, train_category)
    VALUES(4, 'Amsterdam Centraal', 'NS 3957', "3", "ic");