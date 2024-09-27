from flask import Blueprint, render_template, send_from_directory
from . import db

main = Blueprint('main', __name__)

@main.route('/')
def index():
    return render_template('index.html')

@main.route('/static/<path:name>')
def static(name):
    return send_from_directory('static', name)

@main.errorhandler(404)
def not_found(error):
    return render_template('404.html'), 404