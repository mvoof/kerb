//! Assetto Corsa Evo telemetry example.

use kerb::{Connection, ReadResult, SimConnection, SimError};
use std::io::{self, Write};
use std::time::Duration;

fn main() -> Result<(), SimError> {
    println!("Waiting for Assetto Corsa Evo...");

    loop {
        match SimConnection::connect() {
            Ok(Connection::AcEvo(conn)) => {
                println!("Connected to AC Evo");

                loop {
                    match conn.read_frame(16) {
                        ReadResult::Frame(frame) => {
                            let rpms = frame.physics.rpms;
                            let gear = frame.physics.gear;
                            let speed = frame.physics.speed_kmh;
                            let pad_fl = frame.physics.pad_life[0];
                            print!(
                                "\r{:.0} rpm  gear {}  {:.1} km/h  pad_fl={:.0}%",
                                rpms, gear, speed, pad_fl
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
                eprintln!("A different sim connected — expected AC Evo.");
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
