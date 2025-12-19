// src-tauri/src/db.rs
use rusqlite::{params, Connection, Result};

// Cette fonction va initialiser la DB
pub fn init() -> Result<()> {
    // 1. Connexion à la base de données (ça crée le fichier s'il n'existe pas)
    // Pour l'instant, on crée un fichier "lego_store.db" à la racine du projet
    let conn = Connection::open("lego_store.db")?;
    // Petite astuce de pro : on active les Foreign Keys (désactivées par défaut sur SQLite)
    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    // Création des tables (Syntaxe SQLite standard)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sets (
            set_num TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            year INTEGER,
            theme_id INTEGER,
            num_parts INTEGER,
            img_url TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS inventory (
            id INTEGER PRIMARY KEY AUTOINCREMENT, -- SQLite gère l'auto-incrément différemment
            set_num TEXT,
            quantity INTEGER DEFAULT 1,
            is_built BOOLEAN DEFAULT 0, -- SQLite n'a pas de vrai booléen, on utilise 0/1
            date_added DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY(set_num) REFERENCES sets(set_num)
        )",
        [],
    )?;

    println!("💾 Base de données SQLite initialisée avec succès !");
    Ok(())
}

// Fonction d'ajout (CRUD)
pub fn add_set(set_num: String, name: String) -> Result<(), String> {
    let conn = Connection::open("lego_store.db").map_err(|e| e.to_string())?;

    // Insertion
    conn.execute(
        "INSERT OR IGNORE INTO sets (set_num, name, year, theme_id, num_parts, img_url) VALUES (?1, ?2, 2024, 0, 0, '')",
        params![set_num, name],
    ).map_err(|e| e.to_string())?;

    // On ajoute aussi une entrée dans l'inventaire pour dire qu'on le possède
    conn.execute(
        "INSERT INTO inventory (set_num, quantity) VALUES (?1, 1)",
        params![set_num],
    ).map_err(|e| e.to_string())?;

    println!("✅ Set ajouté dans SQLite : {} - {}", set_num, name);
    Ok(())
}