use std::io::{self, Read};
use riverbraid_types::{RelationalMap};

fn main() {
    let mut buffer = String::new();
    if let Ok(_) = io::stdin().read_to_string(&mut buffer) {
        // Strict JSON parsing from the scanner's output
        match serde_json::from_str::<RelationalMap>(&buffer) {
            Ok(map) => {
                let total = map.nodes.len();
                let healthy = map.relations.iter()
                    .filter(|r| r.weight >= 1.0)
                    .count();

                let status = if healthy == total && total > 0 {
                    "STATIONARY"
                } else {
                    "DRIFT"
                };

                println!("BRIDGE_VERDICT: {}", status);
                println!("INTEGRITY_SCORE: {}/{}", healthy, total);
            }
            Err(e) => {
                eprintln!("BRIDGE_ERROR: Failed to parse relay data: {}", e);
                std::process::exit(1);
            }
        }
    }
}
