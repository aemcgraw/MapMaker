from flask import Flask, jsonify, render_template, request, Response

from typing import Any, Dict, List, TYPE_CHECKING

from PIL import Image

import os

app = Flask(__name__)

uploads_dir = os.path.join(app.instance_path, 'uploads')
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
            #data = request.data
            #print(type(data))
            files = request.files
            file = files.get('file')
            print(type(file))
    except Exception as e:
        result['meta']['status'] = 'fail'
        result['meta']['reason'] = str(e)
        traceback.print_exc()
    else:
        result['meta']['status'] = 'ok'
        result['content'] = None
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
                imageobj = Image.open(fulltarget, 'r');
                imagedata = list(imageobj.getdata());
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
