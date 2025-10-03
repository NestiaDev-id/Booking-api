use std::time::SystemTime;

use chrono::Utc;
use mongodb::bson::{oid::ObjectId, DateTime};

use crate::models::{dog_model::Dog, owner_model::Owner};

#[derive(Debug, serde::Serialize, serde::Deserialize)]

pub struct Booking {
    pub _id: ObjectId, 
    pub owner: ObjectId,
    pub start_date: DateTime,
    pub duration_minutes: u8,
    pub cancelled: bool,
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]

pub struct BookingRequest {
    pub owner: String,
    pub start_date: String, // ISO 8601 format
    pub duration_minutes: u8,
}

pub struct FullBooking {
    pub _id: ObjectId,
    pub owner: Owner,
    pub dogs: Vec<Dog>,
    pub start_time: DateTime,
    pub duration_in_minutes: u8,
    pub cancelled: bool,
}


impl TryFrom<BookingRequest> for Booking {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: BookingRequest) -> Result<Self, Self::Error> {
        // let owner_id = ObjectId::parse_str(&value.owner)
        //     .map_err(|e| format!("Invalid owner ID: {}", e))?;

        let chrono_datetime: SystemTime = chrono::DateTime::parse_from_rfc3339(&value.start_date)
            .map_err(|err| format!("Failed to parse start_time: {}", err))?
            .with_timezone(&Utc)
            .into();

          Ok(Self {
            _id: ObjectId::new(),
            owner: ObjectId::parse_str(&value.owner).expect("Failed to parse owner"),
            start_date: DateTime::from(chrono_datetime),
            duration_minutes: value.duration_minutes,
            cancelled: false,
        })
    }
}