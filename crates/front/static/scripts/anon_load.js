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
        }
      }
      ajax_link.send();
    }
  }

  check_first_load();
