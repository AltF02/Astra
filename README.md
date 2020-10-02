# Rust discord template
This template is configured with postgres and serenity.

## How to start
Clone the repo with 
```shell script
git clone https://github.com/DankDumpster/rust-discord-bot-template
```

Now we need to setup sqlx and we do that by doing
```shell script
cargo install --version=0.1.0-beta.1 sqlx-cli --no-default-features --features postgres
```

Before we run sqlx we need to set the database environment variable temporarily, this is different for most shells

#### cmd
```shell script
setx DATABASE_URL = "postgres://postgres:postgres@localhost/postgres"
```
#### fish
```shell script
set -g -x  DATABASE_URL "postgres://postgres:postgres@localhost/postgres"
```
etc...

Now once that's done we're going to run, this will take some time as its compiling everything
```shell script
cargo sqlx prepare
```

NOTE: this will panic

This will ask you to enter each of the configuration values. If you wish edit the physical config.yml file then type "n" when it asks you if you wish to enter the config details and the bot will close and generate a predefined config.yml file for you to configure.
If anything goes wrong for any reason you can manually make a config.yml in the root with this:
```yaml
---
token: ""
prefix: ;
db_uri: "postgres://postgres:postgres@localhost/postgres"
```

Now run it again and it should work
```shell script 
cargo run
```

## Notes
### Custom Config Location
To set a custom config location, set the enviroment variable "CONFIG_PATH" and point it to a file.

###### Massive thanks to [dylan](https://github.com/dylhack) for the config function
