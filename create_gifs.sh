#!/bin/bash

BLENDER_FILE="./rantoni.blend"
PYTHON_SCRIPT="./gif_render.py"

framerate=12
# Must be multiple of 100 for keeping pixel art proper.
scale=500

inputs=(
  "idle-o2"
  "jab-o0"
  "jab-o1"
  "jab-o2"
  "jab-o3"
  "punch-o0"
  "punch-o1"
  "punch-o2"
  "punch-o3"
  "spinning_back_kick-o0"
  "spinning_back_kick-o1"
  "spinning_back_kick-o2"
  "spinning_back_kick-o3"
  "death-o0"
  "death-o1"
  "death-o2"
  "death-o3"
  "axe_kick-o0"
  "axe_kick-o1"
  "axe_kick-o2"
  "axe_kick-o3"
  "front_kick-o0"
  "front_kick-o1"
  "front_kick-o2"
  "front_kick-o3"
)

for input in "${inputs[@]}"; do
  i="./render/$input-%04d.png"
  ffmpeg -framerate $framerate -i $i -vf "scale=$scale:$scale:flags=neighbor,palettegen" -y out_pal.png
  ffmpeg -framerate $framerate -i $i -i out_pal.png -lavfi "scale=$scale:$scale:flags=neighbor [x]; [x][1:v] paletteuse=dither=none" -plays 0 -y $"$input.gif"
done

rm out_pal.png

convert jab-o0.gif jab-o1.gif jab-o2.gif jab-o3.gif jab.gif
convert punch-o0.gif punch-o1.gif punch-o2.gif punch-o3.gif punch.gif
convert spinning_back_kick-o0.gif spinning_back_kick-o1.gif spinning_back_kick-o2.gif spinning_back_kick-o3.gif spinning_back_kick.gif
convert death-o0.gif death-o1.gif death-o2.gif death-o3.gif death.gif
convert axe_kick-o0.gif axe_kick-o1.gif axe_kick-o2.gif axe_kick-o3.gif axe_kick.gif
convert front_kick-o0.gif front_kick-o1.gif front_kick-o2.gif front_kick-o3.gif front_kick.gif

rm *-o?.gif
