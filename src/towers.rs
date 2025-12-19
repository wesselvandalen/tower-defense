pub enum Tower {
    BasicMonkey(Stats)
}


struct Stats {
    damage  : usize,
    speed   : usize,
}


impl Stats {
    fn level_up(&mut self) {
        self.damage += 10;
        self.speed += 1;
    }
}