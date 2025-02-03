# Development

Your new jumpstart project includes basic organization with an organized `assets` folder and a `components` folder. 
If you chose to develop with the router feature, you will also have a `views` folder.

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

# Ciggy Buddy - Cigarette Tracking App
A minimalist mobile app for tracking cigarette usage, built with Rust and Dioxus for Android.

## Features
- Track cigarettes smoked across multiple time periods:
    - Daily
    - Weekly
    - Monthly
    - Yearly
    - Total

- Simple one-tap logging
- Local data storage (no cloud synchronization)
- Clean, intuitive mobile-first interface

## Prerequisites
- Android device with USB debugging enabled
- Rust toolchain (1.70+)
- Dioxus CLI (`cargo install dioxus-cli`)
- Android SDK/NDK configured

## Building & Deployment
### Compile the APK
```bash
dx build --platform android # Debug build
dx build --platform android --release # Release build
```

###  Serving the app:
```bash
dx serve --platform android
```

###  Install on Device
```bash
adb install target/dx/ciggy-buddy/debug/android/app/app/build/outputs/apk/debug/app-debug.apk
```
### Find Package Name
```bash
adb shell pm list packages | grep ciggy
```
## Development Workflow
### View Logs
```bash
adb logcat | grep -E 'Dioxus|Rust'
```
### Access App Data
```bash
adb shell
run-as com.example.CiggyBuddy
cat files/smoking_data.json
```
## Common ADB Commands
| Command | Description |
|---------|-------------|
| `adb devices` | List connected devices |
| `adb uninstall <package>` | Remove the app |
| `adb shell input keyevent KEYCODE_WAKEUP` | Wake device |
| `adb reboot` | Reboot device |

## Architecture
### Tech Stack:
- Rust (core logic)
- Dioxus (UI framework)
- JSON storage
- Chrono (date/time handling)

### Key Components:
- Daily counter with automatic rollover
- Persistent local storage
- Touch-optimized UI
- Mobile-first responsive design


