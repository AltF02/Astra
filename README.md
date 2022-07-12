# Astra 
![Build](https://github.com/AltF02/Astra/workflows/Rust/badge.svg?style=flat-square)
[![Discord Bots](https://top.gg/api/widget/status/675542011457044512.svg)](https://top.gg/bot/675542011457044512)

Astra is a discord bot written in rust to keep you reminded on rocket launches and more! An always online bot is available **[here](https://discord.com/oauth2/authorize?client_id=675542011457044512&permissions=322624&scope=bot%20applications.commands)**
## How to start

### Native
Clone the repo with 
```shell script
git clone https://github.com/AltF02/astra.git
```

We need to copy the example.env to .env
```shell script
cp .example.env .env
```
Fill out this with your token and configuration. Once that's done we're going to run, this will take some time as its compiling everything
```shell script
cargo run
```

### Docker
**NOTE: Docker support is currently in beta and can cause issues**

#### docker-compose
Using docker compose requires an .env file, an template can be found in `.example.env`

```yaml
version: "3"
services:
  astra:
    restart: always
    build: .
    networks:
      - astra
    env_file:
      - .env
  astra-db:
    image: postgres:14.1-alpine3.14
    volumes:
      - data:/var/lib/postgresql/data
    networks:
      - astra
    ports:
      - "5432:5432"

volumes:
  data:

networks:
  astra:
```

## == We're Using GitHub Under Protest ==

This project is currently hosted on GitHub.  This is not ideal; GitHub is a
proprietary, trade-secret system that is not Free and Open Souce Software
(FOSS).  We are deeply concerned about using a proprietary system like GitHub
to develop our FOSS project.  We have an
[open issue](https://github.com/AltF02/Astra) where the
project contributors are actively discussing how we can move away from GitHub
in the long term.  We urge you to read about the
[Give up GitHub](https://GiveUpGitHub.org) campaign from
[the Software Freedom Conservancy](https://sfconservancy.org) to understand
some of the reasons why GitHub is not a good place to host FOSS projects.

If you are a contributor who personally has already quit using GitHub, please
[check this resource](https://codeberg.org/Matthew/Astra) for how to send us contributions without
using GitHub directly.

Any use of this project's code by GitHub Copilot, past or present, is done
without our permission.  We do not consent to GitHub's use of this project's
code in Copilot.

![Logo of the GiveUpGitHub campaign](https://sfconservancy.org/img/GiveUpGitHub.png)

