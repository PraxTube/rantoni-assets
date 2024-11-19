files=(./render/punch-*.png) # Get all matching files in an array
total=${#files[@]}          # Total number of files

# Find the starting index (22 in this case)
start=12

# Reorder the files
reordered_files=()
for ((i=0; i<total; i++)); do
  index=$(( (start + i) % total )) # Wrap around using modulo
  reordered_files+=("${files[index]}")
done

for i in "${!reordered_files[@]}"; do
  f="${reordered_files[i]}"
  magick "$f" -scale 700x700 -crop 700x555+0+40 tmp.png
  magick background.png tmp.png -gravity center -composite "cover_out/cover_$(printf '%02d' $i).png"
done

framerate=15

i="./cover_out/cover_%02d.png"
ffmpeg -framerate $framerate -i $i -vf "scale=700:555:flags=neighbor,palettegen" -y out_pal.png
ffmpeg -framerate $framerate -i $i -i out_pal.png -lavfi "scale=700:555:flags=neighbor [x]; [x][1:v] paletteuse=dither=none" -plays 0 -y "cover.gif"
