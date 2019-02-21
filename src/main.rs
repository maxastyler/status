extern crate chrono;

use chrono::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let bat_status = String::from(std::fs::read_to_string("/sys/class/power_supply/BAT0/status")?.trim());
    let energy_now = std::fs::read_to_string("/sys/class/power_supply/BAT0/energy_now")?
        .trim()
        .parse::<f32>()
        .unwrap_or(0.0);
    let energy_full = std::fs::read_to_string("/sys/class/power_supply/BAT0/energy_full")?
        .trim()
        .parse::<f32>()
        .unwrap_or(0.0);
    let date = Local::now().format("%Y-%m-%d %H:%M:%S");
    println!("Bat: {} - {:.1}% | {}", bat_status, energy_now/energy_full*100.0, date);
    Ok(())
}
