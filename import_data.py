import argparse
import csv
import sqlite3
import re

IS_CUR = re.compile(r"^\$\d+\.\d+$")
IS_INT = re.compile(r"^\d+$")
IS_DATE = re.compile(r"^(\d{1,2})/(\d{1,2})/(\d{4})$")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-m", "--members", action="store_true", help="Load members file"
    )
    parser.add_argument(
        "-t", "--transactions", action="store_true", help="Load transactions file"
    )
    parser.add_argument(
        "-y", "--transaction-types", action="store_true", help="Load transactions file"
    )
    parser.add_argument("-a", "--all", action="store_true", help="Load all files")

    args = parser.parse_args()

    if args.members or args.all:
        load_members()
    if args.transaction_types or args.all:
        load_transaction_types()
    if args.transactions or args.all:
        load_transactions()


def fmt_csv_vals(row):
    for k, v in row.items():
        v = v.strip()
        if v == "":
            row[k] = None
        elif IS_CUR.match(v):
            row[k] = float(v[1:])
        elif IS_INT.match(v):
            row[k] = int(v)
        elif IS_DATE.match(v):
            month, day, year = [int(x) for x in IS_DATE.match(v).groups()]
            row[k] = f"{year}-{month:02}-{day:02}"

    return row


def load_members():
    member_type_map = {0: 1, 2: 2, 1: 3, None: 1}

    conn = sqlite3.connect("db.sqlite")
    cur = conn.cursor()
    with open("data/tblMember.csv") as members:
        reader = csv.DictReader(members, skipinitialspace=True)
        for row in reader:
            row = fmt_csv_vals(row)
            try:
                addr1, city, state_zip = row["FullAddress"].split(", ")
                state, _zip = state_zip.split(" ")
                row["Address1"] = addr1
                row["City"] = city
                row["State"] = state
                row["Zip"] = _zip
            except:
                pass
            if row["StatusID"] == 0:
                row["StatusID"] = 1

            print(row)
            cur.execute(
                """
INSERT INTO Members(MemberId,CardId,ECard,MemberType,MemberStatus,WorkFlag,FirstName,LastName,Address1,Address2,City,State,Zip,Phone1,Phone2,Email,Birthday,MemberDate)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
""",
                (
                    row["MemID"],
                    row["CardID"],
                    row["ECard"],
                    member_type_map[row["MemberTypeID"]],
                    row["StatusID"],
                    row["WorkFlag"],
                    row["Firstname"],
                    row["Lastname"],
                    row["Address1"],
                    row["Address2"],
                    row["City"],
                    row["State"],
                    row["Zip"],
                    row["Phone1"],
                    row["Phone2"],
                    row["Email"],
                    row["Birthday"],
                    row["MemberDate"],
                ),
            )

    conn.commit()


def load_transactions():
    id_map = dict()
    with open("data/refTransType.csv") as members:
        reader = csv.DictReader(members)
        for row in reader:
            row = fmt_csv_vals(row)
            id_map[row["TransTypeID"]] = "{}-01-01".format(
                row["TransType"].replace("Dues ", "")
            )

    print(id_map)

    conn = sqlite3.connect("db.sqlite")
    cur = conn.cursor()
    with open("data/tblTransactions.csv") as members:
        reader = csv.DictReader(members)
        for row in reader:
            row = fmt_csv_vals(row)
            print(row)
            cur.execute(
                """
INSERT INTO Transactions(Id,Timestamp,TransType,MemberId,Amount)
VALUES (?, ?, (
    SELECT Id FROM DuesRates
    WHERE StartDate < ?
    AND (EndDate IS NULL OR EndDate > ?)
    AND Amount = ?
), ?, ?)
""",
                (
                    row["ID"],
                    row["TransDate"],
                    id_map[row["TransTypeID"]],
                    id_map[row["TransTypeID"]],
                    row["Amt"],
                    row["MemberID"],
                    row["Amt"],
                ),
            )

    conn.commit()


def load_transaction_types():
    ADULT = 1
    JUNIOR = 2

    dues = {ADULT: [], JUNIOR: []}
    with open("data/refTransType.csv") as members:
        reader = csv.DictReader(members)
        for row in reader:
            row = fmt_csv_vals(row)
            print(row)
            date = "{}-01-01".format(row["TransType"].replace("Dues ", ""))
            if dues[ADULT] == []:
                # init
                dues[ADULT].append([row["TransType"], date, None, row["Amt"]])
                dues[JUNIOR].append([row["TransType"], date, None, row["Jr_Amt"]])
                continue

            if dues[ADULT][-1][3] != row["Amt"]:
                dues[ADULT][-1][2] = date
                dues[ADULT].append([row["TransType"], date, None, row["Amt"]])
            if dues[JUNIOR][-1][-1] != row["Jr_Amt"]:
                dues[JUNIOR][-1][2] = date
                dues[JUNIOR].append([row["TransType"], date, None, row["Jr_Amt"]])

    conn = sqlite3.connect("db.sqlite")
    cur = conn.cursor()

    rows = []
    for mem in dues:
        for due in dues[mem]:
            rows.append((mem, *due))

    for row in sorted(rows, key=lambda r: (r[2], r[0])):
        print(row)
        cur.execute(
            """
INSERT INTO DuesRates(MemberType,Description,StartDate,EndDate,Amount)
VALUES (?, ?, ?, ?, ?)
""",
            row,
        )

    conn.commit()


if __name__ == "__main__":
    main()
