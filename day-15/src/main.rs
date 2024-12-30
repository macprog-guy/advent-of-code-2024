use core::str;

use array2d::Array2D;

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    pub fn inverse(&self) -> Self {
        use Heading::*;
        match self {
            North => South,
            East  => West,
            South => North,
            West  => East,
        }
    }
}

impl TryFrom<u8> for Heading {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'^' => Ok(Self::North),
            b'>' => Ok(Self::East),
            b'v' => Ok(Self::South),
            b'<' => Ok(Self::West),
            _    => Err("Invalid Heading")
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct IVec2 { x: i32, y: i32 }
impl IVec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn head(&self, head: Heading) -> Self {
        use Heading::*;
        match head {
            North => Self { x: self.x, y: self.y-1 },
            East  => Self { x: self.x+1, y: self.y },
            South => Self { x: self.x, y: self.y+1 },
            West  => Self { x: self.x-1, y: self.y },
        }
    }
    pub fn jump(&self, head: Heading) -> Self {
        use Heading::*;
        match head {
            North => Self { x: self.x, y: self.y-2 },
            East  => Self { x: self.x+2, y: self.y },
            South => Self { x: self.x, y: self.y+2 },
            West  => Self { x: self.x-2, y: self.y },
        }
    }
}

impl From<IVec2> for (usize, usize) {
    fn from(value: IVec2) -> Self {
        (value.y as usize, value.x as usize)
    }
}


fn main() {

    let mut text: Vec<_> = include_bytes!("input-mini.txt")
        .split(|c| *c == b'\n')
        .map(|v| v.to_owned())
        .collect();

    let heading_index = text.iter()
        .position(|v|v.is_empty())
        .unwrap_or_default();

    let headings: Vec<u8> = text.split_off(heading_index+1)
        .into_iter()
        .flatten()
        .collect();

    text.pop();

    // Create an Array2D to represent the room
    let original_room = Array2D::from_rows(&text).unwrap();
    let mut original_robot_pos: Option<IVec2> = None;

    for (y, row_iter) in original_room.rows_iter().enumerate() {
        for (x, byte) in row_iter.enumerate() {
            if *byte == b'@' {
                original_robot_pos = Some(IVec2::new(x as i32, y as i32));
            }
        }
    }

    let original_robot_pos = original_robot_pos
        .expect("No initial robot position!");

    // Replace the robot by an empty square
    let mut room = original_room.clone();
    let mut robot_pos = original_robot_pos.clone();
    room[robot_pos.into()] = b'.';
    
    for head_byte in &headings {

        let direction = Heading::try_from(*head_byte)
            .expect("Invalid Heading");

        let next_pos = robot_pos.head(direction);
        
        match room[next_pos.into()] {
            b'.' => { 
                robot_pos = next_pos; 
            }
            b'O' => {
                if push_boxes(next_pos, direction, &mut room) {
                    robot_pos = next_pos;
                }
            },            
            _ => { }
        }
    }

    let sum: usize = room.rows_iter()
        .enumerate()
        .flat_map(|(y, row)|
            row.enumerate()
            .filter_map(move |(x, c)| (*c == b'O').then_some(100*y+x))
        )
        .sum();

    println!("Part #1, sum of GPS Coords: {}\n", sum);

    // ----------------------------------------------------------------------
    // Part #2
    // ----------------------------------------------------------------------

    // Replace the robot by an empty square
    let mut room = scale_room(&original_room);

    for (y, row_iter) in room.rows_iter().enumerate() {
        for (x, byte) in row_iter.enumerate() {
            if *byte == b'@' {
                robot_pos = IVec2::new(x as i32, y as i32);
            }
        }
    }
    
    room[robot_pos.into()] = b'.';
    for head_byte in &headings {

        let direction = Heading::try_from(*head_byte)
            .expect("Invalid Heading");

        show_room(&room, robot_pos, direction);
        let next_pos = robot_pos.head(direction);

        match room[next_pos.into()] {
            b'.' => {
                println!("Found empty space. Moving there.");
                robot_pos = next_pos;
            }
            b'[' | b']' => {
                if push_big_boxes(next_pos, direction, &mut room) {
                    println!("Found a box. Pushing it.");
                    robot_pos = next_pos;
                } else {
                    println!("Found a box. But it's stuck!");
                }
            },
            _ => { }
        }
    }

    let sum: usize = room
        .rows_iter()
        .enumerate()
        .flat_map(|(y, row)| row.enumerate().filter_map(move |(x, c)| (*c == b'O').then_some(100*y+x)))
        .sum();

    println!("Part #2, sum of GPS Coords: {}\n", sum);
}

fn push_big_boxes(box_pos: IVec2, dir: Heading, room: &mut Array2D<u8>) -> bool {

    use Heading::*;

    let next_pos = box_pos.head(dir);
    let next_idx = next_pos.into();

    match room[next_idx] {
        b'[' if dir == East => {
            if push_big_boxes(next_pos.jump(dir), dir, room) {
                room[box_pos.into()] = b'.';
                room[next_pos.into()] = b'[';
                room[next_pos.head(dir).into()] = b']';
                true
            } else {
                false
            }     
        }
        b']' if dir == West => {
            if push_big_boxes(next_pos.jump(dir), dir, room) {
                room[box_pos.into()] = b'.';
                room[next_pos.into()] = b']';
                room[next_pos.head(dir).into()] = b'[';
                true
            } else {
                false
            }     
        }
        b'[' if dir == North || dir == South => {
            if push_big_boxes(next_pos, dir, room) && push_big_boxes(next_pos.head(East), dir, room) {
                room[box_pos.into()] = b'.';
                room[box_pos.head(East).into()] = b'.';
                room[next_pos.into()] = b'[';
                room[next_pos.head(East).into()] = b']';
                true
            } else {
                false
            }
        }
        b']' if dir == North || dir == South => {
            if push_big_boxes(next_pos, dir, room) && push_big_boxes(next_pos.head(West), dir, room) {
                room[box_pos.into()] = b'.';
                room[box_pos.head(West).into()] = b'.';
                room[next_pos.into()] = b'[';
                room[next_pos.head(West).into()] = b']';
                true
            } else {
                false
            }
        }
        b'.' => {
            let inv_dir = dir.inverse();
            room[box_pos.into()] = b'.';
            room[box_pos.into()] = b'[';
            room[next_pos.head(dir).into()] = b']';
            true
        }
        _ => false,
    }
}

fn push_boxes(box_pos: IVec2, dir: Heading, room: &mut Array2D<u8>) -> bool {
    
    let next_pos = box_pos.head(dir);
    let next_idx = next_pos.into();

    match room[next_idx] {
        b'O' => {
            if push_boxes(next_pos, dir, room) {
                room[next_pos.into()] = b'O';
                room[box_pos.into()] = b'.';
                true
            } else {
                false
            }     
        }
        b'.' => {
            room[next_idx] = b'O';
            room[box_pos.into()] = b'.';
            true
        }
        _ => false,
    }
}

#[allow(unused)]
fn show_room(room: &Array2D<u8>, robot_pos:IVec2, heading: Heading) {

    println!("Position ({},{}) heading {:?}", robot_pos.x, robot_pos.y, heading);

    let mut room1 = room.clone();
    room1[robot_pos.into()] = match heading {
        Heading::North => b'^',
        Heading::East  => b'>',
        Heading::South => b'V',
        Heading::West  => b'<',
    };

    for row in room1.rows_iter() {
        let v: Vec<_> = row.copied().collect();
        let s = str::from_utf8(&v).expect("Invalid UTF8");
        println!("{}", &s);
    }
    println!("");
}

#[allow(unused)]
fn scale_room(room: &Array2D<u8>) -> Array2D<u8> {
    let rows: Vec<Vec<u8>> =
        room.rows_iter()
            .map(|row|
                row.flat_map(|c| {
                        match *c {
                            b'@' => [b'@', b'.'],
                            b'.' => [b'.', b'.'],
                            b'O' => [b'[', b']'],
                            _    => [b'#', b'#'],
                        }
                    })
                    .collect()
            )
            .collect();

    Array2D::from_rows(&rows).unwrap()
}