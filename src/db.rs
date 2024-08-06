use crate::member::Member;

pub struct Db {
    conn: sqlite::ConnectionThreadSafe,
}

impl Db {
    pub fn new() -> Result<Self, crate::AppError> {
        Ok(Self {
            conn: sqlite::Connection::open_thread_safe("db.sqlite")?,
        })
    }

    pub fn add_member(&self, member: Member) -> Result<(), crate::AppError> {
        self.conn
            .prepare(
                r#"
INSERT INTO Members
VALUES (:memid, :cardid)
        "#,
            )?
            .bind_iter::<_, (_, sqlite::Value)>([
                (":memid", (member.member_id as i64).into()),
                (":cardid", (member.card_id as i64).into()),
            ])?;

        Ok(())
    }

    pub fn get_members(&self) -> Result<Vec<Member>, crate::AppError> {
        let mut query = self.conn.prepare("SELECT * FROM Members")?;

        let members = query
            .iter()
            .map(|row| row.map(|row| Member::from(row)))
            .collect::<Result<Vec<Member>, sqlite::Error>>()?;

        Ok(members)
    }
}
