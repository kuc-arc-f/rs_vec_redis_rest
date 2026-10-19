use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post},
    Router,
};
use redis::{Commands, Connection, RedisResult};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::{Arc, Mutex};
use std::sync::{LazyLock};
use std::fs;
use std::io;
use std::thread;
use std::time::Duration;
use uuid::Uuid;
use dotenvy::dotenv;

mod mod_model;
mod mod_handle;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdataReq {
    pub table: String,
    pub sql: String,
}

// アプリケーションの状態
type AppState = Arc<Mutex<Connection>>;

#[tokio::main]
async fn main() -> anyhow::Result<()>
{
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut conn = client.get_connection()?;
    
    let state = Arc::new(Mutex::new(conn));

    // ルーティングの設定
    let app = Router::new()
        .route("/api/insert", post(update))
        .route("/api/select", post(mod_handle::select_handle))
        .with_state(state);

    // サーバーの起動
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8888")
        .await
        .expect("Failed to bind");
    
    println!("Server running on http://localhost:8888");
    
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");

    Ok(())
}

async fn update(
    State(state): State<AppState>, 
    Json(payload): Json<mod_model::InsertItem>
) -> (StatusCode, String) {
    let mut conn = state.lock().unwrap();

    println!("prefix={}", &payload.prefix);
    println!("content={}", &payload.content);
    let prefix_str = format!("{}*", &payload.prefix);
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

        // KEY を配列に追加
        keys.append(&mut result_keys);

        // 次のカーソルを設定
        cursor = next_cursor;

        // カーソルが 0 なら終了
        if cursor == 0 {
            break;
        }
    }

    println!("keys.len={}", keys.len());
    let mut vec1: Vec<mod_model::SelectDbItem> = Vec::new();

    for key in &keys {
        //println!("KEY: {}", key);
        let key2 = key.replace(&payload.prefix, "");
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
        
        let row = mod_model::SelectDbItem{
            id: key2,
            content: content_str,
            vector: vec3,
        };
        vec1.push(row);

    }  
    println!("vector.len={}", &payload.vector.len());    
    let res1: &String = &payload.vector.iter() .map(|v| v.to_string()) .collect::<Vec<String>>() .join(", ");
    //println!("{}", res1);    
    let str2 = format!("[{}]", res1 );
    //println!("{}", str2);
    let uuid_value = Uuid::new_v4();
    let vecStr = format!("{}{}{}", "vec".to_string(), &payload.prefix, uuid_value);
    println!("vecStr={}", vecStr);
    let keyStr = format!("{}{}", &payload.prefix, uuid_value);
    println!("keyStr={}", keyStr);
    println!("vec1.len={}", vec1.len());  
    if vec1.len() > 0 {
        let target_row = vec1[0].vector.clone();
        println!("target_row.len={}", target_row.len());
        let input_len = &payload.vector.len();
        println!("input_len={}", input_len);
        if *input_len != target_row.len() {
            println!("error, valid_vector_len = NG");
            return (StatusCode::BAD_REQUEST, "NG".to_string());
        }
    }

    //vec
    let _: () = conn.set(&vecStr, &str2).unwrap();
    //text
    let res3: () = conn.set(&keyStr, &payload.content).unwrap();

    (StatusCode::CREATED, "OK".to_string())
}

