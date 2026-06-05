use crate::db;

pub fn run(json: bool) {
    let conn = db::open_local().expect("Cannot open local DB");

    let total_sessions: i64 = conn.query_row("SELECT COUNT(*) FROM sessions", [], |r| r.get(0)).unwrap_or(0);
    let active_sessions: i64 = conn.query_row("SELECT COUNT(*) FROM sessions WHERE ended_at IS NULL", [], |r| r.get(0)).unwrap_or(0);
    let total_adrs: i64 = conn.query_row("SELECT COUNT(*) FROM adrs", [], |r| r.get(0)).unwrap_or(0);
    let total_traces: i64 = conn.query_row("SELECT COUNT(*) FROM traces", [], |r| r.get(0)).unwrap_or(0);
    let success_traces: i64 = conn.query_row("SELECT COUNT(*) FROM traces WHERE outcome = 'success'", [], |r| r.get(0)).unwrap_or(0);
    let fail_traces: i64 = conn.query_row("SELECT COUNT(*) FROM traces WHERE outcome = 'failure'", [], |r| r.get(0)).unwrap_or(0);
    let total_stories: i64 = conn.query_row("SELECT COUNT(*) FROM stories", [], |r| r.get(0)).unwrap_or(0);
    let done_stories: i64 = conn.query_row("SELECT COUNT(*) FROM stories WHERE status = 'done'", [], |r| r.get(0)).unwrap_or(0);
    let total_intakes: i64 = conn.query_row("SELECT COUNT(*) FROM intakes", [], |r| r.get(0)).unwrap_or(0);
    let tiny_intakes: i64 = conn.query_row("SELECT COUNT(*) FROM intakes WHERE lane = 'tiny'", [], |r| r.get(0)).unwrap_or(0);
    let normal_intakes: i64 = conn.query_row("SELECT COUNT(*) FROM intakes WHERE lane = 'normal'", [], |r| r.get(0)).unwrap_or(0);
    let hr_intakes: i64 = conn.query_row("SELECT COUNT(*) FROM intakes WHERE lane = 'high_risk'", [], |r| r.get(0)).unwrap_or(0);
    let open_backlog: i64 = conn.query_row("SELECT COUNT(*) FROM backlog WHERE status = 'open'", [], |r| r.get(0)).unwrap_or(0);

    // Test matrix score
    let matrix_total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM test_matrix WHERE unit != 0 OR integration != 0 OR e2e != 0 OR platform != 0",
        [], |r| r.get(0)
    ).unwrap_or(0);
    let matrix_passed: i64 = conn.query_row(
        "SELECT SUM(
            (CASE WHEN unit = 1 THEN 1 ELSE 0 END) +
            (CASE WHEN integration = 1 THEN 1 ELSE 0 END) +
            (CASE WHEN e2e = 1 THEN 1 ELSE 0 END) +
            (CASE WHEN platform = 1 THEN 1 ELSE 0 END)
        ) FROM test_matrix",
        [], |r| r.get(0)
    ).unwrap_or(0);

    if json {
        println!("{{");
        println!("  \"sessions\": {{ \"total\": {}, \"active\": {} }},", total_sessions, active_sessions);
        println!("  \"adrs\": {},", total_adrs);
        println!("  \"traces\": {{ \"total\": {}, \"success\": {}, \"failure\": {} }},", total_traces, success_traces, fail_traces);
        println!("  \"stories\": {{ \"total\": {}, \"done\": {} }},", total_stories, done_stories);
        println!("  \"intakes\": {{ \"total\": {}, \"tiny\": {}, \"normal\": {}, \"high_risk\": {} }},", total_intakes, tiny_intakes, normal_intakes, hr_intakes);
        println!("  \"backlog_open\": {},", open_backlog);
        println!("  \"test_matrix\": {{ \"passed\": {}, \"total\": {} }}", matrix_passed, matrix_total);
        println!("}}");
        return;
    }

    println!("📊 AGKit Project Statistics");
    println!("{}", "━".repeat(45));
    println!("📂 Sessions:     {} total ({} active)", total_sessions, active_sessions);
    println!("🏗  ADRs:          {}", total_adrs);
    println!("📝 Traces:        {} total (✅{} ❌{})", total_traces, success_traces, fail_traces);
    println!("📖 Stories:       {}/{} done", done_stories, total_stories);
    println!("🔄 Intakes:       {} total (🟢{} 🟡{} 🔴{})", total_intakes, tiny_intakes, normal_intakes, hr_intakes);
    println!("📋 Backlog:       {} open items", open_backlog);

    if matrix_total > 0 {
        let all_checks: i64 = conn.query_row(
            "SELECT SUM((CASE WHEN unit != 0 THEN 1 ELSE 0 END) + (CASE WHEN integration != 0 THEN 1 ELSE 0 END) + (CASE WHEN e2e != 0 THEN 1 ELSE 0 END) + (CASE WHEN platform != 0 THEN 1 ELSE 0 END)) FROM test_matrix",
            [], |r| r.get(0)
        ).unwrap_or(0);
        let pct = if all_checks > 0 { (matrix_passed as f64 / all_checks as f64 * 100.0) as u32 } else { 0 };
        println!("🧪 Test Matrix:   {}/{} checks passed ({}%)", matrix_passed, all_checks, pct);
    } else {
        println!("🧪 Test Matrix:   No entries yet");
    }
    println!("{}", "━".repeat(45));
}
