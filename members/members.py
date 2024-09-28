from flask import Blueprint, request, abort, render_template, make_response
from flask_login import login_required
from . import db, logger
from .models import Member, Transaction

members = Blueprint('members', __name__)

@members.route('/members')
@login_required
def member_mgmt():
	return render_template('members.html')

@members.route('/members/all')
@login_required
def listing():
	return [m.all() for m in Member.query.all()]

@members.get('/members/<id>')
@login_required
def member(id):
	logger.debug("requested member: {}".format(id))
	return Member.query.get(id).as_dict()

@members.put('/members/<id>')
@login_required
def add_member(id):
	logger.debug("adding member: {}".format(id))
	print(request.json)
	
	return abort(501)

@members.post('/members/<id>')
@login_required
def update_member(id):
	logger.debug("updating member: {}".format(id))

	match request.json['paymentType']:
		case 'dues':
			logger.debug("dues payment")

			transact = Transaction(member_id=id, desc='Dues', amount=request.json['amount'])
			db.session.add(transact)
			db.session.commit()

			return make_response({ "transactionId": transact.id }), 201
		case 'fob':
			logger.debug("fob payment")

			transact = Transaction(member_id=id, desc='KeyFob', amount=request.json['amount'])
			db.session.add(transact)
			db.session.commit()

			return make_response({ "transactionId": transact.id }), 201
		case _:
			logger.debug("unknown payment type")
			return abort(501)

@members.get('/members/transactions/<id>')
@login_required
def receipt(id):
	"""Generate a receipt for the member."""

	transact = Transaction.query.get(id)
	member = Member.query.get(transact.member_id)

	response = make_response(render_template('receipt.txt', member=member, transact=transact))
	response.headers['Content-Type'] = 'text/plain'

	return response