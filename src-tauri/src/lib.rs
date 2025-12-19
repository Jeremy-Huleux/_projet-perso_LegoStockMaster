// On déclare notre nouveau module
mod db;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // C'est ici qu'on configure le démarrage (setup)
        .setup(|_app| {
            // On appelle notre fonction d'initialisation DB
            // Le .expect sert à faire crasher l'appli explicitement si la DB ne se charge pas (mieux vaut savoir tout de suite)
            db::init().expect("Erreur lors de l'initialisation de la base de données");
            
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
