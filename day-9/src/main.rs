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

    println!("Checksum = {}", checksum);
}
