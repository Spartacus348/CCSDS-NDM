// the raw info for custom ser/deser

use serde_derive::{Deserialize, Serialize};
use common::Comment;
use constants::Classification;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OPM{
    header: OPMHeader,
    metadata: OPMMetadata,
    data: OPMData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct OPMHeader {
    ccsds_opm_version: String,
    comment: Comment,
    classification: Option<Classification>,
    creation_date: String,
    originator: String,
    message_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct OPMMetadata{
    comment: Comment,
    object_name: String,
    object_id: String,
    center_name: String,
    ref_frame: String,
    ref_frame_epoch: Option<String>,
    time_system: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct OPMData{
    state_vector: StateVectorDoc,
    keplerian_elements: Option<KeplerianElementsDoc>,
    spacecraft_parameters: Option<SpacecraftParametersDoc>,
    pos_vel_covariance: Option<PosVelCovarianceDoc>,
    maneuver_parameters: Option<Vec<ManeuverParametersDoc>>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct StateVectorDoc{
    comment: Comment,
    epoch: String,
    x: f32,
    y: f32,
    z: f32,
    x_dot: f32,
    y_dot: f32,
    z_dot: f32
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct KeplerianElementsDoc{
    comment: Comment,
    semi_major_axis: f32,
    eccentricity: f32,
    inclination: f32,
    ra_of_asc_node: f32,
    arg_of_pericenter: f32,
    true_anomaly: Option<f32>,
    mean_anomaly: Option<f32>,
    gm: f32
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct SpacecraftParametersDoc{
    comment: Comment,
    mass: Option<f32>,
    solar_rad_area: Option<f32>,
    solar_rad_coeff: Option<f32>,
    drag_area: Option<f32>,
    drag_coeff: Option<f32>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct PosVelCovarianceDoc{
    comment: Comment,
    cov_ref_frame: Option<f32>,
    cx_x: f32,
    cy_x: f32,
    cz_x: f32,
    cx_dot_x: f32,
    cx_dot_y: f32,
    cx_dot_z: f32,
    cx_dot_x_dot: f32,
    cy_dot_x: f32,
    cy_dot_y: f32,
    cy_dot_z: f32,
    cy_dot_x_dot: f32,
    cy_dot_y_dot: f32,
    cz_dot_x: f32,
    cz_dot_y: f32,
    cz_dot_z: f32,
    cz_dot_x_dot: f32,
    cz_dot_y_dot: f32,
    cz_dot_z_dot: f32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct ManeuverParametersDoc{
    comment: Comment,
    man_epoch_ignition: Option<String>,
    man_duration: Option<f32>,
    man_delta_mass: Option<f32>,
    man_ref_frame: Option<String>,
    man_dv_1: Option<f32>,
    man_dv_2: Option<f32>,
    man_dv_3: Option<f32>,
}
