use crate::server::AppState;
use axum::{
    extract::{Path, Query, State},
    response::Html,
    routing::get,
    Router,
};
use minijinja::context;
use rusqlite::named_params;
use std::sync::Arc;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Member {
    pub member_id: u32,
    pub card_id: u32,
    pub e_card: Option<u32>,
    pub member_type_id: u32,
    pub first_name: String,
    pub last_name: String,
    pub addr1: Option<String>,
    pub addr2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<u32>,
    pub phone1: Option<String>,
    pub phone2: Option<String>,
    pub email: Option<String>,
    pub status_id: u32,
    pub birthday: Option<String>,
    pub member_date: Option<String>,
    pub work_flag: bool,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct QueryParams {
    page: Option<u32>,
    search: Option<String>,
}

impl Member {
    pub fn routes() -> Router<Arc<AppState>> {
        Router::new()
            .route_service(
                "/",
                tower_http::services::ServeFile::new("public/members.html"),
            )
            .route("/list", get(Self::get_members).post(Self::add_member))
            .route("/:id", get(Self::get_member).delete(Self::delete_member))
    }

    pub async fn get_members(
        State(state): State<Arc<AppState>>,
        Query(params): Query<QueryParams>,
    ) -> Result<Html<String>, crate::AppError> {
        let state = state.clone();

        log::info!("Params {:?}", params);

        let members = state
            .db
            .lock()
            .await
            .prepare(
                r#"
SELECT *
FROM Members
WHERE FirstName LIKE :search
    OR LastName LIKE :search
    OR Address1 LIKE :search
    OR Address2 LIKE :search
    OR Email LIKE :search
    OR Phone1 LIKE :search
    OR Phone2 LIKE :search
LIMIT 30 OFFSET (:page - 1)*30
"#,
            )?
            .query_map(
                named_params! { ":page": params.page.unwrap_or(1), ":search": format!("%{}%", params.search.unwrap_or_default()) },
                |row| Member::try_from(row),
            )?
            .collect::<Result<Vec<Member>, rusqlite::Error>>()?;

        let template = state.fragments.get_template("members")?;
        let rendered = template.render(context! {
            members
        })?;

        // std::thread::sleep(std::time::Duration::from_secs(1));

        Ok(Html(rendered))
    }

    pub async fn get_member(
        State(state): State<Arc<AppState>>,
        Path(id): Path<u32>,
    ) -> Result<Html<String>, crate::AppError> {
        log::trace!("Getting member with id: {id}");

        let state = state.clone();
        let member = state
            .db
            .lock()
            .await
            .prepare("SELECT * FROM Members WHERE MemberId = :id")?
            .query_row(named_params! { ":id": id }, |row| Member::try_from(row))?;

        Ok(Html(state.fragments.get_template("member")?.render(
            context! {
                member
            },
        )?))
    }

    pub async fn add_member(
        State(state): State<Arc<AppState>>,
        Query(member): Query<Member>,
    ) -> Result<(), crate::AppError> {
        log::trace!("Adding member: {member:?}");

        let state = state.clone();
        state
            .db
            .lock()
            .await
            .prepare(
                r#"
INSERT OR REPLACE INTO Members(
    MemberId,
    CardId,
    ECard,
    MemberTypeId,
    FirstName,
    LastName,
    Address1,
    Address2,
    City,
    State,
    Zip,
    Phone1,
    Phone2,
    Email,
    StatusId,
    Birthday,
    WorkFlag
)
VALUES (
    :member_id,
    :card_id,
    :ecard,
    :member_type_id,
    :first_name,
    :last_name,
    :addr1,
    :addr2,
    :city,
    :state,
    :zip,
    :phone1,
    :phone2,
    :email,
    1,
    :birthday,
    0
)"#,
            )?
            .insert(named_params! {
                ":member_id": member.member_id,
                ":card_id": member.card_id,
                ":ecard": member.e_card,
                ":member_type_id": member.member_type_id,
                ":first_name": member.first_name,
                ":last_name": member.last_name,
                ":addr1": member.addr1,
                ":addr2": member.addr2,
                ":city": member.city,
                ":state": member.state,
                ":zip": member.zip,
                ":phone1": member.phone1,
                ":phone2": member.phone2,
                ":email": member.email,
                ":birthday": member.birthday
            })?;
        Ok(())
    }

    pub async fn delete_member(
        State(state): State<Arc<AppState>>,
        Path(id): Path<u32>,
    ) -> Result<(), crate::AppError> {
        log::trace!("Setting deletion of member: {id}");

        let state = state.clone();
        state
            .db
            .lock()
            .await
            .execute("UPDATE Members SET StatusId=4 WHERE MemberId=?1", [id])?;

        Ok(())
    }
}

impl TryFrom<&rusqlite::Row<'_>> for Member {
    type Error = rusqlite::Error;

    fn try_from(row: &rusqlite::Row) -> Result<Self, Self::Error> {
        Ok(Member {
            member_id: row.get::<_, u32>("MemberId")?,
            card_id: row.get::<_, u32>("CardId")?,
            e_card: row.get::<_, u32>("ECard").ok(),
            member_type_id: row.get::<_, u32>("MemberTypeId")?,
            first_name: row.get::<_, String>("FirstName")?,
            last_name: row.get::<_, String>("LastName")?,
            addr1: row.get::<_, String>("Address1").ok(),
            addr2: row.get::<_, String>("Address2").ok(),
            city: row.get::<_, String>("City").ok(),
            state: row.get::<_, String>("State").ok(),
            zip: row.get::<_, u32>("Zip").ok(),
            phone1: row.get::<_, String>("Phone1").ok(),
            phone2: row.get::<_, String>("Phone2").ok(),
            email: row.get::<_, String>("Email").ok(),
            status_id: row.get::<_, u32>("StatusId")?,
            birthday: row.get::<_, String>("Birthday").ok(),
            member_date: row.get::<_, String>("MemberDate").ok(),
            work_flag: row.get::<_, bool>("WorkFlag")?,
        })
    }
}
