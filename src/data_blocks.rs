/*
Holds the data sections
 */
use std::iter::Map;
use chrono::{DateTime, Utc};
use uom::si::{
    angle::degree,
    area::{
        square_kilometer,
        square_meter},
    length::kilometer,
    mass::kilogram,
    Quantity as Q,
    specific_area::square_meter_per_kilogram,
    time::second,
    velocity::kilometer_per_second};
use uom::si::acceleration::kilometer_per_second_squared;
use constants::{Classification, EphemerisType, InterpolationType, LocalReferenceFrame};
use crate::common::{Comment, UTCTime};

#[macro_use]
mod gm {
    quantity! {
        quantity: GM; "gravitational parameter";
        dimension: Q<P3,Z0,N2>;
        units {
            @mu: 1.0E0; "m^3/s^2", "meter^3 per second^2", "meters^3 per seconds^2";
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
            @kilometer_squared_per_second_squared: 1.0E6; "km^2/s^2","kilometer^2 per second^2", "kilometers^2 per second^2";
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
            @revolutions_per_day_squared: 7.46496E9; "1/day^2","day per second^2", "days per second^2";
        }
    }
}


pub(crate) struct StateVector{
    comment:Comment,
    epoch:UTCTime,
    x:kilometer,
    y:kilometer,
    z:kilometer,
    x_dot:kilometer_per_second,
    y_dot:kilometer_per_second,
    z_dot: kilometer_per_second,
}

pub(crate) struct KeplerianElements{
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

pub(crate) struct SpacecraftParameters{
    comment:Comment,
    mass: kilogram,
    solar_rad_area: Option<square_meter>,
    solar_rad_coff: Option<f32>,
    drag_area: Option<square_meter>,
    drag_coeff: Option<f32>
}

pub(crate) struct PosVelCovariance{
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

pub(crate) struct ManeuverParameters{
    comment:Comment,
    man_epoch_ignition:DateTime<Utc>,
    man_duration:second,
    man_delta_mass:kilogram,
    man_reference_frame:LocalReferenceFrame,
    man_dv1: kilometer_per_second,
    man_dv2: kilometer_per_second,
    man_dv3: kilometer_per_second
}

pub(crate) struct TLEParameters{
    comment:Comment,
    ephemeris_type: EphemerisType,
    classification_type: Classification,
    norad_cat_id: Option<i32>,
    element_set_no: Option<i32>,
    rev_at_epoch: Option<f32>,
    bterm: square_meter_per_kilogram,
    mean_motion_dot: cycle_per_day_squared,
    agom: square_meter_per_kilogram
}

pub(crate) struct ExtendedStateVector{
    state_vector:StateVector,
    accel_block:Option<AccelVector>
}

struct AccelVector{
    x_ddot: kilometer_per_second_squared,
    y_ddot: kilometer_per_second_squared,
    z_ddot: kilometer_per_second_squared
}


pub(crate) struct InterpolationInfo{
    interpolation:InterpolationType,
    interpolation_degree: i32,
}

pub(crate) struct NextLeapInfo{
    next_leap_epoch:UTCTime,
    next_leap_taimutc:second
}

pub(crate) struct UserParameters{
    params: Map<String,String>
}
