uom::quantity! {
    quantity: Mass; "mass";
    dimension: IAUQ<
        Z0,     // length
        P1,     // mass
        Z0>;    // time

    units {
        @solar_mass: 1.0; "Msun", "solar mass", "solar masses";

        @gram: 1.988_5_E33 ; "g", "gram", "grams";
        @kilogram: 1.988_5_E30 ; "kg", "kilogram", "kilograms";
        @jupiter_mass: 1.047_35_E3; "Mjupiter", "Jupiter mass", "Jupiter masses";
        @earth_mass: 3.329_50_E5; "Mearth", "Earth mass", "Earth masses";
    }
}
