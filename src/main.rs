mod map;
mod towers;

use towers::*;
use map::Map;

use std::io::Result as IOResult;
use std::io::{self, Write, stdout};
use crossterm::{QueueableCommand};
use crossterm::terminal::{enable_raw_mode, Clear, ClearType};

fn main() -> IOResult<()> {
    let mut map = Map::new_map1();
    
    let mut terminal = stdout();
    enable_raw_mode()?;

    loop {
        map.draw(&mut terminal)?;
    }

    Ok(())
}
