set windows-shell : ["powershell.exe", "-Command"]

[linux]
deploy:
    cargo build --release
    mkdir dist
    cp target/release/delaches dist/
    cp -r assets dist/
    cp -r public dist/
    cp -r style dist/
    mkdir dist/logs
    cp .env dist/
    cp schema dist/
    cp import_data.py dist/

[windows]
deploy:
    cargo build --release
    mkdir dist
    cp target/release/delaches.exe dist/
    cp -r assets dist/
    cp -r public dist/
    cp -r style dist/
    mkdir dist/logs
    cp .env dist/
    cp schema dist/
    cp import_data.py dist/