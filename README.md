# delaches

Delaches member management

# Install


```powershell
python -m venv venv
source venv/bin/activate
pip install -r requirements.txt
python manage.py --deploy
```

## Import data
```
python manage.py --load path/to/excel/file
```

## Configure
Using something like [SQLite Studio]() change the pin codes
(in the users) table.

# Develop

```
flask run
```

# Deploy

```
waitress-serve call members:create_app
```

navigate to `localhost:8080`

# Reverse proxy
If you don't want to have to navigate to the site using the port

install and nginx from https://nginx.org/en/download.html

C:\nginx\nginx.conf
```
server {
    listen 80;
    server_name _;

    location / {
        proxy_pass http://127.0.0.1:8000/;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_set_header X-Forwarded-Host $host;
        proxy_set_header X-Forwarded-Prefix /;
    }
}
```


