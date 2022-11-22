# Platform Departures

Used to get the JSON response with the current and next departure of a specific platform.

**URL** : `/station/{station_code}/platform/{platform_code}`

**Method** : `GET`

## Success Response

**Code** : `200 OK`

### Content Example

```json
{
    "data": {
        "current": {
            "id": 7,
            "station_code": "ASS",
            "direction": "Amersfoort Schothorst",
            "name": "NS  1657",
            "planned_date_time": "2022-11-18 15:39:00",
            "actual_date_time": "2022-11-18 15:39:00",
            "planned_track": "12",
            "product": {
                "id": 7,
                "number": "1657",
                "category_code": "IC",
                "short_category_name": "NS Intercity",
                "long_category_name": "Intercity",
                "operator_code": "NS",
                "operator_name": "NS",
                "type": "TRAIN"
            },
            "train_category": "IC",
            "is_cancelled": false,
            "route_stations": [
                {
                    "id": 14,
                    "uic_code": "8400058",
                    "medium_name": "Amsterdam C."
                },
                {
                    "id": 18,
                    "uic_code": "8400322",
                    "medium_name": "Hilversum"
                },
                {
                    "id": 19,
                    "uic_code": "8400055",
                    "medium_name": "Amersfoort C."
                }
            ],
            "messages": [],
            "departure_status": "INCOMING"
        },
        "next": null,
    },
    "details": {
        "station_code": "HN",
        "current_date_time": "2022-11-18 17:01:36"
    }
}
```
