use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Timestamp, StdResult, StdError};
use chrono::{NaiveDateTime, Datelike, Timelike};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EzTimeStruct {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

pub trait EzTime {
    fn eztime_struct(&self) -> StdResult<EzTimeStruct>;
    fn eztime_string(&self) -> StdResult<String>;
}

impl EzTime for cosmwasm_std::Timestamp {
    fn eztime_struct(&self) -> StdResult<EzTimeStruct> {
        let seconds = &self.seconds();
        let nano = &self.subsec_nanos();
        let Some(dt) = NaiveDateTime::from_timestamp_opt(*seconds as i64, *nano as u32) else {
            return Err(StdError::GenericErr { msg: "Invalid Timestamp".to_string() });
        };
        Ok(EzTimeStruct {
            year: dt.year() as u32,
            month: dt.month(),
            day: dt.day(),
            hour: dt.hour(),
            minute: dt.minute(),
            second: dt.second(),
        })
    }

    fn eztime_string(&self) -> StdResult<String> {
        let seconds = &self.seconds();
        let nano = &self.subsec_nanos();
        let Some(dt) = NaiveDateTime::from_timestamp_opt(*seconds as i64, *nano as u32) else {
            return Err(StdError::GenericErr { msg: "Invalid Timestamp".to_string() });
        };
        match dt.month() {
            1 => {
                return Ok(format!(
                    "January {}, {} | {}:{}:{}", 
                    dt.day(), dt.year(), dt.hour(), dt.minute(), dt.second()
                ));
            },
            2 => {
                return Ok(format!(
                    "February {}, {} | {}:{}:{}", 
                    dt.day(), dt.year(), dt.hour(), dt.minute(), dt.second()
                ));
            },
            3 => {
                return Ok(format!(
                    "March {}, {} | {}:{}:{}", 
                    dt.day(), dt.year(), dt.hour(), dt.minute(), dt.second()
                ));
            },
            4 => {
                return Ok(format!(
                    "April {}, {} | {}:{}:{}", 
                    dt.day(), dt.year(), dt.hour(), dt.minute(), dt.second()
                ));
            },
            5 => {
                return Ok(format!(
                    "May {}, {} | {}:{}:{}", 
                    dt.day(), dt.year(), dt.hour(), dt.minute(), dt.second()
                ));
            },
            6 => {
                return Ok(format!(
                    "June {}, {} | {}:{}:{}", 
                    dt.day(), dt.year(), dt.hour(), dt.minute(), dt.second()
                ));
            },
            7 => {
                return Ok(format!(
                    "July {}, {} | {}:{}:{}", 
                    dt.day(), dt.year(), dt.hour(), dt.minute(), dt.second()
                ));
            },
            8 => {
                return Ok(format!(
                    "August {}, {} | {}:{}:{}", 
                    dt.day(), dt.year(), dt.hour(), dt.minute(), dt.second()
                ));
            },
            9 => {
                return Ok(format!(
                    "September {}, {} | {}:{}:{}", 
                    dt.day(), dt.year(), dt.hour(), dt.minute(), dt.second()
                ));
            },
            10 => {
                return Ok(format!(
                    "October {}, {} | {}:{}:{}", 
                    dt.day(), dt.year(), dt.hour(), dt.minute(), dt.second()
                ));
            },
            11 => {
                return Ok(format!(
                    "November {}, {} | {}:{}:{}", 
                    dt.day(), dt.year(), dt.hour(), dt.minute(), dt.second()
                ));
            },
            12 => {
                return Ok(format!(
                    "December {}, {} | {}:{}:{}", 
                    dt.day(), dt.year(), dt.hour(), dt.minute(), dt.second()
                ));
            },
            _ => {
                return Err(StdError::GenericErr {msg: "Invalid Timestamp".to_string()});
            },
        };
    }
}