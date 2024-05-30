/*
Holds the OPM object
 */

use common::{Comment, CommonMetaData, Header};
use data_blocks::{KeplerianElements, ManeuverParameters, PosVelCovariance,
                  SpacecraftParameters, StateVector, UserParameters};

pub struct OPM{
    header: Header,
    metadata: CommonMetaData,
    data: OPMData,
    comment:Comment
}

pub struct OPMData{
    state_vector_components: StateVector,
    osculating_keplerian_elements: Option<KeplerianElements>,
    spacecraft_parameters: Option<SpacecraftParameters>,
    pos_vel_covariance: Option<PosVelCovariance>,
    maneuver_parameters:Option<[ManeuverParameters]>,
    user_defined_parameters:Option<UserParameters>
}
