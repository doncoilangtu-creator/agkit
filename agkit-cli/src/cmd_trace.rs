use crate::db;

pub fn run(summary: &str, outcome: &str) {
    let conn = db::open_local().expect("Cannot open local DB");

    let session_id: Option<i64> = conn
        .query_row(
            "SELECT id FROM sessions WHERE ended_at IS NULL ORDER BY id DESC LIMIT 1",
            [],
            |row| row.get(0),
        )
        .ok();

    conn.execute(
        "INSERT INTO traces (summary, outcome, session_id) VALUES (?1, ?2, ?3)",
        rusqlite::params![summary, outcome, session_id],
    )
    .expect("Failed to record trace");

    let icon = match outcome {
        "success" => "✅",
        "failure" => "❌",
        "partial" => "⚠️",
        "escalated" => "🆘",
        _ => "📝",
    };

    println!("{} Trace recorded: {} [{}]", icon, summary, outcome);

    // Also write to global DB
    if let Ok(global_conn) = db::open_global() {
        let project_path = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        let project_id: Option<i64> = global_conn
            .query_row(
                "SELECT id FROM projects WHERE path = ?1",
                rusqlite::params![project_path],
                |row| row.get(0),
            )
            .ok();

        if let Some(pid) = project_id {
            global_conn
                .execute(
                    "INSERT INTO global_traces (project_id, summary, outcome) VALUES (?1, ?2, ?3)",
                    rusqlite::params![pid, summary, outcome],
                )
                .ok();
        }
    }
}
