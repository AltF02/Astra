# Apollo
Apollo is a discord bot written in rust to keep you reminded on rocket launches and more! An always online bot is available **[here](https://discord.com/oauth2/authorize?client_id=755775587716563015&permissions=355328&scope=bot)**

## How to start
Clone the repo with 
```shell script
git clone https://github.com/DankDumpster/apollo.git
```

Now we need to setup sqlx and we do that by doing
```shell script
cargo install --version=0.1.0-beta.1 sqlx-cli --no-default-features --features postgres
```

Before we run we need to set the database environment variable temporarily, this is different for most shells. This is needed for the library sqlx.

#### cmd
```shell script
set DATABASE_URL=postgres://postgres:postgres@localhost/postgres
```
#### fish
```shell script
set -g -x  DATABASE_URL "postgres://postgres:postgres@localhost/postgres"
```
etc...

Now once that's done we're going to run, this will take some time as its compiling everything
```shell script
cargo run
```

NOTE: this will panic

This will ask you to enter each of the configuration values. If you wish edit the physical config.yml file then type "n" when it asks you if you wish to enter the config details and the bot will close and generate a predefined config.yml file for you to configure.
If anything goes wrong for any reason you can manually make a config.yml in the root with this:
```yaml
---
token: 
prefix: ";"
db_uri: "postgres://postgres:password@localhost/postgres"
nasa_key: 
log_channel_id: 
```

Now run it again and it should work
```shell script 
cargo run
```
