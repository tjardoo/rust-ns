use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum TrainCategory {
    SPR,
    IC,
    ICD,
    ICE,
    THA,
    EX,
    NSM,
    UNKNOWN,
}

impl std::str::FromStr for TrainCategory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SPR" => Ok(TrainCategory::SPR),
            "IC" => Ok(TrainCategory::IC),
            "ICD" => Ok(TrainCategory::ICD),
            "ICE" => Ok(TrainCategory::ICE),
            "THA" => Ok(TrainCategory::THA),
            "EX" => Ok(TrainCategory::EX),
            "NSM" => Ok(TrainCategory::NSM),
            "UNKOWN" => Ok(TrainCategory::UNKNOWN),
            _ => Err(format!("'{}' is not a valid value for TrainCategory", s)),
        }
    }
}
