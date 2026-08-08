
## Installation

```sh
# DATABASE_URL="mssql://sa:YourStrong()Passw0rd@localhost/AdventureWorksLT2016"
# DATABASE_URL="mysql://root:root@localhost/AdventureWorksLT2016"
DATABASE_URL="postgres://root:root@localhost/adventureworkslt2016"
#DATABASE_URL="sqlite://db.sqlite?mode=rwc"
```

2. If you want to target other databases, please enable the database backend in `Cargo.toml` accordingly:

```toml
[features]
default = ["sqlx-sqlite"] # <- change this
sqlx-mysql = ["sea-orm/sqlx-mysql"]
sqlx-postgres = ["sea-orm/sqlx-postgres"]
sqlx-sqlite = ["sea-orm/sqlx-sqlite"]
```

3. Setup database schema and seed database with .xls file contents

```sh
cargo run task seed_graph
```

4. Start the Loco.rs server

```sh
cargo run start

listening on [::]:8086
```

The grapher takes a .xls file with a specific form to decode the family relations. The amount of '*' denotes the generation, and the order is highly relevant, like if the amount of stars goes down, then this next person is not a child, but perhaps a sister/brother, or an uncle, or great uncle etc..

## License

Licensed under MIT license ([LICENSE](https://github.com/SeaQL/sea-orm-pro/blob/main/LICENSE) or <http://opensource.org/licenses/MIT>)
