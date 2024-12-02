#!/bin/bash

# Adjust these names

BLENDER_FILE="../enemey_goon.blend"

TRICKFILM="enemy/goon/goon.trickfilm.ron"
CHARACTER_NAME="goon"

TEXTURE_OUT="assets/enemy/goon/"
TRICKFILM_OUT="assets/$TRICKFILM"

# Don't change stuff below

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

"$SCRIPT_DIR/main.sh" "$SCRIPT_DIR/$BLENDER_FILE" $TEXTURE_OUT $TRICKFILM_OUT $CHARACTER_NAME $TRICKFILM
