framerate=30

ffmpeg -i input.mkv -vf "fps=$framerate,scale=1280:720:flags=neighbor,palettegen" -y out_pal.png
ffmpeg -i input.mkv -i out_pal.png -lavfi "fps=$framerate,scale=1280:720:flags=neighbor [x]; [x][1:v] paletteuse=dither=none" -plays 0 -y output.gif
