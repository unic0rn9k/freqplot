#!/usr/bin/env fish

fd '.*left.*\.mp3$' | while read AUDIO_FILE
    echo "== Processing $AUDIO_FILE =="
    AUDIO_FILE=$AUDIO_FILE ./target/release/freqplot
end
