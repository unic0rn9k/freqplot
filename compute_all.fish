#!/usr/bin/env fish

cargo b --release

set -x JOBS 4

echo "Computing fourier transforms on $JOBS threads..."

fd '.*(left|right).*\.mp3$' | parallel -j $JOBS '
    set -x AUDIO_FILE {};
    echo "== Processing $AUDIO_FILE ==";
    ./target/release/freqplot'
