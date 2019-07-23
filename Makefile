
all: copy
	make logcat

build:
	cargo apk build

copy: build
	adb push ./target/android-artifacts/app/build/outputs/apk/debug/app-debug.apk /storage/emulated/0/Downloads/app-debug.apk

logcat:
	adb logcat -c
	adb logcat | grep -a 'threaded_app\|RustStdoutStderr'

