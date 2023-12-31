#!/bin/bash

# Remove existing 'frames' directory if it exists
rm -rf frames

# Create a new 'frames' directory
mkdir frames

# Extract frames from video using ffmpeg
ffmpeg -i $1 frames/thumb%06d.png -hide_banner
# Build the Rust project
cargo build --release

# Run and measure the execution time of the program
time ./target/release/rustsound

# Combine the video and the sound.
ffmpeg -i $1 -i ses.wav -map 0:v -map 1:a -c:v copy -shortest output.mp4
