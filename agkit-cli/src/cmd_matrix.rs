use crate::db;

pub fn set(story_id: &str, behavior: &str, unit: i32, integration: i32, e2e: i32, platform: i32) {
    let conn = db::open_local().expect("Cannot open local DB");

    conn.execute(
        "INSERT INTO test_matrix (story_id, behavior, unit, integration, e2e, platform)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(story_id, behavior) DO UPDATE SET
           unit = ?3, integration = ?4, e2e = ?5, platform = ?6,
           updated_at = datetime('now','localtime')",
        rusqlite::params![story_id, behavior, unit, integration, e2e, platform],
    )
    .expect("Failed to set test matrix");

    fn icon(v: i32) -> &'static str {
        match v {
            1 => "✅",
            -1 => "❌",
            _ => "—",
        }
    }

    println!(
        "📋 Matrix updated: {} | {} | U:{} I:{} E:{} P:{}",
        story_id, behavior,
        icon(unit), icon(integration), icon(e2e), icon(platform)
    );
}

pub fn query(story_id: &str) {
    let conn = db::open_local().expect("Cannot open local DB");
    let mut stmt = conn
        .prepare("SELECT behavior, unit, integration, e2e, platform, updated_at FROM test_matrix WHERE story_id = ?1 ORDER BY id")
        .expect("Failed to prepare query");

    let rows = stmt
        .query_map(rusqlite::params![story_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i32>(1)?,
                row.get::<_, i32>(2)?,
                row.get::<_, i32>(3)?,
                row.get::<_, i32>(4)?,
                row.get::<_, String>(5)?,
            ))
        })
        .expect("Failed to query test matrix");

    fn icon(v: i32) -> &'static str {
        match v {
            1 => "✅",
            -1 => "❌",
            _ => "—",
        }
    }

    println!("📋 Test Matrix — {}", story_id);
    println!("{:<35} {:<6} {:<13} {:<6} {:<10}", "Behavior", "Unit", "Integration", "E2E", "Platform");
    println!("{}", "─".repeat(75));

    let mut total_checks = 0;
    let mut passed_checks = 0;
    let mut count = 0;

    for row in rows {
        let (behavior, unit, integration, e2e, platform, _updated) = row.unwrap();
        let beh_display = if behavior.len() > 33 {
            format!("{}…", &behavior[..32])
        } else {
            behavior
        };
        println!(
            "{:<35} {:<6} {:<13} {:<6} {:<10}",
            beh_display,
            icon(unit),
            icon(integration),
            icon(e2e),
            icon(platform)
        );

        for v in [unit, integration, e2e, platform] {
            if v != 0 {
                total_checks += 1;
                if v == 1 {
                    passed_checks += 1;
                }
            }
        }
        count += 1;
    }

    if count == 0 {
        println!("   (Chưa có behavior nào. Dùng: agkit-cli matrix set --story {} --behavior <text>)", story_id);
    } else {
        println!("{}", "─".repeat(75));
        let pct = if total_checks > 0 {
            (passed_checks as f64 / total_checks as f64 * 100.0) as u32
        } else {
            0
        };
        println!(
            "Score: {}/{} ({}%)",
            passed_checks, total_checks, pct
        );
    }
}

pub fn query_numeric() {
    let conn = db::open_local().expect("Cannot open local DB");
    let mut stmt = conn
        .prepare(
            "SELECT story_id,
                    SUM(CASE WHEN unit = 1 THEN 1 ELSE 0 END) as u_pass,
                    SUM(CASE WHEN unit != 0 THEN 1 ELSE 0 END) as u_total,
                    SUM(CASE WHEN integration = 1 THEN 1 ELSE 0 END) as i_pass,
                    SUM(CASE WHEN integration != 0 THEN 1 ELSE 0 END) as i_total,
                    SUM(CASE WHEN e2e = 1 THEN 1 ELSE 0 END) as e_pass,
                    SUM(CASE WHEN e2e != 0 THEN 1 ELSE 0 END) as e_total,
                    SUM(CASE WHEN platform = 1 THEN 1 ELSE 0 END) as p_pass,
                    SUM(CASE WHEN platform != 0 THEN 1 ELSE 0 END) as p_total
             FROM test_matrix GROUP BY story_id ORDER BY story_id",
        )
        .expect("Failed to prepare query");

    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i32>(1)?, row.get::<_, i32>(2)?,
                row.get::<_, i32>(3)?, row.get::<_, i32>(4)?,
                row.get::<_, i32>(5)?, row.get::<_, i32>(6)?,
                row.get::<_, i32>(7)?, row.get::<_, i32>(8)?,
            ))
        })
        .expect("Failed to query");

    println!("📊 Test Matrix Summary (numeric):");
    println!("{:<12} {:<10} {:<14} {:<10} {:<10}", "Story", "Unit", "Integration", "E2E", "Platform");
    println!("{}", "─".repeat(60));

    for row in rows {
        let (story, up, ut, ip, it, ep, et, pp, pt) = row.unwrap();
        println!(
            "{:<12} {}/{:<8} {}/{:<12} {}/{:<8} {}/{}",
            story, up, ut, ip, it, ep, et, pp, pt
        );
    }
}
