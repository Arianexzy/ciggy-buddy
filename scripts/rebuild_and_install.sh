#!/bin/bash

# Set paths
PROJECT_DIR="../target/dx/ciggy-buddy/debug/android/app"
GRADLEW="$PROJECT_DIR/gradlew"
APK_PATH="$PROJECT_DIR/app/build/outputs/apk/debug/app-debug.apk"

# Ensure gradlew exists
if [ ! -f "$GRADLEW" ]; then
    echo "❌ Error: 'gradlew' not found at '$GRADLEW'!"
    exit 1
fi

# Ensure adb is installed
if ! command -v adb &> /dev/null; then
    echo "❌ Error: 'adb' command not found! Please install Android Platform Tools."
    exit 1
fi

# Clean the project
echo "🧹 Cleaning project..."
"$GRADLEW" -p "$PROJECT_DIR" clean
if [ $? -ne 0 ]; then
    echo "❌ Error: Gradle clean failed!"
    exit 1
fi

# Build the APK
echo "⚙️  Building APK in debug mode..."
"$GRADLEW" -p "$PROJECT_DIR" assembleDebug
if [ $? -ne 0 ]; then
    echo "❌ Error: Build failed!"
    exit 1
fi

# Ensure APK exists before installing
if [ ! -f "$APK_PATH" ]; then
    echo "❌ Error: APK not found at '$APK_PATH'!"
    exit 1
fi

# Install APK
echo "📲 Installing new APK..."
adb install -r "$APK_PATH"
if [ $? -ne 0 ]; then
    echo "❌ Error: Failed to install APK!"
    exit 1
fi

echo "🎉 Build and install complete!"
