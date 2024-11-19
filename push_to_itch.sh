#!/bin/bash
# It is required to have itch `butler` installed.

itch_target="PraxTube/martial-arts-fighter"
free_zip="free.zip"
premium_zip="premium.zip"
tmp_out="fighter"

rm -rf $tmp_out
mkdir $tmp_out

cp ./spritesheet-bundler/out/*.png $tmp_out/
zip --recurse-paths $premium_zip $tmp_out/

butler push \
  --fix-permissions \
  --userversion=$tag \
  $premium_zip \
  $itch_target:premium

rm -rf $tmp_out
mkdir $tmp_out

cp ./spritesheet-bundler/out/idle.png $tmp_out/
cp ./spritesheet-bundler/out/run.png $tmp_out/
cp ./spritesheet-bundler/out/stagger.png $tmp_out/
cp ./spritesheet-bundler/out/stagger_recover.png $tmp_out/
cp ./spritesheet-bundler/out/jab.png $tmp_out/
cp ./spritesheet-bundler/out/jab_recover.png $tmp_out/
cp ./spritesheet-bundler/out/punch.png $tmp_out/
cp ./spritesheet-bundler/out/punch_recover.png $tmp_out/
cp ./spritesheet-bundler/out/front_kick.png $tmp_out/
cp ./spritesheet-bundler/out/front_kick_recover.png $tmp_out/
cp ./spritesheet-bundler/out/death.png $tmp_out/
zip --recurse-paths $free_zip $tmp_out/

butler push \
  --fix-permissions \
  --userversion=$tag \
  $free_zip \
  $itch_target:free

rm $premium_zip
rm $free_zip
rm -rf $tmp_out
