#!/bin/bash
# Force register the app with LaunchServices
APP_PATH="/Applications/URLRouter.app"

if [ ! -d "$APP_PATH" ]; then
    echo "URLRouter.app not found in /Applications. Please install it first."
    exit 1
fi

echo "Registering $APP_PATH with LaunchServices..."
/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -f "$APP_PATH"
echo "Done."
echo "You might need to restart System Settings to see it in the list."
