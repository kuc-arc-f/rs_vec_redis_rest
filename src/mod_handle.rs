use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post},
    Router,
};
use redis::{Commands, RedisResult};
use redis::{Connection};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::fs;
use std::io;
use serde_json::{json, Map, Value};
use uuid::Uuid;
use dotenvy::dotenv;
use std::env;

fn cosine_similarity(a: &[f64], b: &[f64]) -> Result<f64, Box<dyn std::error::Error>> {
    if a.len() != b.len() {
        return Err("error , vectors must have the same length".into());
    }

    let mut dot_product = 0.0_f64;
    let mut a_magnitude = 0.0_f64;
    let mut b_magnitude = 0.0_f64;

    for i in 0..a.len() {
        dot_product += (a[i] * b[i]) as f64;
        a_magnitude += (a[i] * a[i]) as f64;
        b_magnitude += (b[i] * b[i]) as f64;
    }

    if a_magnitude == 0.0 || b_magnitude == 0.0 {
        return Ok(0.0);
    }

    Ok(dot_product / (a_magnitude.sqrt() * b_magnitude.sqrt()))
}

pub fn valid_vector_len(
    items : Vec<super::mod_model::SelectDbItem>, embedding: Vec<f64>
)-> bool
{
    let mut ret: bool = false;
    println!("items.len={}", items.len());
    println!("embedding.len={}", embedding.len());
    if items.len() == 0 {
        //println!("error, valid_vector_len =0");
        return true;
    }
    println!("{:?}", items[0]);
    println!("vlen={}", items[0].vector.len());

    return ret;
}

pub fn get_items_list(
    state: State<super::AppState> , prefix: String
) -> Vec<super::mod_model::SelectDbItem>
{
    let mut conn = state.lock().unwrap();
    let prefix_str = format!("{}*", &prefix);
    println!("prefix_str={}", &prefix_str);
    let mut cursor: u64 = 0;

    // 取得した KEY を格納
    let mut keys: Vec<String> = Vec::new();

    loop {
        // SCAN で KEY を取得
        let (next_cursor, mut result_keys): (u64, Vec<String>) =
            redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(&prefix_str)
                .arg("COUNT")
                .arg(100)
                .query(&mut conn).unwrap();
//              .arg("doc:*")

        // KEY を配列に追加
        keys.append(&mut result_keys);

        // 次のカーソルを設定
        cursor = next_cursor;

        // カーソルが 0 なら終了
        if cursor == 0 {
            break;
        }
    }

    let mut vec1: Vec<super::mod_model::SelectDbItem> = Vec::new();

    for key in &keys {
        println!("KEY: {}", key);
        let key2 = key.replace(&prefix, "");
        println!("id={}", &key2);
        // GET コマンドで VALUE を取得
        let value: Option<String> = conn.get(key).unwrap();
        // 結果を表示
        let mut content_str = "".to_string();
        match value {
            Some(v) => {
                content_str = v.clone();
                println!("VALUE: {}", v);
            }
            None => {
                println!("KEY が存在しません");
            }
        }    
        //vector_value
        let keyVec = format!("vec{}", key);
        let value2: Option<String> = conn.get(keyVec).unwrap();
        let mut vec_str : String = "".to_string();
        match value2 {
            Some(v2) => {
                vec_str = v2.clone();
                //println!("vec={}", v2);
            }
            None => {
                println!("KEY が存在しません");
            }
        }
        //println!("vec_str={}", vec_str);        
        let vec3: Vec<f64> = vec_str.trim_matches(|c| c == '[' || c == ']') 
        .split(',') 
        .map(|s| s.trim().parse::<f64>().unwrap()) 
        .collect();        
        //println!("vec3={:?}", vec3);   
        
        let row = super::mod_model::SelectDbItem{
            id: key2,
            content: content_str,
            vector: vec3,
        };
        vec1.push(row);

    }
    return vec1;
}


pub async fn select_handle(
    State(state): State<super::AppState>,
    Json(payload): Json<super::mod_model::SelectReq>,
) -> (StatusCode, String) {
    let mut conn = state.lock().unwrap();
    let prefix = &payload.prefix;
    println!("prefix={}", &prefix);
    let prefix_str = format!("{}*", &prefix);
    println!("prefix_str={}", &prefix_str);
    let limit = &payload.limit;
    println!("limit={}", &limit);

    let mut cursor: u64 = 0;

    // 取得した KEY を格納
    let mut keys: Vec<String> = Vec::new();

    loop {
        // SCAN で KEY を取得
        let (next_cursor, mut result_keys): (u64, Vec<String>) =
            redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(&prefix_str)
                .arg("COUNT")
                .arg(100)
                .query(&mut conn).unwrap();
//              .arg("doc:*")

        // KEY を配列に追加
        keys.append(&mut result_keys);

        // 次のカーソルを設定
        cursor = next_cursor;

        // カーソルが 0 なら終了
        if cursor == 0 {
            break;
        }
    }

    // 取得した KEY を表示
    println!("取得件数: {}", keys.len());

    let mut vec1: Vec<super::mod_model::SelectDbItem> = Vec::new();

    for key in &keys {
        //println!("KEY: {}", key);
        let key2 = key.replace(prefix, "");
        //println!("id={}", &key2);
        // GET コマンドで VALUE を取得
        let value: Option<String> = conn.get(key).unwrap();
        // 結果を表示
        let mut content_str = "".to_string();
        match value {
            Some(v) => {
                content_str = v.clone();
                //println!("VALUE: {}", v);
            }
            None => {
                println!("KEY が存在しません");
            }
        }    
        //vector_value
        let keyVec = format!("vec{}", key);
        let value2: Option<String> = conn.get(keyVec).unwrap();
        let mut vec_str : String = "".to_string();
        match value2 {
            Some(v2) => {
                vec_str = v2.clone();
                //println!("vec={}", v2);
            }
            None => {
                println!("KEY が存在しません");
            }
        }
        //println!("vec_str={}", vec_str);        
        let vec3: Vec<f64> = vec_str.trim_matches(|c| c == '[' || c == ']') 
        .split(',') 
        .map(|s| s.trim().parse::<f64>().unwrap()) 
        .collect();        
        //println!("vec3={:?}", vec3);   
        
        let row = super::mod_model::SelectDbItem{
            id: key2,
            content: content_str,
            vector: vec3,
        };
        vec1.push(row);

    }
    let vector = &payload.vector;
    if vec1.len() > 0 {
        let target_row = vec1[0].vector.clone();
        println!("target_row.len={}", target_row.len());
        let input_len = &vector.len();
        println!("input_len={}", input_len);
        if *input_len != target_row.len() {
            println!("error, valid_vector_len = NG");
            return (StatusCode::BAD_REQUEST, "NG".to_string());
        }
    }
    //println!("vector={:?}", vector);

    let mut scoreItems = Vec::<super::mod_model::DbValueItem>::new();
    for item in &vec1 {
        //println!("id={}", item.id);
        //println!("content={}", item.content);
        //println!("vector={:?}", item.vector);
        let distance = cosine_similarity(&vector, &item.vector).unwrap();
        if distance > 0.6 {
            //println!("id={}, distance={} \n", item.id, distance);
            scoreItems.push(super::mod_model::DbValueItem {
                id: item.id.clone(),
                content: item.content.clone(),
                distance: distance,
            });
        }
    }
    // score の降順ソート
    scoreItems.sort_by(|a, b| {
        b.distance
            .partial_cmp(&a.distance)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    //println!("scoreItems={:?}", scoreItems);
    let mut outItems = Vec::<super::mod_model::DbValueItem>::new(); 
    let top_k = limit;
    let mut outCount: i32 = 0;
    for row_item in &scoreItems {
        if outCount < *top_k {
            println!("id={}, distance={} \n", row_item.id, row_item.distance);
            outItems.push(super::mod_model::DbValueItem {
                id: row_item.id.clone(),
                content: row_item.content.clone(),
                distance: row_item.distance,
            });            
        }
        outCount += 1;
    }        
    //println!("outItems={:?}", outItems);
    let output = json!({
        "rows": outItems,
    });
    let mut ouStr: String = "".to_string();
    match serde_json::to_string(&output) {
        Ok(json_str) => {
            println!("{}", json_str);
            ouStr = json_str.clone();
        },
        Err(e) => eprintln!("エラー: {}", e),
    }    
    //let json_str: String = serde_json::to_string_pretty(&output).unwrap();
    println!("{}", &ouStr);      
    (StatusCode::CREATED, ouStr.to_string())
}
