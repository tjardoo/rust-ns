use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::str::{self, FromStr};

#[derive(Debug, Deserialize, Serialize)]
pub enum TrainCategory {
    SPR,
    IC,
    ICD,
    ICE,
    THA,
    UNKNOWN,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum DepartureStatus {
    INCOMING,
    ON_STATION,
}

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
        #[allow(non_snake_case)]
        struct Outer {
            direction: String,
            name: String,
            plannedDateTime: String,
            actualDateTime: String,
            plannedTrack: String,
            trainCategory: String,
            cancelled: bool,
            routeStations: Vec<RouteStation>,
            messages: Option<Vec<Message>>,
            departureStatus: String,
        }

        impl<'de> Deserialize<'de> for RouteStation {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #[derive(Deserialize)]
                #[allow(non_snake_case)]
                struct InnerRouteStation {
                    uicCode: String,
                    mediumName: String,
                }

                let helper = InnerRouteStation::deserialize(deserializer)?;

                Ok(RouteStation {
                    code: helper.uicCode,
                    name: helper.mediumName,
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
                    message: String,
                    style: String,
                }

                let helper = InnerMessage::deserialize(deserializer)?;

                Ok(Message {
                    content: helper.message,
                    style: helper.style,
                })
            }
        }

        let helper = Outer::deserialize(deserializer)?;

        let actual_date_time =
            NaiveDateTime::parse_from_str(&helper.actualDateTime, "%Y-%m-%dT%H:%M:%S").unwrap();

        let planned_date_time =
            NaiveDateTime::parse_from_str(&helper.plannedDateTime, "%Y-%m-%dT%H:%M:%S").unwrap();

        let delay_in_minutes = planned_date_time - actual_date_time;

        Ok(Departure {
            direction: helper.direction,
            train_name: helper.name,
            planned_date_time: planned_date_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            actual_date_time: actual_date_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            planned_track: helper.plannedTrack,
            train_category: TrainCategory::from_str(&helper.trainCategory).unwrap(),
            is_cancelled: helper.cancelled,
            route_stations: helper.routeStations,
            messages: helper.messages,
            departure_status: helper.departureStatus,
            delay_in_minutes: delay_in_minutes.num_minutes(),
        })
    }
}

impl str::FromStr for TrainCategory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SPR" => Ok(TrainCategory::SPR),
            "IC" => Ok(TrainCategory::IC),
            "ICD" => Ok(TrainCategory::ICD),
            "ICE" => Ok(TrainCategory::ICE),
            "THA" => Ok(TrainCategory::THA),
            "UNKOWN" => Ok(TrainCategory::UNKNOWN),
            _ => Err(format!("'{}' is not a valid value for TrainCategory", s)),
        }
    }
}
