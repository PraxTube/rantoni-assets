#!/bin/bash

BLENDER_FILE="./rantoni.blend"
PYTHON_SCRIPT="./anim_render.py"
# TODO: Improve the spritesheet bundler
# TODO: Make the output locations variables

blender $BLENDER_FILE --python $PYTHON_SCRIPT --background

cd ./spritesheet-bundler/
cargo run -- /home/rancic/code/rantoni-assets/
mv out/*.png ~/code/game-dev/rantoni/assets/dude/

# TODO: Move the out.trickfilm to the target location in rantoni
