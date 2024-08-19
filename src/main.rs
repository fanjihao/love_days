/*
    2024/08/19

*/

use chrono::{Local, NaiveDate};
use errors::CustomError;
use reqwest::Client;
use serde_json::Value;
use utils::{BAIDU_AK, LOVE_DAYS, OFFCIAL_APP_ID, OFFCIAL_APP_SECRET, PUSH_ID, TEMPLATE_ID};

mod errors;
mod utils;

async fn weather_handler() -> Result<Value, CustomError> {
    let client = Client::new();
    let url = format!("https://api.map.baidu.com/weather/v1/?district_id=510107&data_type=all&ak={}", BAIDU_AK);
    let res = client.get(url).send().await?;
    println!("res {:?}", res);
    let response_text = res.text().await?;
    let data: Result<Value, serde_json::Error> = serde_json::from_str(&response_text);

    data.map_err(Into::into)
}

pub async fn fetch_set_access_token() -> Result<String, CustomError> {
    let body = reqwest::get(
        "https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid="
            .to_string()
            + OFFCIAL_APP_ID
            + "&secret="
            + OFFCIAL_APP_SECRET,
    )
    .await?
    .text()
    .await?;
    let response_json: Result<serde_json::Value, serde_json::Error> =
        serde_json::from_str(&body);
    match response_json {
        Ok(obj) => {
            let mut token: &str = "";
            if let Some(val) = obj.get("access_token") {
                if let Some(t) = val.as_str() {
                    token = t;
                }
            }
            Ok(token.to_string())
        }
        Err(_) => Err(CustomError::BadRequest("access_token 获取失败".to_string())),
    }
}

async fn send_template() -> Result<(), CustomError> {
    let token = fetch_set_access_token().await?;
    
println!("{:?}", token);
    let client = Client::new();
    match weather_handler().await {  
        Ok(value) => {
            let forecasts = value.get("result").unwrap().get("forecasts").unwrap();
            let location = value.get("result").unwrap().get("location").unwrap();
            
            let given_date = NaiveDate::parse_from_str(LOVE_DAYS, "%Y-%m-%d")?;  
            let today = Local::now().naive_local().date();
            let days_diff = (today - given_date).num_days();

            let json_data = serde_json::json!({
                "touser": PUSH_ID,
                "template_id": TEMPLATE_ID,
                "url": "http://weixin.qq.com/download",
                "topcolor":"#FF0000",
                "data":{
                    "date":{
                        "value": format!("{}", forecasts.get(0).unwrap().get("date").unwrap()).replace("\"", ""),
                        "color":"#f5f5f5"
                    },
                    "city":{
                        "value": format!("{}", location.get("city").unwrap()).replace("\"", ""),
                        "color":"#173177"
                    },
                    "weather": {
                        "value": format!("{}", forecasts.get(0).unwrap().get("text_day").unwrap()).replace("\"", ""),
                        "color":"#173177"
                    },
                    "low": {
                        "value": format!("{}°", forecasts.get(0).unwrap().get("low").unwrap()),
                        "color":"#173177"
                    },
                    "high": {
                        "value": format!("{}°", forecasts.get(0).unwrap().get("high").unwrap()),
                        "color":"#173177"
                    },
                    "loveDays": {
                        "value": format!("{}天 ^_^", days_diff),
                        "color":"#173177"
                    },
                }
            });
            println!("{:?}", 123);
            let resp = client
                .post(format!(
                    "https://api.weixin.qq.com/cgi-bin/message/template/send?access_token={}",
                    token
                ))
                .json(&json_data)
                .send()
                .await?;
            let response_text = resp.text().await?;
            println!("Response: {}", response_text);
        
            Ok(())
        },  
        Err(e) => {  
            println!("{:?}", e);
            Err(CustomError::BadRequest("error".to_string()))  
        },  
    }  
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let _ = send_template().await;
}
