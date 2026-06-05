use crate::db;

pub fn run(input_type: &str, lane: &str, summary: &str) {
    let conn = db::open_local().expect("Cannot open local DB");

    // Find active session
    let session_id: Option<i64> = conn
        .query_row(
            "SELECT id FROM sessions WHERE ended_at IS NULL ORDER BY id DESC LIMIT 1",
            [],
            |row| row.get(0),
        )
        .ok();

    conn.execute(
        "INSERT INTO intakes (input_type, lane, summary, session_id) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![input_type, lane, summary, session_id],
    )
    .expect("Failed to record intake");

    let lane_icon = match lane {
        "tiny" => "🟢",
        "normal" => "🟡",
        "high_risk" => "🔴",
        _ => "⚪",
    };

    println!("{} Intake recorded: [{}] {} — {}", lane_icon, lane, input_type, summary);

    match lane {
        "tiny" => println!("   → Quy trình: Sửa trực tiếp → /verify → Done"),
        "normal" => println!("   → Quy trình: /plan → Code → Unit test → /verify → /review"),
        "high_risk" => println!("   → Quy trình: /plan chi tiết + Mermaid → User duyệt → /security → /verify → /review"),
        _ => {}
    }
}
