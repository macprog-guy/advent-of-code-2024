#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct IVec2 { x: i64, y: i64 }

impl IVec2 {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl std::ops::Mul<i64> for IVec2 {
    type Output = IVec2;
    fn mul(self, rhs: i64) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl std::ops::Add<IVec2> for IVec2 {
    type Output = IVec2;
    fn add(self, rhs: IVec2) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::fmt::Debug for IVec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}


#[derive(Debug)]
struct Game {
    button_a: IVec2,
    button_b: IVec2,
    target: IVec2,
}

impl Game {
    pub fn new(params: &[IVec2]) -> Option<Self> {

        // For Part I set this value to 0
        const OFFSET: i64 = 10000000000000;

        if params.len() < 3 {
            None
        } else {
            Some(Game { 
                button_a: params[0], 
                button_b: params[1], 
                target: params[2] + IVec2::new(OFFSET,OFFSET),
            })
        }
    }

    pub fn solve(&self) -> Option<(i64,i64)> {
        
        let a = self.button_a.x as i128;
        let b = self.button_b.x as i128;
        let c = self.button_a.y as i128;
        let d = self.button_b.y as i128;

        if a*d == b*c {
            None
        } else {
            
            let det = a*d - b*c;
            let tx = self.target.x as i128;
            let ty = self.target.y as i128;

            let n = (d*tx - b*ty) / det;
            let m = (a*ty - c*tx) / det;

            // Check that we have an actual integer solution!
            println!("n={}, m={}, check0 ={},{}", n, m, n*a + m*b - tx, n*c + m*d - ty);
            (n*a + m*b == tx && n*c + m*d == ty).then_some((n as i64, m as i64))
        }
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
                let x: i64 = a.parse().unwrap();
                let y: i64 = b.parse().unwrap();
                Some(IVec2::new(x, y))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let cost: i64 = ivecs.chunks(3)
        .filter_map(Game::new)
        .filter_map(|game| game.solve())
        .map(|(a, b)| 3*a + b)
        .sum();

    
    println!("Tokens spent = {:#?}", cost);
}
