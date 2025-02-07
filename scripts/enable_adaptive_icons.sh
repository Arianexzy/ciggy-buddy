#!/bin/bash

## CAN'T GET THIS TO WORK ##

# Define file paths
PROJECT_DIR="../target/dx/ciggy-buddy/debug/android/app/app/src/main/res"
FOREGROUND_FILE="$PROJECT_DIR/drawable/ic_launcher_foreground.xml"
BACKGROUND_FILE="$PROJECT_DIR/drawable/ic_launcher_background.xml"
ADAPTIVE_FILE="$PROJECT_DIR/mipmap-anydpi-v26/ic_launcher.xml"

# Ensure the project directory exists
if [ ! -d "$PROJECT_DIR" ]; then
    echo "❌ Error: Project directory '$PROJECT_DIR' not found!"
    exit 1
fi

# Ensure the drawable directory exists
if [ ! -d "$PROJECT_DIR/drawable" ]; then
    echo "❌ Error: Drawable folder not found!"
    exit 1
fi

# Ensure the mipmap directory exists
if [ ! -d "$PROJECT_DIR/mipmap-anydpi-v26" ]; then
    echo "❌ Error: Mipmap-anydpi-v26 folder not found!"
    exit 1
fi

# Create the foreground XML (replace with actual icon path or vector)
echo "🎨 Creating foreground icon..."
cat > "$FOREGROUND_FILE" <<EOL
<vector xmlns:android="http://schemas.android.com/apk/res/android"
    android:width="108dp"
    android:height="108dp"
    android:viewportWidth="108"
    android:viewportHeight="108">
    <path
        android:fillColor="#FF0000"
        android:pathData="M20,20h60v60h-60z" />
</vector>
EOL

if [ $? -ne 0 ]; then
    echo "❌ Error: Failed to create foreground icon!"
    exit 1
fi
echo "✅ Foreground icon created at '$FOREGROUND_FILE'"

# Create the adaptive icon XML
echo "📝 Creating adaptive icon XML..."
cat > "$ADAPTIVE_FILE" <<EOL
<adaptive-icon xmlns:android="http://schemas.android.com/apk/res/android">
    <background android:drawable="@drawable/ic_launcher_background"/>
    <foreground android:drawable="@drawable/ic_launcher_foreground"/>
</adaptive-icon>
EOL

if [ $? -ne 0 ]; then
    echo "❌ Error: Failed to create adaptive icon XML!"
    exit 1
fi
echo "✅ Adaptive icon configuration created at '$ADAPTIVE_FILE'"

echo "🎉 Adaptive icons enabled successfully!"
