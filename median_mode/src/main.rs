use std::collections::HashMap;

fn main() {
    let vec_1 = vec![1, 1, 2, 3, 4];
    let vec_2 = vec![2, 2, 8, 9, 26, 27];

    print(get_median(&vec_1), get_mode(&vec_1));
    print(get_median(&vec_2), get_mode(&vec_2));
}

fn print(median: f64, mode: i32) {
    println!("median: {} | mode: {}", median, mode);
}

fn get_median(vec: &[i32]) -> f64 {
    let median_index = vec.len() / 2;

    if vec.len() % 2 != 0 {
        f64::from(vec[median_index])
    } else {
        f64::from(vec[median_index - 1] + vec[median_index]) / 2f64
    }
}

fn get_mode(vec: &[i32]) -> i32 {
    let mut tally: HashMap<i32, i32> = HashMap::new();
    
    let mut mode = 0;

    let mut frequency = 0;

    for int in vec {
        *tally
            .entry(*int)
            .or_insert(0)
             += 1;

        let value = *tally
            .get(int)
            .expect("Value doesn't exist");

        if value > frequency {
            frequency = value;
            mode = *int;
        }
    }
    mode
}

// More elegant answer found on SO using features I haven't learned about yet
// https://codereview.stackexchange.com/questions/173338/calculate-mean-median-and-mode-in-rust
fn _get_mode_fancy(vec: &[i32]) -> i32 {
    let mut tally: HashMap<i32, i32> = HashMap::new();

    for i in vec {
        *tally
            .entry(*i)
            .or_insert(0)
             += 1
    }

    tally
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}