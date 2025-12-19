mod map;
mod towers;

use towers::*;
use map::Map;

fn main() {
    let mut map = Map::new();

    let path = vec![
        (1, 1),
        (2, 2)
    ];
    map.set_path(path);

    println!("Hello, world!");
}
