import uvicorn
from fastapi import FastAPI

from app.core.resources.app_config import config
from app.core.routers import bestiary, health

app = FastAPI(
    title=config.service_name,
    version="0.0.1",
    description=config.service_description,
)

app.include_router(bestiary.router)
app.include_router(health.router)

if __name__ == "__main__":
    uvicorn.run(app, host=config.service_ip, port=config.service_port)
