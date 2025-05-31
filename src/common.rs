use chrono::{DateTime, Utc};
use constants::{Classification, NaturalBody, ReferenceFrame, TimeSystem};
use serde_derive::{Deserialize, Serialize};

pub type Comment = Option<Vec<String>>;
pub type UTCTime = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    ccsds_opm_vers: String,
    comment: Comment,
    classification: Classification,
    creation_date: UTCTime,
    originator: String,
    message_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMetaData {
    comment: Comment,
    object_name: String,
    object_id: String,
    center_name: NaturalBody,
    ref_frame: ReferenceFrame,
    ref_frame_epoch: UTCTime,
    time_system: TimeSystem,
}
