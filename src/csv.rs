use std::{fs::File, path::PathBuf};

const TRANS_TYPE_HEADER: [&str; 8] = [
    "TransTypeID",
    "TransType",
    "Dues",
    "Amt",
    "Jr_Amt",
    "Life_Amt",
    "Active",
    "Default",
];
const TRANSACTIONS_HEADER: [&str; 8] = [
    "ID",
    "TransTypeID",
    "MemberID",
    "LastName",
    "FirstName",
    "TransDate",
    "TransTime",
    "Amt",
];
const MEMBERS_HEADER: [&str; 19] = [
    "MemID",
    "CardID",
    "ECard",
    "MemberTypeID",
    "Lastname",
    "Firstname",
    "Address1",
    "Address2",
    "City",
    "State",
    "Zip",
    "Phone1",
    "Phone2",
    "Email",
    "StatusID",
    "Birthday",
    "MemberDate",
    "FullAddress",
    "WorkFlag",
];

pub fn load_csv_files(files: Vec<PathBuf>) -> Result<(), crate::AppError> {
    for path in files {
        log::info!("Loading file {} into database", path.display());

        let rdr = csv::Reader::from_path(path.clone())?;
        match path.file_name().map(|s| s.to_str().unwrap()) {
            Some("tblTransactions.csv") => load_transactions(rdr),
            Some("tblMember.csv") => load_members(rdr),
            Some("refTransType.csv") => load_trans_types(rdr),
            _ => Err(crate::AppError::CsvParsingError(format!(
                "Unmatched file name to load: {}",
                path.display()
            ))),
        }?;
    }

    Ok(())
}

fn load_transactions(mut rdr: csv::Reader<File>) -> Result<(), crate::AppError> {
    if rdr.headers()? == Vec::from(TRANSACTIONS_HEADER) {
        return Err(crate::AppError::CsvParsingError(format!(
            "Transactions csv header does not match expected ({:?})",
            TRANSACTIONS_HEADER
        )));
    }

    for result in rdr.records().take(20) {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        log::debug!("{:?}", record);
    }

    Ok(())
}

fn load_members(mut rdr: csv::Reader<File>) -> Result<(), crate::AppError> {
    if rdr.headers()? == Vec::from(MEMBERS_HEADER) {
        return Err(crate::AppError::CsvParsingError(format!(
            "Members csv header does not match expected ({:?})",
            MEMBERS_HEADER
        )));
    }

    for result in rdr.records().take(20) {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        log::debug!("{:?}", record);
    }

    Ok(())
}

fn load_trans_types(mut rdr: csv::Reader<File>) -> Result<(), crate::AppError> {
    if rdr.headers()? == Vec::from(TRANS_TYPE_HEADER) {
        return Err(crate::AppError::CsvParsingError(format!(
            "Transtype csv header does not match expected ({:?})",
            TRANS_TYPE_HEADER
        )));
    }

    for result in rdr.records().take(20) {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        log::debug!("{:?}", record);
    }

    Ok(())
}
