use std::{env, collections::HashMap};
use chrono::NaiveDate;

pub type GenericError = Box<dyn std::error::Error + Send + Sync>;


pub struct RequestBuilder {
    api_key: String,
    params: HashMap<&'static str, String>,
}


impl RequestBuilder {

    /// instantiate a new RequestBuilder using an API key
    pub fn new(api_key: &str) -> Self {
        let api_key = api_key.to_string();
        let params = HashMap::<&'static str, String>::new();
        RequestBuilder{api_key, params}
    }

    /// instantiate a new RequestBuilder by getting the api_key from an environment variable
    pub fn new_env(variable: &str) -> Self {
        let api_key = env::var(variable).unwrap();
        RequestBuilder::new(&api_key)
    }

    /// generate the url for this request
    pub fn to_url(&self) -> String {
        let mut url = format!("https://api.goperigon.com/v1/all?apiKey={}", &self.api_key);
        for (k, v) in &self.params {
            url.push('&');
            url.push_str(k);
            url.push('=');
            url.push_str(&v);
        }
        url
    }

}


/// These methods below are related to specifying the query 
impl RequestBuilder {

    /// provide a query phrase for searching within title, description, and content fields
    pub fn query(&mut self, query: &str) {
        self.params.insert("q", query.to_string());
    } 

    /// search withing article title+headlines
    pub fn title(&mut self, title: &str) {
        self.params.insert("title", title.to_string());
    } 

    /// search withing article content
    pub fn content(&mut self, content: &str) {
        self.params.insert("content", content.to_string());
    } 

    /// search withing article url: i.e. "travel" etc.
    pub fn url(&mut self, url: &str) {
        self.params.insert("url", url.to_string());
    } 

    /// search by know author ID
    pub fn journalist_id(&mut self, journalist_id: &str) {
        self.params.insert("journalistId", journalist_id.to_string());
    }

    /// filter by articles published on or after a specified date
    pub fn from(&mut self, from_date: &NaiveDate) {
        self.params.insert("from", from_date.to_string());
    } 

    /// filter by articles published on or before a specified date
    pub fn to(&mut self, to_date: &NaiveDate) {
        self.params.insert("to", to_date.to_string());
    } 



}