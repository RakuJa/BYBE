from fastapi import FastAPI

from app.core.resources.app_config import config
from app.core.routers import bestiary, health, encounter

app = FastAPI(
    title=config.service_name,
    version="0.0.1",
    description=config.service_description,
)

app.include_router(bestiary.router)
app.include_router(encounter.router)
app.include_router(health.router)
