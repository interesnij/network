function loadScripts( src ) {
    var script = document.createElement("SCRIPT"),
        head = document.getElementsByTagName( "head" )[ 0 ],
        error = false;

    script.type = "text/javascript";
    script.onload = script.onreadystatechange = function( e ){

        if ( ( !this.readyState || this.readyState == "loaded" || this.readyState == "complete" ) ) {
            if ( !error ) {
                removeListeners();
            } else {
                null
            }
        }
    };

    script.onerror = function() {
        error = true;
        removeListeners();
    }

    function errorHandle( msg, url, line ) {

        if ( url == src ) {
            error = true;
            removeListeners();
        }
        return false;
    }

    function removeListeners() {
        script.onreadystatechange = script.onload = script.onerror = null;

        if ( window.removeEventListener ) {
            window.removeEventListener('error', errorHandle, false );
        } else {
            window.detachEvent("onerror", errorHandle );
        }
    }

    if ( window.addEventListener ) {
        window.addEventListener('error', errorHandle, false );
    } else {
        window.attachEvent("onerror", errorHandle );
    }

    script.src = src;
    head.appendChild( script );
};

loadScripts('/static/scripts/lib/progressive-image.js');
loadScripts('/static/scripts/lib/Chart.min.js');
loadScripts('/static/scripts/lib/video_player.js');
loadScripts('/static/scripts/lib/video_init.js');
