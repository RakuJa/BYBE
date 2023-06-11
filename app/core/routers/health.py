import logging

from starlette import status
from starlette.responses import Response

from app.core.resources.network import redis_communicator
from app.core.resources.api_router import APIRouter

router = APIRouter(
    prefix="/health",
    tags=["health"],
    responses={
        502: {"description": "The database is offline."},
    },
)
logger = logging.getLogger(__name__)


@router.get("/")
def health() -> dict:
    redis_up: bool = redis_communicator.is_redis_up()
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
def health_ready() -> Response:
    redis_up = redis_communicator.is_redis_up()
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
def health_live() -> Response:
    logger.debug("Health live with status code 200")
    return Response(status_code=status.HTTP_200_OK)
