"use strict;"

import init, { makeimage } from "./pkg/making_maps.js";

function createmap() {
    const canvas = document.getElementById('map-canvas');
    const ctx = canvas.getContext('2d');

    var width = parseInt(document.getElementById('widthbox').value);
    var height = parseInt(document.getElementById('heightbox').value);
    var chaos = parseFloat(document.getElementById('chaosbox').value);
    var algorithm = document.getElementById('algorithm').value;
    var water = parseFloat(document.getElementById('waterbox').value);
    //var coloring = document.getElementById('coloring').value;

    //var waterperc = parseFloat("0.5")

    if (!isNaN(width) && width != "0" && !isNaN(height) && height != "0") {
        canvas.width = width;
        canvas.height = height;
        makeimage(ctx, width, height, water);
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

async function run() {
    await init();
    //createmap();

    var bcollapse = document.getElementById('options-collapse');
    bcollapse.addEventListener('click', function() {collapse()});

    var bmake = document.getElementById('makebutton');
    bmake.addEventListener('click', function() {createmap()})
}

window.onload = run()
