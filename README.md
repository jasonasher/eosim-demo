This is a demonstration SIR model using `eosim`.

To run the model:

`cargo build -r`

Run a single example scenario:

`target/release/eosim-demo -i test/input/config.yaml -o test/output/`

Run multiple example scenarios with 4 threads:

`target/release/eosim-demo -i test/input/config_multi.yaml -o test/output/ -t 4`

Inspect the output in R using the script `test/output/plot_output.R`