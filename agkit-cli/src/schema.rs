use rusqlite::Connection;

/// Initialize local project database schema
pub fn init_local(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS sessions (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            started_at  TEXT NOT NULL DEFAULT (datetime('now','localtime')),
            ended_at    TEXT,
            summary     TEXT,
            files_read  TEXT,
            files_changed TEXT
        );

        CREATE TABLE IF NOT EXISTS intakes (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at  TEXT NOT NULL DEFAULT (datetime('now','localtime')),
            input_type  TEXT NOT NULL,
            lane        TEXT NOT NULL,
            summary     TEXT NOT NULL,
            session_id  INTEGER REFERENCES sessions(id)
        );

        CREATE TABLE IF NOT EXISTS traces (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at  TEXT NOT NULL DEFAULT (datetime('now','localtime')),
            summary     TEXT NOT NULL,
            outcome     TEXT NOT NULL,
            session_id  INTEGER REFERENCES sessions(id)
        );

        CREATE TABLE IF NOT EXISTS adrs (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            adr_number  TEXT NOT NULL UNIQUE,
            title       TEXT NOT NULL,
            status      TEXT NOT NULL DEFAULT 'accepted',
            decision    TEXT,
            rationale   TEXT,
            tradeoffs   TEXT,
            created_at  TEXT NOT NULL DEFAULT (datetime('now','localtime'))
        );

        CREATE TABLE IF NOT EXISTS stories (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            story_id    TEXT NOT NULL UNIQUE,
            title       TEXT NOT NULL,
            lane        TEXT NOT NULL,
            status      TEXT NOT NULL DEFAULT 'planned',
            created_at  TEXT NOT NULL DEFAULT (datetime('now','localtime')),
            updated_at  TEXT
        );

        CREATE TABLE IF NOT EXISTS test_matrix (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            story_id        TEXT NOT NULL,
            behavior        TEXT NOT NULL,
            unit            INTEGER DEFAULT 0,
            integration     INTEGER DEFAULT 0,
            e2e             INTEGER DEFAULT 0,
            platform        INTEGER DEFAULT 0,
            updated_at      TEXT NOT NULL DEFAULT (datetime('now','localtime')),
            UNIQUE(story_id, behavior)
        );

        CREATE TABLE IF NOT EXISTS backlog (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            title       TEXT NOT NULL,
            priority    TEXT NOT NULL DEFAULT 'medium',
            status      TEXT NOT NULL DEFAULT 'open',
            category    TEXT,
            created_at  TEXT NOT NULL DEFAULT (datetime('now','localtime')),
            done_at     TEXT
        );
        ",
    )
}

/// Initialize global database schema
pub fn init_global(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS projects (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        TEXT NOT NULL,
            path        TEXT NOT NULL UNIQUE,
            created_at  TEXT NOT NULL DEFAULT (datetime('now','localtime')),
            last_used   TEXT
        );

        CREATE TABLE IF NOT EXISTS global_traces (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id  INTEGER REFERENCES projects(id),
            created_at  TEXT NOT NULL DEFAULT (datetime('now','localtime')),
            summary     TEXT NOT NULL,
            outcome     TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS global_stats (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id      INTEGER REFERENCES projects(id),
            total_sessions  INTEGER DEFAULT 0,
            total_adrs      INTEGER DEFAULT 0,
            total_traces    INTEGER DEFAULT 0,
            updated_at      TEXT NOT NULL DEFAULT (datetime('now','localtime'))
        );
        ",
    )
}
