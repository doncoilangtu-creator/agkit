use crate::db;

pub fn add(story_id: &str, title: &str, lane: &str) {
    let conn = db::open_local().expect("Cannot open local DB");
    conn.execute(
        "INSERT INTO stories (story_id, title, lane) VALUES (?1, ?2, ?3)",
        rusqlite::params![story_id, title, lane],
    )
    .expect("Failed to add story");
    println!("✅ Story {} added: {} [{}]", story_id, title, lane);
}

pub fn update(story_id: &str, status: &str) {
    let conn = db::open_local().expect("Cannot open local DB");
    let affected = conn
        .execute(
            "UPDATE stories SET status = ?1, updated_at = datetime('now','localtime') WHERE story_id = ?2",
            rusqlite::params![status, story_id],
        )
        .expect("Failed to update story");

    if affected > 0 {
        println!("✅ Story {} updated → {}", story_id, status);
    } else {
        eprintln!("⚠️ Story {} not found", story_id);
    }
}

pub fn list() {
    let conn = db::open_local().expect("Cannot open local DB");
    let mut stmt = conn
        .prepare("SELECT story_id, title, lane, status, created_at FROM stories ORDER BY id")
        .expect("Failed to prepare query");

    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })
        .expect("Failed to query stories");

    println!("📋 Stories:");
    println!("{:<10} {:<30} {:<12} {:<14} {}", "ID", "Title", "Lane", "Status", "Created");
    println!("{}", "─".repeat(90));

    let mut count = 0;
    for row in rows {
        let (id, title, lane, status, created) = row.unwrap();
        let lane_icon = match lane.as_str() {
            "tiny" => "🟢",
            "normal" => "🟡",
            "high_risk" => "🔴",
            _ => "⚪",
        };
        let title_display = if title.len() > 28 { format!("{}…", &title[..27]) } else { title };
        println!("{:<10} {:<30} {} {:<10} {:<14} {}", id, title_display, lane_icon, lane, status, created);
        count += 1;
    }

    if count == 0 {
        println!("   (Chưa có story nào)");
    }
}

pub fn verify(story_id: &str) {
    let conn = db::open_local().expect("Cannot open local DB");

    // Check test matrix for this story
    let mut stmt = conn
        .prepare("SELECT behavior, unit, integration, e2e, platform FROM test_matrix WHERE story_id = ?1")
        .expect("Failed to prepare query");

    let rows = stmt
        .query_map(rusqlite::params![story_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i32>(1)?,
                row.get::<_, i32>(2)?,
                row.get::<_, i32>(3)?,
                row.get::<_, i32>(4)?,
            ))
        })
        .expect("Failed to query test matrix");

    let mut total = 0;
    let mut passed = 0;
    let mut all_ok = true;

    println!("🔍 Verifying story {}...", story_id);

    for row in rows {
        let (behavior, unit, integration, e2e, platform) = row.unwrap();
        let checks = [unit, integration, e2e, platform];
        for c in &checks {
            if *c != 0 {
                total += 1;
                if *c == 1 {
                    passed += 1;
                } else {
                    all_ok = false;
                }
            }
        }
        let _ = behavior; // used implicitly
    }

    if total == 0 {
        println!("⚠️ No test matrix entries for {}. Add behaviors first.", story_id);
    } else if all_ok {
        update(story_id, "done");
        println!("✅ All {}/{} checks passed. Story marked as done.", passed, total);
    } else {
        println!("❌ {}/{} checks passed. Story not ready.", passed, total);
    }
}
