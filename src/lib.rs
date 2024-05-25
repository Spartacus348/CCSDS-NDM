/*
Attempt to design from the bottom up, starting with a single opm
 */
extern crate chrono;
#[macro_use]
extern crate uom;

use std::iter::Map;

use chrono::{DateTime, Utc};
use uom::si::angle::degree;
use uom::si::area::{square_kilometer, square_meter};
use uom::si::length::kilometer;
use uom::si::mass::kilogram;
use uom::si::specific_area::square_meter_per_kilogram;
use uom::si::time::second;
use uom::si::velocity::kilometer_per_second;
use constants::{LocalReferenceFrame, ReferenceFrame};

mod constants;

#[macro_use]
mod gm {
    quantity! {
        quantity: GM; "gravitational parameter";
        dimension: Q<P3,Z0,N2>;
        units {
            @mu: 1.0E0; "m^3/s^2" "meter^3 per second^2", "meters^3 per seconds^2";
        }
    }
}

#[macro_use]
mod square_kilometer_per_second{
    quantity!{
        quantity: KmSqPerSec; "kilometer squared per second";
        dimension: Q<P2,N1>;
        units {
            @meter_squared_per_second: 1.0E0; "m^2/s","meter^2 per second", "meters^2 per second";
            @kilometer_squared_per_second: 1.0E6; "km^2/s","kilometer^2 per second", "kilometers^2 per second";
        }
    }
}

#[macro_use]
mod square_kilometer_per_second_squared{
    quantity!{
        quantity: KmSqPerSec; "kilometer squared per second squared";
        dimension: Q<P2,N2>;
        units {
            @meter_squared_per_second_squared: 1.0E0; "m^2/s^2","meter^2 per second^2", "meters^2 per second^2";
            @kilometer_squared_per_second_squared: 1.0E6; "km^2/s^2","kilometer^2 per second^2", "kilometers^2 per second"^2;
        }
    }
}

#[macro_use]
mod cycle_per_day_squared{
    quantity!{
        quantity: RevPerDaySq; "revolutions per day squared";
        dimension: Q<Z0,N2>;
        units {
            @revolutions_per_second_squared: 1.0E0; "1/s^2","cycle per second^2", "cycles per second^2";
            @revolutions_per_day_squared: 7.46496E9; "1/day^2","day per second^2", "days per second"^2;
        }
    }
}

type Comment = Option<[str]>;

struct OPM{
    header: Header,
    meta_data: CommonMetaData,
    data: OPMData,
    comment:Comment
}

struct Header{
    ccsds_opm_vers: str,
    comment: Comment,
    classification: Option<str>,
    creation_date: DateTime<Utc>,
    originator: str,
    message_id: Option<str>
}

struct CommonMetaData{
    comment: Comment,
    object_name: str,
    object_id: str,
    center_name: constants::NaturalBody,
    ref_frame: constants::ReferenceFrame,
    ref_frame_epoch: DateTime<Utc>,
    time_system: constants::TimeSystem
}

struct OPMData{
    state_vector_components: StateVector,
    osculating_keplerian_elements: Option<KeplerianElements>,
    spacecraft_parameters: Option<SpacecraftParameters>,
    pos_vel_covariance: Option<PosVelCovariance>,
    maneuver_parameters:Option<[ManeuverParameters]>,
    user_defined_parameters:Option<Map<str,str>>
}

struct StateVector{
    comment:Comment,
    epoch:DateTime<Utc>,
    x:kilometer,
    y:kilometer,
    z:kilometer,
    x_dot:kilometer_per_second,
    y_dot:kilometer_per_second,
    z_dot: kilometer_per_second,
}

struct KeplerianElements{
    comment:Comment,
    semi_major_axis: kilometer,
    eccentricity:f32,
    inclination:degree,
    ra_of_asc_node: degree,
    arg_of_pericenter: degree,
    anomaly:Anomaly,
    gm:Option<gm>
}

enum Anomaly{
    TrueAnomaly(degree),
    MeanAnomaly(degree)
}

struct SpacecraftParameters{
    comment:Comment,
    mass: kilogram,
    solar_rad_area: Option<square_meter>,
    solar_rad_coff: Option<f32>,
    drag_area: Option<square_meter>,
    drag_coeff: Option<f32>
}

struct PosVelCovariance{
    comment:Comment,
    cov_reference_frame:LocalReferenceFrame,
    cx_x: square_kilometer,
    cy_x: square_kilometer,
    cy_y: square_kilometer,
    cz_x: square_kilometer,
    cz_y: square_kilometer,
    cz_z: square_kilometer,
    cx_dot_x: square_kilometer_per_second,
    cx_dot_y: square_kilometer_per_second,
    cx_dot_z: square_kilometer_per_second,
    cx_dot_x_dot: square_kilometer_per_second_squared,
    cy_dot_x: square_kilometer_per_second,
    cy_dot_y: square_kilometer_per_second,
    cy_dot_z: square_kilometer_per_second,
    cy_dot_x_dot: square_kilometer_per_second_squared,
    cy_dot_y_dot: square_kilometer_per_second_squared,
    cz_dot_x: square_kilometer_per_second,
    cz_dot_y: square_kilometer_per_second,
    cz_dot_z: square_kilometer_per_second,
    cz_dot_x_dot: square_kilometer_per_second_squared,
    cz_dot_y_dot: square_kilometer_per_second_squared,
    cz_dot_z_dot: square_kilometer_per_second_squared
}

struct ManeuverParameters{
    comment:Comment,
    man_epoch_ignition:DateTime<Utc>,
    man_duration:second,
    man_delta_mass:kilogram,
    man_reference_frame:LocalReferenceFrame,
    man_dv1:kilometer_per_second,
    man_dv2: kilometer_per_second,
    man_dv3: kilometer_per_second
}

struct OMM{
    header: Header,
    meta_data: OMMMetaData,
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

struct TLEParameters{
    comment:Comment,
    ephemeris_type: Option<i8>,
    classification_type: Option<str>,
    norad_cat_id: Option<i32>,
    element_set_no: Option<i32>,
    rev_at_epoch: Option<f32>,
    bterm:square_meter_per_kilogram,
    mean_motion_dot:cycle_per_day_squared,
    agom: square_meter_per_kilogram
}

