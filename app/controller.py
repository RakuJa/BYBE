from fastapi import FastAPI

from app.core.resources.app_config import config
from app.core.routers import bestiary, health, encounter
from fastapi.middleware.cors import CORSMiddleware

app = FastAPI(
    title=config.service_name,
    version="0.1.1",
    description=config.service_description,
)

origins = ["*"]

methods = ["OPTIONS", "GET", "POST"]
app.add_middleware(
    middleware_class=CORSMiddleware, allow_origins=origins, allow_methods=methods
)

app.include_router(bestiary.router)
app.include_router(encounter.router)
app.include_router(health.router)
