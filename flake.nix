{
    description = "Not For Normies Music App";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
        rust-overlay.url = "github:oxalica/rust-overlay";
    };

    outputs = {nixpkgs, rust-overlay, ...}: let 
        system = "x86_64-linux";

        overlays = [ (import rust-overlay) ];              
        pkgs = import nixpkgs {
            inherit system overlays;
            config = {
                allowUnfree = true;
                android_sdk.accept_license = true;
            };

        };
        buildToolsVersion = "34.0.0";
        androidSdk = (pkgs.androidenv.composeAndroidPackages {
            includeNDK = true;
            includeEmulator = true;
          platformToolsVersion = "35.0.1";
          buildToolsVersions = [ "34.0.0" ];
          platformVersions = [ "34"];
          cmakeVersions = [ "3.10.2" ];
          extraLicenses = [
            "android-googletv-license"
            "android-sdk-arm-dbt-license"
            "android-sdk-license"
            "android-sdk-preview-license"
            "google-gdk-license"
            "intel-android-extra-license"
            "intel-android-sysimage-license"
            "mips-android-sysimage-license"
          ];
        }).androidsdk;

    in {
        devShells.${system}.default = pkgs.mkShell {
            ANDROID_HOME = "${androidSdk}/libexec/android-sdk";
            NDK_HOME = "${androidSdk}/libexec/android-sdk/ndk-bundle";
            JAVA_HOME = "${pkgs.jetbrains.jdk}";
            GRADLE_OPTS = "-Dorg.gradle.project.android.aapt2FromMavenOverride=${androidSdk}/libexec/android-sdk/build-tools/${buildToolsVersion}/aapt2";

            nativeBuildInputs = with pkgs; [
                pkg-config gobject-introspection
                cargo-tauri
                nodejs
                (rust-bin.stable.latest.default.override {
                    extensions = [ "rust-src" "rustfmt" "clippy" "cargo" ];
                    targets = [
                        "aarch64-linux-android" 
                        "armv7-linux-androideabi" 
                        "i686-linux-android" 
                        "x86_64-linux-android"
                    ];
                })
            ];

            buildInputs = with pkgs; [
                alsa-lib.dev
                nodejs pnpm
                at-spi2-atk
                atkmm
                cairo
                gdk-pixbuf
                glib
                gtk3
                harfbuzz
                librsvg
                libsoup_3
                pango
                webkitgtk_4_1
                openssl
            ];
        };
    };
}

