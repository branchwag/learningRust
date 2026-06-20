use std::time::Duration;

fn main() {
    trpl::block_on(async {
        //polls future
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await; //let other tasks run
            }
        }); //creating a new task to print one thing while the main task prints something else

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await; //let other tasks run
        }
    });
}
