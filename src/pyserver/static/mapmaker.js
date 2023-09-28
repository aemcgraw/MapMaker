"use strict;"

import init, { makeimage, MapArgs} from "./pkg/making_maps.js";
import { CanvasUtil } from './canvasutil.js';
import { FileUtil } from './fileutil.js';

function createmap() {
    const canvas = document.getElementById('map-canvas');
    const ctx = canvas.getContext('2d');

    // TODO : This could be more robust
    var algorithm = document.getElementById('algorithm').value;
    var width = parseInt(document.getElementById('widthbox').value);
    var height = parseInt(document.getElementById('heightbox').value);
    var chaos = parseFloat(document.getElementById('chaosbox').value);
    if (isNaN(chaos)) {chaos = 0.5;}
    var damping = parseFloat(document.getElementById('dampingbox').value);
    if (isNaN(damping)) {damping = 0.8;}
    var blocksize = parseInt(document.getElementById('blockbox').value);
    if (isNaN(blocksize)) {blocksize = width;}
    var seed = parseInt(document.getElementById('seedbox').value);
    if (isNaN(seed)) {seed = 0;}
    console.log(seed);
    var coloring = document.getElementById('coloring').value;
    var water = parseFloat(document.getElementById('waterRange').value);
    if (isNaN(water)) { water = 0.0; }

    const mapargs = new MapArgs(width, height, chaos, damping, blocksize, water, seed);

    if (!isNaN(width) && width != "0" && !isNaN(height) && height != "0") {
        canvas.width = width < 1000 ? width : 1000;     //Set maximum allowed map height and width to 1000
        canvas.height = height < 1000 ? height: 1000;

        var backing_vec = makeimage(ctx, mapargs, algorithm, coloring);
    } else if (width == NaN || width == 0) {
        alert('Could not interpret value given for width');
    } else {
        alert('Could not interpret value given for height');
    }
}

function collapse() {
    document.getElementById('sidebar').classList.toggle('collapsed');

    document.getElementById('container').classList.toggle('dataview');
    document.getElementById('container').classList.toggle('fullview');
}

function dothing(ev) {
    var caller = ev.currentTarget;

    var dClassList = document.getElementsByClassName('Color');
    for (let i = 0; i < dClassList.length; i++) {
        dClassList[i].classList.add('hidden');
    }

    switch(caller.options[caller.selectedIndex].text) {
        case "BlueGreen":
                var dClassList = document.getElementsByClassName('BlueGreen');
                for (let i = 0; i < dClassList.length; i++) {
                    dClassList[i].classList.remove('hidden');
                }
            break;
        case "Rainbow":
                var dClassList = document.getElementsByClassName('Rainbow');
                for (let i = 0; i < dClassList.length; i++) {
                    dClassList[i].classList.remove('hidden');
                }
            break;
        case "Topographical":
                var dClassList = document.getElementsByClassName('Topographical');
                for (let i = 0; i < dClassList.length; i++) {
                    dClassList[i].classList.remove('hidden');
                }
            break;
    }
}

async function run() {
    await init();
    //createmap();

    var bcollapse = document.getElementById('options-collapse');
    bcollapse.addEventListener('click', function() {collapse()});

    var bmake = document.getElementById('makebutton');
    bmake.addEventListener('click', function() {createmap()})

    var colorselector = document.getElementById('coloring');
    colorselector.addEventListener('change', dothing)

    var reset = document.getElementById('reset');
    reset.addEventListener('click', function() {CanvasUtil.reset()});

    var zoomin = document.getElementById('zoomin');
    zoomin.addEventListener('click', function() {CanvasUtil.zoom_in_graph()});

    var zoomout = document.getElementById('zoomout');
    zoomout.addEventListener('click', function() {CanvasUtil.zoom_out_graph()});

    mainpage.addEventListener('mousedown', function() {CanvasUtil.drag_element(event)});

    var loadbutton = document.getElementById('load');
    loadbutton.addEventListener('click', function() {FileUtil.loadfile()});

    var savebutton = document.getElementById('save');
    savebutton.addEventListener('click', function() {FileUtil.save_to_server()});

    var downloadbutton = document.getElementById('download');
    downloadbutton.addEventListener('click', function() {FileUtil.download()});
}

window.onload = run()
