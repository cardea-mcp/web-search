use rmcp::schemars;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct WebSearchConfig {
    pub max_results: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct WebSearchRequest {
    #[schemars(description = "The query to search for")]
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct WebSearchResponse {
    #[schemars(description = "The query that was used for the search")]
    pub query: String,
    #[schemars(description = "The response time of the search")]
    pub response_time: f64,
    #[schemars(description = "The results of the search")]
    pub results: Vec<WebSearchResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct WebSearchResult {
    #[schemars(description = "The content of the search result")]
    pub content: String,
    #[schemars(description = "The raw content of the search result")]
    pub raw_content: Option<String>,
    #[schemars(description = "The score of the search result")]
    pub score: f64,
    #[schemars(description = "The title of the search result")]
    pub title: String,
    #[schemars(description = "The URL of the search result")]
    pub url: String,
}
