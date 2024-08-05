#[derive(Debug, Default, serde::Serialize)]
pub struct Member {
    member_id: u32,
    card_id: u32,
    ecard: u32,
    member_id_type: u32,
    firstname: String,
    lastname: String,
    addr1: Option<String>,
    addr2: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip: Option<u32>,
    phone1: Option<String>,
    phone2: Option<String>,
    email: Option<String>,
}

impl Member {
    pub fn load_data() -> Vec<Member> {
        vec![
            Member {
                member_id: 1,
                card_id: 1,
                ecard: 753928,
                member_id_type: 0,
                lastname: String::from("Bender"),
                firstname: String::from("Clint"),
                ..Default::default()
            },
            Member {
                member_id: 2,
                card_id: 2,
                ecard: 1183802,
                member_id_type: 0,
                lastname: String::from("Bradley"),
                firstname: String::from("Glenn"),
                addr1: Some(String::from("3344 TURNPIKE RD")),
                city: Some(String::from("E-TOWN")),
                state: Some(String::from("PA")),
                zip: Some(17022),
                phone1: Some(String::from("228-8172c")),
                ..Default::default()
            },
            Member {
                member_id: 3,
                card_id: 3,
                ecard: 42707,
                member_id_type: 0,
                lastname: String::from("Muldoon"),
                firstname: String::from("Kevin"),
                addr1: Some(String::from("51 WESTMINSTER DR")),
                city: Some(String::from("E-TOWN")),
                state: Some(String::from("PA")),
                zip: Some(17022),
                phone1: Some(String::from("717-575-0367")),
                ..Default::default()
            },
            Member {
                member_id: 4,
                card_id: 4,
                ecard: 1184801,
                member_id_type: 0,
                lastname: String::from("Henry"),
                firstname: String::from("Jeffrey"),
                addr1: Some(String::from("150 Forest Circle")),
                city: Some(String::from("Palmyra")),
                state: Some(String::from("PA")),
                zip: Some(17078),
                phone1: Some(String::from("717-838-4412")),
                email: Some(String::from("Jhenry6999@aol.com")),
                ..Default::default()
            },
            Member {
                member_id: 6,
                card_id: 6,
                ecard: 1183831,
                member_id_type: 0,
                lastname: String::from("Bowman"),
                firstname: String::from("Todd"),
                addr1: Some(String::from("1331 MT WILSON RD")),
                city: Some(String::from("LEBANON")),
                state: Some(String::from("PA")),
                zip: Some(17042),
                ..Default::default()
            },
            Member {
                member_id: 7,
                card_id: 7,
                ecard: 1184160,
                member_id_type: 0,
                lastname: String::from("Dove"),
                firstname: String::from("Matthew"),
                addr1: Some(String::from("48 Norfolk Lane")),
                city: Some(String::from("LEBANON")),
                state: Some(String::from("PA")),
                zip: Some(17042),
                phone1: Some(String::from("717-273-5514")),
                ..Default::default()
            },
            Member {
                member_id: 8,
                card_id: 8,
                ecard: 1184367,
                member_id_type: 0,
                lastname: String::from("Kinsey III"),
                firstname: String::from("Glenn R"),
                addr1: Some(String::from("2041 QUENTIN RD")),
                city: Some(String::from("LEBANON")),
                state: Some(String::from("PA")),
                zip: Some(17042),
                phone1: Some(String::from("717-228-1789")),
                ..Default::default()
            },
            Member {
                member_id: 9,
                card_id: 9,
                ecard: 1411366,
                member_id_type: 0,
                lastname: String::from("Altman"),
                firstname: String::from("Lawrence"),
                ..Default::default()
            },
            Member {
                member_id: 10,
                card_id: 10,
                ecard: 1184802,
                member_id_type: 0,
                lastname: String::from("Giovengo"),
                firstname: String::from("Kathy"),
                addr1: Some(String::from("2216 Lebanon Rd")),
                city: Some(String::from("Manheim")),
                state: Some(String::from("PA")),
                zip: Some(17545),
                phone1: Some(String::from("717-665-3358")),
                email: Some(String::from("slimbo02@yahoo.com")),
                ..Default::default()
            },
        ]
    }
}
