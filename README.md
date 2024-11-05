# As frontend user
## I want to send a visitor credentials to the backend service
### And receive an approvement of them

# As a visitor
## I want to register my credentials into the backend service
### So that the backend service can approve me for frontend user

# Before you start
We need `psql` installed:
```
sudo apt-get install -y postgresql-client
```
Make `sqlx-cli` available:
```
cargo install --version="~0.7" sqlx-cli --no-default-features --features rustls,postgres
```

In `.env` file prepare
```
DATABASE_URL="postgres://[postgres]:[password]@localhost:5432/newsletter"
```

Every day first of all run
```
./scripts/init_db.sh
```

# Last point in da book
Continue at `4.5 Structured Logging`