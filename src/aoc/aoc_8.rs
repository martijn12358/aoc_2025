


pub fn solve_p1(input: &str) -> u64 {
    let mut answer = 0;
    let mut cords = Vec::new();
    for line in input.lines() {
        let data = line.split(',').flat_map(|d| d.parse::<i64>()).collect_array::<3>().unwrap();
        cords.push(data);
    }
    //println!("{:?}", cords);

    let mut closest:Vec<[i64; 3]> = Vec::new();
    let mut pairs:Vec<([i64;3], [i64;3])> = Vec::new();
    for i in 0..cords.len() {
        let mut distance = i64::MAX;;
        for j in 0..cords.len() {
            if i != j {
                let distance_x = cords[i][0] - cords[j][0];
                let distance_y = cords[i][1] - cords[j][1];
                let distance_z = cords[i][2] - cords[j][2];
                let new_distance = ((distance_x.pow(2) + distance_y.pow(2)) + distance_z.pow(2)).isqrt() ;
                if new_distance < distance {
                    distance = new_distance;
                    if i < closest.len() {
                        closest[i] = cords[j];
                    } else {
                        closest.insert(i, cords[j]);
                    }

                }
            }
        }
    }
    for i in 0..closest.len() {
        if !pairs.contains(&(cords[i], closest[i])) && !pairs.contains(&(closest[i], cords[i])) {
            if cords[i][0] < closest[i][0] {
                pairs.push((cords[i], closest[i]));
            } else {
                pairs.push((closest[i], cords[i]));
            }

        }

    }
    pairs.sort_unstable();
    //vec of vecs containing all cords combined together
    let mut circuits: Vec<Vec<[i64;3]>> = Vec::new();
    //circuits[0].push(pairs[0].0);
    for i in 0..pairs.len() {

    }
    // make tree, insert data check if cord in tree
    //println!("{:?}", pairs);
    answer
}