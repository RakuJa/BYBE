import os.path
from configparser import ConfigParser
from pathlib import Path

from pydantic.class_validators import validator
from pydantic.main import BaseModel


class AppConfig(BaseModel):
    # Service
    service_name: str
    service_description: str
    service_ip: str
    service_port: int
    service_timeout: int

    log_path: str
    log_format: str
    log_level: str

    number_of_workers: int

    redis_ip: str
    redis_port: int
    redis_db: int
    redis_username: str
    redis_password: str

    @validator("log_level")
    def log_level_must_be_valid(cls, v):
        # Add any additional validation you need here
        if v.upper() not in ["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"]:
            raise ValueError(f"Log level is not valid. Given value {v}")
        else:
            return v

    @validator("number_of_workers")
    def workers_number_must_be_valid(cls, v):
        # Add any additional validation you need here
        if v <= 0:
            raise ValueError(f"Worker number is not valid. Given value {v}")
        else:
            return v

    @classmethod
    def from_ini(cls, ini_file: str):
        parser = ConfigParser()
        parser.read(Path(ini_file).absolute())
        values = {s: dict(parser.items(s)) for s in parser.sections()}
        return cls(**values.get("app", {}))


config = AppConfig.from_ini(os.path.join(os.path.dirname(__file__), "config.ini"))
