use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct IVec2 { x: i32, y: i32 }
impl IVec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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
    pub fn map_height(&self, map: &Vec<Vec<u8>>) -> Option<u8> {
        map.get(self.y as usize)
            .and_then(|row| row.get(self.x as usize))
            .copied()
    }
}


#[derive(Debug, Clone, Copy)]
enum Heading {
    North,
    East,
    South,
    West,
}


fn main() {
    let map:Vec<_> = include_bytes!("input.txt")
        .split(|c| *c == b'\n')
        .map(|v| v.iter().map(|c| *c - b'0').collect::<Vec<_>>())
        .collect();

    let mut trail_heads: Vec<_> = Vec::default();
    for (y, row) in map.iter().enumerate() {
        for (x, &c) in row.iter().enumerate()  {
            if c == 0 {
                trail_heads.push(IVec2::new(x as i32, y as i32));
            }
        }
    }

    let mut score = 0i32;
    let mut raiting = 0i32;

    for pos in &trail_heads {
        let mut summits: BTreeSet<IVec2> = BTreeSet::default();
        search(&mut vec![*pos], &map, &mut summits, &mut raiting);
        score += summits.len() as i32;
    }

    println!("Score = {}", score);
    println!("Raiting = {}", raiting);
}

fn search(crumbs: &mut Vec<IVec2>, map: &Vec<Vec<u8>>, summits: &mut BTreeSet<IVec2>, raiting: &mut i32) {

    use Heading::*;

    let pos = crumbs.last().copied().unwrap();
        
    if let Some(height) = pos.map_height(map) {
        if height as usize == crumbs.len()-1 {            
            if height == 9 {
                *raiting += 1;
                summits.insert(pos);
            } else {        
                for dir in [North, East, South, West] {
                    let new_pos = pos.head(dir);
                    crumbs.push(new_pos);
                    search(crumbs, map, summits, raiting);
                    crumbs.pop();
                }
            }
        }
    }
}

/*

fn print_solution(crumbs: &Vec<IVec2>, map: &Vec<Vec<u8>>) {
    let mut map = map.clone();
    for y in 0..map.len() {    
        for x in 0..map[y].len() {
            if !crumbs.contains(&IVec2::new(x as i32, y as i32)) {
                map[y][x] = b' ';
            }
        }
    }
    let lines:Vec<_> = map.into_iter()
        .map(|v| v.into_iter().map(|c| if c != b' ' { c + b'0' } else { c }).collect::<Vec<_>>())
        .map(|v| String::from_utf8(v).unwrap()).collect();

    println!("{}", lines.join("\n"));
}

*/