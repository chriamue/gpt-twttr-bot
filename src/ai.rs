use gptj;
use std::{cmp::Ordering, convert::TryInto};

pub async fn response(context: String, url: String, max_len: u16) -> String {
    let responses = get_n_responses(context, url, max_len.into(), 3).await;
    let best = best(responses, max_len as i16);
    best.chars().into_iter().take(max_len.into()).collect()
}

async fn get_n_responses(context: String, url: String, max_len: usize, n: usize) -> Vec<String> {
    let mut responses: Vec<String> = Vec::new();

    let mut tokens = max_len / 5;

    for _ in 1..n {
        let response = gpt_response(context.clone(), tokens.try_into().unwrap()).await;
        let full_response = match response {
            Ok(response) => format!("{}\n{}", url, response),
            Err(err) => {
                println!("{:?}", err);
                "".to_string()
            }
        };
        tokens = recalc_tokens(tokens, full_response.len(), max_len);
        let diff = full_response.len() as i32 - max_len as i32;
        let new_tokens = (tokens as i32) + (diff/5);
        tokens = new_tokens as usize;
        responses.push(full_response);
    }
    responses
}

async fn gpt_response(context: String, token_max_length: u16) -> Result<String, reqwest::Error> {
    let gpt = gptj::GPT::default();
    Ok(gpt
        .generate(context, token_max_length, 0.9, 0.9, None)
        .await?
        .text)
}

fn sort_by_dist_to_max_len(a: &String, b: &String, max_len: i16) -> Ordering {
    let a_abs = (max_len - (a.len() as i16)).abs();
    let b_abs = (max_len - (b.len() as i16)).abs();
    a_abs.cmp(&b_abs)
}

fn best(responses: Vec<String>, max_len: i16) -> String {
    let mut responses = responses;
    responses.sort_by(|a, b| sort_by_dist_to_max_len(a, b, max_len));
    responses[0].to_string()
}

fn recalc_tokens(tokens: usize, current_len: usize, max_len: usize) -> usize {
    let diff = max_len as i16 - current_len as i16;
    let new_tokens = (tokens as i16) + (diff/5);
    new_tokens as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_by_dist_to_max_len() {
        let a = "123456789".to_string();
        let b = "1".to_string();
        let ord = sort_by_dist_to_max_len(&a, &b, 4);
        assert_eq!(Ordering::Greater, ord);
        let ord = sort_by_dist_to_max_len(&a, &b, 8);
        assert_eq!(Ordering::Less, ord);
        let ord = sort_by_dist_to_max_len(&a, &b, 5);
        assert_eq!(Ordering::Equal, ord);
    }

    #[test]
    fn test_best() {
        let best = best(vec!["1".to_string(), "123456789".to_string()], 8);
        assert_eq!(best, "123456789".to_string());
    }

    #[test]
    fn test_recalc_tokens() {
        let tokens = 5;
        let max_len = 20;
        assert_eq!(4, recalc_tokens(tokens, 25, max_len));
        assert_eq!(6, recalc_tokens(tokens, 15, max_len));
        assert_eq!(5, recalc_tokens(tokens, 19, max_len));
        assert_eq!(5, recalc_tokens(tokens, 21, max_len));
    }
}
