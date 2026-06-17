pub fn run() {
  eprintln!("DEBUG: run() called");
  let context = tauri::generate_context!();
  eprintln!("DEBUG: context generated");
  tauri::Builder::default()
    .setup(|app| {
      eprintln!("DEBUG: setup called");
      Ok(())
    })
    .run(context)
    .expect("error while running tauri application");
  eprintln!("DEBUG: This should never print");
}
