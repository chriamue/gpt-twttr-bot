use super::ai::AI;
use async_trait::async_trait;
use rust_bert::gpt2::GPT2Generator;
use rust_bert::pipelines::generation_utils::GenerateOptions;
use rust_bert::pipelines::generation_utils::LanguageGenerator;

pub struct GPT2 {
    model: GPT2Generator,
}

impl GPT2 {
    pub fn new() -> Self {
        let model = std::thread::spawn(move || {
            let model = GPT2Generator::new(Default::default()).unwrap();
            model
        })
        .join()
        .expect("Thread panicked");
        GPT2 { model }
    }
}

unsafe impl Send for GPT2 {}

unsafe impl Sync for GPT2 {}

#[async_trait]
impl AI for GPT2 {
    async fn response(
        &self,
        context: String,
        token_max_length: u16,
    ) -> Result<String, reqwest::Error> {
        let generate_options = GenerateOptions {
            max_length: Some(token_max_length.into()),
            do_sample: Some(true),
            early_stopping: Some(true),
            repetition_penalty: Some(1.2),
            temperature: Some(1.4),
            top_p: Some(0.9),
            top_k: Some(40),
            ..Default::default()
        };

        let output = self
            .model
            .generate(Some(&[context.to_string()]), Some(generate_options));
        let response = output[0].text.to_string();
        let response = response.replace(context.as_str(), "");
        Ok(response)
    }

    fn name(&self) -> String {
        "gpt2".to_string()
    }
}
