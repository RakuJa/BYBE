import asyncio

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from app.core.resources.app_config import config
from app.core.resources.network import redis_proxy
from app.core.routers import bestiary, encounter, health

app = FastAPI(
    title=config.service_name,
    version="0.4.1",
    description=config.service_description,
)

origins = ["*"]

methods = ["OPTIONS", "GET", "POST"]
app.add_middleware(
    middleware_class=CORSMiddleware,
    allow_origins=origins,
    allow_methods=methods,
)

app.include_router(bestiary.router)
app.include_router(encounter.router)
app.include_router(health.router)


@app.on_event("startup")
async def startup_event() -> None:
    # create the event loop
    loop = asyncio.get_event_loop()

    # create a task to run the update_cache coroutine in the event loop
    loop.create_task(redis_proxy.update_cache())
