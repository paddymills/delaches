
from argparse import ArgumentParser
import polars

from flask_migrate import upgrade, migrate, init, stamp
from members import create_app, db
from members.models import *

def deploy():
    """Run deployment tasks."""

    app = create_app()
    app.app_context().push()
    db.create_all()
    
    # TODO: cli to get pins
    db.session.add_all([
        User(pin=1234, user='bar'),
        User(pin=5678, user='admin', api_key='<api-key>'),
    ])
    db.session.commit()

    # migrate database to latest revision
    init()
    stamp()
    migrate()
    upgrade()

def import_data(filename):
    """Import data into the database."""

    columns = {
        "Mem#": "id",
        "Type": "type",
        "Last Name": "last_name",
        "First Name": "first_name",
        "Address": "address",
        "City": "city",
        "ST": "state",
        "Zip ": "zip",
        "Phone": "phone",
        "e-mail": "email",
        "Birthdate": "birthdate",
        "Year Joined": "join_year",
    }
    data = polars.read_excel(source=filename, columns=list(columns.keys()))
    data = data.rename(columns)
    data = data.filter(data['first_name'].is_not_null())
    # change birthdate to datetime
    data = data.with_columns(polars.col('birthdate').str.to_date("%Y-%m-%d %H:%M:%S", strict=False))
    print(data)

    # dump data to the database
    app = create_app()
    app.app_context().push()
    db.session.bulk_insert_mappings(Member, data.to_dicts())
    db.session.commit()

# ------------------------ Main entrypoint ------------------------
parser = ArgumentParser()
parser.add_argument('--deploy', action='store_true', help='Deploy the application database')
parser.add_argument('--load', help='Import data into the application database')

args = parser.parse_args()
if args.deploy:
    deploy()
elif args.load:
    import_data(args.load)
else:
    parser.print_help()