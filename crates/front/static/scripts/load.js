function on(elSelector, eventName, selector, fn) {var element = document.querySelector(elSelector);element.addEventListener(eventName, function(event) {var possibleTargets = element.querySelectorAll(selector);var target = event.target;for (var i = 0, l = possibleTargets.length; i < l; i++) {var el = target;var p = possibleTargets[i];while (el && el !== element) {if (el === p) {return fn.call(p, event);}el = el.parentNode;}}});};
function getCookie(name) {
    const cookies = document.cookie.split(';');
    for (let i = 0; i < cookies.length; i++) {
        let c = cookies[i].trim().split('=');
        if (c[0] === name) {
            return c[1];
        }
    }
    return "";
}
function setCookie(name, value, days) {
    let cookie = `${name}=${encodeURIComponent(value)}`;
    if (days) {
        const expiry = new Date();
        expiry.setDate(expiry.getDate() + days);
        cookie += `; expires=${expiry.toUTCString()}`;
    }
    document.cookie = cookie + "; path=/";
};

function addStyleSheets(href) {
    $head = document.head, $link = document.createElement('link');
    $link.rel = 'stylesheet';
    $link.classList.add("color");
    $link.href = href;
    $head.appendChild($link)
};

function get_custom_design() {
  color = "white";
  background = getCookie("background");
  if (background != "") {
    color = background;
    addStyleSheets("/static/styles/color/" + background + ".css")
  }
};

function check_first_load() {
    span = document.body.querySelector(".app");
  
    if (window.location.href.indexOf('ajax=1') > -1) {
      span.innerHTML = "Permission Denied"; 
    }
    else if (!span.firstChild) {
      url = window.location.href;
      ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      ajax_link.open( 'GET', url + "?ajax=1", true );
      ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
      ajax_link.onreadystatechange = function () {
        if ( this.readyState == 4 && this.status == 200 ) {
            get_custom_design();
            elem_ = document.createElement('span');
            elem_.innerHTML = ajax_link.responseText;
            span.innerHTML = elem_.innerHTML;
            window.history.pushState ({"url":url}, document.title, url);
            
            loadScripts('/static/scripts/lib/Chart.min.js?ver1');
            loadScripts('/static/scripts/lib/plugin_for_player_1.js?ver1');
            loadScripts('/static/scripts/lib/music_player.js?ver1');
            loadScripts('/static/scripts/lib/audio_video.js?ver1');
            loadScripts('/static/scripts/lib/dragula.min.js?ver1');
            loadScripts('/static/scripts/chart_v1.js?ver1');

            loadScripts('/static/scripts/get.js?ver1');
            loadScripts('/static/scripts/functions/other.js?ver1');
            loadScripts('/static/scripts/functions/preview.js?ver1');
            loadScripts('/static/scripts/functions/comment_attach.js?ver1');
            loadScripts('/static/scripts/functions/post_attach.js?ver1');
            loadScripts('/static/scripts/functions/message_attach.js?ver1');
            loadScripts('/static/scripts/functions/reload.js?ver1');

            loadScripts('/static/scripts/docs/get.js?ver1');
            loadScripts('/static/scripts/docs/post.js?ver1');

            loadScripts('/static/scripts/gallery/user_get.js?ver1');
            loadScripts('/static/scripts/gallery/user_post.js?ver1');
            loadScripts('/static/scripts/gallery/community_post.js?ver1');

            loadScripts('/static/scripts/goods/user_get.js?ver1');
            loadScripts('/static/scripts/goods/user_post.js?ver1');
            loadScripts('/static/scripts/goods/community_post.js?ver1');

            loadScripts('/static/scripts/music/get.js?ver1');
            loadScripts('/static/scripts/music/post.js?ver1');

            loadScripts('/static/scripts/posts/user_get.js?ver1');
            loadScripts('/static/scripts/posts/community_get.js?ver1');
            loadScripts('/static/scripts/posts/user_post.js?ver1');
            loadScripts('/static/scripts/posts/community_post.js?ver1');

            loadScripts('/static/scripts/user/settings.js?ver1');
            loadScripts('/static/scripts/user/connect.js?ver1');
            loadScripts('/static/scripts/user/chat.js?ver1');

            loadScripts('/static/scripts/video/user_get.js?ver1');
            loadScripts('/static/scripts/video/user_post.js?ver1');
            loadScripts('/static/scripts/video/community_post.js?ver1');

            loadScripts('/static/scripts/survey/get.js?ver1');
            loadScripts('/static/scripts/survey/post.js?ver1');

            loadScripts('/static/scripts/community/get.js?ver1');
            loadScripts('/static/scripts/community/post.js?ver1');
        }
      }
      ajax_link.send();
    }
  }

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

check_first_load();


