use std::hint::unreachable_unchecked;

pub fn solve_1(input :&'static[u8]) -> u64 {
    let mut dial = 50;
    let mut answer =0;

    for line in input.split(|&b| b == b'\n') {
        let len = line.len();

        let val = if len == 2 {
            // Vorm "L5"
            (line[1] & 0x0F) as i64
        } else if len == 3 {
            // Vorm "L50"
            let tens = (line[1] & 0x0F) as i64;
            let ones = (line[2] & 0x0F) as i64;
            tens * 10 + ones
        } else {

            let tens = (line[2] & 0x0F) as i64;
            let ones = (line[3] & 0x0F) as i64;
            tens * 10 + ones
        };

        if line[0] == b'R' {
            dial += val;
        }else {
            dial += 100 - val;
        }
        if dial >= 100 {
            dial -= 100;
        }
        if dial == 0 { answer += 1 }
    }
    answer
}

pub fn solve_1_f(input :&'static[u8]) -> u32 {
    let mut dial = 50;
    let mut answer =0;

    let mut index = 0;
    for line in input.split(|&b| b == b'\n') {
        let len = line.len();

        let val =match len {
            2 => line[1] as i64 - 48,
            3 => 10 * line[1] as i64 + line[2] as i64 - 48 - 480,
            _ => {answer+=line[1] as u32 - 48;  10 * line[2] as i64 +  line[ 3] as i64 - 48 - 480},
        };
        if input[0] == b'R' {
            dial += val;
        }else {
            dial += 100 - val;
        }
        if dial >= 100 {
            dial -= 100;
        }
        if dial == 0 { answer += 1 }
    }
    answer
}

pub fn solve_2(input :&'static[u8]) -> u64 {
    let mut dial = 50;
    let mut answer =0;

    let mut index = 0;
    let mut bytes = 0;
    while index < input.len() {
        if input[index] == b'\n' || index == input.len() - 1 {
            if index == input.len() - 1 {
                bytes +=1;
            }
            let num =match bytes {
                2 => input[index -1] as i64 - 48,
                3 => 10 * input[index -2] as i64 + input[index -1] as i64 - 48 - 480,
                _ => {answer+=input[index-3] as u64 - 48;  10 * input[index -2] as i64 +  input[index -1] as i64 - 48 - 480},
            };

            match input[index - bytes] {
                b'L' => {

                    if num > dial {
                        if dial != 0 {
                            answer += 1;
                        }
                        dial = 100 - (num - dial);
                    } else {
                        dial = dial - num;
                    }
                }
                _ => {
                    if num + dial > 99 {
                        if num + dial != 100 {
                            answer += 1;

                            dial = num + dial - 100;

                        }else {
                            dial = 0;
                        }


                    } else {
                        dial = num + dial;
                    }
                }
            }
            if dial == 0 {
                answer += 1;
            }
            bytes = 0;
        }else {
            bytes +=1;
        }
        index +=1;
    }
    answer as u64
}

pub fn solve_2f(input :&'static[u8]) -> u64 {
    let mut dial: i64 = 50;
    let mut answer = 0;

    for line in input.split(|&b| b == b'\n') {
        let len = line.len();

        let val =match len {
            2 => line[1] as i64 - 48,
            3 => 10 * line[1] as i64 + line[2] as i64 - 48 - 480,
            _ => {answer+=line[1] as u64 - 48;  10 * line[2] as i64 +  line[ 3] as i64 - 48 - 480},
        };

        if line[0] == b'L' {
            let next_dial = dial - val;
            if next_dial < 0 {
                if dial != 0 { answer += 1; }
                dial = next_dial + 100;
            } else {
                dial = next_dial;
            }
        } else {
            // 'R' case
            let next_dial = dial + val;
            if next_dial >= 100 {
                if next_dial != 100 { answer += 1; }
                dial = next_dial - 100;
            } else {
                dial = next_dial;
            }
        }

        if dial == 0 {
            answer += 1;
        }
    }
    answer
}