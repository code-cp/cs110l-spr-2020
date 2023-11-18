```shell
(base) ➜  farm git:(master) ✗ cargo run 100 500 900 10000
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/farm 100 500 900 10000`
Farm starting on 8 cpus
create 1 thread
create 2 thread
create 3 thread
create 4 thread
create 5 thread
500 = 2 * 2 * 5 * 5 * 5 [time: 13.375µs]
create 6 thread
100 = 2 * 2 * 5 * 5 [time: 130.917µs]
900 = 2 * 2 * 3 * 3 * 5 * 5 [time: 19.416µs]
10000 = 2 * 2 * 2 * 2 * 5 * 5 * 5 * 5 [time: 71.042µs]
create 7 thread
create 8 thread
total execution time 315.5µs
```
