use crate::modules::response_models::author_page::GoogleScholarResponse;
use anyhow::{Context, Error, Result};
use reqwest::blocking::Client;
use std::env;

const GOOGLE_SCHOLAR_AUTHOR_ENDPOINT: &str = "https://serpapi.com/search";

pub fn get_n_author_pages(pages: u8, all: bool) -> Result<GoogleScholarResponse> {
    // get env variables
    let author_id =
        env::var("GOOGLE_SCHOLAR_ID").context("Can not found GOOGLE_SCHOLAR_ID env variable.")?;
    let serp_api_key =
        env::var("SERP_API_KEY").context("Can not found SERP_API_KEY env variable.")?;

    // send request
    let client = Client::new();
    let mut cur_response = client
        .get(GOOGLE_SCHOLAR_AUTHOR_ENDPOINT)
        .query(&[
            ("engine", "google_scholar_author"),
            ("author_id", &author_id),
            ("api_key", &serp_api_key),
            ("sort", "pubdate"),
        ])
        .send()?
        .error_for_status()?
        .json::<GoogleScholarResponse>()?;

    let mut all_responses = vec![cur_response.clone()];
    let mut counter = 1;

    while let Some(next_url) = cur_response.pagination.next.clone() {
        counter += 1;
        if !all {
            if counter > pages {
                break;
            }
        }

        let response = client
            .get(&next_url)
            .query(&[
                ("api_key", &serp_api_key),
                ("sort", &String::from("pubdate")),
            ])
            .send()?
            .error_for_status()?;

        let response_text = response.text()?;

        match serde_json::from_str::<GoogleScholarResponse>(&response_text) {
            Ok(parsed_response) => {
                cur_response = parsed_response;
                all_responses.push(cur_response.clone());
                counter += 1;
                if cur_response.pagination.next.is_none() {
                    break;
                }
            }
            Err(e) => {
                return Err(Error::msg(format!("Failed to parse response: {}", e)));
            }
        }
    }

    // merge all response
    let merged_response = all_responses
        .into_iter()
        .reduce(|mut acc, response| {
            let _ = acc.merge_articles(response);
            acc
        })
        .ok_or_else(|| Error::msg("Failed to merge responses"))?;

    Ok(merged_response)
}
