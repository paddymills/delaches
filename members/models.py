
import datetime
from . import db
from flask_login import UserMixin

class BaseModel(db.Model):
	__abstract__ = True

	def as_dict(self):
		kv = dict()
		for c in self.__table__.columns:
			val = getattr(self, c.name)
			if type(val) == 'datetime':
				kv[c.name] = val.isoformat()
			else:
				kv[c.name] = val
		return kv

class User(UserMixin, db.Model):
	__tablename__ = 'users'

	pin = db.Column(db.Integer, primary_key=True)
	user = db.Column(db.String(64), unique=True)
	api_key = db.Column(db.String(64))

	def get_id(self):
		return str(self.pin)

	def __repr__(self):
		return '<User {}>'.format(self.user)

class Member(BaseModel):
	__tablename__ = 'members'

	id = db.Column(db.Integer, primary_key=True)
	type = db.Column(db.String(8))
	join_year = db.Column(db.Integer)
	active = db.Column(db.Boolean, default=True)

	first_name = db.Column(db.String(256))
	last_name = db.Column(db.String(256))
	birthdate = db.Column(db.Date)

	address = db.Column(db.String(256))
	city = db.Column(db.String(256))
	state = db.Column(db.String(2))
	zip = db.Column(db.String(5))

	phone = db.Column(db.String(12))
	email = db.Column(db.String(255))

	def transactions(self):
		return Transaction.cur_year().filter_by(member_id=self.id)

	def all(self):
		vals = self.as_dict()
		vals['dues'] = self.dues
		vals['fob'] = Dues.fob_cost()

		return vals

	@property
	def dues(self):
		# should only be one transaction record for either Dues or KeyFob
		if self.transactions().filter_by(desc='Dues').first():
			return 0.00
		return Dues.query.filter_by(type=self.type).first().amount

class Transaction(BaseModel):
	__tablename__ = 'transactions'

	id = db.Column(db.Integer, primary_key=True)
	member_id = db.Column(db.Integer, db.ForeignKey("members.id"))
	member = db.relationship("Member", backref=db.backref("members", uselist=False))
	
	timestamp = db.Column(db.DateTime, server_default=db.func.now())
	desc = db.Column(db.String(64))
	amount = db.Column(db.Float)

	def cur_year():
		thisyear = datetime.datetime.now().year
	
		return Transaction.query.filter(
			Transaction.timestamp.between(datetime.date(thisyear, 1, 1), datetime.date(thisyear, 12, 31))
		)

class Dues(BaseModel):
	__tablename__ = 'dues'

	id = db.Column(db.Integer, primary_key=True)
	type = db.Column(db.String(8), nullable=True)
	amount = db.Column(db.Float, nullable=True)

	def fob_cost():
		return Dues.query.filter_by(type='KeyFob').first().amount