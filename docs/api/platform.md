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
            "id": 23,
            "station_code": "HN",
            "direction": "Nijmegen",
            "name": "NS  3069",
            "planned_date_time": "2022-11-18 18:08:00",
            "actual_date_time": "2022-11-18 18:08:00",
            "planned_track": "2",
            "product_id": 23,
            "train_category": "IC",
            "is_cancelled": false,
            "departure_status": "INCOMING"
        },
        "next": {
            "id": 25,
            "station_code": "HN",
            "direction": "Hoofddorp",
            "name": "NS  3371",
            "planned_date_time": "2022-11-18 18:19:00",
            "actual_date_time": "2022-11-18 18:19:00",
            "planned_track": "2",
            "product_id": 25,
            "train_category": "SPR",
            "is_cancelled": false,
            "departure_status": "INCOMING"
        }
    },
    "details": {
        "station_code": "HN",
        "current_date_time": "2022-11-18 17:01:36"
    }
}
```
