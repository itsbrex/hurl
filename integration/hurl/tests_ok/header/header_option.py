from app import app
from flask import request


@app.route("/header-option")
def header_option():
    headers = [h.strip() for h in request.headers.get("key").split(",")]
    assert headers == [
        "from-header-syntax",
        "from-cli",
        "from-option-1",
        "from-option-2",
        "from-variable",
    ]
    assert request.headers.get("another-key") == "another-from-option"
    assert request.headers.get("key-from-variable") == "value-from-variable"
    return ""
