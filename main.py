from app import controller
import uvicorn
from app.core.resources.app_config import config

if __name__ == "__main__":
    uvicorn.run(controller.app, host=config.service_ip, port=config.service_port)
