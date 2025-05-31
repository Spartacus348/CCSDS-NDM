/*
Holds the data sections
 */
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use std::iter::Map;
use uom::si::{
    acceleration::kilometer_per_second_squared,
    angle::degree,
    area::{square_kilometer, square_meter},
    length::kilometer,
    mass::kilogram,
    specific_area::square_meter_per_kilogram,
    time::second,
    velocity::kilometer_per_second,
};

use common::{Comment, UTCTime};
use constants::{Classification, EphemerisType, InterpolationType, LocalReferenceFrame};
use unit::{
    area_rate::kilometer_squared_per_second,
    gm::cubic_kilometers_per_second_square,
    si_eu::{angular_acceleration::cycles_per_day_squared, angular_jerk::cycles_per_day_cubed},
    velocity_squared::kilometer_squared_per_second_squared,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) struct StateVector {
    comment: Comment,
    epoch: UTCTime,
    x: kilometer,
    y: kilometer,
    z: kilometer,
    x_dot: kilometer_per_second,
    y_dot: kilometer_per_second,
    z_dot: kilometer_per_second,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) struct KeplerianElements {
    comment: Comment,
    semi_major_axis: kilometer,
    eccentricity: f32,
    inclination: degree,
    ra_of_asc_node: degree,
    arg_of_pericenter: degree,
    true_anomaly: Option<degree>,
    mean_anomaly: Option<degree>,
    gm: Option<cubic_kilometers_per_second_square>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) struct SpacecraftParameters {
    comment: Comment,
    mass: kilogram,
    solar_rad_area: Option<square_meter>,
    solar_rad_coff: Option<f32>,
    drag_area: Option<square_meter>,
    drag_coeff: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) struct PosVelCovariance {
    comment: Comment,
    cov_reference_frame: LocalReferenceFrame,
    cx_x: square_kilometer,
    cy_x: square_kilometer,
    cy_y: square_kilometer,
    cz_x: square_kilometer,
    cz_y: square_kilometer,
    cz_z: square_kilometer,
    cx_dot_x: kilometer_squared_per_second,
    cx_dot_y: kilometer_squared_per_second,
    cx_dot_z: kilometer_squared_per_second,
    cx_dot_x_dot: kilometer_squared_per_second_squared,
    cy_dot_x: kilometer_squared_per_second,
    cy_dot_y: kilometer_squared_per_second,
    cy_dot_z: kilometer_squared_per_second,
    cy_dot_x_dot: kilometer_squared_per_second_squared,
    cy_dot_y_dot: kilometer_squared_per_second_squared,
    cz_dot_x: kilometer_squared_per_second,
    cz_dot_y: kilometer_squared_per_second,
    cz_dot_z: kilometer_squared_per_second,
    cz_dot_x_dot: kilometer_squared_per_second_squared,
    cz_dot_y_dot: kilometer_squared_per_second_squared,
    cz_dot_z_dot: kilometer_squared_per_second_squared,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) struct ManeuverParameters {
    comment: Comment,
    man_epoch_ignition: DateTime<Utc>,
    man_duration: second,
    man_delta_mass: kilogram,
    man_reference_frame: LocalReferenceFrame,
    man_dv1: kilometer_per_second,
    man_dv2: kilometer_per_second,
    man_dv3: kilometer_per_second,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) struct TLEParameters {
    comment: Comment,
    ephemeris_type: EphemerisType,
    classification_type: Classification,
    norad_cat_id: Option<i32>,
    element_set_no: Option<i32>,
    rev_at_epoch: Option<f32>,
    bterm: square_meter_per_kilogram,
    mean_motion_dot: cycles_per_day_squared,
    mean_motion_ddot: Option<cycles_per_day_cubed>,
    agom: square_meter_per_kilogram,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) struct ExtendedStateVector {
    state_vector: StateVector,
    accel_block: Option<AccelVector>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct AccelVector {
    x_ddot: kilometer_per_second_squared,
    y_ddot: kilometer_per_second_squared,
    z_ddot: kilometer_per_second_squared,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) struct InterpolationInfo {
    interpolation: InterpolationType,
    interpolation_degree: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) struct NextLeapInfo {
    next_leap_epoch: UTCTime,
    next_leap_taimutc: second,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) struct UserParameters {
    params: Map<String, String>,
}
