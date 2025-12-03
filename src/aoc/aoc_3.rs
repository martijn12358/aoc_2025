pub fn solve_p1(input: &[u8]) -> u32 {
    let mut answer = 0;
    let mut i = 1;
    let len = input.len();

    while i < len {
        let mut j = i;
        let mut num = input[j-1];
        let mut num2 = input[j];

        while input[j+1] != 10 {

            if input[j] > num {
                num = input[j];
                num2 = input[j + 1];
            } else if input[j] > num2 {
                num2 = input[j];
            }
            j +=1;
            if j+2 == len {
                break;
            }

        }

        if input[j] > num2 {
            num2 =input[j ];
        }
        answer += (((num - 48) * 10) + num2 - 48) as u32;

        i = j + 3;
    }

    answer
}

pub fn solve_p1_fast(input: &[u8]) -> u32 {
    let mut answer = 0;
    let mut i = 1;
    let len = input.len();

    let mut num = input[0];
    let mut num2 = input[1];
    while i < len {
        let mut j = i;
        num = input[j-1];
        num2 = input[j];

        while input[j+1] != 10 {

            if input[j] > num {
                num = input[j];
                num2 = input[j + 1];
            } else if input[j] > num2 {
                num2 = input[j];
            }
            j +=1;
            if j+2 == len {
                break;
            }

        }

        if input[j] > num2 {
            num2 =input[j ];
        }
        answer += (((num - 48) * 10) + num2 - 48) as u32;

        i = j + 3;
    }

    answer
}

pub fn solve_p2(input: &[u8]) -> u64 {
    let mut answer = 0;

    for line in input.split(|&x| x == 10) {
        let mut end = line.len() - 12;
        let total = 12;
        let mut start = 0;
        let mut val = [0u8; 12];
        for k in 0..total {
            let mut num = line[start];
            start += 1;
            for i in start..=end {
                if line[i] > num {
                    num = line[i];
                    start = i + 1;
                }
            }
            val[k] = num;
            end += 1;
        }
        let mut num = 0;
        for i in 0..=11 {
            num = num * 10 + (val[i] - 48) as u64;
        }
        answer += num;
    }
    answer
}

pub fn solve_p2_fast(input: &[u8]) -> u64 {
    let mut answer = 0;

    for line in input.split(|&x| x == 10) {
        let mut end = line.len() - 12;
        let total = 12;
        let mut start = 0;
        let mut val = [0u8; 12];
        for k in 0..total {
            let mut num = line[start];
            start += 1;
            for i in start..=end {
                if line[i] > num {
                    num = line[i];
                    start = i + 1;
                }
            }
            val[k] = num;
            end += 1;
        }
        let mut num = 0;
        for i in 0..=11 {
            num = num * 10 + (val[i] - 48) as u64;
        }
        answer += num;
    }
    answer
}
