import uvicorn

from app import controller
from app.core.resources.app_config import config

if __name__ == "__main__":
    uvicorn.run(controller.app, host=config.service_ip, port=int(config.service_port))
