#![allow(unused)]

struct MazeTile {
    visited: bool,
    possibilities: [bool; 11],
}

impl Default for MazeTile {
    fn default() -> MazeTile {
        MazeTile {
            visited: false,
            possibilities: [true; 11]
        }
    }
}
impl Copy for MazeTile {}
impl Clone for MazeTile {
    fn clone(&self) -> Self {
        MazeTile {
            visited: self.visited.clone(),
            possibilities: self.possibilities.clone()
        }
    }
}

pub fn generator() {
    let def = MazeTile {..Default::default()};
    let mut grid: [[MazeTile; 100]; 100] = [[def; 100]; 100];

    
    
}
