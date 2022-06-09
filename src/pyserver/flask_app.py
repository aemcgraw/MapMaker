from flask import Flask, render_template

from typing import TYPE_CHECKING

app = Flask(__name__)

@app.route('/')
def index() -> str:
    return render_template('index.html')
