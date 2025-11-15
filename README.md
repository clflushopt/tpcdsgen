# tpcds but it's Rust instead

This is a WIP port of [Trino's TPCDS](https://github.com/trinodb/tpcds) to Rust that is slowly
taking shape. It is developed out of tree, for now, but will end up as part of
the [tpchgen](https://github.com/clflushopt/tpchgen-rs) once I am satisfied with the port and
probably rewrite a lot of it to be more idiomatic Rust instead of the current Java-ism OOP heavy
mess it is right now.

Currently 12 of 25 tables have been ported with byte-for-byte compatibility verified against the Java
reference implementation. Each table has a dedicated binary generator and automated conformance testing
via CI. Progress: 48% complete (call_center, customer_demographics, date_dim, household_demographics,
income_band, promotion, reason, ship_mode, time_dim, warehouse, web_page, web_site).

## Known Bugs

The TPC-DS reference implementation contains several bugs that must be replicated for benchmark compliance.
These bugs originated in the C implementation and were faithfully reproduced in the Java port. Our Rust implementation
also replicates these bugs to ensure byte-for-byte compatibility with the reference implementation.

See [BUGS.md](BUGS.md) for a detailed list of documented bugs, more will be added.
