# Departure Details

Used to get the JSON response with the details of a specific departure.

**URL** : `/station/{station_code}/departure/{departure_id}`

**Method** : `GET`

## Success Response

**Code** : `200 OK`

### Content Example

```json
{
    "data": {
        "id": 23,
        "station_code": "HN",
        "direction": "Nijmegen",
        "name": "NS  3069",
        "planned_date_time": "2022-11-18 18:08:00",
        "actual_date_time": "2022-11-18 18:08:00",
        "planned_track": "2",
        "product": {
            "id": 23,
            "number": "3069",
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
            "id": 9,
            "uic_code": "8400731",
            "medium_name": "Zaandam"
        },
        {
            "id": 14,
            "uic_code": "8400058",
            "medium_name": "Amsterdam C."
        },
        {
            "id": 16,
            "uic_code": "8400621",
            "medium_name": "Utrecht C."
        },
        {
            "id": 38,
            "uic_code": "8400059",
            "medium_name": "Sloterdijk"
        }
        ],
        "messages": [],
        "departure_status": "INCOMING"
    },
    "details": {
        "station_code": "HN",
        "current_date_time": "2022-11-18 17:04:06"
    }
}
```
