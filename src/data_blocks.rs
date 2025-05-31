/*
Holds the data sections
 */
use common::{Comment, UTCTime};
use constants::{Classification, EphemerisType, InterpolationType, LocalReferenceFrame};
use serde_derive::{Deserialize, Serialize};
use unit::gm::cubic_kilometers_per_second_square;
use uom::si::f32::{
    Acceleration, Angle, AngularAcceleration, AngularJerk, Area, AvailableEnergy,
    DiffusionCoefficient, Length, Mass, SpecificArea, Time, Velocity,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) struct StateVector {
    comment: Comment,
    epoch: UTCTime,
    x: Length,
    y: Length,
    z: Length,
    x_dot: Velocity,
    y_dot: Velocity,
    z_dot: Velocity,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) struct KeplerianElements {
    comment: Comment,
    semi_major_axis: Length,
    eccentricity: f32,
    inclination: Angle,
    ra_of_asc_node: Angle,
    arg_of_pericenter: Angle,
    true_anomaly: Option<Angle>,
    mean_anomaly: Option<Angle>,
    gm: Option<cubic_kilometers_per_second_square>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) struct SpacecraftParameters {
    comment: Comment,
    mass: Mass,
    solar_rad_area: Option<Area>,
    solar_rad_coff: Option<f32>,
    drag_area: Option<Area>,
    drag_coeff: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) struct PosVelCovariance {
    comment: Comment,
    cov_reference_frame: LocalReferenceFrame,
    cx_x: Area,
    cy_x: Area,
    cy_y: Area,
    cz_x: Area,
    cz_y: Area,
    cz_z: Area,
    cx_dot_x: DiffusionCoefficient,
    cx_dot_y: DiffusionCoefficient,
    cx_dot_z: DiffusionCoefficient,
    cx_dot_x_dot: AvailableEnergy,
    cy_dot_x: DiffusionCoefficient,
    cy_dot_y: DiffusionCoefficient,
    cy_dot_z: DiffusionCoefficient,
    cy_dot_x_dot: AvailableEnergy,
    cy_dot_y_dot: AvailableEnergy,
    cz_dot_x: DiffusionCoefficient,
    cz_dot_y: DiffusionCoefficient,
    cz_dot_z: DiffusionCoefficient,
    cz_dot_x_dot: AvailableEnergy,
    cz_dot_y_dot: AvailableEnergy,
    cz_dot_z_dot: AvailableEnergy,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) struct ManeuverParameters {
    comment: Comment,
    man_epoch_ignition: UTCTime,
    man_duration: Time,
    man_delta_mass: Mass,
    man_reference_frame: LocalReferenceFrame,
    man_dv1: Velocity,
    man_dv2: Velocity,
    man_dv3: Velocity,
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
    bterm: SpecificArea,
    mean_motion_dot: AngularAcceleration,
    mean_motion_ddot: Option<AngularJerk>,
    agom: SpecificArea,
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
    x_ddot: Acceleration,
    y_ddot: Acceleration,
    z_ddot: Acceleration,
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
    next_leap_taimutc: Time,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) struct UserParameters {
    //params: Map<String, String>,
}
