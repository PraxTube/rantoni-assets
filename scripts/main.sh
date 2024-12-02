#!/bin/bash
# Script that is getting run by all other scripts to render animations.
# Don't change this, if you need to add a new character then simply run the render through this script.
# Parse the arguments correctly.

GAME_REPO=../../game-dev/rantoni/
PYTHON_SCRIPT="../anim_render.py"
SPRITE_BUNDLER="../spritesheet-bundler/"

BLENDER_FILE=$1
TEXTURE_OUT=$2
TRICKFILM_OUT=$3
CHARACTER_NAME=$4
TRICKFILM=$5

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cd "$SCRIPT_DIR/$GAME_REPO"
GAME_REPO=$(pwd)
if ! git diff-index --quiet HEAD --; then
  echo "Game repo: '$GAME_REPO' has uncommited changes, aborting."
  exit 1;
fi
cd "$SCRIPT_DIR/.."

if [ -d ./render/ ]; then
  rm -rf ./render/
fi
blender $BLENDER_FILE --python "$SCRIPT_DIR/$PYTHON_SCRIPT" --background

cd "$SCRIPT_DIR/$SPRITE_BUNDLER"
cargo run -- $CHARACTER_NAME $TRICKFILM
mv out/*.png "$GAME_REPO/$TEXTURE_OUT"
mv out/out.trickfilm.ron "$GAME_REPO/$TRICKFILM_OUT"
