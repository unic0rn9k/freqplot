#!/usr/bin/env fish

fd '.*\.mp3$' | while read file
    rm -v $file
end

fd '.*\.m4a$' | while read file
    echo "Converting $file to mp3 and stripping right channel..."
    ffmpeg -i $file -acodec mp3 $file.mp3
    ffmpeg -i $file.mp3 -map_channel 0.0.0 $file.left.mp3
end
