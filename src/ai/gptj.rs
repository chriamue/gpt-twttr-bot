use super::ai::AI;
use async_trait::async_trait;

#[derive(Default)]
pub struct GPTJ {}

#[async_trait]
impl AI for GPTJ {
    async fn response(
        &self,
        context: String,
        token_max_length: u16,
    ) -> Result<String, reqwest::Error> {
        let gpt = gptj::GPT::default();
        let response = gpt
            .generate(context, token_max_length, 0.9, 0.9, None)
            .await;
        Ok(response?.text)
    }

    fn name(&self) -> String {
        "gptj".to_string()
    }
}
