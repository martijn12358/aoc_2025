use std::io::BufRead;

pub fn solve_p1(input: &str ) -> u64 {
    let mut answer = 0;
    for line in input.split(',') {
        let (num1, num2) = line.split_once('-').map(|n| {(n.0.parse::<u64>().unwrap(), n.1.parse::<u64>().unwrap())}).unwrap();
        for i  in num1..=num2 {
            let text = i.to_string();
            if text.len()%2 == 0 {
                let splits = text.split_at(text.len()/2);
                if splits.0==splits.1 {
                    answer +=i;
                }
            }
        }
    }


    answer
}

pub fn solve_p2(input: &str ) -> u64 {
    let mut answer = 0;
    for line in input.split(',') {
        let num_range = line.split('-').map(|num| { num.parse::<u64>().unwrap_or(0) }).collect::<Vec<u64>>();
        for i in num_range[0]..=num_range[1] {
            let text = i.to_string();
            for j in 1..=text.len() / 2 {
                if text.len() % j == 0 {
                    if text[0..j].repeat(text.len() / j) == text {
                        answer += i;
                        break;
                    }
                }
            }
        }
    }
    answer
}