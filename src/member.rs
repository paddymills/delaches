use crate::server::AppState;
use axum::{
    extract::{Path, Query, State},
    response::Html,
};
use minijinja::context;
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
    pub fn sqlite_binds<'m>(self) -> [(&'m str, sqlite::Value); 14] {
        [
            (":member_id", (self.member_id as i64).into()),
            (":card_id", (self.card_id as i64).into()),
            (":ecard", (self.ecard as i64).into()),
            (":member_id_type", (self.member_id_type as i64).into()),
            (":firstname", (self.firstname).into()),
            (":lastname", (self.lastname).into()),
            (":addr1", (self.addr1).into()),
            (":addr2", (self.addr2).into()),
            (":city", (self.city).into()),
            (":state", (self.state).into()),
            (":zip", (self.zip.map(|x| x as i64)).into()),
            (":phone1", (self.phone1).into()),
            (":phone2", (self.phone2).into()),
            (":email", (self.email).into()),
        ]
    }

    pub async fn get_members(
        State(state): State<Arc<AppState>>,
    ) -> Result<Html<String>, crate::AppError> {
        let members = state
            .db
            .prepare("SELECT * FROM Members")?
            .iter()
            .map(|row| row.map(|row| Member::from(row)))
            .collect::<Result<Vec<Member>, sqlite::Error>>()?;

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

        let mut query = state
            .db
            .prepare("SELECT * FROM Members WHERE MemberId = ?")?;
        query.bind((1, id as i64))?;

        let member = match query.iter().next() {
            Some(Ok(row)) => Ok(Self::from(row)),
            _ => Err(crate::AppError::NotFound(format!(
                "Member not found in database with id: {id}"
            ))),
        }?;

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

        Ok(())
    }

    pub async fn update_member(
        State(state): State<Arc<AppState>>,
        Path(id): Path<u32>,
        Query(member): Query<Member>,
    ) -> Result<(), crate::AppError> {
        log::trace!("Updating member {member:?}");

        let mut query = state.db.prepare(
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
        )?;
        query.bind(&member.sqlite_binds()[..])?;
        query.bind((":id", id as i64))?;

        Ok(())
    }
}

impl From<sqlite::Row> for Member {
    fn from(row: sqlite::Row) -> Self {
        Member {
            member_id: row.read::<i64, _>("MemberId") as u32,
            card_id: row.read::<i64, _>("CardId") as u32,
            ecard: row.read::<i64, _>("ECard") as u32,
            member_id_type: row.read::<i64, _>("MemberTypeId") as u32,
            firstname: row.read::<&str, _>("FirstName").into(),
            lastname: row.read::<&str, _>("LastName").into(),
            addr1: row.read::<Option<&str>, _>("Address1").map(Into::into),
            addr2: row.read::<Option<&str>, _>("Address2").map(Into::into),
            city: row.read::<Option<&str>, _>("City").map(Into::into),
            state: row.read::<Option<&str>, _>("State").map(Into::into),
            zip: row.read::<Option<i64>, _>("Zip").map(|x| x as u32),
            phone1: row.read::<Option<&str>, _>("Phone1").map(Into::into),
            phone2: row.read::<Option<&str>, _>("Phone2").map(Into::into),
            email: row.read::<Option<&str>, _>("Email").map(Into::into),
        }
    }
}
