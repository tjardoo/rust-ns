use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::str::{self, FromStr};

use super::train_category::TrainCategory;

#[derive(Serialize, Debug)]
pub struct Departure {
    pub direction: String,
    pub name: String,
    pub planned_time: String,
    pub planned_track: String,
    pub train_category: TrainCategory,
    pub is_cancelled: bool,
    // pub stations: Vec<Station>,
    // pub messages: Option<Vec<Message>>,
    pub delay_in_minutes: i64,
}

#[derive(Serialize, Debug)]
pub struct Station {
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
            // route_stations: Vec<Station>,
            // messages: Option<Vec<Message>>,
        }

        impl<'de> Deserialize<'de> for Station {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #[derive(Deserialize)]
                struct InnerStation {
                    uic_code: String,
                    medium_name: String,
                }

                let helper = InnerStation::deserialize(deserializer)?;

                Ok(Station {
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
            NaiveDateTime::parse_from_str(&helper.planned_date_time, "%Y-%m-%d %H:%M:%S").unwrap();

        let actual_date_time =
            NaiveDateTime::parse_from_str(&helper.actual_date_time, "%Y-%m-%d %H:%M:%S").unwrap();

        let delay_in_minutes = actual_date_time - planned_date_time;

        Ok(Departure {
            direction: helper.direction,
            name: helper.name,
            planned_time: planned_date_time.format("%H:%M").to_string(),
            planned_track: helper.planned_track,
            train_category: TrainCategory::from_str(&helper.train_category).unwrap(),
            is_cancelled: helper.is_cancelled,
            // stations: helper.route_stations,
            // messages: helper.messages,
            delay_in_minutes: delay_in_minutes.num_minutes(),
        })
    }
}
