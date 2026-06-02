//! Assetto Corsa / AC Evo telemetry example.
//!
//! Works for both games — auto-detects which one is running.
//! For Evo-specific fields, match on `AcFrame::Evo`.

use kerb::ac::connection::AcFrame;
use kerb::{Connection, SimConnection, SimError};
use std::io::{self, Write};

fn main() -> Result<(), SimError> {
    println!("Waiting for Assetto Corsa or AC Evo...");

    loop {
        match SimConnection::connect() {
            Ok(Connection::Ac(conn)) => {
                let game = match conn.frame()? {
                    AcFrame::Classic(_) => "AC",
                    AcFrame::Evo(_) => "AC Evo",
                };
                println!("Connected to {}", game);

                while conn.is_connected() {
                    conn.wait_for_data(16);

                    let frame = conn.frame()?;

                    // Common fields — work for both AC and AC Evo
                    let rpms = frame.rpms();
                    let gear = frame.gear();
                    let speed = frame.speed_kmh();

                    print!("\r{:.0} rpm  gear {}  {:.1} km/h", rpms, gear, speed);

                    // Evo-specific fields
                    if let AcFrame::Evo(f) = &frame {
                        let pad_fl = f.physics.pad_life[0];
                        print!("  pad_fl={:.0}%", pad_fl);
                    }

                    let _ = io::stdout().flush();
                }

                println!("\nDisconnected.");
            }

            Ok(_) => {
                eprintln!("A different sim connected — expected AC/AC Evo.");
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
