pub fn solve_p1(input: &str) -> u64 {
    let mut answer = 0;
    let lines = input.lines().count();
    let input = input.split_ascii_whitespace().collect::<Vec<_>>();
//    println!("{:?}", input.len()/lines);

    let len =  input.len()/lines;
    for i in 0..len {
        if input[i+(lines-1)*len] == "+" {
            let mut sum = input[i].parse::<u64>().unwrap();
            for j in 1..lines-1{
                sum += input[i+ len* j].parse::<u64>().unwrap();
            }
            answer += sum;
        } else {
            let mut product = input[i].parse::<u64>().unwrap();
            for j in 1..lines-1{
                product *= input[i+ len* j].parse::<u64>().unwrap();
            }
            answer += product;
        }
    }
    answer
}

pub fn solve_p2(input: &str) -> u64 {
    let mut answer = 0;
    let lines = 4;
    let input = input.lines().collect::<Vec<_>>();
    //println!("{:?}", input);
    let mut len = [0;5];
    for i in 0..lines{
        len[i] = input[i].len();
    }

    let max_idx = *len.iter().max().unwrap();


    let mut temp_ans = 0;
    let mut start_idx = 0;
    let mut op = false;
    for i in 0..max_idx{
        if i == start_idx {
            if input[4].chars().collect::<Vec<_>>()[i] == '+'{
                op = false;
            }else {
                op = true;
            }
        }

        let mut num1 = ' ';
        if i < len[0] {
            num1 = input[0].chars().collect::<Vec<_>>()[i];
        }
        let mut num2 = ' ';
        if i < len[1] {
            num2 = input[1].chars().collect::<Vec<_>>()[i];
        }
        let mut num3 = ' ';
        if i < len[2] {
            num3 = input[2].chars().collect::<Vec<_>>()[i];
        }
        let mut num4 = ' ';
        if i < len[3] {
            num4 = input[3].chars().collect::<Vec<_>>()[i];
        }
        let full_num = format!("{}{}{}{}",num1 , num2 , num3, num4).trim().parse::<u64>().unwrap_or(0);

        if full_num != 0 {
            if op {
                if temp_ans != 0{
                    temp_ans *= full_num;
                } else {
                    temp_ans = full_num;
                }

            } else {
                temp_ans += full_num;
            }
        }else {
            //println!("{}", temp_ans);
            answer += temp_ans;
            start_idx = i+1;
            temp_ans = 0;
        }
    }
    answer += temp_ans;


    answer
}

pub fn solve_p2_f(input: &str) -> u64 {
    let mut answer = 0;
    let lines = 4;
    let input = input.lines().map(| x| {x.chars().collect()}).collect::<Vec<Vec<char>>>();
    println!("{:?}", input);
    let mut len = [0;5];
    for i in 0..lines{
        len[i] = input[i].len();
    }

    let max_idx = *len.iter().max().unwrap();


    let mut temp_ans = 0;
    let mut start_idx = 0;
    let mut op = false;
    for i in 0..max_idx{
        if i == start_idx {
            if input[4][i] == '+'{
                op = false;
            }else {
                op = true;
            }
        }

        let mut num1 = ' ';
        if i < len[0] {
            num1 = input[0][i];
        }
        let mut num2 = ' ';
        if i < len[1] {
            num2 = input[1][i];
        }
        let mut num3 = ' ';
        if i < len[2] {
            num3 = input[2][i];
        }
        let mut num4 = ' ';
        if i < len[3] {
            num4 = input[3][i];
        }
        let full_num = format!("{}{}{}{}",num1 , num2 , num3, num4).trim().parse::<u64>().unwrap_or(0);

        if full_num != 0 {
            if op {
                if temp_ans != 0{
                    temp_ans *= full_num;
                } else {
                    temp_ans = full_num;
                }

            } else {
                temp_ans += full_num;
            }
        }else {
            //println!("{}", temp_ans);
            answer += temp_ans;
            start_idx = i+1;
            temp_ans = 0;
        }
    }
    answer += temp_ans;


    answer
}