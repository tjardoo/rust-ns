use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::str::{self, FromStr};

use super::train_category::TrainCategory;

#[derive(Serialize, Debug)]
pub struct Departure {
    pub direction: String,
    pub train_name: String,
    pub planned_date_time: String,
    pub actual_date_time: String,
    pub planned_track: String,
    pub train_category: TrainCategory,
    pub is_cancelled: bool,
    pub route_stations: Vec<RouteStation>,
    pub messages: Option<Vec<Message>>,
    pub departure_status: String,
    pub delay_in_minutes: i64,
}

#[derive(Serialize, Debug)]
pub struct RouteStation {
    pub code: String,
    pub name: String,
}

#[derive(Serialize, Debug)]
pub struct Message {
    pub content: String,
    pub style: String,
}

impl<'de> Deserialize<'de> for Departure {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Outer {
            direction: String,
            name: String,
            planned_date_time: String,
            actual_date_time: String,
            planned_track: String,
            train_category: String,
            is_cancelled: bool,
            route_stations: Vec<RouteStation>,
            messages: Option<Vec<Message>>,
            departure_status: String,
        }

        impl<'de> Deserialize<'de> for RouteStation {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #[derive(Deserialize)]
                struct InnerRouteStation {
                    uic_code: String,
                    medium_name: String,
                }

                let helper = InnerRouteStation::deserialize(deserializer)?;

                Ok(RouteStation {
                    code: helper.uic_code,
                    name: helper.medium_name,
                })
            }
        }

        impl<'de> Deserialize<'de> for Message {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #[derive(Deserialize)]
                struct InnerMessage {
                    content: String,
                    style: String,
                }

                let helper = InnerMessage::deserialize(deserializer)?;

                Ok(Message {
                    content: helper.content,
                    style: helper.style,
                })
            }
        }

        let helper = Outer::deserialize(deserializer)?;

        let planned_date_time =
            NaiveDateTime::parse_from_str(&helper.planned_date_time, "%Y-%m-%dT%H:%M:%S").unwrap();

        let actual_date_time =
            NaiveDateTime::parse_from_str(&helper.actual_date_time, "%Y-%m-%dT%H:%M:%S").unwrap();

        let delay_in_minutes = actual_date_time - planned_date_time;

        Ok(Departure {
            direction: helper.direction,
            train_name: helper.name,
            planned_date_time: planned_date_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            actual_date_time: actual_date_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            planned_track: helper.planned_track,
            train_category: TrainCategory::from_str(&helper.train_category).unwrap(),
            is_cancelled: helper.is_cancelled,
            route_stations: helper.route_stations,
            messages: helper.messages,
            departure_status: helper.departure_status,
            delay_in_minutes: delay_in_minutes.num_minutes(),
        })
    }
}
