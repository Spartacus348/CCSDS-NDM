/*
holds certain constants, like valid reference frames
 */

use std::collections::HashMap;

pub(crate) enum TimeSystem {
    GMST, GPS, MET, MRT,
    SCLK, TAI, TCB, TDB,
    TCG, TT, UT1, UTC,
    OTHER(str)
}

pub(crate) enum ReferenceFrame {
    EME2000, GCRF, GRC, ICRF,
    ITRF2000, ITRF93, ITRF97,
    MCI, TDR, TEME, TOD,
    OTHER(str)
}

pub(crate) enum NaturalBody {
    EARTH, MOON, SUN, EarthMoonBarycenter,
    SunJupyterBarycenter, OTHER(str)
}

pub(crate) enum LocalReferenceFrame{
    RSW, RTN, TNW, Other(str)
}

pub(crate) enum InterpolationType{
    Hermite,Linear,Lagrange,Other(str)
}

pub(crate) enum Classification{
    U, S, Other(str)
}

pub(crate) enum EphemerisType{
    SGP, SGP4, PPT3, SGP4_XP, Special_Perturbations,
    OtherStr(str), OtherInt(i8)
}

pub(crate) const EPHEMERIS_TYPE_KEY: HashMap<i8,EphemerisType> = HashMap::from([
    (0,EphemerisType::SGP),
    (2,EphemerisType::SGP4),
    (3,EphemerisType::PPT3),
    (4,EphemerisType::SGP4_XP),
    (6,EphemerisType::Special_Perturbations)
]);
