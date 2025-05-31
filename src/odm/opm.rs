/*
Holds the OPM object
 */
use common::{Comment, CommonMetaData, Header};
use data_blocks::{
    KeplerianElements, ManeuverParameters, PosVelCovariance, SpacecraftParameters, StateVector,
    UserParameters,
};
use docs;

#[derive(Debug)]
pub struct OPM {
    header: Header,
    metadata: CommonMetaData,
    data: OPMData,
    comment: Comment,
}

#[derive(Debug)]
pub struct OPMData {
    state_vector_components: StateVector,
    osculating_keplerian_elements: Option<KeplerianElements>,
    spacecraft_parameters: Option<SpacecraftParameters>,
    pos_vel_covariance: Option<PosVelCovariance>,
    maneuver_parameters: Option<Vec<ManeuverParameters>>,
    user_defined_parameters: Option<UserParameters>,
}

impl OPM{
    pub fn new(source: docs::opm::OPM) -> Self {
        todo!()
    }
}
