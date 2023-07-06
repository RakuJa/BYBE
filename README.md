![Python](https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54)
![FastAPI](https://img.shields.io/badge/FastAPI-005571?style=for-the-badge&logo=fastapi)
![Redis](https://img.shields.io/badge/redis-%23DD0031.svg?style=for-the-badge&logo=redis&logoColor=white)
[![Ruff](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/charliermarsh/ruff/main/assets/badge/v2.json)](https://github.com/astral-sh/ruff)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)


[![SonarCloud](https://sonarcloud.io/images/project_badges/sonarcloud-orange.svg)](https://sonarcloud.io/summary/new_code?id=RakuJa_BYBE)

# BYBE
Pathfinder 2e - Encounter Builder BACKEND

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

Instead of creating a bybe docker instance you may want to run it directly.

```
gunicorn app.controller:app --config app/gunicorn.conf.py
```


Gunicorn will have first class support, but you may use uvicorn like:
```
uvicorn app.controller:app --host 127.0.0.1 --port 25566
```


