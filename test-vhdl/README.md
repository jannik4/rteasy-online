# Test VHDL Export

Tests for the VHDL export of RTeasy-Online. To run the tests first `cd` into this directory and then execute `cargo run`.

All tests are located in the [testbenches](./testbenches/) folder. A test `X` always consists of `X/X.rt` and `X/X_tb.vhdl`. Tests whose name begins with `fail` must fail to be considered successful.
