use std::simd::prelude::*;
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
    let size = input.len()/100 -1;
    //let size = 1;
    const LANES: usize = 33;
    let mut idx = 0;
    type SimdVec = Simd<u8, LANES>;
    for j in 0..size {
        let line = &input[idx..idx+100];
        //println!("{:?}", line);
        for i in (1..=9) {
            let target = SimdVec::splat(58-i);

            let (prefix, chunks, suffix) = line.as_simd::<LANES>();

            // 4. Process the main SIMD chunks
            let mask :u128 = (chunks[0].simd_eq(target).to_bitmask() as u128) | (chunks[1].simd_eq(target).to_bitmask() as u128) << LANES | (chunks[2].simd_eq(target).to_bitmask() as u128) << 2*LANES ;
            let count = mask.count_ones() ;
           // println!("{:?} ",  count);

            if count >= 2 {
                answer += ((10 * (10 - i)) + 10 -i) as u32;
                break;
            } else if count == 1 {
                if 58-i == suffix[0] {
                    answer += ((10 * (10-i)) + suffix[0] - 48) as u32;
                    break;
                }
                if (58-i)-1 == suffix[0] {
                    answer += ((10 * (10-i)) + suffix[0] - 48) as u32;
                    break;
                }
                let idx = mask.trailing_zeros() as usize +1;
                //println!("{:?}, {}, {}", line, idx, 58-i);
                let line = &line[idx..100];
                let mut num = suffix[0];
                //println!("{:?}", line);
                for v in line.iter() {
                    if v > &num {
                        num = *v;
                    }
                }
                //println!("{}", num);
                answer += (((10 - i) * 10) + num -48) as u32;
                break;
            }

        }
        idx += 101;

    // 5. Handle the remaining elements (head and tail) via standard scalar iteration
    // These are the parts of the array that didn't fit into a perfect 32-byte chunk

        //println!("nines {}", count + scalar_check(prefix) + scalar_check(suffix));
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
