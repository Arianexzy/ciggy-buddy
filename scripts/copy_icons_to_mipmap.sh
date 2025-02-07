#!/bin/bash

# Define paths
SRC_DIR="../assets/fixed_icons"
DEST_DIR="../target/dx/ciggy-buddy/debug/android/app/app/src/main/res"

# Check if source directory exists
if [ ! -d "$SRC_DIR" ]; then
    echo "❌ Error: Source directory '$SRC_DIR' does not exist!"
    exit 1
fi

# Define icon resolutions and corresponding mipmap folders
declare -A mipmap_folders=(
    ["mdpi"]="mipmap-mdpi"
    ["hdpi"]="mipmap-hdpi"
    ["xhdpi"]="mipmap-xhdpi"
    ["xxhdpi"]="mipmap-xxhdpi"
    ["xxxhdpi"]="mipmap-xxxhdpi"
)

echo "🔄 Copying icons to Android project..."

# Loop through the resolutions and copy files
for size in "${!mipmap_folders[@]}"; do
    src_file="$SRC_DIR/$size.png"
    dest_folder="$DEST_DIR/${mipmap_folders[$size]}"
    dest_file="$dest_folder/ic_launcher.webp"

    # Ensure the source file exists
    if [ ! -f "$src_file" ]; then
        echo "❌ Error: Source icon '$src_file' not found!"
        continue
    fi

    # Ensure the destination directory exists
    if [ ! -d "$dest_folder" ]; then
        echo "📁 Creating missing directory: $dest_folder"
        mkdir -p "$dest_folder"
    fi

    # Copy the file
    cp "$src_file" "$dest_file"

    if [ $? -eq 0 ]; then
        echo "✅ Copied $size icon to $dest_file"
    else
        echo "❌ Failed to copy $size icon"
    fi
done

echo "🎉 Icon copying completed!"
