"use strict";

var FileUtil = {
    loadfile: function() {
        var canvas = document.getElementById('map-canvas');
        var ctx = canvas.getContext('2d');
        var img = new Image;
        var filename = document.getElementById('loadfilename').value;
        img.src = '/static/uploads/'.concat(filename, '/');
        img.onload = function() {
            console.log(img.naturalWidth);
            canvas.setAttribute('width', img.naturalWidth);
            canvas.setAttribute('height', img.naturalHeight);
            ctx.drawImage(img, 0, 0);
        }
    },

    download: function() {
        var canvas = document.getElementById('map-canvas');
        var downloadLink = document.createElement('a');
        downloadLink.setAttribute('download', 'CanvasAsImage.png');
        canvas.toBlob(function(blob) {
            var url = URL.createObjectURL(blob);
            downloadLink.setAttribute('href', url);
            downloadLink.click();
        });

        // TODO: close object url?
    },

    save_to_server: function() {
        var url = '/savetoserver/';

        //Convert canvas to base64 encoding
        var canvas = document.getElementById('map-canvas');
        var imageurl = canvas.toDataURL("image/png");

        //Convert base64 encoding to binary blob
        var blobBin = atob(imageurl.split(',')[1]);
        var array = [];
        for(var i = 0; i < blobBin.length; i++) {
            array.push(blobBin.charCodeAt(i));
        }
        var imageblob = new Blob([new Uint8Array(array)], {type: 'image/png'});

        var filename = document.getElementById('savefilename').value;

        var request = new XMLHttpRequest();
        request.onload = function () {
            if (request.status == 200) {
                var response = JSON.parse(request.responseText);
                if (response['meta']['status'] = 'ok') {
                    alert("Successfully uploaded to " + response['content']);
                }
            } else {
                alert("Server Error");
            }
        };

        request.open("POST", url);
        request.setRequestHeader("Content-Type", "image/png");
        request.setRequestHeader("Filename", filename);
        request.send(imageblob);

        // TODO: Save complete map data
    },

//    loadfile: function() {
//        var url = '/load/';
//
//        var loadfilename = document.getElementById('loadfilename').value;
//        
//        var request = new XMLHttpRequest();
//        request.onload = function () {
//            if (request.status == 200) {
//                var response = JSON.parse(request.responseText);
//                if(response['meta']['status'] = 'ok') {
//                    FileUtil.displaymap(response['content']);
//                }
//            } else {
//                alert("Server Error");
//            }
//        }; 
//        request.open("GET", url);
//        request.send();
//    },

//    displaymap: function(map) {
//        var canvas = document.getElementById('map-canvas');
//        var ctx = canvas.getContext("2d");
//        var img = new Image;
//        img.src = "/static/uploads/worldmap.jpg";
//        ctx.drawImage(img, 0, 0);
//    }
}

export { FileUtil };
