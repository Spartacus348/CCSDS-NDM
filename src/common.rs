use chrono::{DateTime, Utc};
use constants::{NaturalBody, ReferenceFrame, TimeSystem};


pub type Comment = Option<[str]>;
pub type UTCTime = DateTime<Utc>;

pub struct Header{
    ccsds_opm_vers: str,
    comment: Comment,
    classification: Option<str>,
    creation_date: UTCTime,
    originator: str,
    message_id: Option<str>
}

pub struct CommonMetaData{
    comment: Comment,
    object_name: str,
    object_id: str,
    center_name: NaturalBody,
    ref_frame: ReferenceFrame,
    ref_frame_epoch: UTCTime,
    time_system: TimeSystem
}
