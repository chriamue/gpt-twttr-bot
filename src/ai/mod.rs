mod ai;
mod gptj;

pub use ai::response;
pub use ai::AI;

pub fn create_ai(ai: String) -> Box<dyn ai::AI> {
    match ai.as_str() {
        "gptj" => Box::new(gptj::GPTJ::default()),
        _ => Box::new(gptj::GPTJ::default()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_ai() {
        let ai = create_ai("gptj".to_string());
        assert_eq!(ai.name(), "gptj");
        let ai = create_ai("default".to_string());
        assert_eq!(ai.name(), "gptj");
    }
}
