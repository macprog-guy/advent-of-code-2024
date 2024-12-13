use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct IVec2 { x: i32, y: i32 }
impl IVec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn in_map(&self, map_size: Self) -> bool {
        self.x >= 0 && self.x < map_size.x &&
        self.y >= 0 && self.y < map_size.y 
    }
}

impl std::ops::Add<IVec2> for IVec2 {
    type Output = IVec2;
    fn add(self, other: IVec2) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub<IVec2> for IVec2 {
    type Output = IVec2;
    fn sub(self, other: IVec2) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Mul<i32> for IVec2 {
    type Output = IVec2;
    fn mul(self, rhs: i32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl From<(usize, usize)> for IVec2 {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0 as i32, value.1 as i32)
    }
}


fn main() {

    let map:Vec<_> = include_bytes!("input.txt")
        .split(|c| *c == b'\n')
        .map(|v| v.to_owned())
        .collect();

    let map_size: IVec2 = (map[0].len(), map.len()).into();

    let mut signals: HashMap<u8,Vec<IVec2>> = HashMap::default();
    let mut antinodes: HashSet<IVec2> = HashSet::default();

    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().copied().enumerate() {
            if c != b'.' {
                let v = IVec2::new(x as i32, y as i32);
                signals.entry(c)
                    .or_default()
                    .push(v);
            }
        }
    }

    for (_, coords) in &signals {
        for i in 0..coords.len() {
            let a = coords[i];
            for j in (i+1)..coords.len() {
                let b = coords[j];
                let dist = b - a;
                antinodes.insert(a - dist);
                antinodes.insert(b + dist);
            }
        }
    }

    let count = antinodes.iter()
        .filter(|v| v.in_map(map_size))
        .count();

    println!("Count of antinodes on the map: {}", count);


    let mut resonant_nodes: HashSet<IVec2> = HashSet::default();

    for (_, coords) in &signals {
        for i in 0..coords.len() {
            let a = coords[i];
            for j in (i+1)..coords.len() {
                let b = coords[j];
                let dist = b - a;

                let mut a1 = a;
                while a1.in_map(map_size) {
                    resonant_nodes.insert(a1);
                    a1 = a1 - dist;
                }
                
                let mut b1 = b;
                while b1.in_map(map_size) {
                    resonant_nodes.insert(b1);
                    b1 = b1 + dist;
                }
            }
        }
    }

    println!("Count of antinodes on the map: {}", resonant_nodes.len());
}
