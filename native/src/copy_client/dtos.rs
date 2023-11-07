use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub code: u16,
    pub message: String,
    pub results: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tags {
    pub ordering: Vec<Tag>,
    pub theme: Vec<Theme>,
    pub top: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub path_word: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub color_h5: Value,
    pub count: i64,
    pub initials: i64,
    pub logo: Value,
    pub name: String,
    pub path_word: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Page<T> {
    pub list: Vec<T>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComicInSearch {
    pub name: String,
    pub alias: Option<String>,
    pub path_word: String,
    pub cover: String,
    pub ban: i64,
    pub img_type: i64,
    pub author: Vec<Author>,
    pub popular: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub alias: Option<String>,
    pub path_word: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RankItem {
    pub comic: ComicInRank,
    pub date_type: i64,
    pub popular: i64,
    pub rise_num: i64,
    pub rise_sort: i64,
    pub sort: i64,
    pub sort_last: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComicInRank {
    pub author: Vec<Author>,
    pub cover: String,
    pub females: Vec<Value>,
    pub img_type: i64,
    pub males: Vec<Value>,
    pub name: String,
    pub path_word: String,
    pub popular: i64,
    pub theme: Vec<Value>,
}
