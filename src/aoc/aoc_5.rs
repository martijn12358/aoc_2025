pub fn solve_p1(input: &str) -> u64 {
    let mut answer = 0;
    let mut parse_range = true;
    let mut ranges = Vec::new();
    let mut numbers = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            parse_range = false;
            continue;
        }
        if parse_range {
            let nums = line.split("-").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
            ranges.push(nums);
        } else {
            let num = line.parse::<u64>().unwrap();
            numbers.push(num);
        }

    }
    ranges.sort();

    for num in numbers {
        for range in &ranges{
            if num >= range[0] {
                if num <= range[1] {
                    answer += 1;
                    break;
                }
            } else {
                break;
            }
        }
    }
    //println!("{:?} {:?}", ranges, numbers);


    answer
}
pub fn solve_p2(input: &str) -> u64 {
    let mut answer = 0;
    let mut ranges = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let nums = line.split("-").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
        ranges.push(nums);
    }
    ranges.sort();
    let mut new_ranges:Vec<Vec<u64>> = Vec::new();
    let mut new_range = Vec::<u64>::new();
    new_range.push(ranges[0][0]);
    new_range.push(ranges[0][1]);
    for range in ranges.iter() {
        if range[0] >= new_range[0] && range[0] <= new_range[1]  {
            if range[1] > new_range[1] {
                new_range[1] = range[1];
            }
        }else if range[0] > new_range[1] {
            new_ranges.push(Vec::from([new_range[0], new_range[1]]));
            new_range[0] = range[0];
            new_range[1] = range[1];
        }
    }
    new_ranges.push(Vec::from([new_range[0], new_range[1]]));
    for range in new_ranges {
        answer += range[1] - range[0] + 1;
    }


    answer
}