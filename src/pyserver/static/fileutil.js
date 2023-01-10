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
        var canvas = document.getElementById('map-canvas');

        canvas.toBlob(function(blob) {
            var request = new XMLHttpRequest();
            
            var data = new FormData();
            data.append('file', blob);

            request.onload = function() {
                if (request.status == 200) {
                    var response = JSON.parse(request.responseText);
                    if (response['meta']['status'] = 'ok') {
                        alert("Upload successful");
                    }
                } else {
                    alert("Server Error");
                }
            };

            request.open("POST", url);
            request.setRequestHeader("Content-Type", "blob");
            request.send(data);
        });
    },

    //save_to_server: function() {
        //var url = '/savetoserver/';
        //var canvas = document.getElementById('map-canvas');
        //var ctx = canvas.getContext('2d');
        //var image = canvas.toDataURL("image/png");
        //var imageblob = new Blob(ctx, { type :"image/png" } );
        //var data = new FormData();
        //data.append('file', data);

        //var request = new XMLHttpRequest();
        //request.onload = function () {
        //    if (request.status == 200) {
        //        var response = JSON.parse(request.responseText);
        //        if (response['meta']['status'] = 'ok') {
        //            alert("Upload successful");
        //            // TODO: Better upload message
        //        }
        //    } else {
        //        alert("Server Error");
        //    }
        //};

        //request.open("POST", url);
        //request.setRequestHeader("Content-Type", "image/png");
        //request.send(data);

        // TODO: Save Image
        // TODO: Save complete map data
    //},

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
