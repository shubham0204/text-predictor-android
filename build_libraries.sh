cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target i686-linux-android --release
cargo build --target x86_64-linux-android --release

mkdir -p android/jniLibs/arm64-v8a
mkdir -p android/jniLibs/armeabi-v7a
mkdir -p android/jniLibs/x86
mkdir -p android/jniLibs/x86_64

cp target/aarch64-linux-android/release/libpredictor.so android/jniLibs/arm64-v8a
cp target/armv7-linux-androideabi/release/libpredictor.so android/jniLibs/armeabi-v7a
cp target/i686-linux-android/release/libpredictor.so android/jniLibs/x86
cp target/x86_64-linux-android/release/libpredictor.so android/jniLibs/x86_64