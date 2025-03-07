use anyhow::{Context, Error, Result};
use reqwest::blocking::Client;
use serde_json::Value;
use std::env;

const GOOGLE_SCHOLAR_AUTHOR_ENDPOINT: &str = "https://serpapi.com/search";

pub(crate) fn get_author_page() -> Result<Value> {
    // init
    let client = Client::new();
    let author_id =
        env::var("GOOGLE_SCHOLAR_ID").context("Can not found GOOGLE_SCHOLAR_ID env variable.")?;
    let serp_api_key = env::var("SERP_API_KEY").context("Can not found Serp API key.")?;

    // send request
    let r = client
        .get(GOOGLE_SCHOLAR_AUTHOR_ENDPOINT)
        .query(&[
            ("engine", "google_scholar_author"),
            ("author_id", &author_id),
            ("serp_api_key", &serp_api_key),
            ("nums", "20"),
        ])
        .send()?
        .error_for_status()?
        .json::<Value>()?;

    Ok(r)
}

pub(crate) fn get_citation_num_from_author_info(author_info: &Value) -> Result<usize> {
    if let Some(cited_by_info) = author_info.get("cited_by") {
        let citation_num = cited_by_info.get("table").unwrap()[0]
            .get("citations")
            .unwrap()
            .get("all")
            .unwrap()
            .as_u64()
            .unwrap() as usize;
        Ok(citation_num)
    } else {
        Err(Error::msg("No cited by field found in response."))
    }
}

#[derive(Debug)]
pub(crate) struct PaperInfo {
    pub title: String,
    pub num_citations: u16,
    cited_link: Option<String>,
}

pub(crate) fn get_paper_list_from_author_info(author_info: &Value) -> Result<Vec<PaperInfo>> {
    if let Some(paper_info) = author_info.get("articles") {
        let mut paper_list: Vec<PaperInfo> = Vec::new();
        for cur_paper in paper_info.as_array().unwrap() {
            let cur_title = cur_paper
                .get("title")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();

            let cur_num_citation = match cur_paper.get("cited_by") {
                Some(citation_info) => match citation_info.get("value") {
                    Some(val) => val.as_u64().unwrap_or(0) as u16,
                    None => 0,
                },
                None => 0,
            };

            let cur_cited_link = match cur_paper.get("cited_by") {
                Some(citation_info) => citation_info.get("serpapi_link").map(|api_link| api_link.as_str().unwrap().to_string()),
                None => None,
            };

            paper_list.push(PaperInfo {
                title: cur_title,
                num_citations: cur_num_citation,
                cited_link: cur_cited_link,
            });
        }

        Ok(paper_list)
    } else {
        Err(Error::msg("No paper found in response."))
    }
}
