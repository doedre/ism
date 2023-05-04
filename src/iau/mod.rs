uom::system! {
    quantities: IAUQ {
        length: astronomical_unit, L;
        mass: solar_mass, M;
        time: day, T;
    }

    units: IAU {
        length::Length,
        mass::Mass,
        time::Time,
    }
}

pub mod quantities {
    IAUQ!(crate::iau);
}

uom::storage_types! {
    pub types: All;

    IAUQ!(crate::iau, V);
}

