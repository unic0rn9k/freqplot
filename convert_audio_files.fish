#!/usr/bin/env fish

fd '.*\.mp3$' | while read file
    rm -v "$file"
end

fd '.*\.m4a$' | while read file
    echo "Converting $file to left and right mp3 mono channel audio files"
    ffmpeg -i "$file" -acodec mp3 "$file".mp3
    ffmpeg -i "$file".mp3 -map_channel 0.0.0 "$file".left.mp3
    ffmpeg -i "$file".mp3 -map_channel 0.0.1 "$file".right.mp3
end
