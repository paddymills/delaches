from flask import Blueprint, request, abort, render_template, make_response
from flask_login import login_required
from . import db, logger
from .models import Member, Transaction, Dues
from datetime import date

members = Blueprint("members", __name__)


@members.route("/members")
@login_required
def member_mgmt():
    return render_template("members.html")


@members.route("/members/all")
@login_required
def listing():
    return [m.all() for m in Member.query.all()]


@members.get("/members/<id>")
@login_required
def member(id):
    logger.debug("requested member: {}".format(id))

    logger.debug(Dues.query.distinct(Dues.type).all())
    return render_template("member.html", member=Member.query.get(id), types=Dues.query.distinct(Dues.type))


@members.post("/members/")
@login_required
def add_member():
    logger.debug("adding member: {}".format(id))
    print(request.form)

    member = Member(
        first_name=request.form["first_name"],
        last_name=request.form["last_name"],
        birthdate=date.fromisoformat(request.form["birthdate"]),
        address=request.form["address"],
        city=request.form["city"],
        state=request.form["state"],
        zip=request.form["zip"],
        phone=request.form["phone"],
        email=request.form["email"],
        type=request.form["type"],
    )
    db.session.add(member)
    db.session.commit()

    return "OK", 200


@members.post("/members/<id>")
@login_required
def update_member(id):
    logger.debug("updating member: {}".format(id))

    print(request.form)
    member = Member.query.get(id)
    member.first_name = request.form["first_name"]
    member.last_name = request.form["last_name"]
    member.birthdate = request.form["birthdate"]
    member.address = request.form["address"]
    member.city = request.form["city"]
    member.state = request.form["state"]
    member.zip = request.form["zip"]
    member.phone = request.form["phone"]
    member.email = request.form["email"]
    db.session.commit()

    return "OK", 200


@members.post("/members/<id>/pay")
@login_required
def pay_member(id):
    logger.debug("paying for member: {}".format(id))

    match request.json["paymentType"]:
        case "dues":
            logger.debug("dues payment")

            transact = Transaction(
                member_id=id, desc="Dues", amount=request.json["amount"]
            )
            db.session.add(transact)
            db.session.commit()

            return make_response({"transactionId": transact.id}), 201
        case "fob":
            logger.debug("fob payment")

            transact = Transaction(
                member_id=id, desc="KeyFob", amount=request.json["amount"]
            )
            db.session.add(transact)
            db.session.commit()

            return make_response({"transactionId": transact.id}), 201
        case _:
            logger.debug("unknown payment type")
            return abort(501)


@members.get("/members/transactions/<id>")
@login_required
def receipt(id):
    """Generate a receipt for the member."""

    transact = Transaction.query.get(id)
    member = Member.query.get(transact.member_id)

    response = make_response(
        render_template("receipt.txt", member=member, transact=transact)
    )
    response.headers["Content-Type"] = "text/plain"

    return response
