set windows-shell := ["powershell.exe", "-Command"]

export FLASK_APP := "members"
export FLASK_DEBUG := "1"
export FLASK_ENV := "development"

dev:
    source venv/bin/activate
    flask run