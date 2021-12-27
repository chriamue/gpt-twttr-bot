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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_response() {
        let ai = GPTJ {};
        let context = "Lots of Tesla cars to deliver before year end! Your support in taking delivery is much appreciated.".to_string();
        let output = ai.response(context.to_string(), 42).await.unwrap();
        println!("{}", output);
        assert_ne!(output, context);
        assert_ne!(output.len(), 0);
        assert!(output.len() > 10);
    }
}
