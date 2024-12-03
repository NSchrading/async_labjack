#!/bin/bash

set -e

cd src/ljm_constants

# Build the tag generator
if ! cargo build --release; then
    echo "Failed to build generate_tags in src/ljm_constants. Exiting."
    exit 1
fi

# Navigate back to the project root
cd ../..

# Run the tag generator
./src/ljm_constants/target/release/generate_tags || echo "Tag generation failed."

# fixup any suggestions, e.g. wrapping html links
cargo fix --lib -p tokio_labjack || echo "cargo fix --lib failed"