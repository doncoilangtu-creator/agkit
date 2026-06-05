mod db;
mod schema;
mod cmd_init;
mod cmd_session;
mod cmd_intake;
mod cmd_trace;
mod cmd_adr;
mod cmd_story;
mod cmd_matrix;
mod cmd_backlog;
mod cmd_stats;
mod cmd_history;

use clap::{Parser, Subcommand};

/// AGKit CLI v3.0 — Durable Layer for AGKit Developer Kit
#[derive(Parser)]
#[command(name = "agkit-cli", version = "3.0.0", about = "AGKit Durable Layer CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize databases (local + global)
    Init,

    /// Session management
    Session {
        #[command(subcommand)]
        action: SessionAction,
    },

    /// Intake risk classification
    Intake {
        /// Input type: new_spec, change_request, bug_fix, refactor, maintenance, harness_improvement
        #[arg(long, short = 't')]
        r#type: String,
        /// Risk lane: tiny, normal, high_risk
        #[arg(long, short)]
        lane: String,
        /// Summary description
        #[arg(long, short)]
        summary: String,
    },

    /// Record a trace
    Trace {
        /// Summary of the action
        #[arg(long, short)]
        summary: String,
        /// Outcome: success, failure, partial, escalated
        #[arg(long, short)]
        outcome: String,
    },

    /// Architecture Decision Records
    Adr {
        #[command(subcommand)]
        action: AdrAction,
    },

    /// Story management
    Story {
        #[command(subcommand)]
        action: StoryAction,
    },

    /// Test Matrix management
    Matrix {
        #[command(subcommand)]
        action: MatrixAction,
    },

    /// Backlog management
    Backlog {
        #[command(subcommand)]
        action: BacklogAction,
    },

    /// Project statistics
    Stats {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// View history
    History {
        /// Number of records to show
        #[arg(long, default_value = "10")]
        last: usize,
        /// Filter: sessions, traces, intakes (default: all)
        #[arg(long, default_value = "")]
        filter: String,
        /// Outcome filter for traces: success, failure, partial, escalated
        #[arg(long, default_value = "")]
        outcome: String,
    },
}

#[derive(Subcommand)]
enum SessionAction {
    /// Start a new session
    Start {
        #[arg(long, short, default_value = "Session started")]
        summary: String,
    },
    /// End the current session
    End {
        #[arg(long, short, default_value = "Session ended")]
        summary: String,
    },
    /// List recent sessions
    List {
        #[arg(long, default_value = "10")]
        last: usize,
    },
}

#[derive(Subcommand)]
enum AdrAction {
    /// Add a new ADR
    Add {
        #[arg(long)]
        title: String,
        #[arg(long, default_value = "")]
        decision: String,
        #[arg(long, default_value = "")]
        rationale: String,
        #[arg(long, default_value = "")]
        tradeoffs: String,
    },
    /// List all ADRs
    List,
}

#[derive(Subcommand)]
enum StoryAction {
    /// Add a new story
    Add {
        #[arg(long)]
        id: String,
        #[arg(long)]
        title: String,
        #[arg(long, default_value = "normal")]
        lane: String,
    },
    /// Update story status
    Update {
        #[arg(long)]
        id: String,
        #[arg(long)]
        status: String,
    },
    /// List all stories
    List,
    /// Verify a story against test matrix
    Verify {
        /// Story ID to verify
        id: String,
    },
}

#[derive(Subcommand)]
enum MatrixAction {
    /// Set test result for a behavior
    Set {
        #[arg(long)]
        story: String,
        #[arg(long)]
        behavior: String,
        #[arg(long, default_value = "0")]
        unit: i32,
        #[arg(long, default_value = "0")]
        integration: i32,
        #[arg(long, default_value = "0")]
        e2e: i32,
        #[arg(long, default_value = "0")]
        platform: i32,
    },
    /// Query test matrix for a story
    Query {
        #[arg(long, default_value = "")]
        story: String,
        /// Show numeric summary for all stories
        #[arg(long)]
        numeric: bool,
    },
}

#[derive(Subcommand)]
enum BacklogAction {
    /// Add a backlog item
    Add {
        #[arg(long)]
        title: String,
        #[arg(long, default_value = "medium")]
        priority: String,
        #[arg(long, default_value = "product")]
        category: String,
    },
    /// List backlog items
    List {
        #[arg(long, default_value = "open")]
        status: String,
    },
    /// Mark item as done
    Done {
        #[arg(long)]
        id: i64,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => cmd_init::run(),

        Commands::Session { action } => match action {
            SessionAction::Start { summary } => cmd_session::start(&summary),
            SessionAction::End { summary } => cmd_session::end(&summary),
            SessionAction::List { last } => cmd_session::list(last),
        },

        Commands::Intake { r#type, lane, summary } => {
            cmd_intake::run(&r#type, &lane, &summary);
        }

        Commands::Trace { summary, outcome } => {
            cmd_trace::run(&summary, &outcome);
        }

        Commands::Adr { action } => match action {
            AdrAction::Add { title, decision, rationale, tradeoffs } => {
                cmd_adr::add(&title, &decision, &rationale, &tradeoffs);
            }
            AdrAction::List => cmd_adr::list(),
        },

        Commands::Story { action } => match action {
            StoryAction::Add { id, title, lane } => cmd_story::add(&id, &title, &lane),
            StoryAction::Update { id, status } => cmd_story::update(&id, &status),
            StoryAction::List => cmd_story::list(),
            StoryAction::Verify { id } => cmd_story::verify(&id),
        },

        Commands::Matrix { action } => match action {
            MatrixAction::Set { story, behavior, unit, integration, e2e, platform } => {
                cmd_matrix::set(&story, &behavior, unit, integration, e2e, platform);
            }
            MatrixAction::Query { story, numeric } => {
                if numeric {
                    cmd_matrix::query_numeric();
                } else if story.is_empty() {
                    cmd_matrix::query_numeric();
                } else {
                    cmd_matrix::query(&story);
                }
            }
        },

        Commands::Backlog { action } => match action {
            BacklogAction::Add { title, priority, category } => {
                cmd_backlog::add(&title, &priority, &category);
            }
            BacklogAction::List { status } => cmd_backlog::list(&status),
            BacklogAction::Done { id } => cmd_backlog::done(id),
        },

        Commands::Stats { json } => cmd_stats::run(json),

        Commands::History { last, filter, outcome } => {
            cmd_history::run(last, &filter, &outcome);
        }
    }
}
