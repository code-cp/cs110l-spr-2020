use crossbeam_channel;
use std::{thread, time};

fn parallel_map<T, U, F>(mut input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where 
F: FnOnce(T) -> U + Send + Copy + 'static,
T: Send + 'static,
U: Send + 'static + Default, 
{
    let mut output_vec: Vec<U> = Vec::with_capacity(input_vec.len());
    output_vec.resize_with(input_vec.len(), Default::default);
    let (in_tx, in_rx) = crossbeam_channel::unbounded(); 
    let (out_tx, out_rx) = crossbeam_channel::unbounded(); 
    let mut threads = Vec::new();
    for _ in 0..num_threads {
        let in_rx = in_rx.clone();
        let out_tx = out_tx.clone();
        threads.push(
            thread::spawn(
                move || {
                    while let Ok(next_item) = in_rx.recv() {
                        let (index, value) = next_item;
                        out_tx.send((index, f(value))).expect("no receiver");
                    }
                }
            )
        );
    }

    let n = input_vec.len();
    for i in 0..n {
        in_tx.send((n-i-1, input_vec.pop().unwrap())).expect("no receiver");    
    }
    
    drop(in_tx);
    drop(out_tx);

    while let Ok(result) = out_rx.recv() {
        let (index, value) = result;
        output_vec[index] = value;
    }

    for thread in threads {
        thread.join().expect("panic occurred in thread");
    }

    output_vec       
}

fn main() {
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let squares = parallel_map(
        v, 
        10, 
        |num| {
            println!("{} squared is {}", num, num*num);
            thread::sleep(time::Duration::from_millis(500));
            num * num
        }
    );
    println!("squares {:?}", squares);
}
