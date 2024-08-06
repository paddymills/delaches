CREATE TABLE Members (
    MemberId INTEGER,
    CardId INTEGER,
    ECard INTEGER,
    MemberTypeId INTEGER,
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
    StatusID INTEGER,
    Birthday TEXT,
    MemberDate TEXT,
    FullAddress TEXT,
    WorkFlag BOOLEAN,
);

CREATE TABLE TransactionTypes (
    Id INT NOT NULL,
    Description TEXT,
    RegularAmt FLOAT,
    JuniorAmt FLOAT,
    LifetimeAmount FLOAT
);

CREATE TABLE Transactions (
    Id INT PRIMARY KEY,
    Timestamp DATETIME,
    TransType INT FOREIGN KEY REFERENCES TransactionTypes(Id),
    MemberId INT FOREIGN KEY REFERENCES Members(MemberId),
    Amount FLOAT
);