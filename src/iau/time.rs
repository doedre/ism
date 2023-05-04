uom::quantity! {
    quantity: Time; "time";
    dimension: IAUQ<
        Z0,     // length
        Z0,     // mass
        P1>;    // time

    units {
        @day: 1.0; "d", "day", "days";

        @second: 1.157_407_407_41_E-5; "s", "second", "seconds";
        @year: 3.652_5_E2; "y", "year", "years";
    }
}

