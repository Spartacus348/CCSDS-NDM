use uom::si::{length, mass, time};

#[macro_use]
pub(crate) mod gm {
    quantity! {
        quantity: GM; "gravitational parameter";
        dimension: CustomSI<P3,Z0,N2>;
        units {
            @cubic_meters_per_second_square: 1.0E0; "m³/s²", "cubic meter per second squared", "meters^3 per second squared";
            @cubic_kilometers_per_second_square: 1.0E9; "km³/s²", "cubic kilometer per second squared", "cubic kilometers per second squared";
        }
    }
}

#[macro_use]
pub(crate) mod area_rate {
    quantity! {
        quantity: AreaRate; "Rate of Area";
        dimension: CustomSI<P2,Z0,N1>;
        units {
            @meter_squared_per_second: 1.0E0; "m²/s","square meter per second", "square meters per second";
            @kilometer_squared_per_second: 1.0E6; "km²/s","square kilometer per second", "square kilometers per second";
        }
    }
}

#[macro_use]
pub(crate) mod velocity_squared {
    quantity! {
        quantity: VelocitySquared; "velocity squared";
        dimension: CustomSI<P2,Z0,N2>;
        units {
            @meter_squared_per_second_squared: 1.0E0; "m²/s²","square meter per second squared", "square meters per second squared";
            @kilometer_squared_per_second_squared: 1.0E6; "km²/s²","square kilometer per second squared", "square kilometers per second squared";
        }
    }
}

#[macro_use]
pub(crate) mod specific_area {
    quantity! {
        quantity: SpecificArea; "specific area";
        dimension: CustomSI<P2,N1,Z0>;
        units {
            @square_meters_per_kg: 1.0E0; "m²/kg", "square meter per kg", "square meters per kg";
        }
    }
}

system! {
    quantities: CustomSI {
        length: meter, L;
        mass: kilogram, M;
        time: second, T;
    }

    units: U {
        mod gm::GM,
        mod square_kilometer_per_second::KmSqPerSec,
        mod square_kilometer_per_second_squared::KmSqPerSecSq,
        mod cycle_per_day_squared::RevPerDaySq,

    }
}

pub(crate) mod si_eu {
    //modifications to existing SI units
    pub(crate) mod angular_acceleration {
        unit! {
            system: uom::si;
            quantity: uom::si::angular_acceleration;

            @cycles_per_day_squared: 86400.0*86400.0; "1/day²", "revolution per day per day", "revolutions per day per day";
        }
    }

    pub(crate) mod angular_jerk {
        unit! {
            system: uom::si;
            quantity: uom::si::angular_jerk;

            @cycles_per_day_cubed: 86400.0*86400.0*86400.0; "1/day³", "revolution per day per day squared", "revolutions per day per day squared";
        }
    }
}
