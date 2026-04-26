use clap::{Parser, ValueEnum};
use std::process::{Command, ExitCode};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Action {
    Run,
    Check,
    Build,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Board {
    Pico1,
    Pico2,
    Pico1w,
    Pico2w,
}

// Change this once for your project if you do not want to pass --board every time.
const DEFAULT_BOARD: Board = Board::Pico1;

impl Board {
    fn target(self) -> &'static str {
        match self {
            Board::Pico1 | Board::Pico1w => "thumbv6m-none-eabi",
            Board::Pico2 | Board::Pico2w => "thumbv8m.main-none-eabihf",
        }
    }

    fn features(self) -> &'static str {
        match self {
            Board::Pico1 => "pico1",
            Board::Pico2 => "pico2",
            Board::Pico1w => "pico1,wifi",
            Board::Pico2w => "pico2,wifi",
        }
    }
}

#[derive(Debug, Parser)]
#[command(name = "xtask")]
#[command(about = "Cross-platform board runner for device-envoy-rp-blinky")]
struct Cli {
    #[arg(value_enum)]
    action: Action,

    #[arg(long, value_enum)]
    board: Option<Board>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let board = cli.board.unwrap_or(DEFAULT_BOARD);

    let mut cargo = Command::new("cargo");

    cargo.arg(match cli.action {
        Action::Run => "run",
        Action::Check => "check",
        Action::Build => "build",
    });

    cargo.arg("--target").arg(board.target());

    let use_release = !matches!(cli.action, Action::Check);
    if use_release {
        cargo.arg("--release");
    }

    cargo
        .arg("--no-default-features")
        .arg("--features")
        .arg(board.features());

    let status = cargo.status().unwrap_or_else(|error| {
        panic!("failed to launch cargo: {error}");
    });

    if status.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
