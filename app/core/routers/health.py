import logging

from fastapi import APIRouter
from starlette import status
from starlette.responses import Response

from app.core.resources import redis_handler

router = APIRouter(
    prefix=f"/health",
    tags=["health"],
    responses={
        502: {"description": "The database is offline."},
    },
)
logger = logging.getLogger(__name__)


@router.get("/")
async def health() -> dict:
    redis_up: bool = await redis_handler.is_redis_up()
    result_dict = {
        "ready": redis_up,
        "dependencies": [
            {
                "name": "redis database",
                "ready": redis_up,
                "live": redis_up,
                "type": "REQUIRED",
            },
        ],
    }
    logger.debug(result_dict)
    return result_dict


@router.get("/ready/")
async def health_ready() -> Response:
    redis_up = await redis_handler.is_redis_up()
    if redis_up:
        logger.debug("Health ready with status code 200")
        return Response(status_code=status.HTTP_200_OK)
    else:
        logger.debug("Health ready with status code 500")
        return Response(
            content="Service or dependencies are not healthy",
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
        )


@router.get("/live/")
async def health_live() -> Response:
    logger.debug("Health live with status code 200")
    return Response(status_code=status.HTTP_200_OK)
