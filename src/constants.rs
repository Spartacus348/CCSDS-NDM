/*
holds certain constants, like valid reference frames
 */

use common::UTCTime;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum TimeSystem {
    GMST,
    GPS,
    MET(UTCTime),
    MRT(UTCTime),
    SCLK,
    TAI,
    TCB,
    TDB,
    TCG,
    TT,
    UT1,
    UTC,
    OTHER(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum ReferenceFrame {
    EME2000,
    GCRF,
    GRC,
    ICRF,
    ITRF2000,
    ITRF93,
    ITRF97,
    MCI,
    TDR,
    TEME,
    TOD,
    OTHER(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum NaturalBody {
    EARTH,
    MOON,
    SUN,
    EarthMoonBarycenter,
    SunJupyterBarycenter,
    OTHER(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum LocalReferenceFrame {
    RSW,
    RTN,
    TNW,
    Other(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum InterpolationType {
    Hermite,
    Linear,
    Lagrange,
    Other(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Classification {
    U,
    S,
    Other(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum EphemerisType {
    SGP,
    SGP4,
    PPT3,
    SGP4XP,
    SpecialPerturbations,
    OtherStr(String),
    OtherInt(i8),
}

pub(crate) enum TrajectoryBasis {
    Predicted,
    Determined,
    Telemetry,
    Simulated,
    Other,
    FreeForm(String),
}
