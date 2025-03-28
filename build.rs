fn main() {
    // SDL2
    println!("cargo:rustc-link-search=native=libraries/SDL2-2.32.0/lib/x64");
    println!("cargo:rustc-link-lib=libraries/SDL2-2.32.0/lib/x64/SDL2");

    // Image
    println!("cargo:rustc-link-search=native=libraries/SDL2_image-devel-2.8.8-VC/SDL2_image-2.8.8/lib/x64");
    println!("cargo:rustc-link-lib=libraries/SDL2_image-devel-2.8.8-VC/SDL2_image-2.8.8/lib/x64/SDL2_image");

    // Mixer
    println!("cargo:rustc-link-search=native=libraries/SDL2_mixer-devel-2.8.1-VC/SDL2_mixer-2.8.1/lib/x64");
    println!("cargo:rustc-link-lib=libraries/SDL2_mixer-devel-2.8.1-VC/SDL2_mixer-2.8.1/lib/x64/SDL2_mixer");

    // ttf
    println!("cargo:rustc-link-search=native=libraries/SDL2_ttf-devel-2.24.0-VC/SDL2_ttf-2.24.0/lib/x64");
    println!("cargo:rustc-link-lib=libraries/SDL2_ttf-devel-2.24.0-VC/SDL2_ttf-2.24.0/lib/x64/SDL2_ttf");

    // gfx
    println!("cargo:rustc-link-search=native=libraries/SDL2_gfx-1.0.4-VC/SDL2_gfx-1.0.4/lib/x64");
    println!("cargo:rustc-link-lib=libraries/SDL2_gfx-1.0.4-VC/SDL2_gfx-1.0.4/lib/x64/SDL2_gfx");
}