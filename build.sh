#!/bin/sh

# TODO: work this into Cargo somehow
cargo run > output.ppm
magick output.ppm output.png
