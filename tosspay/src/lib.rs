pub struct TossPay {
    toss_id: String,
}
impl TossPay {
    pub fn new(toss_id: String) -> Self {
        TossPay { toss_id }
    }
    pub async fn pay(&self, amount: usize) {
        let toss_response = reqwest::Client::new()
            .get(format!("https://api-public.toss.im/api-public/v3/cashtag/transfer-feed/received/list?inputWord={}", self.toss_id));
        let json = serde_json::from_str::<TossPayJson>(
            &toss_response
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
                .replace("null", "\"\""),
        )
        .unwrap();
        println!("{:?}", json);
    }
}

#[allow(non_snake_case)]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TossPayJson {
    pub resultType: String,
    pub success: TossPaySuccess,
}

#[allow(non_snake_case)]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TossPaySuccess {
    pub nextCursor: String,
    pub data: Vec<TossPayData>,
}

#[allow(non_snake_case)]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TossPayData {
    senderDisplayName: String,
    amount: usize,
    msg: String,
}
