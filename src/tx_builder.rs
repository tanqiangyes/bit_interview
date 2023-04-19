use hex::encode;
use crate::parse_tx::*;

struct HttpClient {
}

impl HttpClient {
    pub fn get(&mut self, query: &str) -> String{
        r#"[{"index":0,"capacity":10000},{"index":1,"capacity":10000}]"#.to_string()
    }
}

struct TxBuilder<'a> {
    http_client: &'a mut HttpClient,
}

impl<'a> TxBuilder<'a> {
    // 新建 TxBuilder 实例，需要传入一个可变的 HttpClient 引用
    pub fn new(http_client: &'a mut HttpClient) -> Self {
        Self { http_client }
    }

    // 获取输入事务信息
    pub fn get_input(&mut self, tx_id: [u8; 32]) -> Tx {
        let query = format!("/input/{}", hex::encode(tx_id));
        let inputs = self.http_client.get(&query);
        // 解析 result 并返回 Tx 结构体
        Tx {
            inputs: serde_json::from_str(inputs.as_str()).unwrap(),
            ..Default::default()
        }
    }

    // 获取输出事务信息
    pub fn get_output(&mut self, tx_id: [u8; 32]) -> Tx {
        let query = format!("/output/{}", hex::encode(tx_id));
        let outputs = self.http_client.get(&query);
        // 解析 result 并返回 Tx 结构体

        Tx {
            outputs: serde_json::from_str(outputs.as_str()).unwrap(),
            ..Default::default()
        }
    }

    // 构建事务
    pub fn build(&mut self) -> Tx {
        // 初始化一个tx_id
        let tx_id = [0;32];
        let mut tx = Tx {
            tx_hash: String::from_utf8(tx_id.to_vec()).unwrap(),
            fee: 0,
            action: Action { action: "".to_string(), params: vec![] },
            inputs: vec![],
            outputs: vec![],
            digest: "".to_string(),
        };

        // 获取输入和输出事务信息
        let input_tx = self.get_input(tx_id);
        let output_tx = self.get_output(tx_id);

        // 构建新的事务并返回
        tx.inputs = input_tx.inputs;
        tx.outputs = output_tx.outputs;
        tx
    }
}


mod test {
    use crate::tx_builder::*;

    #[test]
    fn test_tx_build() {
        let mut http_client = HttpClient{};
        let mut tx_build = TxBuilder::new(&mut http_client);
        println!("{:?}", tx_build.build());
    }
}