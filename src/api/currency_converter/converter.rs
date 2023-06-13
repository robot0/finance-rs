use reqwest::Client;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ExchangeRates {
    provider: String,
    base: String,
    date: String,
    time_last_updated: u64,
    rates: HashMap<String, f64>,
}

#[derive(Debug, Deserialize)]
pub struct ConversionRequest {
    from: String,
    to: String,
    amount: f64,
}

pub async fn get_exchange_rates() -> Result<ExchangeRates, reqwest::Error> {
    let client = Client::new();
    let response = client
        .get("https://api.exchangerate-api.com/v4/latest/eur")
        .send()
        .await?;

    let rates: ExchangeRates = response.json().await?;

    // Debug print the received rates
    println!("Received exchange rates: {:?}", rates);

    Ok(rates)
}

pub async fn convert_currency(data: web::Json<ConversionRequest>) -> HttpResponse {
    match get_exchange_rates().await {
        Ok(rates) => {
            match rates.rates.get(&data.from) {
                Some(&from_rate) => {
                    match rates.rates.get(&data.to) {
                        Some(&to_rate) => {
                            let converted_amount = data.amount * (to_rate / from_rate);
                            println!("Converted amount: {}", converted_amount);
                            return HttpResponse::Ok().json(json!({
                                "converted_amount": converted_amount
                            }));
                        },
                        None => {
                            println!("'to' currency not found in exchange rates");
                            return HttpResponse::BadRequest().body("'to' currency not found");
                        }
                    }
                },
                None => {
                    println!("'from' currency not found in exchange rates");
                    return HttpResponse::BadRequest().body("'from' currency not found");
                }
            }
        },
        Err(e) => {
            println!("Error fetching exchange rates: {:?}", e);
            return HttpResponse::BadRequest().body("Failed to fetch exchange rates");
        }
    }
}

