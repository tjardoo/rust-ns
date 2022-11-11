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
pub struct DepartureList {
    pub direction: String,
    pub train_name: String,
    pub planned_date_time: String,
    pub actual_date_time: String,
    pub planned_track: String,
    pub train_category: TrainCategory,
    pub is_cancelled: bool,
    pub departure_status: String,
}

impl<'de> Deserialize<'de> for DepartureList {
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
            departureStatus: String,
        }

        let helper = Outer::deserialize(deserializer)?;

        // TODO convert the

        let actual_date_time =
            NaiveDateTime::parse_from_str(&helper.plannedDateTime, "%Y-%m-%dT%H:%M:%S")
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();

        let planned_date_time =
            NaiveDateTime::parse_from_str(&helper.actualDateTime, "%Y-%m-%dT%H:%M:%S")
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();

        Ok(DepartureList {
            direction: helper.direction,
            train_name: helper.name,
            planned_date_time,
            actual_date_time,
            planned_track: helper.plannedTrack,
            train_category: TrainCategory::from_str(&helper.trainCategory).unwrap(),
            is_cancelled: helper.cancelled,
            departure_status: helper.departureStatus,
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
