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
Prepare toolchain (see eg [here](https://medium.com/visly/rust-on-android-19f34a2fb43))

Remember the NDK toolchain path you set in the .cargo/config - I will refer to them as %YOUR NDK FOLDER%

In terminal execute:

    git clone -b temp-macOS-fix https://github.com/mad-de/IMPP/
    cd IMPP
    cargo build --target aarch64-linux-android --release
    cargo build --target armv7-linux-androideabi --release
    cargo build --target i686-linux-android --release
