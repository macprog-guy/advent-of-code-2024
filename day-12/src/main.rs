use std::collections::BTreeSet;

#[derive(Debug,Clone,Copy,Eq,PartialEq,PartialOrd,Ord)]
enum Heading {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy)]
enum Corners {
    NW,
    NE,
    SE,
    SW,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct IVec2 { x: i32, y: i32 }

impl IVec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn map_char(&self, map: &Vec<Vec<u8>>) -> Option<u8> {
        map.get(self.y as usize)
            .and_then(|row| row.get(self.x as usize))
            .copied()
    }

    pub fn map_char_equals(&self, map:&Vec<Vec<u8>>, k: u8) -> bool {
        match self.map_char(map) {
            Some(c) if c == k => true,
            _ => false
        }
    }

    pub fn head(&self, dir: Heading) -> Self {
        use Heading::*;
        match dir {
            North => Self::new(self.x, self.y-1),
            East  => Self::new(self.x+1, self.y),
            South => Self::new(self.x, self.y+1),
            West  => Self::new(self.x-1, self.y),
        }
    }
}

impl std::fmt::Debug for IVec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}


struct Island<'map> {
    pub map: &'map Vec<Vec<u8>>,
    pub start: IVec2,
    pub tiles: Vec<IVec2>,
    pub crop: u8,
}

impl<'map> Island<'map> {
    pub fn new(map: &'map Vec<Vec<u8>>, start: IVec2) -> Option<Self> {
        start.map_char(map)
            .map(|crop| Self { map, tiles: vec![], start, crop })
    }

    pub fn explore(&mut self) {
        self.explore_tile(self.start);
    }

    pub fn explore_tile(&mut self, pos: IVec2) {
        
        use Heading::*;
        if self.tiles.contains(&pos) {
            return;
        }

        if let Some(c) = pos.map_char(self.map) {
            if c == self.crop {
                self.tiles.push(pos);
                for dir in [East, South, West, North] {
                    self.explore_tile(pos.head(dir));
                }
            }
        }
    }

    pub fn same_crop(&self, pos: IVec2) -> bool {
        pos.map_char_equals(self.map, self.crop)
    }

    pub fn perimiter(&self) -> u64 {
        use Heading::*;
        let mut peri = 0u64;
        for tile in &self.tiles {
            if !self.same_crop(tile.head(North)) { peri += 1; }
            if !self.same_crop(tile.head(East)) { peri += 1; }
            if !self.same_crop(tile.head(South)) { peri += 1; }
            if !self.same_crop(tile.head(West)) { peri += 1; }
        }
        peri
    }

    pub fn area(&self) -> u64 {
        self.tiles.len() as u64
    }

    pub fn sides(&self) -> u64 {
        use Heading::*;
        use Corners::*;
        self.tiles
            .iter()
            .copied()
            .flat_map(|pos| [(pos, NW), (pos, NE), (pos, SE), (pos, SW)])
            .filter(|(pos, corner)| {
                
                let (heading_1, heading_2) = match corner {
                    NW => (North, West),
                    NE => (North, East),
                    SE => (South, East),
                    SW => (South, West),
                };

                let same_crop_1 = self.same_crop(pos.head(heading_1));
                let same_crop_2 = self.same_crop(pos.head(heading_2));
                let same_crop_12 = self.same_crop(pos.head(heading_1).head(heading_2));

                // Could be an inner angle or an outer angle
                !same_crop_1 && !same_crop_2 || same_crop_1 && same_crop_2 && !same_crop_12
            })
            .count() as u64
    }
}


fn main() {
    let map:Vec<_> = include_bytes!("input.txt")
        .split(|c| *c == b'\n')
        .map(|v| v.to_owned())
        .collect();

    let mut remains: BTreeSet<_> = map.iter().enumerate()
        .flat_map(|(y, v)| 
            v.iter()
             .enumerate()
             .map(move |(x, _)| IVec2::new(x as i32, y as i32))
        )
        .collect();

    // Just so we start with (0,0)
    let mut cost1 = 0;
    let mut cost2 = 0;
    while let Some(pos) = remains.pop_first() {
        if let Some(mut island) = Island::new(&map, pos) {
            
            island.explore();
            for p in &island.tiles {
                remains.remove(p);
            }

            let crop = String::from_utf8(vec![island.crop]).unwrap();
            println!("Island {} area={}, perimeter={}, sides={}", crop, island.area(), island.perimiter(), island.sides());
            
            cost1 += island.area() * island.perimiter();
            cost2 += island.area() * island.sides();
        }
    }

    println!("Total cost = {}, bulk cost={}", cost1, cost2);
}
