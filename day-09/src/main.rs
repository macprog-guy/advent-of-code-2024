use std::collections::VecDeque;
use std::iter::repeat_n;

fn main() {

    let map = include_bytes!("input.txt");

    let mut blocks = VecDeque::default();
    let mut blanks = Vec::default();

    let chunks = map.chunks_exact(2);

    for (i, chunk) in chunks.enumerate() {
        blocks.push_back((i, chunk[0] -  b'0'));
        blanks.push(chunk[1] - b'0');
    }

    // Handle odd block
    if map.len() % 2 == 1 {
        blocks.push_back((blocks.len(), map[map.len()-1] - b'0'));
    }

    // So that we can pop of the back.
    blanks.reverse();

    let mut defrag = Vec::default();    
    let mut back = blocks.pop_back();

    while let Some((front_id, n)) = blocks.pop_front() {
        
        defrag.extend(repeat_n(front_id, n as usize));
        
        let mut free_space = blanks.pop().unwrap_or_default();
        loop {
            if let Some((back_id, ref mut m)) = back {
                if *m <= free_space {
                    defrag.extend(repeat_n(back_id, *m as usize));
                    free_space -= *m;
                    back = blocks.pop_back();
                    if free_space == 0 {
                        break;
                    }
                } else {
                    defrag.extend(repeat_n(back_id, free_space as usize));
                    *m -= free_space;                    
                    break;
                }
            } else {
                break;
            }
        }
    }

    // Handle trailing data in back file
    if let Some((back_id, m)) = back {
        defrag.extend(repeat_n(back_id, m as usize));
    }

    let checksum:usize = defrag.iter()
        .enumerate()
        .map(|(i, file_id)| i * *file_id)
        .sum();

    println!("Checksum 1 = {}\n\n", checksum);


    // ----------------------------------------------------------------------
    // Part II
    // ----------------------------------------------------------------------

    #[derive(Debug)]
    enum Block {
        File(usize, u8),
        Empty(u8),
    }

    impl Block {
        pub fn free_space(&self) -> u8 {
            match self {
                Self::File(_,_) =>  0,
                Self::Empty(n)  => *n,
            }
        }
        pub fn fill_space(&mut self, k: u8) {
            if let Self::Empty(n) = self {
                *n -= k;
            }
        }
    }

    let mut blocks: Vec<Block> = Vec::default();

    let chunks = map.chunks_exact(2);

    for (i, chunk) in chunks.enumerate() {
        blocks.push(Block::File(i, chunk[0] -  b'0'));
        blocks.push(Block::Empty(chunk[1] - b'0'));
    }

    // Handle odd block
    if map.len() % 2 == 1 {
        blocks.push(Block::File(blocks.len()/2, map[map.len()-1] - b'0'));
    }

    for j in (0..blocks.len()).rev() {
        if let Block::File(id, n) = blocks[j] {
            if let Some(i) = blocks.iter().position(|b| b.free_space() >= n) {
                if i < j {                                        
                    blocks[i].fill_space(n);
                    blocks[j] = Block::Empty(n);
                    blocks.insert(i, Block::File(id, n));                    
                }
            }
        }
    }

    let defrag:Vec<_> = blocks.iter()
        .flat_map(|block| {
            match block {
                Block::File(id, n) => repeat_n(*id, *n as usize),
                Block::Empty(n)    => repeat_n(0, *n as usize),
            }
        })
        .collect();

    let checksum:usize = defrag.iter()
        .enumerate()
        .map(|(i, file_id)| i * *file_id)
        .sum();

    println!("Checksum 2 = {}", checksum);
}
