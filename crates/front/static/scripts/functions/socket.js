!function(e){if("object"==typeof exports&&"undefined"!=typeof module)module.exports=e();else if("function"==typeof define&&define.amd)define([],e);else{("undefined"!=typeof window?window:"undefined"!=typeof global?global:"undefined"!=typeof self?self:this).channels=e()}}(function(){return function i(c,s,a){function u(o,e){if(!s[o]){if(!c[o]){var n="function"==typeof require&&require;if(!e&&n)return n(o,!0);if(f)return f(o,!0);var t=new Error("Cannot find module '"+o+"'");throw t.code="MODULE_NOT_FOUND",t}var r=s[o]={exports:{}};c[o][0].call(r.exports,function(e){var n=c[o][1][e];return u(n||e)},r,r.exports,i,c,s,a)}return s[o].exports}for(var f="function"==typeof require&&require,e=0;e<a.length;e++)u(a[e]);return u}({1:[function(e,n,o){"use strict";function f(e){return e&&2===e.CLOSING}function h(){return{constructor:"undefined"!=typeof WebSocket&&f(WebSocket)?WebSocket:null,maxReconnectionDelay:1e4,minReconnectionDelay:1500,reconnectionDelayGrowFactor:1.3,connectionTimeout:4e3,maxRetries:1/0,debug:!1}}function b(n,e,o){Object.defineProperty(e,o,{get:function(){return n[o]},set:function(e){n[o]=e},enumerable:!0,configurable:!0})}function w(e){return e.minReconnectionDelay+Math.random()*e.minReconnectionDelay}var E=["onopen","onclose","onmessage","onerror"],k=function(o,t,n){var l,r,i=this;void 0===n&&(n={});var d=0,c=0,v=!0,y=null,p={};if(!(this instanceof k))throw new TypeError("Failed to construct 'ReconnectingWebSocket': Please use the 'new' operator");var s=h();if(Object.keys(s).filter(function(e){return n.hasOwnProperty(e)}).forEach(function(e){return s[e]=n[e]}),!f(s.constructor))throw new TypeError("Invalid WebSocket constructor. Set `options.constructor`");function a(e,o){return setTimeout(function(){var n=new Error(o);n.code=e,Array.isArray(p.error)&&p.error.forEach(function(e){return(0,e[0])(n)}),l.onerror&&l.onerror(n)},0)}function m(){u("close"),u("retries count:",++c),c>s.maxRetries?a("EHOSTDOWN","Too many failed connection attempts"):(d=d?function(e,n){var o=n*e.reconnectionDelayGrowFactor;return o>e.maxReconnectionDelay?e.maxReconnectionDelay:o}(s,d):w(s),u("reconnectDelay:",d),v&&setTimeout(e,d))}var u=s.debug?function(){for(var e=[],n=0;n<arguments.length;n++)e[n-0]=arguments[n];return console.log.apply(console,["RWS:"].concat(e))}:function(){},e=function(){u("connect");var e=l;for(var n in l=new s.constructor(o,t),r=setTimeout(function(){u("timeout"),l.close(),a("ETIMEDOUT","Connection timeout")},s.connectionTimeout),u("bypass properties"),l)["addEventListener","removeEventListener","close","send"].indexOf(n)<0&&b(l,i,n);l.addEventListener("open",function(){clearTimeout(r),u("open"),d=w(s),u("reconnectDelay:",d),c=0}),l.addEventListener("close",m),function(r,n,e){Object.keys(e).forEach(function(t){e[t].forEach(function(e){var n=e[0],o=e[1];r.addEventListener(t,n,o)})}),n&&E.forEach(function(e){r[e]=n[e]})}(l,e,p),l.onclose=l.onclose||y,y=null};u("init"),e(),this.close=function(e,n,o){void 0===e&&(e=1e3),void 0===n&&(n="");var t=void 0===o?{}:o,r=t.keepClosed,i=void 0!==r&&r,c=t.fastClose,s=void 0===c||c,a=t.delay,u=void 0===a?0:a;if(u&&(d=u),v=!i,l.close(e,n),s){var f={code:e,reason:n,wasClean:!0};m(),l.removeEventListener("close",m),Array.isArray(p.close)&&p.close.forEach(function(e){var n=e[0],o=e[1];n(f),l.removeEventListener("close",n,o)}),l.onclose&&(y=l.onclose,l.onclose(f),l.onclose=null)}},this.send=function(e){l.send(e)},this.addEventListener=function(e,n,o){Array.isArray(p[e])?p[e].some(function(e){return e[0]===n})||p[e].push([n,o]):p[e]=[[n,o]],l.addEventListener(e,n,o)},this.removeEventListener=function(e,n,o){Array.isArray(p[e])&&(p[e]=p[e].filter(function(e){return e[0]!==n})),l.removeEventListener(e,n,o)}};n.exports=k},{}],2:[function(e,n,o){"use strict";Object.defineProperty(o,"__esModule",{value:!0}),o.WebSocketBridge=void 0;var t=Object.assign||function(e){for(var n=1;n<arguments.length;n++){var o=arguments[n];for(var t in o)Object.prototype.hasOwnProperty.call(o,t)&&(e[t]=o[t])}return e},r=function(e,n,o){return n&&i(e.prototype,n),o&&i(e,o),e};function i(e,n){for(var o=0;o<n.length;o++){var t=n[o];t.enumerable=t.enumerable||!1,t.configurable=!0,"value"in t&&(t.writable=!0),Object.defineProperty(e,t.key,t)}}var c,s=e("reconnecting-websocket"),a=(c=s)&&c.__esModule?c:{default:c};var u=(r(f,[{key:"connect",value:function(e,n,o){var t=void 0,r=("https:"===window.location.protocol?"wss":"ws")+"://"+window.location.host;t=void 0===e?r:"/"==e[0]?r+e:e,this.socket=new a.default(t,n,o)}},{key:"listen",value:function(e){var i=this;this.default_cb=e,this.socket.onmessage=function(e){var n=JSON.parse(e.data),o=void 0,t=void 0;if(void 0!==n.stream){o=n.payload,t=n.stream;var r=i.streams[t];r&&r(o,t)}else o=n,t=null,i.default_cb&&i.default_cb(o,t)}}},{key:"demultiplex",value:function(e,n){this.streams[e]=n}},{key:"send",value:function(e){this.socket.send(JSON.stringify(e))}},{key:"stream",value:function(o){var t=this;return{send:function(e){var n={stream:o,payload:e};t.socket.send(JSON.stringify(n))}}}}]),f);function f(e){!function(e,n){if(!(e instanceof n))throw new TypeError("Cannot call a class as a function")}(this,f),this.socket=null,this.streams={},this.default_cb=null,this.options=t({},e)}o.WebSocketBridge=u},{"reconnecting-websocket":1}]},{},[2])(2)});



function case_user_notify() {
  console.log('заявки, дружба, приглашения...');
  new Audio('/static/audio/apple/stargaze.mp3').play();
};
function case_user_chat_typed(pk, first_name) {
  if (document.body.querySelector(".chat_container")) {
    if (pk == document.body.querySelector(".chat_container").getAttribute("chat-pk")) {
      console.log('пользователь пишет...');
      typed_box = document.body.querySelector(".user_typed_box");
      typed_box.innerHTML = first_name + " набирает сообщение..."
      setTimeout(function(){
        typed_box.innerHTML = "";
    }, 1000)
    }
  } if (document.body.querySelector(".chat_list_container")) {
    list = document.body.querySelector(".chat_list_container");
    chat = list.querySelector('[data-pk=' + '"' + pk + '"' + ']');
    p = chat.querySelector("p");
    if (!p.nextElementSibling.innerHTML) {
      p.style.display = "none";
      p.nextElementSibling.innerHTML = first_name + " набирает сообщение...";
      setTimeout(function(){
        p.nextElementSibling.innerHTML = "";
        p.style.display = "unset";
      }, 1000);
    } else {
      p.style.display = "unset";
    }
  }
};
function case_user_chat_read(pk) {
  if (document.body.querySelector(".chat_container")) {
    if (pk == document.body.querySelector(".chat_container").getAttribute("chat-pk")) {
      console.log('пользователь прочитал сообщения...');
      box = document.body.querySelector(".chat_container");
      list = box.querySelectorAll(".message");
      for (var i = 0; i < list.length; i++){
        list[i].classList.remove("bg-light-secondary")
      }
    }}
    else if (document.body.querySelector(".chat_list_container")) {
    list = document.body.querySelector(".chat_list_container");
    chat = list.querySelector('[data-pk=' + '"' + pk + '"' + ']');
    chat.querySelector("p").classList.remove("bg-light-secondary");
  }
};
function case_u_post_notify(uuid) {
    console.log('Реакции на записи');
    try{
    if (document.body.querySelector( '[data-uuid=' + '"' + uuid + '"' + ']' )){
      post_update_votes(document.body.querySelector( '[data-uuid=' + '"' + uuid + '"' + ']' ), uuid);
    }}catch{null};
    new Audio('/static/audio/apple/nota.mp3').play();
};
function case_c_post_notify(uuid) {
    console.log('Реакции на записи сообщества');
    try{
    if (document.body.querySelector( '[data-uuid=' + '"' + uuid + '"' + ']' )){
      post_update_votes(document.body.querySelector( '[data-uuid=' + '"' + uuid + '"' + ']' ), uuid);
    }}catch{null};
    new Audio('/static/audio/apple/nota.mp3').play();
};
function case_u_post_repost_notify(uuid) {
    console.log('Репосты на записи');
    try{
    if (document.body.querySelector( '[data-uuid=' + '"' + uuid + '"' + ']' )){
      post = document.body.querySelector( '[data-uuid=' + '"' + uuid + '"' + ']' );
      block = post.querySelector(".repost_count");
      block.innerHTML ? (count = block.innerHTML.replace(/\s+/g, ''), count = count*1) : count = 0;
      count += 1;
      block.innerHTML = count;
    }}catch{null};
    new Audio('/static/audio/apple/nota.mp3').play();
};
function case_c_post_repost_notify(uuid) {
    console.log('Репосты на записи сообщества');
    try{
    if (document.body.querySelector( '[data-uuid=' + '"' + uuid + '"' + ']' )){
      post = document.body.querySelector( '[data-uuid=' + '"' + uuid + '"' + ']' );
      block = post.querySelector(".repost_count");
      block.innerHTML ? (count = block.innerHTML.replace(/\s+/g, ''), count = count*1) : count = 0;
      count += 1;
      block.innerHTML = count;
    }}catch{null};
    new Audio('/static/audio/apple/nota.mp3').play();
};
function case_u_photo_notify(uuid) {
    console.log('Реакции на фото');
    new Audio('/static/audio/apple/nota.mp3').play();
};
function case_c_photo_notify(uuid) {
    console.log('Реакции на фото сообщества');
    new Audio('/static/audio/apple/nota.mp3').play();
};
function case_u_photo_repost_notify(uuid) {
    console.log('Репосты записи');
    new Audio('/static/audio/apple/nota.mp3').play();
};
function case_c_photo_repost_notify(uuid) {
    console.log('Репосты записи сообщества');
    new Audio('/static/audio/apple/nota.mp3').play();
};

function plus_1_badge_message() {
  chats = document.body.querySelector(".new_unread_chats");
  if (chats.querySelector(".tab_badge_left_menu")) {
    tab_badge = chats.querySelector(".tab_badge_left_menu");
    count = tab_badge.innerHTML.replace(/\s+/g, '');
    count = count*1
  } else {
    tab_badge = document.createElement("span");
    tab_badge.classList.add("border", "tab_badge_left_menu");
    chats.append(tab_badge);
    count = 0;
  }
  count += 1;
  tab_badge.innerHTML = "";
  tab_badge.innerHTML = count;
}

function case_u_post_create(uuid) {
  if (document.body.querySelector(".pk_saver") && document.body.querySelector(".pk_saver").getAttribute('data-pk') !=request_user_id) {
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  link_.open('GET', "/posts/user/load_post/" + request_user_id + "/" + uuid + "/", true);
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
          lenta = document.body.querySelector('.post_stream');
          elem = link_.responseText;
          new_post = document.createElement("span");
          new_post.innerHTML = elem;
          lenta.prepend(new_post);
          document.body.querySelector(".items_empty") ? document.body.querySelector(".items_empty").style.display = "none" : null}}
  link_.send()
}};

function case_u_message_create(chat_id, message_uuid, beep) {
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');

  if (document.body.querySelector(".chat_list_container")) {
    // если в момент получения нового сообщения получатель на странице списка чатов
    console.log("Вы на странице сообщений");
  link_.open('GET', "/chat/user_progs/load_message/" + message_uuid + "/", true);
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {

          lenta = document.body.querySelector('.is_paginate');
          elem = link_.responseText;
          new_post = document.createElement("span");
          new_post.innerHTML = elem;

          tab_badge = new_post.querySelector(".tab_badge");
          tab_badge_count = tab_badge.innerHTML.replace(/\s+/g, '');
          tab_badge_count = tab_badge_count*1;
          if (tab_badge_count == 1) {
            plus_1_badge_message()
          };
          lenta.querySelector('[data-pk=' + '"' + chat_id + '"' + ']') ? (li = lenta.querySelector('[data-pk=' + '"' + chat_id + '"' + ']'), li.innerHTML = new_post.innerHTML)
          : lenta.prepend(new_post);
          document.body.querySelector(".items_empty") ? document.body.querySelector(".items_empty").style.display = "none" : null}}
  link_.send()
}
  else if (document.body.querySelector(".chat_container") && document.body.querySelector(".chat_container").getAttribute('chat-pk') == chat_id) {
    // если в момент получения нового сообщения получатель на странице чата, в котором ему написалм
    console.log("Вы на странице чата");
    link_.open('GET', "/chat/user_progs/load_chat_message/" + message_uuid + "/", true);
    link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

    link_.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
        lenta = document.body.querySelector('.is_paginate');
        elem = link_.responseText;
        new_post = document.createElement("span");
        new_post.innerHTML = elem;
        lenta.append(new_post);
        window.scrollTo( 0, 3000 );
        document.body.querySelector(".items_empty") ? document.body.querySelector(".items_empty").style.display = "none" : null}}
  link_.send()
} else {
  // если в момент получения нового сообщения получатель не на странице чата или списка чатов
  console.log("Вы не в сообщениях");
  plus_1_badge_message();
  };
  // добавим единичку к счетчику на панели, а если пользователь на странице чата
  // то добавим программу, которая прочитает сообщение и на единичку убавит счетчик на панели

  if (beep) {
    audio = new Audio('/static/audio/apple/message.mp3');
    audio.volume = 0.4;
    audio.play()
  }
};


request_user_id = document.body.querySelector(".userpic").getAttribute("data-id");
notify = document.body.querySelector(".new_unread_notify");
notify.querySelector(".tab_badge") ? (notify_count = notify.querySelector(".tab_badge").innerHTML.replace(/\s+/g, ''), notify_count = notify_count*1) : notify_count = 0;
tab_span = document.createElement("span");
tab_span.classList.add("tab_badge", "badge-success");

ws_scheme = window.location.protocol == "https:" ? "wss" : "ws";
ws_path = ws_scheme + '://' + window.location.host + ":8443/notify/";
webSocket = new channels.WebSocketBridge();
webSocket.connect(ws_path);

webSocket.socket.onmessage = function(e){ console.log(e.data); };
webSocket.socket.onopen = function () {console.log("Соединение установлено!")};
webSocket.socket.onclose = function () {console.log("Соединение прервано...")};


webSocket.listen(function (event) {
  switch (event.key) {
      case "notification":
        if (event.recipient_id == request_user_id){
          if (event.name == "user_notify"){ case_user_notify() }
          else if (event.name == "u_post_notify"){ case_u_post_notify(event.post_id) }
          else if (event.name == "u_post_repost_notify"){ case_u_post_repost_notify(event.post_id) }
          else if (event.name == "c_post_notify"){ case_c_post_notify(event.post_id) }
          else if (event.name == "c_post_repost_notify"){ case_c_post_repost_notify(event.post_id) }

          else if (event.name == "u_photo_notify"){ case_u_photo_notify(event.photo_id) }
          else if (event.name == "u_photo_repost_notify"){ case_u_photo_repost_notify(event.photo_id) }
          else if (event.name == "c_photo_notify"){ case_c_photo_notify(event.photo_id) }
          else if (event.name == "c_photo_repost_notify"){ case_c_photo_repost_notify(event.photo_id) }

          // добавляем единичку к общему счетчику уведомлений
          notify_count += 1;
          tab_span.innerHTML = notify_count;
          notify.innerHTML = "";
          notify.append(tab_span);

          // если мы на странице блоков уведомлений, то добавляем единичку к блоку, в которое поступает уведомление
          // если есть event.community_id, ищем его по data-pk == event.community_id, иначе добавляем к блоку пользователя
          if (document.body.querySelector(".user_notify_block") && !event.community_id){
            user_notify_block = document.body.querySelector(".user_notify_block");
            user_notify_block.querySelector(".tab_badge") ? (_count = user_notify_block.querySelector(".tab_badge").innerHTML.replace(/\s+/g, ''),_count = _count*1,_count += 1,user_notify_block.querySelector(".tab_badge").innerHTML = _count)
            : document.body.querySelector(".user_notify_counter").innerHTML = "<span class='tab_badge badge-success' style='font-size: 60%;'>1</span>"
          } else if (document.body.querySelector(".community_notify_block") && event.community_id){
            community_notify_block = document.body.querySelector( '[data-pk=' + '"' + event.community_id + '"' + ']' );
            community_notify_block.querySelector(".tab_badge") ? (
                                                            _count = community_notify_block.querySelector(".tab_badge").innerHTML.replace(/\s+/g, ''),
                                                            _count = _count*1,
                                                            _count += 1,
                                                            community_notify_block.querySelector(".tab_badge").innerHTML = _count
                                                           )
           : community_notify_block.querySelector(".community_notify_counter").innerHTML = "<span class='tab_badge badge-success' style='font-size: 60%;'>1</span>"
          }

        }
        break;
event.creator_id != request_user_id
      case "create_item":
        if (event.creator_id != request_user_id){
          console.log("отрисовка созданных элементов для пользователей на странице");
          console.log(event.recipient_ids)
          if (event.name == "u_post_create"){case_u_post_create(event.post_id)}
        }
        break;
    case "message":
      // где уведы со звуком, там посылаем по одному, проверяя включен ли звук.
      // те, что не требуют звука, посылаются скопом. И проверяется, есть ли среди списка
      // создатель события, чтобы и ему не показывать, он то знает.
        if (event.name == "u_message_create"){
          if (event.recipient_id == request_user_id ){
            case_u_message_create(event.chat_id, event.message_id, event.beep)
          }
        }
        else if (event.name == "u_message_typed"){
          if (event.recipient_id != request_user_id){
            case_user_chat_typed(event.chat_id, event.user_name)
          }
        }
        else if (event.name == "u_message_read"){
          if (event.recipient_id != request_user_id){
            case_user_chat_read(event.chat_id)
          }
        }
      break;

    default:
      console.log('error: ', event);
      break;
  }
});
