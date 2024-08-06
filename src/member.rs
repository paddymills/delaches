use crate::server::AppState;
use axum::{
    extract::{Path, Query, State},
    response::Html,
};
use minijinja::context;
use rusqlite::named_params;
use std::sync::Arc;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Member {
    pub member_id: u32,
    pub card_id: u32,
    pub ecard: u32,
    pub member_id_type: u32,
    pub firstname: String,
    pub lastname: String,
    pub addr1: Option<String>,
    pub addr2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<u32>,
    pub phone1: Option<String>,
    pub phone2: Option<String>,
    pub email: Option<String>,
}

impl Member {
    pub async fn get_members(
        State(state): State<Arc<AppState>>,
    ) -> Result<Html<String>, crate::AppError> {
        let state = state.clone();

        let members = state
            .db
            .lock()
            .await
            .prepare("SELECT * FROM Members")?
            .query_map([], |row| Member::try_from(row))?
            .collect::<Result<Vec<Member>, rusqlite::Error>>()?;

        let template = state.fragments.get_template("members")?;
        let rendered = template.render(context! {
            members
        })?;

        std::thread::sleep(std::time::Duration::from_secs(1));

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
INSERT INTO Members
VALUES
    :member_id,
    :card_id,
    :ecard,
    :member_id_type,
    :firstname,
    :lastname,
    :addr1,
    :addr2,
    :city,
    :state,
    :zip,
    :phone1,
    :phone2,
    :email
"#,
            )?
            .insert(named_params! {
                    ":id": member.member_id,
                    ":card_id": member.card_id,
                    ":ecard": member.ecard,
                    ":member_id_type": member.member_id_type,
                    ":firstname": member.firstname,
                    ":lastname": member.lastname,
                    ":addr1": member.addr1,
                    ":addr2": member.addr2,
                    ":city": member.city,
                    ":state": member.state,
                    ":zip": member.zip,
                    ":phone1": member.phone1,
                    ":phone2": member.phone2,
                    ":email": member.email,
            })?;

        Ok(())
    }

    pub async fn update_member(
        State(state): State<Arc<AppState>>,
        Path(id): Path<u32>,
        Query(member): Query<Member>,
    ) -> Result<(), crate::AppError> {
        log::trace!("Updating member {member:?}");

        let state = state.clone();
        let _ = state
            .db
            .lock()
            .await
            .prepare(
                r#"
UPDATE Members
SET
    CardId=:card_id,
    ECard=:ecard,
    MemberTypeId=:member_id_type,
    FirstName=:firstname,
    LastName=:lastname,
    Address1=:addr1,
    Address2=:addr2,
    City=:city,
    State=:state,
    Zip=:zip,
    Phone1=:phone1,
    Phone2=:phone2,
    Email=:email
WHERE MemberId = :id
"#,
            )?
            .execute(named_params! {
                    ":id": id,
                    ":card_id": member.card_id,
                    ":ecard": member.ecard,
                    ":member_id_type": member.member_id_type,
                    ":firstname": member.firstname,
                    ":lastname": member.lastname,
                    ":addr1": member.addr1,
                    ":addr2": member.addr2,
                    ":city": member.city,
                    ":state": member.state,
                    ":zip": member.zip,
                    ":phone1": member.phone1,
                    ":phone2": member.phone2,
                    ":email": member.email,
            })?;

        Ok(())
    }
}

impl TryFrom<&rusqlite::Row<'_>> for Member {
    type Error = rusqlite::Error;

    fn try_from(row: &rusqlite::Row) -> Result<Self, Self::Error> {
        Ok(Member {
            member_id: row.get::<_, u32>("MemberId")?,
            card_id: row.get::<_, u32>("CardId")?,
            ecard: row.get::<_, u32>("ECard")?,
            member_id_type: row.get::<_, u32>("MemberTypeId")?,
            firstname: row.get::<_, String>("FirstName")?,
            lastname: row.get::<_, String>("LastName")?,
            addr1: row.get::<_, String>("Address1").ok(),
            addr2: row.get::<_, String>("Address2").ok(),
            city: row.get::<_, String>("City").ok(),
            state: row.get::<_, String>("State").ok(),
            zip: row.get::<_, u32>("Zip").ok(),
            phone1: row.get::<_, String>("Phone1").ok(),
            phone2: row.get::<_, String>("Phone2").ok(),
            email: row.get::<_, String>("Email").ok(),
        })
    }
}
