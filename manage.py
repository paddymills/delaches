
from members import create_app, db
from flask_migrate import upgrade, migrate, init, stamp
from members.models import User

def deploy():
    """Run deployment tasks."""

    app = create_app()
    app.app_context().push()
    db.create_all()

    # migrate database to latest revision
    init()
    stamp()
    migrate()
    upgrade()

# TODO: change pin function

#if __name__ == "__main__":
# TODO: cli
deploy()