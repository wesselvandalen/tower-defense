mod map;
mod towers;
use map::Map;

fn main() {
    let map = Map::new();
    map.draw();
}