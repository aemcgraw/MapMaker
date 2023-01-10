"use strict";

var CanvasUtil = {
    mouseX : null,
    mouseY : null,

    build_scale_string: function(scale) {
        var canvas = document.getElementById('map-canvas');
        if (canvas.hasAttribute('trX') && canvas.hasAttribute('trY')) {
            var trX = canvas.getAttribute('trX');
            var trY = canvas.getAttribute('trY');
            return CanvasUtil.build_transform_string(trX, trY, scale);
        } else {
            canvas.setAttribute('scale', scale);
            var transform = 'scale(' + scale + ',' + scale + ')';
            return transform;
        }
    },

    build_translate_string : function(trX, trY) {
        var canvas = document.getElementById('map-canvas');
        if (canvas.hasAttribute('scale')) {
            var scale = canvas.getAttribute('scale');
            return CanvasUtil.build_transform_string(trX, trY, scale);
        } else {
            canvas.setAttribute('trX', trX);
            canvas.setAttribute('trY', trY);
            var transform = 'translate(' + trX + 'px,' + trY + 'px)';
            return transform
        }
    },

    build_transform_string: function(trX, trY, scale) {
        var canvas = document.getElementById('map-canvas');
        canvas.setAttribute('scale', scale);
        canvas.setAttribute('trX', trX);
        canvas.setAttribute('trY', trY);
        var transform = 'translate(' + trX + 'px,' + trY + 'px) scale(' + scale + ',' + scale + ')';
        return transform;
    },

    drag_element : function(event) {
        document.onmousemove = CanvasUtil.move_element;
        document.onmouseup = CanvasUtil.stop_move;

        CanvasUtil.mouseX = event.clientX;
        CanvasUtil.mouseY = event.clientY;
    },

    move_element : function(event) {
        var trX = event.clientX - CanvasUtil.mouseX;
        var trY = event.clientY - CanvasUtil.mouseY;

        CanvasUtil.mouseX = event.clientX;
        CanvasUtil.mouseY = event.clientY;

        var canvas = document.getElementById('map-canvas');
        if (canvas.hasAttribute('trX') && canvas.hasAttribute('trY')) {
            var curX = parseInt(canvas.getAttribute('trX'), 10);
            var curY = parseInt(canvas.getAttribute('trY'), 10);
            trX = trX + curX;
            trY = trY + curY;
        }

        var transform = CanvasUtil.build_translate_string(trX.toString(), trY.toString());
        canvas.style.transform = transform;
    },

    reset : function(event) {
        var canvas = document.getElementById('map-canvas');
        canvas.setAttribute('scale', '1.0');
        canvas.removeAttribute('trX');
        canvas.removeAttribute('trY');

        canvas.style.transform = 'scale(1.0, 1.0)';
        canvas.style.webkitTransform = 'scale(1.0, 1.0)';
        canvas.MozTransform = 'scale(1.0, 1.0)';
    },

    //When mouse is released, page elements should no longer move
    stop_move : function(event) {
        document.onmousemove = null;
        document.onmouseup = null;
    },

    zoom_out_graph: function() {
        var canvas = document.getElementById('map-canvas');
        var zoomstr = '0.9';
        if (canvas.hasAttribute('scale')) {
            var zoom = parseFloat(canvas.getAttribute('scale'), 10);
            if (zoom > 0.1) {
                zoom = zoom - 0.1;
            }
            var zoomstr = zoom.toString();
        }
        canvas.setAttribute('scale', zoomstr);

        var new_scale = CanvasUtil.build_scale_string(zoomstr);
        canvas.style.transform = new_scale;
        canvas.style.webkitTransform = new_scale;
        canvas.style.MozTransform = new_scale;
    },

    zoom_in_graph: function() {
        var canvas = document.getElementById('map-canvas');
        var zoomstr = '1.1';
        if (canvas.hasAttribute('scale')) {
            var zoom = parseFloat(canvas.getAttribute('scale'), 10);
            if (zoom < 2.0) {
                zoom = zoom + 0.1;
            }
            var zoomstr = zoom.toString();
        }
        canvas.setAttribute('scale', zoomstr);

        var new_scale = CanvasUtil.build_scale_string(zoomstr);
        canvas.style.transform = new_scale;
        canvas.style.webkitTransform = new_scale;
        canvas.style.MozTransform = new_scale;
    }
}

export { CanvasUtil };
