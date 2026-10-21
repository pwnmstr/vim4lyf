use tokio::net::UnixStream;
use tokio::io::AsyncWriteExt;
use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".into());
    let ipc_path = format!("{}/kbd-daemon.sock", xdg_runtime_dir);

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: kbd-cli <command> [args]");
        eprintln!("Commands:");
        eprintln!("  trigger-scan [region]    Trigger scan mode");
        eprintln!("                           Regions: all, top-left, top-right, bottom-left, bottom-right");
        eprintln!("  show-help                Show help popup overlay");
        std::process::exit(1);
    }

    let command = &args[1];

    if command == "trigger-scan" {
        let region = if args.len() >= 3 { &args[2] } else { "all" };
        let msg = format!("trigger-scan {}", region);

        let mut stream = UnixStream::connect(&ipc_path)
            .await
            .context(format!("Failed to connect to daemon socket at {}. Is kbd-daemon running?", ipc_path))?;

        stream.write_all(msg.as_bytes()).await?;
        println!("Scan mode triggered for region: {}", region);
    } else if command == "show-help" {
        let mut stream = UnixStream::connect(&ipc_path)
            .await
            .context(format!("Failed to connect to daemon socket at {}. Is kbd-daemon running?", ipc_path))?;

        stream.write_all(b"show-help").await?;
        println!("Help popup triggered.");
    } else {
        eprintln!("Unknown command: {}", command);
        std::process::exit(1);
    }

    Ok(())
}
