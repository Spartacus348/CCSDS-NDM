/*
Attempt to design from the bottom up, starting with a single opm
 */
extern crate chrono;
#[macro_use]
extern crate uom;

use std::iter::Map;
use uom::si::time::{day, second};

use common::{Comment, CommonMetaData, Header, UTCTime};
use constants::InterpolationType;
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

struct OCM{
    header: Header,
    metadata: OCMMetaData,
    data: OCMData
}

struct OCMMetaData{
    common_meta_data: CommonMetaData,
    catalog_name:Option<str>,
    object_designator:Option<str>,
    alternate_names:Option<[str]>,
    originator_poc:Option<str>,
    originator_position:Option<str>,
    originator_phone:Option<str>,
    originator_email:Option<str>,
    originator_address:Option<str>,
    tech_org:Option<str>,
    tech_poc:Option<str>,
    tech_position:Option<str>,
    tech_phone:Option<str>,
    tech_address:Option<str>,
    previous_message_id:Option<str>,
    next_message_id:Option<str>,
    adm_msg_link:Option<str>,
    cdm_msg_link:Option<str>,
    prm_msg_link:Option<str>,
    rdm_msg_link:Option<str>,
    tdm_msg_link:Option<str>,
    operator:Option<str>,
    owner:Option<str>,
    country:Option<str>,
    constellation:Option<str>,
    object_type:ObjectType,
    epoch_tzero:Option<UTCTime>,
    ops_status:OpsStatus,
    orbit_category:OrbitCategory,
    ocm_data_elements:Option<[OCMDataBlockName]>,
    sclk_offset_at_epoch:Option<f32>,
    sclk_sec_per_si_sec:Option<f32>,
    previous_message_epoch:Option<UTCTime>,
    next_message_epoch:Option<UTCTime>,
    start_time:Option<UTCTime>,
    stop_time:Option<UTCTime>,
    time_span:Option<day>,
    taimutc_at_tzero:Option<second>,
    next_leap_info:Option<NextLeapInfo>,
    ut1mutc_at_tzero:Option<second>,
    eop_source:Option<str>,
    interp_method_eop:InterpolationType,
    celestial_source:Option<str>,
}

struct OCMData{

}
