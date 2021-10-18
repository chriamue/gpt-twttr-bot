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
            Ok(response) => format!("{}{}", url, response),
            Err(err) => {
                println!("{:?}", err);
                "".to_string()
            }
        };
        match full_response.len() {
            d if d < max_len => tokens += 2,
            d if d > max_len => tokens -= 2,
            _ => {}
        }
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
}
