
use std::simd::prelude::*;
pub fn solve_p1(input: &[u8]) -> u64 {
    let mut answer = 0;
    let mut data:Vec<&u8> = Vec::with_capacity(32768);

    let mut len = 141;

    for line in input.split(|&x| {x == b'\n'}) {
        let mut data_line: Vec<&u8> = line.iter().collect();
        data_line.insert(0, &b'0');
        data_line.insert(data_line.len(), &b'0');
        len = data_line.len();
        data.append(&mut data_line);
    }
    let size = 139;

    for (i,char) in data.clone().into_iter().enumerate() {
        let mut count = 0;
        if *char == b'0' || *char == b'.' {
            continue;
        }
        if char == &b'@' {
            if i as i32 - len as i32 > 0 {
                if data[i - (len-1)] == &b'@' {
                    count += 1;
                }
                if data[i - len] == &b'@' {
                    count += 1;
                }
                if data[i - (len + 1)] == &b'@' {
                    count += 1;
                }
            }
            if i + len < (len * size)  {
                if data[i + len +1] == &b'@' {
                    count += 1;
                }
                if data[i + len] == &b'@' {
                    count += 1;
                }
                if data[i + len -1] == &b'@' {
                    count += 1;
                }
            }
            if data[i - 1] == &b'@' {
                count += 1;
            }
            if data[i + 1] == &b'@' {
                count += 1;
            }
            if count < 4 {
                //println!("count = {}", count);
                answer += 1;
            }


        }
        count = 0;
    }
    answer
}

pub fn solve_p1_f(input: &[u8]) -> u64 {
    let mut answer = 0;

    let len = 140;
    let target = SimdVec::splat(b'@');
    const LANES: usize = 8;
    type SimdVec = Simd<u8, LANES>;

    let mut data = [0;8];

    let offsets = usizex8::from_array([0, 1, 2, 140, 142, 240, 241, 242]);

    for i in len+1..input.len()-len-1 {
        if input[i] == b'@' {
            data= [input[i-1], input[i+1], input[i+len], input[i +len +1], input[i+len -1], input[i-len], input[i -(len +1)], input[i-(len -1)]];
            let chk = SimdVec::from_array(data);
            let count = chk.simd_eq(target).to_bitmask().count_ones();
            //let count = vals.simd_eq(target).to_bitmask().count_ones();
            if count <4 {
                answer += 1;
            }
        }
    }
    for i in 1..len {
        if input[i] == b'@' {
            data= [input[i-1], input[i+1], input[i+len], input[i +len +1], input[i+len -1], 0,0,0];
            let chk = SimdVec::from_array(data);
            let count = chk.simd_eq(target).to_bitmask().count_ones();
            if count <4 {
                answer += 1;
            }
        }
    }
    for i in input.len()-len..input.len()-1 {
        if input[i] == b'@' {
            data= [input[i-1], input[i+1], input[i-len], input[i -(len +1)], input[i-(len-1)], 0,0,0];
            let chk = SimdVec::from_array(data);
            let count = chk.simd_eq(target).to_bitmask().count_ones();
            if count <4 {

                answer += 1;
            }
        }
    }
    if input[len] == b'@' {
        data= [input[len-1], input[len+1], input[0], 0, input[1], 0,0,0];
        let chk = SimdVec::from_array(data);
        let count = chk.simd_eq(target).to_bitmask().count_ones();
        if count <4 {

            answer += 1;
        }
    }
    let idx = input.len()-len -1;
    if input[idx] == b'@' {
        data= [input[idx-1], 0, input[idx - len], input[idx + len], input[idx - (len + 1)], input[idx + (len -1)],0,0];
        let chk = SimdVec::from_array(data);
        let count = chk.simd_eq(target).to_bitmask().count_ones();
        if count <4 {

            answer += 1;
        }
    }
    if input[0] == b'@'{
        answer += 1;
    }
    if input[input.len()-1] == b'@'{
        answer += 1;
    }
    answer
}


pub fn solve_p2(input: &str) -> u16 {
    let mut answer = 0;
    let mut data:Vec<Vec<char>> = Vec::with_capacity(256);

    for line in input.split('\n') {
        let mut data_line: Vec<char> = line.chars().collect();
        data_line.insert(0, '0');
        data_line.insert(data_line.len(), '0');
        data.push(data_line);
    }
    let len = data[0].len();
    let size = data.len();

    let mut data: Vec<&char> = data.iter().flatten().collect();
    let mut ans = 0;
    loop {
        for (i, char) in data.clone().iter().enumerate() {
            let mut count = 0;

            if *char == &'0' || *char == &'.' {
                continue;
            }
            if *char == &'@' {
                if i as i32 - len as i32 > 0 {
                    if data[i - (len - 1)] == &'@' {
                        count += 1;
                    }
                    if data[i - len] == &'@' {
                        count += 1;
                    }
                    if data[i - (len + 1)] == &'@' {
                        count += 1;
                    }
                }
                if i + len < (len * size) {
                    if data[i + len + 1] == &'@' {
                        count += 1;
                    }
                    if data[i + len] == &'@' {
                        count += 1;
                    }
                    if data[i + len - 1] == &'@' {
                        count += 1;
                    }
                }
                if data[i - 1] == &'@' {
                    count += 1;
                }
                if data[i + 1] == &'@' {
                    count += 1;
                }
                if count < 4 {
                    data[i] = &'-';
                    ans += 1;
                }


            }
            count = 0;
        }
        if ans == 0 {
            break;
        } else {
            answer += ans;
        }
        ans = 0;
    }
    answer
}

pub fn solve_p2_f(input: &[u8]) -> u64 {
    let mut input =  input.to_vec();
    let mut answer = 0;

    let len = 140;
    let target = SimdVec::splat(b'@');
    const LANES: usize = 8;
    type SimdVec = Simd<u8, LANES>;
    let mut ans = 0;
    loop {
        for i in len + 1..input.len() - len - 1 {
            if input[i] == b'@' {
                let data = [input[i - 1], input[i + 1], input[i + len], input[i + len + 1], input[i + len - 1], input[i - (len + 1)], input[i - (len - 1)], input[i - len]];
                let (pre, chk, post) = data.as_simd();
                let count = chk[0].simd_eq(target).to_bitmask().count_ones();
                if count < 4 {
                    ans += 1;
                    input[i] = b'.';
                }
            }
        }
        for i in 1..len {
            if input[i] == b'@' {
                let data = [input[i - 1], input[i + 1], input[i + len], input[i + len + 1], input[i + len - 1], 0, 0, 0];
                let (pre, chk, post) = data.as_simd();
                let count = chk[0].simd_eq(target).to_bitmask().count_ones();
                if count < 4 {
                    ans += 1;
                    input[i] = b'.';
                }
            }
        }
        for i in input.len() - len..input.len() - 1 {
            if input[i] == b'@' {
                let data = [input[i - 1], input[i + 1], input[i - len], input[i - (len + 1)], input[i - (len - 1)], 0, 0, 0];
                let (pre, chk, post) = data.as_simd();
                let count = chk[0].simd_eq(target).to_bitmask().count_ones();
                if count < 4 {
                    ans += 1;
                    input[i] = b'.';
                }
            }
        }
        if input[len] == b'@' {
            let data = [input[len - 1], input[len + 1], input[0], 0, input[1], 0, 0, 0];
            let (pre, chk, post) = data.as_simd();
            let count = chk[0].simd_eq(target).to_bitmask().count_ones();
            if count < 4 {
                ans += 1;
                input[len] = b'.';
            }
        }
        let idx = input.len() - len - 1;
        if input[idx] == b'@' {
            let data = [input[idx - 1], 0, input[idx - len], input[idx + len], input[idx - (len + 1)], input[idx + (len - 1)], 0, 0];
            let (pre, chk, post) = data.as_simd();
            let count = chk[0].simd_eq(target).to_bitmask().count_ones();
            if count <4 {

                ans += 1;
                input[idx] = b'.';
            }
        }
        if input[0] == b'@'{
            ans += 1;
            input[0] = b'.';
        }
        let idx = input.len()-1;
        if input[idx] == b'@'{
            ans += 1;
            input[idx] = b'.';
        }
        if ans == 0 {
            break;
        }else {
            answer += ans;
            ans = 0;
        }
    }


    answer
}