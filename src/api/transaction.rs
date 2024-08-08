use super::member::MemberType;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Transaction {
    id: u32,
    timestamp: chrono::NaiveDateTime,
    trans_type: String,
    member_id: u32,
    amount: f64,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DuesRates {
    id: u32,
    member_type: MemberType,
    start_date: chrono::NaiveDate,
    end_date: chrono::NaiveDate,
    description: String,
    amount: f64,
}
