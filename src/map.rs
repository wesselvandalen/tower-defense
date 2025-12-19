use crate::towers::Tower;

#[derive(Debug)]
pub struct Map {
    size: (usize, usize),
    path: Vec<(usize, usize)>,
    towers: Vec<(usize, usize, Tower)>,
}

impl Map {
    pub fn new_empty() -> Self {
        Self {
            size: (6, 12),
            path: Vec::new(),
            towers: Vec::new(),
        }
    }

    pub fn new_map1() -> Self {
        let path = vec![
            (1, 1),
            (2, 2)
        ];

        Self {
            size: (6, 12),
            path,
            towers: Vec::new(),
        }
    }

    pub fn new_map2() -> Self {
        let path = vec![
            (2, 2),
            (3, 3)
        ];

        Self {
            size: (6, 12),
            path,
            towers: Vec::new(),
        }
    }

    pub fn set_path(&mut self, path: Vec<(usize, usize)>) {
        self.path = path;
    }
}