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


fn main() {

    let ivecs:Vec<_> = include_bytes!("input.txt")
        .split(|c| *c == b'\n')
        .filter_map(|bytes| {
            if bytes.len() > 0 {
                let short: Vec<u8> = bytes.iter()
                    .copied()
                    .filter(|c| *c == b',' || *c >= b'0' && *c <= b'9')
                    .collect();

                let text = String::from_utf8(short).unwrap();
                let (a, b) = text.split_once(",").unwrap();
                let x: i32 = a.parse().unwrap();
                let y: i32 = b.parse().unwrap();
                Some(IVec2::new(x, y))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let cost: i32 = ivecs.chunks(3)
        .filter_map(Game::new)
        .filter_map(|game| game.solve())
        .map(|(a, b)| 3*a + b)
        .sum();

    
    println!("Tokens spent = {:#?}", cost);
}
