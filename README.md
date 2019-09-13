# IMPP

Idiotische Multiple-choice Prüfungsfragen Programm

## Install & run

Install [git](https://git-scm.com) (see [here how to install git](https://www.linode.com/docs/development/version-control/how-to-install-git-on-linux-mac-and-windows/))
and [Rust](https://rustup.rs/).

In terminal run:

    git clone https://github.com/mad-de/IMPP
    cd IMPP/
    cargo run --release

## Run tests

    cargo test

## Samples
[Sample table](https://docs.google.com/spreadsheets/d/14fNP2Elca82rryRJ8-a_XwH3_oZgrJyXqh7r7Q7GuEc/edit?usp=drivesdk)

## Build library for Android
Prepare toolchain

.cargo/config should look like

    [target.aarch64-linux-android]
    ar = "#DEIN NDK TOOLCHAIN FOLDER#/arm64/bin/aarch64-linux-android-ar"
    linker = "#DEIN NDK TOOLCHAIN FOLDER#/arm64/bin/aarch64-linux-android29-clang"

    [target.armv7-linux-androideabi]
    ar = "#DEIN NDK TOOLCHAIN FOLDER#/NDK/arm/bin/arm-linux-androideabi-ar"
    linker = "#DEIN NDK TOOLCHAIN FOLDER#/arm/bin/arm-linux-androideabi-clang"

    [target.i686-linux-android]
    ar = "#DEIN NDK TOOLCHAIN FOLDER#/x86/bin/i686-linux-android-ar"
    linker = "#DEIN NDK TOOLCHAIN FOLDER#/x86/bin/i686-linux-android28-clang"

    [target.aarch64-linux-android-gcc]
    linker = "#DEIN NDK TOOLCHAIN FOLDER#/arm64/bin/aarch64-linux-android-gcc"

In terminal execute:
> git clone https://github.com/mad-de/IMPP/
> cd IMPP
> export CC=#DEIN NDK TOOLCHAIN FOLDER#/arm64/bin/aarch64-linux-android-clang
> export AR=#DEIN NDK TOOLCHAIN FOLDER#/arm64/bin/aarch64-linux-android-ar
> cargo build --target aarch64-linux-android --release
