use strength_reduce::StrengthReducedU64;
pub fn solve_p1(input: &str) -> u64 {
    let mut answer = 0;
    for line in input.split(',') {
        let (num1, num2) = line
            .split_once('-')
            .map(|n| (n.0.parse::<u64>().unwrap(), n.1.parse::<u64>().unwrap()))
            .unwrap();
        for i in num1..=num2 {
            let text = i.to_string();
            if text.len() % 2 == 0 {
                let splits = text.split_at(text.len() / 2);
                if splits.0 == splits.1 {
                    answer += i;
                }
            }
        }
    }

    answer
}

const POW10: [u64; 10] = [
    1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
];

pub fn solve_p1_f(input: &str) -> u64 {
    let mut total_answer = 0;

    for line in input.split(',') {
        let Some((s1, s2)) = line.split_once('-') else {
            continue;
        };
        let n1: u64 = s1.parse().unwrap();
        let n2: u64 = s2.parse().unwrap();

        // Iterate through possible "half lengths".
        // u64 max is ~1.8e19, so max length is 19.
        // We only care about even lengths: 2, 4, 6... 18.
        // So 'half_len' goes from 1 to 9.
        let half_len = n1.ilog10() / 2 + 1;
        let power_of_10 = POW10[half_len as usize];
        let multiplier = power_of_10 + 1;

        // X must be a number with exactly 'half_len' digits.
        // Range for X: [10^(d-1), 10^d - 1]
        let x_min_digits = POW10[(half_len - 1) as usize];
        let x_max_digits = power_of_10 - 1;

        // X must also satisfy: n1 <= X * multiplier <= n2
        // Therefore: n1/multiplier <= X <= n2/multiplier
        // We use ceiling for lower bound and floor for upper bound.
        let x_min_val = (n1 + multiplier - 1) / multiplier;
        let x_max_val = n2 / multiplier;

        // Find the intersection of valid digits and valid values
        let start = x_min_digits.max(x_min_val);
        let end = x_max_digits.min(x_max_val);

        if start <= end {
            // Sum of numbers N = X * multiplier
            // Sum = multiplier * Sum(X for X in start..=end)
            // Sum(start..=end) = (start + end) * count / 2
            let count = end - start + 1;
            let sum_x = (start + end) * count / 2;
            total_answer += sum_x * multiplier;
        }
    }

    total_answer
}

pub fn solve_p1_bytes(bytes: &[u8]) -> u64 {
    let mut total_answer = 0;
    let mut i = 0;
    let len = bytes.len();

    while i < len {
        // 1. Parse N1 and count its digits (c1)
        let mut n1: u64 = 0;
        let mut c1: usize = 0;
        while i < len {
            let b = bytes[i];
            if b == b'-' {
                i += 1;
                break;
            }
            n1 = n1 * 10 + (b & 0x0F) as u64;
            c1 += 1;
            i += 1;
        }

        // 2. Parse N2 and count its digits (c2)
        let mut n2: u64 = 0;
        while i < len {
            let b = bytes[i];
            if b == b',' {
                i += 1;
                break;
            }
            n2 = n2 * 10 + (b & 0x0F) as u64;
            i += 1;
        }

        let half_len = (c1 + 1) >> 1; // Bitwise division by 2

        // --- INLINED MATH (No function call overhead) ---
        let power_of_10 = POW10[half_len];
        let multiplier = power_of_10 + 1;

        let x_min_digits = POW10[half_len - 1];
        let x_max_digits = power_of_10 - 1;

        // Fast ceiling division: (n1 + m - 1) / m
        let x_min_val = (n1 + multiplier - 1) / multiplier;
        let x_max_val = n2 / multiplier;

        // Branchless max/min
        let start = if x_min_digits > x_min_val {
            x_min_digits
        } else {
            x_min_val
        };
        let end = if x_max_digits < x_max_val {
            x_max_digits
        } else {
            x_max_val
        };

        if start <= end {
            let count = end - start + 1;
            // Formula: Sum = multiplier * (start + end) * count / 2
            total_answer += multiplier * (start + end) * count / 2;
        }
    }

    total_answer
}

pub fn solve_p1_unsafe(input: &str) -> u64 {
    let mut total_answer = 0;

    // Get raw pointers
    let mut ptr = input.as_ptr();
    // Calculate the end address
    let end = unsafe { ptr.add(input.len()) };

    while ptr < end {
        // 1. Parse N1 (Unsafe, no bounds check)
        let mut n1: u64 = 0;
        let start_ptr = ptr;
        loop {
            // Read byte directly from memory
            let b = unsafe { *ptr };
            if b == b'-' {
                ptr = unsafe { ptr.add(1) }; // Skip '-'
                break;
            }
            // Fast ASCII conversion
            n1 = n1 * 10 + (b & 0x0F) as u64; // b & 0x0F is faster than b - b'0'
            ptr = unsafe { ptr.add(1) };
        }

        // Calculate digits of N1 using pointer difference (Instant)
        // (Current ptr is at dash + 1) - (Start ptr) - 1 (for the dash)
        let c1 = (ptr as usize - start_ptr as usize - 1) as u8;

        // 2. Parse N2
        let mut n2: u64 = 0;
        // let start_ptr2 = ptr; // Not strictly needed unless you want c2
        loop {
            // Check if we hit end of string (EOF protection)
            if ptr == end {
                break;
            }

            let b = unsafe { *ptr };
            if b == b',' {
                ptr = unsafe { ptr.add(1) }; // Skip ','
                break;
            }

            // Only valid digits here
            n2 = n2 * 10 + (b & 0x0F) as u64;
            ptr = unsafe { ptr.add(1) };
        }

        // --- LOGIC ---
        // c1 is enough because we know the length difference is <= 1.
        // If c1 is even, we check c1. If c1 is odd, we check c1 + 1 (which is c2).

        // Trick: We need to check 'c1' if it's even, or 'c1+1' if c1 is odd.
        // Let's just calculate the specific half_len we care about.

        // If c1 is even (e.g. 2), we want half_len=1.
        // If c1 is odd (e.g. 3), we want half_len=2 (because target must be len 4).
        // Formula: half_len = (c1 + 1) / 2
        let half_len = ((c1 + 1) >> 1) as usize;

        let power_of_10 = unsafe { *POW10.get_unchecked(half_len) };
        let multiplier = power_of_10 + 1;

        let x_min_digits = unsafe { *POW10.get_unchecked(half_len - 1) };
        let x_max_digits = power_of_10 - 1;

        let x_min_val = (n1 + multiplier - 1).div_euclid(multiplier);
        let x_max_val = n2 / multiplier;

        let start = if x_min_digits > x_min_val {
            x_min_digits
        } else {
            x_min_val
        };
        let end = if x_max_digits < x_max_val {
            x_max_digits
        } else {
            x_max_val
        };

        if start <= end {
            let count = end - start + 1;
            total_answer += multiplier * (start + end) * count / 2;
        }
    }

    total_answer
}

pub fn solve_p2(input: &str) -> u64 {
    let mut answer = 0;
    for line in input.split(',') {
        let num_range = line
            .split('-')
            .map(|num| num.parse::<u64>().unwrap_or(0))
            .collect::<Vec<u64>>();
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
