use std::{thread, time};
use std::thread::JoinHandle;
use futures::executor::block_on;


fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2
}


async fn do_something2(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2
}

fn main() {
    let now = time::Instant::now();
    let thread_one: JoinHandle<i8> = thread::spawn( || do_something(1));
    let thread_two: JoinHandle<i8> = thread::spawn( || do_something(2));
    let thread_three: JoinHandle<i8> = thread::spawn( || do_something(3));
    let result_one = thread_one.join();
    let result_two = thread_two.join();
    let result_three = thread_three.join();
    println!("time elapsed {:?}", now.elapsed());
    println!("R:{:?}", result_one);
    println!("result {}", result_one.unwrap() + result_two.unwrap() + result_three.unwrap());
/*
    println!("\n");
    let now = time::Instant::now();
    let future_one = do_something2(1);
    let outcome = block_on(future_one);
    println!("time elapsed {:?}", now.elapsed());
    println!("Here is the outcome: {}", outcome);
*/
    println!("\n");
    let now = time::Instant::now();
    let future_two = async {
        return do_something2(2).await
    };
    let future_two = block_on(future_two);
    println!("time elapsed {:?}", now.elapsed());
    println!("Here is the outcome: {:?}", future_two);

    println!("\n");
    let now = time::Instant::now();
    let future_two = async {
        let outcome_two = do_something2(2).await;
        let outcome_three = do_something2(3).await;
        return [outcome_two, outcome_three]
    };
    let future_two = block_on(future_two);
    println!("time elapsed {:?}", now.elapsed());
    println!("Here is the outcome: {:?}", future_two);

    println!("\n");
    let now = time::Instant::now();
    use futures::join;
    let second_outcome = async {
        let future_two = do_something2(22);
        let future_three = do_something2(33);
        return join!(future_two, future_three)
    };
    let now = time::Instant::now();
    let result = block_on(second_outcome);
    println!("time elapsed {:?}", now.elapsed());
    println!("here is the result: {:?}", result);

    println!("1\n");
    use std::vec::Vec;
    use async_std;
    use futures::future::join_all;
    let third_outcome = async {
        println!("3\n");
        let mut futures_vec = Vec::new();
        let future_four = do_something2(4);
        let future_five = do_something2(5);
        futures_vec.push(future_four);
        futures_vec.push(future_five);
        let handles = futures_vec.into_iter().map( async_std::task::spawn).collect::<Vec<_>>();
        let results = join_all(handles).await;
        println!("4\n");
        return results
    };
    println!("2\n");
    let now = time::Instant::now();
    let result = block_on(third_outcome);
    println!("time elapsed for join vec {:?}", now.elapsed());
    println!("Here is the result: {:?}", result);

    println!("--\n");
    use std::sync::Arc;
    use std::thread;
    let names = Arc::new(vec!["dave", "chloe", "simon"]);
    let reference_data = Arc::clone(&names);

    let new_thread = thread::spawn(move || {
        println!("{}", reference_data[1]);
    });

    println!("--\n");
    use std::sync::Mutex;
    let count = Mutex::new(0);

    let new_thread = thread::spawn(move || {
        *count.lock().unwrap() += 1;
    });

}