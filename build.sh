#!/bin/sh

# TODO: work this into Cargo somehow
time cargo run --release | magick - output.png
