use std::collections::BTreeSet;

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    pub fn turn_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East  => Self::South,
            Self::South => Self::West,
            Self::West  => Self::North,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct IVec2 { x: i32, y: i32 }
impl IVec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn head(&mut self, head: Heading) -> Self {
        use Heading::*;
        match head {
            North => Self { x: self.x, y: self.y-1 },
            East  => Self { x: self.x+1, y: self.y },
            South => Self { x: self.x, y: self.y+1 },
            West  => Self { x: self.x-1, y: self.y },
        }
    }
    pub fn extract_map_byte(&self, map:&[Vec<u8>]) -> Option<u8> {
        map.get(self.y as usize)
            .and_then(|row| row.get(self.x as usize))
            .copied()
    }
}

impl From<(usize, usize)> for IVec2 {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0 as i32, value.1 as i32)
    }
}


fn main() {

    let map: Vec<_> = include_bytes!("input.txt")
        .split(|c| *c == b'\n')
        .map(|v| v.to_owned())
        .collect();

    // Find the initial position of the guard
    let orig_guard_pos: IVec2 = map.iter()
        .enumerate()
        .filter_map(|(y, line)| line.iter().position(|c| *c == b'^').map(|x| (x, y)))
        .next()
        .unwrap()
        .into();

    let mut visited: BTreeSet<IVec2> = BTreeSet::default();
    let mut guard_pos = orig_guard_pos.clone();
    let mut guard_dir = Heading::North;

    // Loop until we are off of the map.
    // The loop will stop automatically when we are out of bounds
    while let Some(c) = guard_pos.head(guard_dir).extract_map_byte(&map) {
        if c == b'#' {
            guard_dir = guard_dir.turn_right();
        } else {
            visited.insert(guard_pos);
            guard_pos = guard_pos.head(guard_dir);
        }
    }
    visited.insert(guard_pos);

    println!("Emplacement count = {}\n\n", visited.len());

    // ----------------------------------------------------------------------
    // Part II
    // ----------------------------------------------------------------------

    let mut emplacements: BTreeSet<IVec2> = BTreeSet::default();
    visited.remove(&orig_guard_pos);

    for new_wall in &visited {

        // This is a bit brute force and could be optimized to cut time in half.
        // But it would still be brute force!

        let mut crumbs: BTreeSet<(IVec2,Heading)> = BTreeSet::default();

        let mut guard_pos = orig_guard_pos.clone();
        let mut guard_dir = Heading::North;
        let mut next_pos  = guard_pos.head(guard_dir);

        while let Some(c) = next_pos.extract_map_byte(&map) {
            if c == b'#' || next_pos == *new_wall {

                // We hit an obstacle so turn right
                guard_dir = guard_dir.turn_right();
                next_pos  = guard_pos.head(guard_dir)
            
            } else if crumbs.contains(&(guard_pos, guard_dir)) {
                
                // We found a place we've been to before!
                emplacements.insert(*new_wall);
                break;

            } else {

                // Proceed in the same direction leaving crumbs behind...
                crumbs.insert((guard_pos, guard_dir));
                guard_pos = next_pos;
                next_pos  = guard_pos.head(guard_dir);
            }            
        }
    }

    println!("Loop possibilities = {}", emplacements.len());
}
