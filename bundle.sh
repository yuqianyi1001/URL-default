#!/bin/bash
set -e

APP_NAME="URLRouter.app"
BINARY_NAME="url-default"

echo "Building release binary..."
cargo build --release

echo "Creating app bundle structure..."
rm -rf "$APP_NAME"
mkdir -p "$APP_NAME/Contents/MacOS"
mkdir -p "$APP_NAME/Contents/Resources"

echo "Copying files..."
cp "target/release/$BINARY_NAME" "$APP_NAME/Contents/MacOS/"
cp "Info.plist" "$APP_NAME/Contents/"
cp "config.json" "$APP_NAME/Contents/Resources/"

echo "Done. Created $APP_NAME"
