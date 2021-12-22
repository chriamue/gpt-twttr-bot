mod ai;
#[cfg(feature = "bert")]
mod gpt2;
mod gptj;

pub use ai::response;
pub use ai::AI;

pub async fn create_ai(ai: String) -> Box<dyn ai::AI> {
    match ai.as_str() {
        "gptj" => Box::new(gptj::GPTJ::default()),
        #[cfg(feature = "bert")]
        "gpt2" => Box::new(gpt2::GPT2::new()),
        _ => Box::new(gptj::GPTJ::default()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_ai() {
        let ai = create_ai("gptj".to_string()).await;
        assert_eq!(ai.name(), "gptj");
        let ai = create_ai("default".to_string()).await;
        assert_eq!(ai.name(), "gptj");
    }
}
