#!/bin/sh

# TODO: work this into Cargo somehow
cargo run --release | magick - output.png
