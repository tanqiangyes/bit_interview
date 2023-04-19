mod parse_tx;
mod tx_builder;
use parse_tx::*;

fn main() {
    let tx = parse_tx!({
                tx_hash: "01bee5c80a6bd74440f0f96c983b1107f1a419e028bef7b33e77e8f968cbfae7",
                fee: 10000,
                action: {
                    action: "register",
                    params: "0x48656c6c6f20776f726c6421"
                },
                inputs: [
                    {
                        index: 0,
                        capacity: 10000
                    },
                    {
                        index: 1,
                        capacity: 10000
                    }
                ],
                outputs: [
                    {
                        index: 0,
                        capacity: 10000
                    },
                    {
                        index: 1,
                        capacity: 10000
                    }
                ],
                digest: "01bee5c80a6bd74440f0f96c983b1107f1a419e028bef7b33e77e8f968cbfae7"
            }
        );
    println!("{:?}", tx);
}


