/*
holds certain constants, like valid reference frames
 */

pub(crate) enum TimeSystem {
    GMST, GPS, MET, MRT,
    SCLK, TAI, TCB, TDB,
    TCG, TT, UT1, UTC
}

pub(crate) enum ReferenceFrame {
    EME2000, GCRF, GRC, ICRF,
    ITRF2000, ITRF93, ITRF97,
    MCI, TDR, TEME, TOD
}

pub(crate) enum NaturalBody {}

pub(crate) enum LocalReferenceFrame{
    RSW, RTN, TNW
}