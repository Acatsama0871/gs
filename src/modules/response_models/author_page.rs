use anyhow::Result;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct GoogleScholarResponse {
    pub(crate) author: Author,
    pub(crate) articles: Vec<Article>,
    #[serde(skip_serializing)]
    pub(crate) pagination: Pagination,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub(crate) struct Pagination {
    pub(crate) next: Option<String>,
    pub(crate) current: Option<String>,
    pub(crate) previous: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub(crate) struct Author {
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) author_id: String,
    #[serde(rename = "affiliations")]
    pub(crate) affiliation: String,
    #[serde(default)]
    pub(crate) citations: usize,
    #[serde(default)]
    pub(crate) h_index: usize,
}

#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub(crate) struct ArticlesCiteInfo {
    #[serde(rename = "value")]
    pub(crate) citations: Option<usize>,
    #[serde(rename = "serpapi_link")]
    pub(crate) link: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub(crate) struct Article {
    pub(crate) title: String,
    pub(crate) link: String,
    pub(crate) authors: String,
    pub(crate) citation_id: String,
    #[serde(deserialize_with = "from_str_to_usize_or_default")]
    pub(crate) year: usize,
    #[serde(rename = "cited_by")]
    pub(crate) cited_by: Option<ArticlesCiteInfo>,
}

fn from_str_to_usize_or_default<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        return Ok(0); // Return a default value (0) for empty strings
    }
    s.parse::<usize>().map_err(serde::de::Error::custom)
}

impl<'de> Deserialize<'de> for GoogleScholarResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Value::deserialize(deserializer)?;

        // parse author
        let mut author: Author =
            serde_json::from_value(v["author"].clone()).map_err(serde::de::Error::custom)?;

        // Handle the case where pagination might not exist in the response
        let pagination =
            if v.get("serpapi_pagination").is_some() && !v["serpapi_pagination"].is_null() {
                Pagination {
                    next: v["serpapi_pagination"]["next"]
                        .as_str()
                        .map(|s| s.to_string()),
                    current: v["serpapi_pagination"]["current"]
                        .as_str()
                        .map(|s| s.to_string()),
                    previous: v["serpapi_pagination"]["previous"]
                        .as_str()
                        .map(|s| s.to_string()),
                }
            } else {
                // If pagination doesn't exist, create an empty pagination object
                Pagination {
                    next: None,
                    current: None,
                    previous: None,
                }
            };

        let cited_by_json = &v["cited_by"]["table"];
        let author_id = &v["search_parameters"]["author_id"];
        let citations = cited_by_json
            .get(0)
            .and_then(|c| c.get("citations"))
            .and_then(|c| c.get("all"))
            .and_then(|c| c.as_u64())
            .ok_or_else(|| serde::de::Error::custom("missing citations.all"))?;
        let h_index = cited_by_json
            .get(1)
            .and_then(|h| h.get("h_index"))
            .and_then(|h| h.get("all"))
            .and_then(|h| h.as_u64())
            .ok_or_else(|| serde::de::Error::custom("missing h_index.all"))?;
        author.citations = citations as usize;
        author.h_index = h_index as usize;
        author.author_id = author_id.as_str().unwrap().to_string();

        let articles: Vec<Article> =
            serde_json::from_value(v["articles"].clone()).map_err(serde::de::Error::custom)?;

        Ok(GoogleScholarResponse {
            author,
            articles,
            pagination,
        })
    }
}

impl GoogleScholarResponse {
    pub(crate) fn check_if_author_equals(
        &self,
        another_response_result: &GoogleScholarResponse,
    ) -> bool {
        self.author.author_id == another_response_result.author.author_id
    }

    pub(crate) fn merge_articles(
        &mut self,
        another_response_result: GoogleScholarResponse,
    ) -> Result<()> {
        if !self.check_if_author_equals(&another_response_result) {
            return Err(anyhow::anyhow!("Author ID is not the same!"));
        }
        self.articles.extend(another_response_result.articles);
        Ok(())
    }
}
