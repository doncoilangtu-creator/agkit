use crate::db;

pub fn start(summary: &str) {
    let conn = db::open_local().expect("Cannot open local DB");
    conn.execute(
        "INSERT INTO sessions (summary) VALUES (?1)",
        rusqlite::params![summary],
    )
    .expect("Failed to start session");

    let id: i64 = conn.last_insert_rowid();
    println!("✅ Session #{} started: {}", id, summary);
}

pub fn end(summary: &str) {
    let conn = db::open_local().expect("Cannot open local DB");

    // Find the latest active session (no ended_at)
    let result: Result<i64, _> = conn.query_row(
        "SELECT id FROM sessions WHERE ended_at IS NULL ORDER BY id DESC LIMIT 1",
        [],
        |row| row.get(0),
    );

    match result {
        Ok(id) => {
            conn.execute(
                "UPDATE sessions SET ended_at = datetime('now','localtime'), summary = COALESCE(summary || ' → ', '') || ?1 WHERE id = ?2",
                rusqlite::params![summary, id],
            )
            .expect("Failed to end session");
            println!("✅ Session #{} ended: {}", id, summary);
        }
        Err(_) => {
            eprintln!("⚠️ No active session found. Start one with: agkit-cli session start");
        }
    }
}

pub fn list(last: usize) {
    let conn = db::open_local().expect("Cannot open local DB");
    let mut stmt = conn
        .prepare("SELECT id, started_at, ended_at, summary FROM sessions ORDER BY id DESC LIMIT ?1")
        .expect("Failed to prepare query");

    let rows = stmt
        .query_map(rusqlite::params![last], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, Option<String>>(3)?,
            ))
        })
        .expect("Failed to query sessions");

    println!("📋 Sessions (last {}):", last);
    println!("{:<4} {:<20} {:<20} {}", "#", "Started", "Ended", "Summary");
    println!("{}", "─".repeat(80));

    for row in rows {
        let (id, started, ended, summary) = row.unwrap();
        let ended_str = ended.unwrap_or_else(|| "🟢 active".to_string());
        let summary_str = summary.unwrap_or_default();
        println!("{:<4} {:<20} {:<20} {}", id, started, ended_str, summary_str);
    }
}
