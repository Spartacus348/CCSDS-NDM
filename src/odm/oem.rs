/*
Holds the OEM object
 */

use common::{Comment, CommonMetaData, Header, UTCTime};
use data_blocks::{ExtendedStateVector, InterpolationInfo, PosVelCovariance};

struct OEM{
    header: Header,
    metadata: OEMMetaData,
    comment: Comment,
    ephemeris:[OEMData],
    pos_vel_covariance: PosVelCovariance
}

struct OEMMetaData{
    common_meta_data: CommonMetaData,
    start_time: UTCTime,
    useable_start_time: Option<UTCTime>,
    useable_stop_time: Option<UTCTime>,
    stop_time: UTCTime,
    interpolation:Option<InterpolationInfo>
}

struct OEMData{
    cartesians:ExtendedStateVector,
    covariance:Option<PosVelCovariance>,
    covariance_epoch: UTCTime
}
