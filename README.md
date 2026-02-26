![Rust](https://img.shields.io/badge/Rust-664666?style=for-the-badge&logo=rust&logoColor=red)
![Actix-web](https://img.shields.io/badge/actix-web?style=for-the-badge&logoColor=black&labelColor=pink&color=black
)
![SQLite](https://img.shields.io/badge/sqlite-%2307405e.svg?style=for-the-badge&logo=sqlite&logoColor=white)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

# BYBE - Backend

> Beyond Your Bestiary Explorer (BYBE) provides tools to help Pathfinder/Starfinder 2e Game Masters. Built as the backend of [BYBE - Frontend](https://github.com/TheAsel/BYBE-frontend/)

## Download & Run (Docker)
Install from command line
```bash
docker pull ghcr.io/rakuja/bybe-backend:latest
```
Run
```bash
docker run -p 25566:25566 --name bybe-container bybe-backend
```

To run it

## Download & Run (Application)
If you were looking for the BYBE Local Application, it can be found [Here](https://github.com/rakuJa/BYBE-desktop)

## Features

- Browse and filter a list of all creatures.
- Browse and filter a list of all hazards.
- Balance encounters based on your party size and level (creatures + hazards).
- Generate random encounters based on your requirements (creatures + hazards).
- Support for both remaster and legacy content.
- Browse and filter a list of all items.
- Generate random shop with custom templates.
- Support for Starfinder 2e.
- Generate random npc.
- Share encounters/shops.

## Requirements

Built using:

- [Rust](https://www.rust-lang.org/tools/install)
- [SQLite](https://www.sqlite.org/download.html)

## Building guide - Local Development

1. Install [Rust](https://www.rust-lang.org/tools/install) on your machine.
2. Populate the SQLite database (public release date TBA).
3. Clone this repository:

```bash
git clone https://github.com/RakuJa/BYBE
```

4. Navigate to the project's main directory.
5. Build the project running all the tests and downloading the db (required only once):
```bash
cargo make bybe-build
```
6. Build the project

```bash
cargo build
```
6. Set DATABASE_URL variable to SQLite db path
7. Run the backend in development mode:

```bash
cargo run
```

8. To instead deploy the production build, run:

```bash
cargo build --release
```

```bash
cargo run
```

## Building guide using Docker - Local development

1. Install Docker on your local machine
2. Clone the repository or download the ZIP
```bash
git clone https://github.com/RakuJa/BYBE
```
3. Go to the local BYBE project folder

4. Build docker image of bybe using
```bash
docker build -t bybe .
```
5. Run the image
```bash
docker run -p 25566:25566 --name bybe-container bybe
```

## Support me

If you like this tool, consider supporting me:

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/rakuja)

Also consider supporting [TheAsel](https://github.com/TheAsel), the frontend developer. Thank you!

## Coding guide
1. Install prek
Check out the official installation [guide](https://github.com/j178/prek?tab=readme-ov-file#installation)
```bash
cargo install --locked prek
```
2. Enable prek run before each commit
```bash
prek install
```
3. Install cargo dependencies
```bash
cargo install cargo-udeps --locked
```
