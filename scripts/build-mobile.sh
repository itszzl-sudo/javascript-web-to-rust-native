#!/bin/bash
# Mobile build script for JRust
# Usage: ./build-mobile.sh [ios|android|all]

set -e

PLATFORM=${1:-all}
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DIST_DIR="$PROJECT_ROOT/dist/mobile"

echo "=== JRust Mobile Build ==="
echo "Platform: $PLATFORM"
echo "Project: $PROJECT_ROOT"

build_ios() {
    echo ""
    echo "--- Building iOS ---"
    
    mkdir -p "$DIST_DIR/ios"
    
    echo "Building for iOS (ARM64)..."
    cargo build --release --target aarch64-apple-ios -p jrust-ios
    
    echo "Building for iOS Simulator..."
    cargo build --release --target aarch64-apple-ios-sim -p jrust-ios
    
    echo "✅ iOS build complete"
    echo "Output: $DIST_DIR/ios/"
}

build_android() {
    echo ""
    echo "--- Building Android ---"
    
    mkdir -p "$DIST_DIR/android"
    
    echo "Building for Android (ARM64)..."
    cargo build --release --target aarch64-linux-android -p jrust-android
    
    echo "Building for Android Emulator (x86_64)..."
    cargo build --release --target x86_64-linux-android -p jrust-android
    
    echo "✅ Android build complete"
    echo "Output: $DIST_DIR/android/"
}

case "$PLATFORM" in
    ios)
        build_ios
        ;;
    android)
        build_android
        ;;
    all)
        build_ios
        build_android
        ;;
    *)
        echo "Usage: $0 [ios|android|all]"
        exit 1
        ;;
esac

echo ""
echo "=== Build Complete ==="
