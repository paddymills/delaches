
from members import create_app, db
from flask_migrate import upgrade, migrate, init, stamp
from members.models import *

def deploy():
    """Run deployment tasks."""

    app = create_app()
    app.app_context().push()
    db.create_all()
    
    # TODO: cli to get pins
    db.session.add_all([
        User(pin=1234, user='bar'),
        User(pin=5678, user='admin'),
    ])
    db.session.commit()

    # migrate database to latest revision
    init()
    stamp()
    migrate()
    upgrade()

deploy()