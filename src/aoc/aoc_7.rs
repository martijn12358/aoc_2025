

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

pub fn solve_p2(input: &str) -> u128 {
    let mut answer = 0;
    let mut data = Vec::new();
    for line in input.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        data.push(chars);
    }
    let start_idx = data[0].iter().len() / 2 ;
    data[1][start_idx] = '|';

    answer = traverse(&(1, start_idx), &data);


    answer
}
fn traverse(idx : &(usize, usize), data_in: &Vec<Vec<char>>) -> u128 {
    let mut count = 0;
    if idx.0 +1 >= data_in.len() -6 {
        1
    }else {
        if data_in[idx.0 + 1][idx.1] == '^' {
            count += traverse(&(idx.0+2, idx.1 -1), &data_in);
            count += traverse(&(idx.0+2, idx.1 +1), &data_in);
            return count;
        }
        count += traverse(&(idx.0+1, idx.1), &data_in);
        count
    }

}