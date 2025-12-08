

pub fn solve_p1(input: &str) -> u64 {
    let mut answer = 0;
    let lines = input.lines().count();
    let mut data = Vec::new();
    for line in input.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        data.push(chars);
    }
    let start_idx = data[0].iter().len() / 2 ;
    data[1][start_idx] = '|';

    for (i,line) in data.clone().iter().enumerate() {
        //println!("{:?}", data[i]);
        for (j,char) in line.iter().enumerate() {
            //println!("{}", data[i][j]);
            if data[i][j] == '^' {
                if data[i-1][j] == '|' {
                    answer += 1;
                    data[i][j+1] = '|';
                    data[i][j-1] = '|';
                    data[i+1][j-1] = '|';
                    data[i+1][j+1] = '|';

                }
            }
            else if data[i][j] == '|' {
                if i+1 < data.len() {
                    if data[i+1][j] != '^' {
                        data[i+1][j] = '|';
                    }
                }
            }
        }
    }


    answer
}

pub fn solve_p2(input: &[u8]) -> u64 {
    let mut answer = 0;
    let mut data = Vec::with_capacity(256);
    for line in input.split(|x1| {*x1 == b'\n'}).step_by(1) {
        let chars = line.into_iter().map(|mut x| {if x == &b'.' {x = &0; *x as u64} else {*x as u64}}).collect::<Vec<u64>>();
        data.push(chars);
    }
    let start_idx = data[0].iter().len() / 2 ;
    data[0][start_idx] = 1;


    for i in 0..data.len() {
        //println!("{:?}", data[i]);
        for j in 0..data[0].len(){
            //println!("{}", data[i][j]);
            if data[i][j] == 94 {
                data[i][j] = 0;
                //println!("{}", data[i][j]);
                if data[i-1][j] != 0 || data[i][j-1] == 49 {
                    //println!("{}", data[i-1][j]);
                    let power = data[i-1][j];
                    data[i][j-1] += power;
                    data[i][j+1] += power;

                    data[i+1][j-1] = data[i][j-1];
                    data[i+1][j+1] = data[i][j+1];

                }
            }
            else if data[i][j] != 0 {
                //println!("{}", data[i][j]);
                if i+1 < data.len() {
                    if data[i+1][j] != 94 {
                        data[i+1][j] = data[i][j];
                    }
                }
            }
        }
        answer = data[data.len()-1].iter().sum();
    }
    answer
}
