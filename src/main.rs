use anyhow::anyhow;

static VALID: [[u8; 5]; 12] = [
    [0, 1, 2, 3, 4], // ascending
    [1, 2, 3, 4, 5],
    [2, 3, 4, 5, 6],
    [3, 4, 5, 6, 7],
    [4, 5, 6, 7, 8],
    [5, 6, 7, 8, 9],
    [9, 8, 7, 6, 5], // descending
    [8, 7, 6, 5, 4],
    [7, 6, 5, 4, 3],
    [6, 5, 4, 3, 2],
    [5, 4, 3, 2, 1],
    [4, 3, 2, 1, 0],
];

fn main() {
    for set in VALID {
        let compatible = find_compatible(set).unwrap();
        println!("{:?} -> {:?} options", set, compatible.len());
    }
}

fn find_closest(combo: [u8; 5]) -> Vec<[u8; 5]> {
    let mut min_diff = u8::MAX;
    let mut closest_sets: Vec<[u8; 5]> = Vec::new();

    'outer: for set in VALID {
        let mut total_diff = 0;

        for i in 0..5 {
            let mut diff = set[i].abs_diff(combo[i]);

            if diff > 5 {
                diff = 10 - diff;
            }

            total_diff += diff;

            if total_diff > min_diff {
                continue 'outer;
            }
        }

        if total_diff == min_diff {
            closest_sets.push(set);
        } else {
            min_diff = total_diff;
            closest_sets = vec![set];
        }
    }

    closest_sets
}

fn find_compatible(combo: [u8; 5]) -> Result<Vec<[u8; 5]>, anyhow::Error> {
    if !VALID.contains(&combo) {
        return Err(anyhow!("Not sequential"));
    }

    let mut compatible: Vec<[u8; 5]> = Vec::new();

    for i in 0..100_000 {
        let padded = format!("{:0>5}", i.to_string());
        let char_vec: Vec<char> = padded.chars().collect();

        let test_set = [
            char_vec[0].to_digit(10).unwrap() as u8,
            char_vec[1].to_digit(10).unwrap() as u8,
            char_vec[2].to_digit(10).unwrap() as u8,
            char_vec[3].to_digit(10).unwrap() as u8,
            char_vec[4].to_digit(10).unwrap() as u8,
        ];

        let closest = find_closest(test_set);

        if closest.contains(&combo) {
            compatible.push(test_set);
        }
    }

    Ok(compatible)
}
