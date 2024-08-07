#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Transaction {
    id: u32,
    timestamp: chrono::NaiveDateTime,
    trans_type: u32,
    member_id: u32,
    amount: f64,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TransactionType {
    id: u32,
    description: String,
    regular_amt: f64,
    junior_amt: f64,
    lifetime_amount: f64,
}
