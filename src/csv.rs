use std::{fs::File, path::PathBuf};

use crate::api::{Member, Transaction, TransactionType};

pub async fn load_csv_files(files: Vec<PathBuf>) -> Result<(), crate::AppError> {
    for path in files {
        log::info!("Loading file {} into database", path.display());

        let rdr = csv::Reader::from_path(path.clone())?;
        match path.file_name().map(|s| s.to_str().unwrap()) {
            Some("tblTransactions.csv") => load_transactions(rdr).await,
            Some("tblMember.csv") => load_members(rdr).await,
            Some("refTransType.csv") => load_trans_types(rdr).await,
            _ => Err(crate::AppError::CsvParsingError(format!(
                "Unmatched file name to load: {}",
                path.display()
            ))),
        }?;
    }

    Ok(())
}

async fn load_transactions(mut rdr: csv::Reader<File>) -> Result<(), crate::AppError> {
    let headers = rdr.headers()?;
    log::debug!("original headers: {:?}", headers);

    let fixed_headers = headers
        .iter()
        .map(|val| match val {
            "ID" => "Id",
            "MemberID" => "MemberId",
            "TransTypeID" => "TransTypeId",
            "TransDate" => "Timestmap",
            "Amt" => "Amount",
            x => x,
        })
        .collect();
    rdr.set_headers(fixed_headers);

    let client = reqwest::Client::new();
    for result in rdr.into_deserialize::<Transaction>() {
        let record = result?;
        log::debug!("{:?}", record);

        let res = client
            .post("http://localhost:3000/transactions/list")
            .query(&record)
            .send()
            .await?;

        match res.status() {
            reqwest::StatusCode::INTERNAL_SERVER_ERROR => log::debug!("{:?}", res),
            _ => log::trace!("{:?}", res),
        }
    }

    Ok(())
}

async fn load_members(mut rdr: csv::Reader<File>) -> Result<(), crate::AppError> {
    let headers = rdr.headers()?;
    log::debug!("original headers: {:?}", headers);

    let fixed_headers = headers
        .iter()
        .map(|val| match val {
            "MemID" => "MemberId",
            "CardID" => "CardId",
            "MemberTypeID" => "MemberTypeId",
            "Firstname" => "FirstName",
            "Lastname" => "LastName",
            x => x,
        })
        .collect();
    rdr.set_headers(fixed_headers);

    let client = reqwest::Client::new();
    for result in rdr.into_deserialize::<Member>() {
        let record = result?;
        log::debug!("{:?}", record);

        let res = client
            .post("http://localhost:3000/members/list")
            .query(&record)
            .send()
            .await?;

        match res.status() {
            reqwest::StatusCode::INTERNAL_SERVER_ERROR => log::debug!("{:?}", res),
            _ => log::trace!("{:?}", res),
        }
    }

    Ok(())
}

async fn load_trans_types(mut rdr: csv::Reader<File>) -> Result<(), crate::AppError> {
    let headers = rdr.headers()?;
    log::debug!("original headers: {:?}", headers);

    let fixed_headers = headers
        .iter()
        .map(|val| match val {
            "TransTypeID" => "Id",
            "TransType" => "Description",
            "Amt" => "RegularAmt",
            "Jr_Amt" => "JuniorAmt",
            "Life_Amt" => "LIfetimeAmount",
            x => x,
        })
        .collect();
    rdr.set_headers(fixed_headers);

    let client = reqwest::Client::new();
    for result in rdr.into_deserialize::<TransactionType>() {
        let record = result?;
        log::debug!("{:?}", record);

        let res = client
            .post("http://localhost:3000/transactions/types/list")
            .query(&record)
            .send()
            .await?;

        match res.status() {
            reqwest::StatusCode::INTERNAL_SERVER_ERROR => log::debug!("{:?}", res),
            _ => log::trace!("{:?}", res),
        }
    }

    Ok(())
}
