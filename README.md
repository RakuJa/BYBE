![Rust](https://img.shields.io/badge/Rust-664666?style=for-the-badge&logo=rust&logoColor=red)
![Actix-web](https://img.shields.io/badge/actix-web?style=for-the-badge&logoColor=black&labelColor=pink&color=black
)
![SQLite](https://img.shields.io/badge/sqlite-%2307405e.svg?style=for-the-badge&logo=sqlite&logoColor=white)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)


[![SonarCloud](https://sonarcloud.io/images/project_badges/sonarcloud-orange.svg)](https://sonarcloud.io/summary/new_code?id=RakuJa_BYBE)

# BYBE - Backend

> Beyond Your Bestiary Explorer (BYBE) provides tools to help Pathfinder 2e Game Masters. Built as the backend of [BYBE - Frontend](https://github.com/TheAsel/BYBE-frontend/)

## Features

- Browse and filter a list of all creatures.
- Balance encounters based on your party size and level.
- Generate random encounters based on your requirements.
- More to come...

## Requirements

Built using:

- [Rust](https://www.rust-lang.org/tools/install)
- [SQLite](https://www.sqlite.org/download.html)

## Installation guide - Local

1. Install [Rust](https://www.rust-lang.org/tools/install) on your machine.
2. Populate the SQLite database.
3. Clone this repository:

```
git clone https://github.com/RakuJa/BYBE
```

4. Navigate to the project's main directory.
5. Build the project:

```
cargo build
```
6. Set DATABASE_URL variable to SQLite db path
7. Run the backend in development mode:

```
cargo run
```

8. To instead deploy the production build, run:

```
cargo build --release
```

```
cargo run
```

## Installation guide using Docker

1) Install Docker on your local machine
2) Download redis on your local machine:
```
docker pull redis
```
3) Clone the repository or download the ZIP
```
git clone https://github.com/RakuJa/BYBE
```
4) Go to the local BYBE project folder

5) Build docker image of bybe using
```
docker build -t bybe .
```

6) Run the image
```
docker run -p 25566:25566 --name bybe-container bybe
```

## Support me

If you like this tool, consider supporting me:

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/rakuja)

Also consider supporting [TheAsel](https://github.com/TheAsel), the frontend developer. Thank you!
