uom::quantity! {
    quantity: Length; "length";
    dimension: IAUQ<
        P1,     // length
        Z0,     // mass
        Z0>;    // time

    units {
        @astronomical_unit: 1.0; "au", "astronomical unit", "astronomical units";

        @centimeter: 6.684_587_1_E-15; "cm", "centimeter", "centimeters";
        @meter: 6.684_587_1_E-12; "m", "meter", "meters";
        @kilometer: 6.684_587_1_E-9; "km", "kilometer", "kilometers";
        @gigameter: 6.684_587_1_E-3; "Gm", "gigameter", "gigameters";
        @lunar_distance: 2.569_548_605_21_E-3; "LD", "lunar distance", "lunar distances";
        @light_year: 6.324_107_708_43_E4; "ly", "light year", "light years";
        @parsec: 2.062_648_062_47_E5; "pc", "parsec", "parsecs";
        @kiloparsec: 2.062_648_062_47_E8; "kpc", "kiloparsec", "kiloparsecs";
        @megaparsec: 2.062_648_062_47_E11; "Mpc", "megaparsec", "megaparsecs";
    }
}

