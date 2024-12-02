#!/bin/bash

# Adjust these names

BLENDER_FILE="../rantoni.blend"

TRICKFILM="dude/dude.trickfilm.ron"
CHARACTER_NAME="dude"

TEXTURE_OUT="assets/dude"
TRICKFILM_OUT="assets/$TRICKFILM"

# Don't change stuff below

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

"$SCRIPT_DIR/main.sh" "$SCRIPT_DIR/$BLENDER_FILE" $TEXTURE_OUT $TRICKFILM_OUT $CHARACTER_NAME $TRICKFILM
