use std::{env, vec::Vec};
use serde::{Serialize, Deserialize};
use crate::filters;


/// This is the top-level Stringuct that gets returned when you get articles using the /all endpoint
#[derive(Serialize, Deserialize, Debug)]
pub struct Articles {
    pub status: u8,         // typically 200 for OK http response
    pub numResults: u16 ,    // total number of articles returned 
    pub artices: Vec<Article>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
    pub url: String,
    pub authorsByline: Option<String>,   // seems to always be blank 
    pub articleId: String,
    pub clusterId: String,
    pub source: Source,
    pub imageUrl: String,
    pub country: filters::Country,
    pub language: filters::Language,
    pub pubDate: String,
    pub addDate: String,
    pub refreshDate: String,
    pub title: String,
    pub description: String,
    pub content: String,
    pub medium: String,
    pub labels: Vec<Label>,
    pub matchedAuthors: Vec<Author>,
    pub claim: Option<String>,          // seems to always be blank
    pub verdict: Option<String>,        // seems to always be blank
    pub keywords: Vec<Keyword>,
    pub topics: Vec<Topic>,
    pub categories: Vec<filters::Category>,
    pub entities: Vec<Entity>,
    pub sentiment: Sentiment,
    pub summary: String,
    pub translation: Option<String>,
    pub locations: Vec<Location>,
    pub reprint: bool,
    pub places: Option<String>,         // This seems to always be blank
}


#[derive(Serialize, Deserialize, Debug)]

pub struct Sentiment {
    pub positive: f64,
    pub negative: f64,
    pub neutral: f64,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub id: String,     // CHAR(32) id
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Keyword {
    pub name: String,
    pub weight: f64,    // the weighting of this keyword in the article ?
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Topic {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub name: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    pub name: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    pub data: String,   // the name of the entity
    pub r#type: String, // the type of entity
    pub mentions: u16,  // the number of times this entity is mentioned in the article 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub city: String,
    pub state: String, // US state or country 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    pub domain: String, 
}