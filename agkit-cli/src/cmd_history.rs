use crate::db;

pub fn run(last: usize, filter: &str, outcome_filter: &str) {
    let conn = db::open_local().expect("Cannot open local DB");

    match filter {
        "sessions" => list_sessions(&conn, last),
        "traces" => list_traces(&conn, last, outcome_filter),
        "intakes" => list_intakes(&conn, last),
        _ => list_all(&conn, last),
    }
}

fn list_all(conn: &rusqlite::Connection, last: usize) {
    println!("📜 History (last {} events):", last);
    println!("{:<20} {:<10} {}", "Time", "Type", "Details");
    println!("{}", "─".repeat(80));

    let mut stmt = conn.prepare(
        "SELECT created_at, type, details FROM (
            SELECT started_at as created_at, 'SESSION' as type, COALESCE(summary, 'Started') as details FROM sessions
            UNION ALL
            SELECT created_at, 'TRACE' as type, summary || ' [' || outcome || ']' as details FROM traces
            UNION ALL
            SELECT created_at, 'INTAKE' as type, summary || ' [' || lane || ']' as details FROM intakes
            UNION ALL
            SELECT created_at, 'ADR' as type, adr_number || ': ' || title as details FROM adrs
        ) ORDER BY created_at DESC LIMIT ?1"
    ).expect("Failed to prepare query");

    let rows = stmt.query_map(rusqlite::params![last], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    }).expect("Failed to query");

    let mut count = 0;
    for row in rows {
        let (time, kind, details) = row.unwrap();
        let icon = match kind.as_str() {
            "SESSION" => "📂",
            "TRACE" => "📝",
            "INTAKE" => "🔄",
            "ADR" => "🏗 ",
            _ => "  ",
        };
        println!("{:<20} {} {:<8} {}", time, icon, kind, details);
        count += 1;
    }

    if count == 0 {
        println!("   (Chưa có hoạt động nào)");
    }
}

fn list_sessions(conn: &rusqlite::Connection, last: usize) {
    println!("📂 Session History (last {}):", last);
    println!("{:<4} {:<20} {:<20} {}", "#", "Started", "Ended", "Summary");
    println!("{}", "─".repeat(80));

    let mut stmt = conn.prepare(
        "SELECT id, started_at, ended_at, summary FROM sessions ORDER BY id DESC LIMIT ?1"
    ).expect("Failed to prepare query");

    let rows = stmt.query_map(rusqlite::params![last], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, Option<String>>(2)?,
            row.get::<_, Option<String>>(3)?,
        ))
    }).expect("Failed to query");

    for row in rows {
        let (id, started, ended, summary) = row.unwrap();
        let ended_str = ended.unwrap_or_else(|| "🟢 active".to_string());
        let summary_str = summary.unwrap_or_default();
        println!("{:<4} {:<20} {:<20} {}", id, started, ended_str, summary_str);
    }
}

fn list_traces(conn: &rusqlite::Connection, last: usize, outcome_filter: &str) {
    let query = if outcome_filter.is_empty() || outcome_filter == "all" {
        format!("SELECT id, created_at, summary, outcome FROM traces ORDER BY id DESC LIMIT {}", last)
    } else {
        format!(
            "SELECT id, created_at, summary, outcome FROM traces WHERE outcome = '{}' ORDER BY id DESC LIMIT {}",
            outcome_filter, last
        )
    };

    println!("📝 Trace History (last {}, filter: {}):", last, if outcome_filter.is_empty() { "all" } else { outcome_filter });
    println!("{:<4} {:<20} {:<8} {}", "#", "Time", "Result", "Summary");
    println!("{}", "─".repeat(80));

    let mut stmt = conn.prepare(&query).expect("Failed to prepare query");
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
        ))
    }).expect("Failed to query");

    for row in rows {
        let (id, time, summary, outcome) = row.unwrap();
        let icon = match outcome.as_str() {
            "success" => "✅",
            "failure" => "❌",
            "partial" => "⚠️",
            "escalated" => "🆘",
            _ => "📝",
        };
        println!("{:<4} {:<20} {} {:<6} {}", id, time, icon, outcome, summary);
    }
}

fn list_intakes(conn: &rusqlite::Connection, last: usize) {
    println!("🔄 Intake History (last {}):", last);
    println!("{:<4} {:<20} {:<15} {:<12} {}", "#", "Time", "Type", "Lane", "Summary");
    println!("{}", "─".repeat(85));

    let mut stmt = conn.prepare(
        "SELECT id, created_at, input_type, lane, summary FROM intakes ORDER BY id DESC LIMIT ?1"
    ).expect("Failed to prepare query");

    let rows = stmt.query_map(rusqlite::params![last], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
        ))
    }).expect("Failed to query");

    for row in rows {
        let (id, time, input_type, lane, summary) = row.unwrap();
        let lane_icon = match lane.as_str() {
            "tiny" => "🟢",
            "normal" => "🟡",
            "high_risk" => "🔴",
            _ => "⚪",
        };
        println!("{:<4} {:<20} {:<15} {} {:<10} {}", id, time, input_type, lane_icon, lane, summary);
    }
}
