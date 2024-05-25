/*
Attempt to design from the bottom up, starting with a single opm
 */
extern crate chrono;
#[macro_use]
extern crate uom;

use std::iter::Map;

use common::{Comment, CommonMetaData, Header, UTCTime};
use data_blocks::*;

mod constants;
mod data_blocks;
mod common;

struct OPM{
    header: Header,
    metadata: CommonMetaData,
    data: OPMData,
    comment:Comment
}

struct OPMData{
    state_vector_components: StateVector,
    osculating_keplerian_elements: Option<KeplerianElements>,
    spacecraft_parameters: Option<SpacecraftParameters>,
    pos_vel_covariance: Option<PosVelCovariance>,
    maneuver_parameters:Option<[ManeuverParameters]>,
    user_defined_parameters:Option<Map<str,str>>
}

struct OMM{
    header: Header,
    metadata: OMMMetaData,
    data:OMMData,
    comment:Comment
}

struct OMMMetaData{
    common_meta_data: CommonMetaData,
    mean_element_theory:str
}

struct OMMData{
    mean_keplerian_elements:KeplerianElements,
    spacecraft_parameters: Option<SpacecraftParameters>,
    tle_related_parameters: Option<TLEParameters>,
    pos_vel_covariance: Option<PosVelCovariance>,
    user_defined_parameters: Option<Map<str,str>>
}

struct OEM{
    header: Header,
    metadata: OEMMetaData,
    comment: Comment,
    ephemeris:[OEMEphemeris],
    pos_vel_covariance: PosVelCovariance
}

struct OEMMetaData{
    meta_start:None,
    common_meta_data: CommonMetaData,
    start_time: UTCTime,
    useable_start_time: Option<UTCTime>,
    useable_stop_time: Option<UTCTime>,
    stop_time: UTCTime,
    interpolation:Option<InterpolationInfo>,
    meta_stop:None,
}
