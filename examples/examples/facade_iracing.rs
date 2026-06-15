//! iRacing telemetry example — connect via SimConnection and read typed fields.

use kerb::{Connection, ReadResult, SimConnection, SimError};
use std::io::{self, Write};
use std::time::Duration;

fn main() -> Result<(), SimError> {
    println!("Waiting for iRacing...");

    loop {
        match SimConnection::connect() {
            Ok(Connection::IRacing(conn)) => {
                println!("Connected to iRacing");

                if let Some(session) = conn.session_info() {
                    let track = session
                        .get_value("WeekendInfo.TrackDisplayName")
                        .unwrap_or_else(|| "Unknown".into());

                    println!("Track: {}", track);
                }

                loop {
                    match conn.read_frame(100) {
                        ReadResult::Frame(frame) => {
                            let gear = match frame.gear {
                                -1 => "R".to_string(),
                                0 => "N".to_string(),
                                g => g.to_string(),
                            };

                            print!(
                                "\r[{}] {:.0} rpm  {:.1} km/h  Gas {:.0}%  Brake {:.0}%",
                                gear,
                                frame.rpm,
                                frame.speed * 3.6,
                                frame.throttle * 100.0,
                                frame.brake * 100.0,
                            );

                            let _ = io::stdout().flush();
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
                eprintln!("A different sim connected — expected iRacing.");

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
