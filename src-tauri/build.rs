fn main() {
  #[cfg(target_os = "windows")]
  {
    let lib_path = std::path::Path::new("lib");

    if lib_path.exists() {
      println!("cargo:rustc-link-search=native=lib");
      println!("cargo:rustc-link-lib=wpcap");
      println!("cargo:rustc-link-lib=Packet");
    }
  }

  tauri_build::build()
}
