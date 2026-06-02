//! Assetto Corsa Evo telemetry example.

use kerb::{Connection, SimConnection, SimError};
use std::io::{self, Write};

fn main() -> Result<(), SimError> {
    println!("Waiting for Assetto Corsa Evo...");

    loop {
        match SimConnection::connect() {
            Ok(Connection::Ac(conn)) => {
                println!("Connected to AC Evo");

                while conn.is_connected() {
                    conn.wait_for_data(16);

                    let frame = conn.frame()?;

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

                println!("\nDisconnected.");
            }

            Ok(_) => {
                eprintln!("A different sim connected — expected AC Evo.");
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
