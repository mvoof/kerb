//! Le Mans Ultimate telemetry example.

use kerb::{Connection, ReadResult, SimConnection, SimError};
use std::io::{self, Write};
use std::time::Duration;

fn main() -> Result<(), SimError> {
    println!("Waiting for Le Mans Ultimate...");

    loop {
        match SimConnection::connect() {
            Ok(Connection::Lmu(conn)) => {
                println!("Connected to LMU");

                loop {
                    match conn.read_frame(16) {
                        ReadResult::Frame(frame) => {
                            if let Some(player) = frame.player_telemetry() {
                                let rpm = player.engine_rpm;
                                let gear = player.gear;
                                print!("\r{:.0} rpm  gear {}", rpm, gear);
                                let _ = io::stdout().flush();
                            }
                        }

                        ReadResult::NotReady => continue,

                        ReadResult::Disconnected => {
                            println!("\nDisconnected.");

                            break;
                        }
                    }
                }
            }

            Ok(_) => {
                eprintln!("A different sim connected — expected LMU.");
                break Ok(());
            }

            Err(e) => {
                print!("\r{e}");
                let _ = io::stdout().flush();
                std::thread::sleep(Duration::from_secs(2));
            }
        }
    }
}
