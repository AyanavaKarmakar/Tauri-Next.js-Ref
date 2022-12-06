// disables the command prompt window that would normally pop up on Windows if you run a bundled app
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// The main function is the entry point
// and the first function that gets invoked when your program runs.
fn main() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
