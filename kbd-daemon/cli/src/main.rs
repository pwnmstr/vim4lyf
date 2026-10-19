use tokio::net::UnixStream;
use tokio::io::AsyncWriteExt;
use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".into());
    let ipc_path = format!("{}/kbd-daemon.sock", xdg_runtime_dir);

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: kbd-cli <command>");
        eprintln!("Commands:");
        eprintln!("  trigger-scan    Trigger scan mode");
        std::process::exit(1);
    }

    let command = &args[1];

    if command == "trigger-scan" {
        let mut stream = UnixStream::connect(&ipc_path)
            .await
            .context(format!("Failed to connect to daemon socket at {}. Is kbd-daemon running?", ipc_path))?;

        stream.write_all(b"trigger-scan").await?;
        println!("Scan mode triggered.");
    } else {
        eprintln!("Unknown command: {}", command);
        std::process::exit(1);
    }

    Ok(())
}
