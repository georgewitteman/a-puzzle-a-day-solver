mod block;
mod board;
mod point;
mod solver;

use std::env;

use anyhow::{bail, Context, Result};
use getopts::Options;

use block::*;
use board::*;
use point::*;
use solver::*;

const MONTH_NAMES: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.reqopt(
        "m",
        "month",
        "month",
        &format!("[{}]", MONTH_NAMES.to_vec().join("|")),
    );
    opts.reqopt("d", "day", "day", "[1-31]");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", opts.short_usage(&program));
            bail!(f.to_string())
        }
    };
    if matches.opt_present("h") {
        println!("{}", opts.short_usage(&program));
        return Ok(());
    }

    let month_str: String = matches.opt_get("month").unwrap().unwrap();
    let month_pos = match MONTH_NAMES.iter().position(|m| *m == month_str) {
        None => {
            bail!("unexpected month name: {}", month_str);
        }
        Some(p) => {
            let x = if p <= 5 { 0 } else { 1 };
            let y = p - x * 6;
            Point::new(x as i32, y as i32)
        }
    };

    let day: u32 = matches
        .opt_get::<String>("day")
        .unwrap()
        .unwrap()
        .parse()
        .context("invalid number was given as day")?;
    let day_pos = {
        let x = (day - 1) / 7 + 2;
        let y = (day - 1) % 7;
        Point::new(x as i32, y as i32)
    };

    let board = Board::new_from_day_pos(month_pos, day_pos);
    let blocks = Block::get_blocks();

    match solve(&board, &blocks) {
        Ok(Solution { board, .. }) => {
            assert!(board.first_empty_cell().is_none());
            println!("Solution:\n{}", board);
        }
        Err(e) => {
            println!("Solution not found: {}", e);
        }
    }

    Ok(())
}
