use crate::db;

pub fn add(title: &str, decision: &str, rationale: &str, tradeoffs: &str) {
    let conn = db::open_local().expect("Cannot open local DB");

    // Auto-generate ADR number
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM adrs", [], |row| row.get(0))
        .unwrap_or(0);
    let adr_number = format!("ADR-{:03}", count + 1);

    conn.execute(
        "INSERT INTO adrs (adr_number, title, decision, rationale, tradeoffs) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![adr_number, title, decision, rationale, tradeoffs],
    )
    .expect("Failed to add ADR");

    println!("✅ {} recorded: {}", adr_number, title);
    println!("   Decision:  {}", decision);
    println!("   Rationale: {}", rationale);
    if !tradeoffs.is_empty() {
        println!("   Tradeoffs: {}", tradeoffs);
    }
}

pub fn list() {
    let conn = db::open_local().expect("Cannot open local DB");
    let mut stmt = conn
        .prepare("SELECT adr_number, title, status, decision, created_at FROM adrs ORDER BY id")
        .expect("Failed to prepare query");

    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, String>(4)?,
            ))
        })
        .expect("Failed to query ADRs");

    println!("📋 Architecture Decision Records:");
    println!("{:<10} {:<30} {:<12} {:<20} {}", "Number", "Title", "Status", "Date", "Decision");
    println!("{}", "─".repeat(100));

    let mut count = 0;
    for row in rows {
        let (number, title, status, decision, date) = row.unwrap();
        let decision_str = decision.unwrap_or_default();
        let title_display = if title.len() > 28 { format!("{}…", &title[..27]) } else { title };
        println!("{:<10} {:<30} {:<12} {:<20} {}", number, title_display, status, date, decision_str);
        count += 1;
    }

    if count == 0 {
        println!("   (Chưa có ADR nào. Dùng: agkit-cli adr add --title <title> --decision <decision>)");
    }
}
