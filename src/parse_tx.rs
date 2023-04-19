use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Tx {
    pub tx_hash: String,
    pub fee: u64,
    pub action: Action,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub digest: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Action {
    pub  action: String,
    pub  params: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Input {
    pub index: u32,
    pub capacity: u64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Output {
    pub index: u32,
    pub capacity: u64,
}

#[macro_export]
macro_rules! parse_tx {
    (
        {
            tx_hash: $tx_hash:expr,
            fee: $fee:expr,
            action: {
                action: $action:expr,
                params: $params:expr
            },
            inputs: [
                $( {
                    index: $input_index:expr,
                    capacity: $input_capacity:expr
                } ),*
            ],
            outputs: [
                $( {
                    index: $output_index:expr,
                    capacity: $output_capacity:expr
                } ),*
            ],
            digest: $digest:expr
        }
    ) => {{
        let tx = Tx {
            tx_hash: $tx_hash.to_string(),
            fee: $fee,
            action: $crate::parse_tx::Action {
                action: $action.to_string(),
                params: hex::decode($params.trim_start_matches("0x")).unwrap(),
            },
            inputs: vec![
                $(
                    $crate::parse_tx::Input {
                        index: $input_index,
                        capacity: $input_capacity,
                    }
                ),+
            ],
            outputs: vec![
                $(
                    $crate::parse_tx::Output {
                        index: $output_index,
                        capacity: $output_capacity,
                    }
                ),+
            ],
            digest: $digest.to_string(),
        };

        tx
    }};
}

mod test {
    use crate::parse_tx::*;
    #[test]
    fn test_parse_tx() {

         let tx = parse_tx!({
                tx_hash: "01bee5c80a6bd74440f0f96c983b1107f1a419e028bef7b33e77e8f968cbfae7",
                fee: 10000,
                action: {
                    action: "register",
                    params: "0x48656c6c6f20776f726c6421" //Hello world!
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
}