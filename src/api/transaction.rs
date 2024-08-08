use super::member::MemberType;

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
#[serde(rename_all = "PascalCase")]
pub struct Transaction {
    id: u32,
    timestamp: time::PrimitiveDateTime,
    trans_type: String,
    member_id: u32,
    amount: f64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
#[serde(rename_all = "PascalCase")]
pub struct DuesRates {
    id: u32,
    member_type: MemberType,
    start_date: time::Date,
    end_date: time::Date,
    description: String,
    amount: f64,
}
