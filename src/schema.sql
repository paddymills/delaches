CREATE TABLE IF NOT EXISTS Members (
    MemberId INT PRIMARY KEY ON CONFLICT FAIL,
    CardId INT,
    ECard INT,
    MemberTypeId INT,
    FirstName TEXT,
    LastName TEXT,
    Address1 TEXT,
    Address2 TEXT,
    City TEXT,
    State TEXT,
    Zip TEXT,
    Phone1 TEXT,
    Phone2 TEXT,
    Email TEXT,
    StatusID INT,
    Birthday TEXT,
    MemberDate TEXT,
    FullAddress TEXT,
    WorkFlag BOOLEAN
);
CREATE TABLE IF NOT EXISTS TransactionTypes (
    Id INT PRIMARY KEY ON CONFLICT FAIL,
    Description TEXT,
    RegularAmt FLOAT,
    JuniorAmt FLOAT,
    LifetimeAmount FLOAT
);
CREATE TABLE IF NOT EXISTS Transactions (
    Id INT PRIMARY KEY ON CONFLICT FAIL,
    Timestamp DATETIME,
    TransType INT REFERENCES TransactionTypes(Id),
    MemberId INT REFERENCES Members(MemberId),
    Amount FLOAT
);