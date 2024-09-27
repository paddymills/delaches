from flask import Flask
from flask_login import LoginManager
from flask_migrate import Migrate
from flask_sqlalchemy import SQLAlchemy
import logging

# init db, logins and migrations for use in app
db = SQLAlchemy()
login_manager = LoginManager()
migrate = Migrate()
logger = logging.getLogger(__name__)

def create_app():
    # init logging
    logging.basicConfig(filename='server.log', level=logging.INFO)

    app = Flask(__name__)

    app.config['SECRET_KEY'] = 'secret-key-goes-here'
    app.config['SQLALCHEMY_DATABASE_URI'] = 'sqlite:///db.sqlite'
    app.config['SQLALCHEMY_TRACK_MODIFICATIONS'] = True

    db.init_app(app)
    migrate.init_app(app, db)

    login_manager.login_view = 'auth.login_page'
    login_manager.init_app(app)

    # blueprint for auth routes in our app
    from .auth import auth as auth_blueprint
    app.register_blueprint(auth_blueprint)

    # blueprint for non-auth parts of app
    from .main import main as main_blueprint
    app.register_blueprint(main_blueprint)

    return app