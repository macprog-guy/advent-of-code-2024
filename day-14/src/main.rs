use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct IVec2 { x: i32, y: i32 }

impl IVec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add<IVec2> for IVec2 {
    type Output = IVec2;
    fn add(self, rhs: IVec2) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::Mul<i32> for IVec2 {
    type Output = IVec2;
    fn mul(self, rhs: i32) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl std::fmt::Debug for IVec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl From<&str> for IVec2 {
    fn from(value: &str) -> Self {
        let (sx, sy) = value.split_once(",").unwrap();
        let x: i32 = sx.parse().unwrap();
        let y: i32 = sy.parse().unwrap();
        IVec2::new(x, y)
    }
}


fn main() {

    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;
    const TIME: i32 = 100;

    let robots:Vec<_> = include_bytes!("input.txt")
        .split(|c| *c == b'\n')
        .filter_map(|bytes| {
            if bytes.len() > 0 {
                
                let short: Vec<u8> = bytes.iter()
                    .copied()
                    .filter(|c| *c != b'p' && *c != b'v' && *c != b'=')
                    .collect();

                let text = String::from_utf8(short).unwrap();
                let (spos, sdir) = text.split_once(" ").unwrap();
                let pos: IVec2 = spos.into();
                let dir: IVec2 = sdir.into();

                Some((pos, dir))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
   
    let mut quads = vec![0,0,0,0];
    for (pos, dir) in robots.iter().copied() {
        
        let p = pos + dir * TIME;            
        let x = p.x.rem_euclid(WIDTH);
        let y = p.y.rem_euclid(HEIGHT);

        if x != WIDTH/2 && y != HEIGHT/2 {            
            let qy = y*2 / HEIGHT;
            let qx = x*2 / WIDTH;
            let i  = qy*2 + qx;
            quads[i as usize] += 1;
        }
    }

    println!("Quadrants = {:?}", &quads);
    println!("Safety = {}", quads.into_iter().product::<i32>());

    let mut found_egg = false;

    for t in 0..1_000_000 {
                
        let mut places: HashSet<IVec2> = HashSet::default();
             
        for (pos, dir) in robots.iter().copied() {
            
            let mut p = pos + dir * t;
            p.x = p.x.rem_euclid(WIDTH);
            p.y = p.y.rem_euclid(HEIGHT);

            if places.contains(&p) { break; }

            places.insert(p);
            if places.len() == robots.len() {
                found_egg = true;
                println!("Easter egg at T={}", t);
            }
        }

        if found_egg { break; }
    }
}
