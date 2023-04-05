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

Instead of creating a bybe docker instance you may want to run it directly, gunicorn currenty does not work. Use uvicorn like:

```
uvicorn app.controller:app --host 127.0.0.1 --port 25566
```
