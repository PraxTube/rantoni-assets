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
  "run-o0"
  "run-o1"
  "run-o2"
  "run-o3"
  "parry-o0"
  "parry-o1"
  "parry-o2"
  "parry-o3"
  "parry_success-o0"
  "parry_success-o1"
  "parry_success-o2"
  "parry_success-o3"
  "fall-o0"
  "fall-o1"
  "fall-o2"
  "fall-o3"
  "fall_recover-o0"
  "fall_recover-o1"
  "fall_recover-o2"
  "fall_recover-o3"
  "slide-o0"
  "slide-o1"
  "slide-o2"
  "slide-o3"
  "slide_recover-o0"
  "slide_recover-o1"
  "slide_recover-o2"
  "slide_recover-o3"
  "stance_broken-o0"
  "stance_broken-o1"
  "stance_broken-o2"
  "stance_broken-o3"
  "stance_broken_recover-o0"
  "stance_broken_recover-o1"
  "stance_broken_recover-o2"
  "stance_broken_recover-o3"
  "left_hook-o0"
  "left_hook-o1"
  "left_hook-o2"
  "left_hook-o3"
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
convert run-o0.gif run-o1.gif run-o2.gif run-o3.gif run.gif
convert parry-o0.gif parry_success-o0.gif parry-o1.gif parry_success-o1.gif parry-o2.gif parry_success-o2.gif parry-o3.gif parry_success-o3.gif parry.gif
convert fall-o0.gif fall_recover-o0.gif fall-o1.gif fall_recover-o1.gif fall-o2.gif fall_recover-o2.gif fall-o3.gif fall_recover-o3.gif fall.gif
convert slide-o0.gif slide_recover-o0.gif slide-o1.gif slide_recover-o1.gif slide-o2.gif slide_recover-o2.gif slide-o3.gif slide_recover-o3.gif slide.gif
convert stance_broken-o0.gif stance_broken_recover-o0.gif stance_broken-o1.gif stance_broken_recover-o1.gif stance_broken-o2.gif stance_broken_recover-o2.gif stance_broken-o3.gif stance_broken_recover-o3.gif stance_broken.gif
convert left_hook-o0.gif left_hook-o1.gif left_hook-o2.gif left_hook-o3.gif left_hook.gif

rm *-o?.gif
