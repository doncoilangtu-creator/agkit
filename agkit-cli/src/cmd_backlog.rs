use crate::db;

pub fn add(title: &str, priority: &str, category: &str) {
    let conn = db::open_local().expect("Cannot open local DB");
    conn.execute(
        "INSERT INTO backlog (title, priority, category) VALUES (?1, ?2, ?3)",
        rusqlite::params![title, priority, category],
    )
    .expect("Failed to add backlog item");

    let id: i64 = conn.last_insert_rowid();
    println!("✅ Backlog #{} added: {} [{}] ({})", id, title, priority, category);
}

pub fn list(status: &str) {
    let conn = db::open_local().expect("Cannot open local DB");

    let query = if status == "all" {
        "SELECT id, title, priority, status, category, created_at FROM backlog ORDER BY
         CASE priority WHEN 'critical' THEN 1 WHEN 'high' THEN 2 WHEN 'medium' THEN 3 WHEN 'low' THEN 4 END,
         id".to_string()
    } else {
        format!(
            "SELECT id, title, priority, status, category, created_at FROM backlog WHERE status = '{}' ORDER BY
             CASE priority WHEN 'critical' THEN 1 WHEN 'high' THEN 2 WHEN 'medium' THEN 3 WHEN 'low' THEN 4 END,
             id",
            status
        )
    };

    let mut stmt = conn.prepare(&query).expect("Failed to prepare query");
    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, String>(5)?,
            ))
        })
        .expect("Failed to query backlog");

    println!("📋 Backlog ({}):", if status == "all" { "all" } else { status });
    println!("{:<4} {:<35} {:<10} {:<12} {:<12} {}", "#", "Title", "Priority", "Status", "Category", "Created");
    println!("{}", "─".repeat(95));

    let mut count = 0;
    for row in rows {
        let (id, title, priority, stat, category, created) = row.unwrap();
        let priority_icon = match priority.as_str() {
            "critical" => "🔴",
            "high" => "🟠",
            "medium" => "🟡",
            "low" => "🟢",
            _ => "⚪",
        };
        let cat = category.unwrap_or_else(|| "—".to_string());
        let title_display = if title.len() > 33 { format!("{}…", &title[..32]) } else { title };
        println!("{:<4} {:<35} {} {:<8} {:<12} {:<12} {}", id, title_display, priority_icon, priority, stat, cat, created);
        count += 1;
    }

    if count == 0 {
        println!("   (Trống)");
    }
}

pub fn done(id: i64) {
    let conn = db::open_local().expect("Cannot open local DB");
    let affected = conn
        .execute(
            "UPDATE backlog SET status = 'done', done_at = datetime('now','localtime') WHERE id = ?1",
            rusqlite::params![id],
        )
        .expect("Failed to update backlog item");

    if affected > 0 {
        println!("✅ Backlog #{} marked as done", id);
    } else {
        eprintln!("⚠️ Backlog #{} not found", id);
    }
}
