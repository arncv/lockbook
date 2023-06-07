#!/bin/bash
set -ae

cargo ndk --target aarch64-linux-android --platform 22 -- build --release
cargo ndk --target armv7-linux-androideabi --platform 22 -- build --release
cargo ndk --target i686-linux-android --platform 22 -- build --release
cargo ndk --target x86_64-linux-android --platform 22 -- build --release

cd ../../..

mkdir -p clients/android/egui-editor/src/main/jniLibs/arm64-v8a/
mkdir -p clients/android/egui-editor/src/main/jniLibs/armeabi-v7a/
mkdir -p clients/android/egui-editor/src/main/jniLibs/x86/
mkdir -p clients/android/egui-editor/src/main/jniLibs/x86_64/

mv target/aarch64-linux-android/release/libegui_editor.so clients/android/egui-editor/src/main/jniLibs/arm64-v8a/
mv target/armv7-linux-androideabi/release/libegui_editor.so clients/android/egui-editor/src/main/jniLibs/armeabi-v7a/
mv target/i686-linux-android/release/libegui_editor.so clients/android/egui-editor/src/main/jniLibs/x86/
mv target/x86_64-linux-android/release/libegui_editor.so clients/android/egui-editor/src/main/jniLibs/x86_64/
