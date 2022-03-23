#!/usr/bin/env fish

fd '.*left.*\.mp3$' data/fløjte/ | while read AUDIO_FILE
    echo "== Processing $AUDIO_FILE =="
    AUDIO_FILE=$AUDIO_FILE cargo r --release
end
