#!/bin/bash

BLENDER_FILE="./rantoni.blend"
PYTHON_SCRIPT="./anim_render.py"

GAME_REPO=../game-dev/rantoni/

TEXTURE_OUT="assets/dude"
TRICKFILM_OUT="assets/dude/dude.trickfilm.ron"

cd $GAME_REPO
GAME_REPO=$(pwd)
if ! git diff-index --quiet HEAD --; then
  echo "Game repo: '$GAME_REPO' has uncommited changes, aborting."
  exit 1;
fi
cd -

blender $BLENDER_FILE --python $PYTHON_SCRIPT --background

cd ./spritesheet-bundler/
cargo run -- "dude" "dude/dude.trickfilm.ron"
mv out/*.png "$GAME_REPO/$TEXTURE_OUT"
mv out/out.trickfilm.ron "$GAME_REPO/$TRICKFILM_OUT"
