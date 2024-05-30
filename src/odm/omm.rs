/*
Holds the OMM object
 */

use common::{Comment, CommonMetaData, Header};
use data_blocks::{KeplerianElements, PosVelCovariance, SpacecraftParameters,
                  TLEParameters, UserParameters};

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
    user_defined_parameters: Option<UserParameters>
}
