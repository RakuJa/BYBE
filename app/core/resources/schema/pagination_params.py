from pydantic import BaseModel
from fastapi import Query


class PaginationParams(BaseModel):
    cursor: int = Query(0, ge=0)
    page_size: int = Query(100, ge=1, le=100)
