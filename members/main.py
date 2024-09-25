from flask import Blueprint, render_template
from . import db

main = Blueprint('main', __name__)

@main.route('/')
def index():
    return 'Index'

@main.route('/profile')
def profile():
    return 'Profile'

@app.errorhandler(404)
def not_found(error):
    resp = render_template('static/404.html'), 404