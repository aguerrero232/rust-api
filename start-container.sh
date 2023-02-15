#!/bin/bash
# waiting a few seconds for the database to be ready
sleep 10;

# run diesel setup
diesel setup;

# run cargo watch to compile and run the project
cargo watch -x run;