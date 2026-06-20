use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async {
            //completed once finishing sleeping after sending last msg in vals
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            //won't complete until while let loop ends
            while let Some(value) = rx.recv().await {
                //wont end while awaiting until rx.recv
                //produces None, but None only comes once other side of channel is closed
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await; //completed only once BOTH futures have completed
    });
}
