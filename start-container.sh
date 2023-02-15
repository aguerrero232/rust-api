#!/bin/bash

# $1 is to run in prod mode
enable_dev_mode=true;

if [ ! -z "$1" ]; then
    if [ "$1" = "prod" ]; then
        enable_dev_mode=false;
    else
        echo "Invalid argument: $1";
        echo "Usage: ./start-container.sh [prod]";
        exit 1;
    fi
fi

# waiting a few seconds for the database to be ready
sleep 10;
# run diesel setup
diesel setup;

if [ $enable_dev_mode = false ]; then
    # run diesel migration
    cargo start;
else
    # run cargo watch to compile and run the project
    cargo watch -x run;
fi