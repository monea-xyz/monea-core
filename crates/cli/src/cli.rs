use crate::commands::{AuthArgs, BillingArgs, CheckArgs, InitArgs, RunArgs, StopArgs};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "Monea CLI")]
#[clap(version = "0.1.0")]
#[clap(about = "Tool for local development of Monea projects.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(name = "init", about = "Initialize a new Monea repo")]
    Init(InitArgs),

    #[clap(name = "tui", aliases = ["ui", "dashboard", "viz"], about = "Open the Monea TUI")]
    Tui,

    #[clap(name = "check", aliases = ["verify", "verify-config"], about = "Verify the Monea project configuration")]
    Check(CheckArgs),

    #[clap(name = "run", about = "Spin up a local development environment")]
    Run(RunArgs),

    #[clap(name = "stop", about = "Stop the project")]
    Stop(StopArgs),

    #[clap(name = "auth", about = "Authentication with Monea")]
    Auth(AuthArgs),

    #[clap(name = "billing", about = "Billing information")]
    Billing(BillingArgs),
}
