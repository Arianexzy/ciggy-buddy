#!/bin/bash

# Define file path
PROJECT_DIR="../target/dx/ciggy-buddy/debug/android/app/app/src/main/res"
ADAPTIVE_FILE="$PROJECT_DIR/mipmap-anydpi-v26/ic_launcher.xml"

# Ensure the project directory exists
if [ ! -d "$PROJECT_DIR" ]; then
    echo "❌ Error: Project directory '$PROJECT_DIR' not found!"
    exit 1
fi

# Ensure the mipmap directory exists
if [ ! -d "$PROJECT_DIR/mipmap-anydpi-v26" ]; then
    echo "❌ Error: Mipmap-anydpi-v26 folder not found!"
    exit 1
fi

# Check if the adaptive icon file exists before deleting
if [ -f "$ADAPTIVE_FILE" ]; then
    echo "🗑️  Disabling adaptive icons by removing '$ADAPTIVE_FILE'..."
    rm "$ADAPTIVE_FILE"

    # Confirm removal
    if [ ! -f "$ADAPTIVE_FILE" ]; then
        echo "✅ Adaptive icons disabled successfully!"
    else
        echo "❌ Error: Failed to delete '$ADAPTIVE_FILE'!"
        exit 1
    fi
else
    echo "⚠️  Adaptive icon file not found. It might already be disabled."
fi
