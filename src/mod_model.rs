use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use std::env;
use std::io::{self, BufRead, Write};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueItem {
    pub sql: String,
    pub uuid: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InsertItem {
    pub prefix: String,
    pub content: String,
    pub vector: Vec<f64>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelectReq {
    pub prefix: String,
    pub limit: i32,
    pub vector: Vec<f64>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelectDbItem {
    pub id: String,
    pub content: String,
    pub vector: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbValueItem {
    pub id: String,
    pub content: String,
    pub distance: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRes {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserCreateReq {
    pub email: String,
    pub password: String,
}
