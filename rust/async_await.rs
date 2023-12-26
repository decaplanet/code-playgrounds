// Add tokio in dependencies.
// [dependencies]
// tokio = { version = "1.28.2", features = ["rt", "rt--thread", "macros"] }

use std::{thread::sleep, time::Duration};

#[tokio::main]
async fn main() {
    println!("Here we go!");
    let fn1 = string_function().await;

    println!("{}", fn1);
    println!("Exiting main function..");
}

async fn string_function() -> String {
    println!("Waiting 5 seconds..");
    sleep(Duration::from_secs(5));

    String::from("string_function completed!")
}
