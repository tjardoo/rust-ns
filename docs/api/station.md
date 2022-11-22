# Station Departure Overview

Used to get the JSON response with the departures of a specific station.

**URL** : `/station/{station_code}`

**Method** : `GET`

## Success Response

**Code** : `200 OK`

### Content Example

```json
{
    "data": {
        "departures": [
        {
            "id": 21,
            "station_code": "HN",
            "direction": "Enkhuizen",
            "name": "NS  3054",
            "planned_date_time": "2022-11-18 17:59:00",
            "actual_date_time": "2022-11-18 17:59:00",
            "planned_track": "1",
            "product_id": 21,
            "train_category": "IC",
            "is_cancelled": false,
            "departure_status": "INCOMING"
        },
        ...
        ]
    },
    "details": {
        "station_code": "HN",
        "current_date_time": "2022-11-18 16:56:15"
    }
}
```
