//! Le Mans Ultimate telemetry example.

use kerb::{Connection, SimConnection, SimError};
use std::io::{self, Write};

fn main() -> Result<(), SimError> {
    println!("Waiting for Le Mans Ultimate...");

    loop {
        match SimConnection::connect() {
            Ok(Connection::Lmu(conn)) => {
                println!("Connected to LMU");

                while conn.is_plugin_active() {
                    if !conn.is_session_started() {
                        print!("\r  Waiting for session...   ");
                        let _ = io::stdout().flush();
                        conn.wait_for_data(500);
                        continue;
                    }
                    conn.wait_for_data(16);

                    let frame = match conn.frame() {
                        Ok(f) => f,
                        Err(_) => continue,
                    };

                    if let Some(player) = frame.player_telemetry() {
                        let rpm = player.engine_rpm;
                        let gear = player.gear;
                        print!("\r{:.0} rpm  gear {}", rpm, gear);
                        let _ = io::stdout().flush();
                    }
                }

                println!("\nDisconnected.");
            }

            Ok(_) => {
                eprintln!("A different sim connected — expected LMU.");
                break Ok(());
            }

            Err(e) => {
                print!("\r{e}");
                let _ = io::stdout().flush();
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
        }
    }
}
