uom::quantity! {
    quantity: Area; "area";
    dimension: IAUQ<
        P2,
        Z0,
        Z0>;

    units {
        @square_astronomical_unit: 1.0; "au2", "square astronomical unit", "square astronomical units";

        @square_centimeter: 4.468_370_5_E-29; "cm2", "square centimeter", "square centimeters";
        @square_meter: 4.468_370_5_E-23; "m2", "square meter", "square meters";
        @square_kilometer: 4.468_370_5_E-17; "km2", "square kilometer", "square kilometers";
        @square_gigameter: 4.468_370_5_E-5; "Gm2", "square gigameter", "square gigameters";
        @square_lunar_distance: 6.602_580_034_54_E-6; "LD2", "square lunar distance", "square lunar distances";
        @square_light_year: 3.999_433_830_78_E9; "ly2", "square light year", "square light years";
        @square_parsec: 4.254_517_029_61_E10; "pc2", "square parsec", "square parsecs";
        @square_kiloparsec: 4.254_517_029_61_E16; "kpc2", "square kiloparsec", "square kiloparsecs";
        @square_megaparsec: 4.254_517_029_61_E22; "Mpc2", "square megaparsec", "square megaparsecs";
    }
}

/*
#[cfg(test)]
mod tests {
    uom::storage_types! {
        use uom::num_traits::One;
        use crate::iau::length::*;
        use crate::iau::area::*;
        use crate::iau::length::Length;
        use crate::iau::area::Area;

        #[test]
        fn check_dimension() {
            let _: Area<V> =
                Length::new::<astronomical_unit>(V::one()) * Length::new::<astronomical_unit>(V::one());
        }
    }
}
*/
