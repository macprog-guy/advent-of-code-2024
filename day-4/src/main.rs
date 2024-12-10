fn main() {

    let input = include_bytes!("input.txt");
    let matrix: Vec<_> = input.split(|c| *c == b'\n').collect();
    let mut count = 0;

    // This will only work on square matrices.
    // Ours is 140 x 140 so it's OK.
    for i in 0..matrix.len() as isize {
        for j in 0..matrix.len() as isize {
            
            let word_coords = [
                [(i, j), (i+1, j-1), (i+2, j-2), (i+3, j-3)], // Nord-Est
                [(i, j), (i+1, j  ), (i+2, j  ), (i+3, j  )], // Est,
                [(i, j), (i+1, j+1), (i+2, j+2), (i+3, j+3)], // Sud-Est,
                [(i, j), (i  , j+1), (i  , j+2), (i  , j+3)], // Sud
            ];

            for char_coords in word_coords {
                
                let word = char_coords.iter()
                    .map(|(x,y)| matrix.get(*x as usize).and_then(|row| row.get(*y as usize).copied()).unwrap_or_default())                    
                    .collect::<Vec<u8>>();

                if word == b"XMAS" || word == b"SAMX" {
                    count += 1;
                }
            }
        }
    }
    println!("XMAS occurences: {}", count);

    // This will only work on square matrices.
    // Ours is 140 x 140 so it's OK.
    count = 0;
    for i in 1..(matrix.len()-1) as isize {
        for j in 1..(matrix.len()-1) as isize {
            
            let word_coords = [
                [(i-1, j-1), (i, j), (i+1, j+1)], // NW - SE
                [(i-1, j+1), (i, j), (i+1, j-1)], // SW - NE
            ];

            let is_xmas = word_coords.into_iter()
                .map(|char_coords| 
                    char_coords.iter()
                        .map(|(x,y)| matrix.get(*x as usize).and_then(|row| row.get(*y as usize).copied()).unwrap_or_default())
                        .collect::<Vec<u8>>()
                )
                .map(|word| word == b"MAS" || word == b"SAM")
                .fold(true, |acc, xmas| acc && xmas);

            if is_xmas {
                count += 1;
            }
        }
    }
    println!("X-MAS occurences: {}", count);
}
