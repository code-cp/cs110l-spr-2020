use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Instant; 
use std::{env, process, thread};

#[allow(dead_code)]
fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false; 
    }
    for factor in 2..((num as f64).sqrt().floor() as u32) {
        if num % factor == 0 {
            return false; 
        }
    }
    true
}

fn factor_number(num: u32) {
    let start = Instant::now(); 
    if num == 1 || is_prime(num) {
        println!("{} = {} [time: {:?}]", num, num, start.elapsed());
        return; 
    }
    let mut factors = Vec::new();
    let mut curr_num = num; 
    for factor in 2..num {
        while curr_num % factor == 0 {
            factors.push(factor);
            curr_num /= factor; 
        }
    }
    factors.sort(); 
    let factors_str = factors
        .into_iter()
        .map(|f| f.to_string())
        .collect::<Vec<String>>()
        .join(" * ");
    println!("{} = {} [time: {:?}]", num, factors_str, start.elapsed());
}

fn get_input_numbers() -> VecDeque<u32> {
    let mut numbers = VecDeque::new();
    for arg in env::args().skip(1) {
        if let Ok(val) = arg.parse::<u32>() {
            numbers.push_back(val);
        } else {
            println!("{} is not a valid number", arg);
            process::exit(1);
        }
    }
    numbers
}

fn handle_queue<T>(q: Arc<Mutex<VecDeque<T>>>) -> Option<T>{
    let mut q = q.lock().unwrap(); 
    q.pop_front()
}

fn main() {
    let num_threads = num_cpus::get();
    println!("Farm starting on {} cpus", num_threads);
    let start = Instant::now();
    let numbers = get_input_numbers();
    let q = Arc::new(Mutex::new(numbers));
    let mut handles = Vec::new();
    for i in 0..num_threads {
        let q = q.clone();
        println!("create {} thread", i+1);
        let handle = thread::spawn(move || {
            let num = handle_queue(q); 
            if let Some(num) = num {
                factor_number(num);
            }
        }); 
        handles.push(handle);
    }
    for handle in handles {
        handle.join().expect("panic occurred in thread");
    }
    println!("total execution time {:?}", start.elapsed());
}
