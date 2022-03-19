# Test VHDL Export

Tests for the VHDL export of RTeasy-Online. To run the tests first `cd` into this directory and then execute `cargo run`.

## How it works

All tests are located in the [testbenches](./testbenches) folder. A test `X` always consists of the rt code `src/X/X.rt` and a testbench `src/X/X_tb.vhdl`. Tests whose name begins with `fail` must have a failing assert to be considered successful.

For each test `X` the following steps are performed:

1. Parse, compile and generate VHDL code for `src/X/X.rt`. The generated VHDL code is saved in `target/X/X.vhdl`.
2. Analyze
   - `ghdl -a --std=93 X.vhdl`
   - `ghdl -a --std=93 X_tb.vhdl`
3. Elaborate
   - `ghdl -e --std=93 X_tb`
4. Run
   - `ghdl -r --std=93 X_tb --assert-level=error --wave=X.ghw`

After execution, the generated VHDL code can be viewed in `target/X/X.vhdl` and the waveform in `target/X/X.ghw`.
