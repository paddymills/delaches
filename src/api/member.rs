use crate::server::AppState;
use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use std::sync::Arc;

#[derive(Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, sqlx::Type)]
#[serde(rename_all = "PascalCase")]
#[repr(u32)]
pub enum MemberType {
    #[default]
    Adult = 1,
    Junior = 2,
    Lifetime = 3,
}

#[derive(Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, sqlx::Type)]
#[serde(rename_all = "PascalCase")]
#[repr(u8)]
pub enum MemberStatus {
    #[default]
    Active = 1,
    Inactive = 2,
    Deceased = 3,
    Deleted = 4,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct Member {
    pub member_id: u32,
    pub card_id: u32,
    pub e_card: Option<u32>,
    pub member_type: MemberType,
    pub member_status: MemberStatus,
    pub work_flag: bool,
    pub first_name: String,
    pub last_name: String,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<u32>,
    pub phone1: Option<String>,
    pub phone2: Option<String>,
    pub email: Option<String>,
    pub birthday: Option<time::Date>,
    pub member_date: Option<time::Date>,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MemberDues {
    pub member_id: u32,
    pub card_id: u32,
    pub first_name: String,
    pub last_name: String,
    pub dues: f64,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct QueryParams {
    page: Option<u32>,
    search: Option<String>,
    active: Option<bool>,
    id: Option<u32>,
}

impl Member {
    pub fn routes() -> Router<Arc<AppState>> {
        Router::new()
            .route("/", get(Self::get_members).post(Self::add_member))
            .route("/dues", get(Self::get_dues).post(Self::pay_dues))
            .route("/:id", get(Self::get_member).delete(Self::delete_member))
    }

    async fn get_members(
        State(state): State<Arc<AppState>>,
        Query(params): Query<QueryParams>,
    ) -> Result<Json<Vec<Self>>, crate::AppError> {
        log::info!("Get members listing with Params: {:?}", params);

        let state = state.clone();
        let pool = &state.db;
        let mut results: Vec<Self> = sqlx::query_as("SELECT * FROM Members")
            .fetch_all(pool)
            .await?;

        for m in results.iter() {
            log::trace!("{:?}", m)
        }

        if let Some(true) = params.active {
            results = results
                .into_iter()
                .filter(|m| m.member_status == MemberStatus::Active)
                .collect()
        }

        Ok(Json(results))
    }

    async fn get_dues(
        State(state): State<Arc<AppState>>,
    ) -> Result<Json<Vec<MemberDues>>, crate::AppError> {
        log::info!("Get member dues listing");

        let state = state.clone();
        let pool = &state.db;
        let results = sqlx::query_as(
            r#"
SELECT
    MemberId,
    CardId,
    FirstName,
    LastName,
	CASE WHEN MemberId NOT IN (SELECT MemberId FROM Transactions WHERE Timestamp > DATE('now','start of year'))
		THEN Amount
		ELSE 0.0
	END AS Dues
FROM Members
LEFT OUTER JOIN
    (SELECT * FROM DuesRates WHERE EndDate IS NULL) AS Dues
    ON Dues.MemberType=Members.MemberType
WHERE MemberStatus = (SELECT Id FROM MemberStatus WHERE Description = 'Active')
"#,
        )
        .fetch_all(pool)
        .await?;

        for m in results.iter() {
            log::trace!("{:?}", m)
        }

        Ok(Json(results))
    }

    async fn pay_dues(
        State(state): State<Arc<AppState>>,
        Query(params): Query<QueryParams>,
    ) -> Result<Json<bool>, crate::AppError> {
        match params.id {
            Some(id) => {
                let state = state.clone();
                let pool = &state.db;
                let updates = sqlx::query(
                    r#"
INSERT INTO Transactions(TransType, MemberId, Amount)
SELECT Id, $1, Amount FROM DuesRates
WHERE EndDate IS NULL AND MemberType = (SELECT MemberType FROM Members WHERE MemberId=$1)
"#,
                )
                .bind(id)
                .execute(pool)
                .await?
                .rows_affected();

                Ok(Json(updates == 1))
            }
            None => Err(crate::AppError::NotFound(String::from(
                "No id given to pay dues for",
            ))),
        }
    }

    async fn get_member(
        State(state): State<Arc<AppState>>,
        Path(id): Path<u32>,
    ) -> Result<Json<Self>, crate::AppError> {
        log::info!("Get Member with MemberId: {}", id);

        let state = state.clone();
        let pool = &state.db;
        let member = sqlx::query_as("SELECT * FROM Members")
            .fetch_one(pool)
            .await?;

        Ok(Json(member))
    }

    async fn add_member(
        State(state): State<Arc<AppState>>,
        Query(member): Query<Member>,
    ) -> Result<Json<bool>, crate::AppError> {
        log::info!("Member: {:?}", member);

        let pool = &state.clone().db;
        let updates = sqlx::query(r#"
INSERT INTO Members(MemberId,CardId,ECard,MemberType,StatusId,WorkFlag,FirstName,LastName,Address1,Address2,City,State,Zip,Phone1,Phone2,Email,Birthday,MemberDate)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
"#)
            .bind(member.member_id)
            .bind(member.card_id)
            .bind(member.e_card)
            .bind(member.member_type)
            .bind(member.member_status)
            .bind(member.work_flag)
            .bind(member.first_name)
            .bind(member.last_name)
            .bind(member.address1)
            .bind(member.address2)
            .bind(member.city)
            .bind(member.state)
            .bind(member.zip)
            .bind(member.phone1)
            .bind(member.phone2)
            .bind(member.email)
            .bind(member.birthday)
            .bind(member.member_date)
            .execute(pool)
            .await?
            .rows_affected();

        Ok(Json(updates == 1))
    }

    async fn delete_member(
        State(state): State<Arc<AppState>>,
        Path(id): Path<u32>,
    ) -> Result<Json<bool>, crate::AppError> {
        log::info!("Delete member with MemberId: {}", id);

        let pool = &state.clone().db;
        let updates = sqlx::query(r#"UPDATE Members SET MemberId=$2 WHERE MemberId=$1"#)
            .bind(id)
            .bind(MemberStatus::Deleted)
            .execute(pool)
            .await?
            .rows_affected();

        Ok(Json(updates > 0))
    }
}
