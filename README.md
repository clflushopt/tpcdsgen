# tpcds but it's Rust instead

This is a WIP port of [Trino's TPCDS](https://github.com/trinodb/tpcds) to Rust that is slowly
taking shape. It is developed out of tree, for now, but will end up as part of
the [tpchgen](https://github.com/clflushopt/tpchgen-rs) once I am satisfied with the port and
probably rewrite a lot of it to be more idiomatic Rust instead of the current Java-ism OOP heavy
mess it is right now.

Currently it offers no CLI instead I just hack together custom generators on demand to test things
the outputs are tested against the Java implementation (by hand for now) but we will move to CI
and do everything properly later on, for now only 5 tables are supported out of 24.
