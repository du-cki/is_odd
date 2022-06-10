from fastapi import FastAPI
from fastapi.responses import JSONResponse

app = FastAPI(debug=False, title="Is Even?", version="0.1.0", openapi_url=None, redoc_url=None, docs_url=None)


@app.get("/{number}")
async def is_odd(number: int | float) -> JSONResponse:
    return JSONResponse({"number": number, "is_odd": not number % 2 == 0}, status_code=200)
