
from . import db
from flask_login import UserMixin
from sqlalchemy import ForeignKey

class User(UserMixin, db.Model):
	__tablename__ = 'users'

	pin = db.Column(db.Integer, primary_key=True)
	user = db.Column(db.String(64), unique=True)
	api_key = db.Column(db.String(64))

	def get_id(self):
		return str(self.pin)

	def __repr__(self):
		return '<User {}>'.format(self.user)

class Member(db.Model):
	__tablename__ = 'members'

	id = db.Column(db.Integer, primary_key=True)
	type = db.Column(db.String(8))
	join_year = db.Column(db.Integer)
	active = db.Column(db.Boolean)

	first_name = db.Column(db.String(256))
	last_name = db.Column(db.String(256))
	birthdate = db.Column(db.Date)

	address = db.Column(db.String(256))
	city = db.Column(db.String(256))
	state = db.Column(db.String(2))
	zip = db.Column(db.String(5))

	phone = db.Column(db.String(12))
	email = db.Column(db.String(255))

class Transaction(db.Model):
	__tablename__ = 'transactions'

	id = db.Column(db.Integer, primary_key=True)
	member_id = db.Column(db.Integer, db.ForeignKey("members.id"))
	member = db.relationship("Member", backref=db.backref("members", uselist=False))
	
	timestamp = db.Column(db.DateTime, server_default=db.func.now())
	desc = db.Column(db.String(64))
	amount = db.Column(db.Float)

class Dues(db.Model):
	__tablename__ = 'dues'

	id = db.Column(db.Integer, primary_key=True)
	desc = db.Column(db.String(64), nullable=True)
	amount = db.Column(db.Float, nullable=True)