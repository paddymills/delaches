
from . import db
from flask_login import UserMixin

class User(UserMixin, db.Model):
	__tablename__ = 'users'

	pin = db.Column(db.Integer, primary_key=True)
	user = db.Column(db.String(64), unique=True)

	def get_id(self):
		return str(self.pin)

	def __repr__(self):
		return '<User {}>'.format(self.user)

class Member(db.Model):
	__tablename__ = 'members'

	id = db.Column(db.Integer, primary_key=True)
	first_name = db.Column(db.String(256))
	last_name = db.Column(db.String(256))

class Transaction(db.Model):
	__tablename__ = 'transactions'

	id = db.Column(db.Integer, primary_key=True)
	timestamp = db.Column(db.DateTime, nullable=False)