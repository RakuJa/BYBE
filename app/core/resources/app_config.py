import os.path
from configparser import ConfigParser
from pathlib import Path
from typing import Self

from pydantic.class_validators import validator
from pydantic.main import BaseModel


class AppConfig(BaseModel):
    # Service
    service_name: str
    service_description: str
    service_ip: str
    service_port: str
    service_timeout: str

    log_path: str
    log_format: str
    log_level: str

    number_of_workers: str

    redis_ip: str
    redis_port: str

    @validator("log_level")
    def log_level_must_be_valid(cls, value: str) -> str:
        # Add any additional validation you need here
        if value.upper() not in ["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"]:
            raise ValueError(f"Log level is not valid. Given value {value}")
        else:
            return value

    @validator("number_of_workers")
    def workers_number_must_be_valid(cls, value: int) -> int:
        # Add any additional validation you need here
        if value <= 0:
            raise ValueError(f"Worker number is not valid. Given value {value}")
        else:
            return value

    @classmethod
    def from_ini(cls, ini_file: str) -> Self:
        parser = ConfigParser()
        parser.read(Path(ini_file).absolute())
        values = {s: dict(parser.items(s)) for s in parser.sections()}
        return cls(**values.get("app", {}))


config = AppConfig.from_ini(os.path.join(os.path.dirname(__file__), "config.ini"))
