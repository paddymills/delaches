from flask import (
    abort,
    Blueprint,
    redirect,
    render_template,
    request,
    url_for
)
from flask_login import login_user

from . import db, login_manager, logger
from .models import User

auth = Blueprint('auth', __name__)

@auth.get('/login')
def login_page():
    return render_template('login.html')

@auth.post('/login')
def authenticate():
    pin = request.headers.get('Authorization')
    if pin:
        logger.info("authentication requested with pin: {}".format(pin))
        user = User.query.filter_by(pin=int(pin)).first()
        if user:
            login_user(user)

            redirect_to = request.args.get('next')
            return redirect(redirect_to or url_for('main.index'))

    return abort(400)

@login_manager.user_loader
def load_user(user_id):
    return User.query.get(int(user_id))