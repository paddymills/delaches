
DROP TABLE IF EXISTS Transactions;
DROP TABLE IF EXISTS Members;
DROP TABLE IF EXISTS DuesRates;
DROP TABLE IF EXISTS MemberType;
DROP TABLE IF EXISTS MemberStatus;

CREATE TABLE MemberType (
    Id INTEGER PRIMARY KEY ON CONFLICT FAIL AUTOINCREMENT,
	Description TEXT
);
CREATE TABLE MemberStatus (
    Id INTEGER PRIMARY KEY ON CONFLICT FAIL AUTOINCREMENT,
	Description TEXT
);
CREATE TABLE DuesRates (
    Id INTEGER PRIMARY KEY ON CONFLICT FAIL AUTOINCREMENT,
	MemberType INTEGER REFERENCES MemberType(Id),
    StartDate DATE NOT NULL,
    EndDate DATE,
    Description TEXT,
    Amount FLOAT
);

CREATE TABLE Members (
    MemberId INTEGER PRIMARY KEY ON CONFLICT FAIL AUTOINCREMENT,
    CardId INTEGER,
    ECard INTEGER,
    MemberType INTEGER REFERENCES MemberType(Id),
    StatusId INTEGER REFERENCES MemberStatus(Id),
    WorkFlag BOOLEAN,
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
    Birthday TEXT,
    MemberDate DATETIME
);
CREATE TABLE Transactions (
    Id INTEGER PRIMARY KEY ON CONFLICT FAIL AUTOINCREMENT,
    Timestamp DATETIME,
    TransType INTEGER REFERENCES DuesRates(Id),
    MemberId INTEGER REFERENCES Members(MemberId),
    Amount FLOAT
);

INSERT INTO MemberType(Description) VALUES ('Adult'),('Junior'),('Lifetime');
INSERT INTO MemberStatus(Description) VALUES ('Active'),('Inactive'),('Deceased'),('Deleted');
INSERT INTO DuesRates(MemberType, StartDate, Description, Amount) VALUES(3, '1900-01-01', 'No Lifetime Dues', 0.0);

COMMIT;