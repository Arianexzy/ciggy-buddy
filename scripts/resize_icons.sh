#!/bin/bash

# Set the source icon path
ICON_PATH="../assets/icon.png"
DEST_DIR="../assets/fixed_icons"

# Check if the source icon exists
if [ ! -f "$ICON_PATH" ]; then
    echo "❌ Error: Source icon '$ICON_PATH' not found!"
    exit 1
fi

# Ensure the destination directory exists
mkdir -p "$DEST_DIR"

# Define icon sizes
declare -A sizes=(
    ["mdpi"]=48
    ["hdpi"]=72
    ["xhdpi"]=96
    ["xxhdpi"]=144
    ["xxxhdpi"]=192
)

echo "🔄 Generating icons from $ICON_PATH..."

# Loop through sizes and generate icons
for size in "${!sizes[@]}"; do
    convert "$ICON_PATH" -resize "${sizes[$size]}x${sizes[$size]}!" \
        -background transparent -gravity center -extent "${sizes[$size]}x${sizes[$size]}" \
        "$DEST_DIR/$size.png"
    
    if [ $? -eq 0 ]; then
        echo "✅ Created $size icon (${sizes[$size]}x${sizes[$size]})"
    else
        echo "❌ Failed to create $size icon"
    fi
done

echo "🎉 All icons generated and saved in $DEST_DIR!"
