//! iRacing telemetry example — connect via SimConnection and read typed fields.

use kerb::{Connection, SimConnection, SimError};
use std::io::{self, Write};

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

                while conn.wait_for_data(100) {
                    let frame = conn.frame()?;

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

                println!("\nDisconnected.");
            }

            Ok(_) => {
                eprintln!("A different sim connected — expected iRacing.");

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
