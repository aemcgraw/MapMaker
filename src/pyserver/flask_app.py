from flask import Flask, jsonify, render_template, request, Response
from werkzeug.utils import secure_filename

from typing import Any, Dict, List, TYPE_CHECKING

from PIL import Image

import os
import traceback
import json

import base64

app = Flask(__name__)

uploads_dir = os.path.join(os.path.dirname(app.instance_path), "uploads")
if not os.path.exists(uploads_dir):
    os.makedirs(uploads_dir, exist_ok=True)

@app.route('/')
def index() -> str:
    return render_template('index.html')

@app.route('/save/', methods=['GET', 'POST'])
def save():
    if request.method == 'POST':
        profile = request.files['profile']
        profile.save(os.path.join(uploads_dir, secure_filename(profile.filename)))

@app.route('/savetoserver/', methods=['GET', 'POST'])
def savetoserver():
    result: Dict[str, Dict[str, Any]] = {}
    result['meta'] = {}
    try:
        if request.method == 'POST':
            data = request.get_data()
            filepath = os.path.join(uploads_dir, secure_filename(request.headers["filename"]))
            if not filepath.endswith('.png'):
                filepath = filepath + '.png'
            with open(filepath, "wb") as binary_file:
                binary_file.write(data)
    except Exception as e:
        result['meta']['status'] = 'fail'
        result['meta']['reason'] = str(e)
        traceback.print_exc()
    else:
        result['meta']['status'] = 'ok'
        result['content'] = filepath
    return jsonify(result)

#TODO : finish typing
@app.route('/load/', methods=['GET'])
def load() -> Response:
#def load(target: str):
    result: Dict[str, Dict[str, Any]] = {}
    result['meta'] = {}
    try:
        target = 'worldmap.jpg';
        fulltarget = os.path.join(uploads_dir, target)
        if os.path.isfile(fulltarget):
            if fulltarget.lower().endswith(('.bmp', '.jpg', '.png')):
                imageobj = Image.open(fulltarget, 'r')
                imagedata = list(imageobj.getdata())
                print(imagedata[0])
    except Exception as e:
        result['meta']['status'] = 'fail'
        result['meta']['reason'] = str(e)
        traceback.print_exc()
    else:
        result['meta']['status'] = 'ok'
        result['content'] = None
    return jsonify(result)

@app.route('/list/', methods=['GET'])
def list() -> Response:
    result: Dict[str, Dict[str, Any]] = {}
    result['meta'] = {}
    try:
        filelist = os.listdir(uploads_dir)
    except Exception as e:
        result['meta']['status'] = 'fail'
        result['meta']['reason'] = str(e)
        traceback.print_exc()
    else:
        result['meta']['status'] = 'ok'
        result['content'] = filelist
    return jsonify(result)
