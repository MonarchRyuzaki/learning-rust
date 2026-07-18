use std::{thread, time::Duration};

use trpl::{Either, StreamExt};

fn main() {
    trpl::block_on(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|x| 2 * x);
        let mut stream = trpl::stream_from_iter(iter);
        while let Some(value) = stream.next().await {
            print!("The value was: {value}\n");
        }
    });
}
