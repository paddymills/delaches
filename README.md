# delaches

Delaches member management

# Build

```bash
cargo build --release
```

# Deploy

Make sure you grab all the static assets:

- `/assets/
- `/public`
- `/style`
- `/.env`

Might also want the `import_data.py` script for utility purposes

Create a `config.toml` file as

```toml
port = 3000
code = "<some secret numerical pin>"
```

# run

`./delaches`

## Hot reload

Keep in mind that the authentication tokens are stored in memory. So, a reload of the server resets all authentication.

Use [watchexec](https://watchexec.github.io/) to reload on config file changes.

```bash
watchexec --restart --watch config.toml -- delaches
```
