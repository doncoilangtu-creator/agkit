use rusqlite::Connection;
use std::path::Path;

/// Local project database path: .agkit/agkit.db
pub fn local_db_path() -> String {
    ".agkit/agkit.db".to_string()
}

/// Global database path: ~/.gemini/agkit-global.db
pub fn global_db_path() -> String {
    let home = dirs::home_dir().expect("Cannot determine home directory");
    let gemini_dir = home.join(".gemini");
    std::fs::create_dir_all(&gemini_dir).ok();
    gemini_dir
        .join("agkit-global.db")
        .to_string_lossy()
        .to_string()
}

/// Open a connection to the local project database
pub fn open_local() -> Result<Connection, rusqlite::Error> {
    let path = local_db_path();
    let parent = Path::new(&path).parent().unwrap();
    std::fs::create_dir_all(parent).ok();
    Connection::open(&path)
}

/// Open a connection to the global database
pub fn open_global() -> Result<Connection, rusqlite::Error> {
    let path = global_db_path();
    Connection::open(&path)
}
