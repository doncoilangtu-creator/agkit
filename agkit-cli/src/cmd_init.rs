use crate::db;
use crate::schema;

pub fn run() {
    // Init local DB
    match db::open_local() {
        Ok(conn) => {
            match schema::init_local(&conn) {
                Ok(_) => println!("✅ Local database initialized: {}", db::local_db_path()),
                Err(e) => eprintln!("❌ Failed to init local DB: {}", e),
            }
        }
        Err(e) => eprintln!("❌ Cannot open local DB: {}", e),
    }

    // Init global DB
    match db::open_global() {
        Ok(conn) => {
            match schema::init_global(&conn) {
                Ok(_) => println!("✅ Global database initialized: {}", db::global_db_path()),
                Err(e) => eprintln!("❌ Failed to init global DB: {}", e),
            }
        }
        Err(e) => eprintln!("❌ Cannot open global DB: {}", e),
    }

    // Register project in global DB
    let project_name = std::env::current_dir()
        .map(|p| p.file_name().unwrap_or_default().to_string_lossy().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    let project_path = std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| ".".to_string());

    if let Ok(conn) = db::open_global() {
        conn.execute(
            "INSERT OR IGNORE INTO projects (name, path) VALUES (?1, ?2)",
            rusqlite::params![project_name, project_path],
        )
        .ok();
        conn.execute(
            "UPDATE projects SET last_used = datetime('now','localtime') WHERE path = ?1",
            rusqlite::params![project_path],
        )
        .ok();
    }

    println!("\n🎉 AGKit Durable Layer ready! Use 'agkit-cli --help' for available commands.");
}
