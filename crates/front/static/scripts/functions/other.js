function on(elSelector,eventName,selector,fn) {var element = document.querySelector(elSelector);element.addEventListener(eventName, function(event) {var possibleTargets = element.querySelectorAll(selector);var target = event.target;for (var i = 0, l = possibleTargets.length; i < l; i++) {var el = target;var p = possibleTargets[i];while(el && el !== element) {if (el === p) {return fn.call(p, event);}el = el.parentNode;}}});};

function format_text(text) {
  text.innerHTML = text.innerHTML.replace(/<(?!img)(?!br)\/?[a-z][^>]*(>|$)/gi, "").replace(/\&nbsp;/g, ' ');
  brs = text.querySelectorAll("br");
  img = text.querySelectorAll("img")
  for (var i = 0; i < brs.length; i++){
    brs[i].removeAttribute("style")
  };
  for (var i = 0; i < img.length; i++){
    img[i].removeAttribute("style")
  }
  return text
};

function get_count_ru(count, word_1, word_2, word_3) {
  a = count % 10, b = count % 100;
  if (a == 1 && b != 11){
    return count + " " + word_1
  }
  else if (a >= 2 && a <= 4 && (b < 10 || b >= 20)) {
    return count + " " + word_2
  }
  else {
    return count + " " + word_3
  };
};
function get_count_ru_alt(count, word_1, word_2, word_3) {
  a = count % 10, b = count % 100;
  if (a == 1 && b != 11){
    return " " + word_1
  }
  else if (a >= 2 && a <= 4 && (b < 10 || b >= 20)) {
    return " " + word_2
  }
  else {
    return " " + word_3
  };
};

function phone_check() {
    if (document.getElementById('phone').value.length > 9)
        document.getElementById("phone_send").removeAttribute('disabled');
    else
        document.getElementById("phone_send").setAttribute("disabled", "true");
};
function post_include_exclude_users(form, url) {
  form.querySelector(".form_btn").disabled = true;
  form_data = new FormData(form);

    var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      ajax_link.open( 'POST', url, true );
      ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
      ajax_link.onreadystatechange = function () {
        if ( this.readyState == 4 && this.status == 200 ) {
            collector = document.body.querySelector(".collector_active");
            users_block = form.querySelector(".card-header");
            users_list = users_block.querySelectorAll(".custom_color");
            final_list = ": ";
            for (var i = 0; i < users_list.length; i++){
              a = users_list[i].querySelector("a");
              final_list += '<a href="' + a.getAttribute("href") + '" target="_blank">' + a.innerHTML + '</a>'
              final_list += '<input type="hidden" name="' + collector.nextElementSibling.getAttribute("data-action") + '_users" value="' + users_list[i].getAttribute("data-pk") + '" />'
            };
            collector.innerHTML = final_list;
            form.classList.remove("cool_private_form");
            close_work_fullscreen();
        }
      };
      ajax_link.send(form_data);
};

function setEndOfContenteditable(contentEditableElement) {
    var range,selection;
    if(document.createRange) {
        range = document.createRange();
        range.selectNodeContents(contentEditableElement);
        range.collapse(false);
        selection = window.getSelection();
        selection.removeAllRanges();
        selection.addRange(range);
    }
    else if(document.selection) {
        range = document.body.createTextRange();
        range.moveToElementText(contentEditableElement);
        range.collapse(false);
        range.select();
    }
};

function fullscreen_resize() {
  container = document.body.querySelector("#fullscreens_container");
  if (container.innerHTML) {
    fullscreen = container.querySelector(".card_fullscreen");
    loader = fullscreen.querySelector("#fullscreen_loader");

    height = loader.scrollHeight*1 + 30;
    if (height < 500) {
      fullscreen.style.height = height + "px";
      loader.style.overflowY = "unset";

      _height = (window.innerHeight - height - 50) / 2;
      fullscreen.style.top = _height + "px";
      prev_next_height = _height*1 + 50 + "px";
      try {loader.querySelector(".prev_item").style.top = "-" + prev_next_height}catch {null};
      try {loader.querySelector(".next_item").style.top = "-" + prev_next_height}catch {null}
    } else {
      fullscreen.style.height = "100%";
      fullscreen.style.top = "15px";
      loader.style.overflowY = "auto";
      try {loader.querySelector(".prev_item").style.top = "0"}catch {null};
      try {loader.querySelector(".next_item").style.top = "0"}catch {null}
    };
  }
};

on('body', 'click', '.this_fullscreen_hide', function() {
  close_fullscreen()
});
on('body', 'click', '.this_mob_fullscreen_hide', function() {
  close_fullscreen()
});

on('body', 'click', '.body_overlay', function() {
  close_fullscreen()
});

function check_message_form_btn() {
  input = document.body.querySelector(".message_text");
  btn_block = document.body.querySelector(".input_group_message");
  if (input.innerHTML.replace(/<[^>]*(>|$)|&nbsp;|&zwnj;|&raquo;|&laquo;|&gt;/g,'').trim() == "" && document.body.querySelector(".files_0")){
     btn_block.querySelector("#voice_start_btn").style.display = "block";
     btn_block.querySelector("#message_post_btn").style.display = "none";
  } else {
    btn_block.querySelector("#voice_start_btn").style.display = "none";
    btn_block.querySelector("#message_post_btn").style.display = "block";
  }
};
function show_message_form_send_btn() {
  document.body.querySelector("#voice_start_btn").style.display = "none";
  document.body.querySelector("#message_post_btn").style.display = "block";
};
function show_message_form_voice_btn() {
  document.body.querySelector("#voice_start_btn").style.display = "block";
  document.body.querySelector("#message_post_btn").style.display = "none";
};

function remove_class_timeout(el) {
  setTimeout(function(){
    el.classList.remove("draft_created");
}, 3000)
};

function remove_item_and_show_restore_block(item, url, _class, title) {
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    ajax_link.open( 'GET', url + item.getAttribute("data-pk") + "/", true );
		ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    ajax_link.onreadystatechange = function () {
      if ( this.readyState == 4 && this.status == 200 ) {
        p = document.createElement("div");
        p.classList.add("media", "p-1");
        p.innerHTML = "<span style='width:100%' class='" + _class + " pointer border p-2' data-pk='" + item.getAttribute("data-pk") + "'>" + title + ". <span class='underline'>Восстановить</span></span>";
        item.parentElement.insertBefore(p, item), item.style.display = "none";
        item.classList.remove("custom_color");
      }
    }
    ajax_link.send();
};

function send_change_items(array, link) {
  // функция передает новый порядок элементов, принимая их массив и ссылку, по которой нужно отправить изменения.
  len = array.length + 1;
  token = document.body.getAttribute("data-csrf");
  post_array = []
  for (var i=0; i<array.length; i++) {
    count = len -= 1;
    post_array.push({key:array[i].getAttribute("data-pk"),value: count});
  };
  var xmlhttp = new XMLHttpRequest();
  xmlhttp.open("POST", link);
  xmlhttp.setRequestHeader("Content-Type", "application/json;charset=UTF-8");
  xmlhttp.setRequestHeader('X-CSRFToken', token);
  xmlhttp.send(JSON.stringify(post_array));
};

function profile_list_block_attach(_this, url, actions_class) {
  var request = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  request.open( 'GET', "/users/load" + url + _this.parentElement.parentElement.parentElement.getAttribute("data-uuid") + "/", true );
  request.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  request.onreadystatechange = function () {
    if ( request.readyState == 4 && request.status == 200 ){
        elem_ = document.createElement('span');
        elem_.innerHTML = request.responseText;
       _this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.nextElementSibling.innerHTML = elem_.querySelector(".load_attach_block").innerHTML;
       class_to_add = _this.parentElement.parentElement.parentElement.parentElement.parentElement.querySelectorAll(".list_toggle")
       for (var i = 0; i < class_to_add.length; i++) {
         class_to_add[i].classList.add(actions_class, "pointer");
         class_to_add[i].parentElement.parentElement.parentElement.classList.replace("active_border", "border");
       };
       parent = _this.parentElement.parentElement.parentElement;
       parent.querySelector(".list_svg")? parent.querySelector(".list_svg").classList.remove(actions_class, "pointer") : null;
       parent.querySelector(".list_name")? parent.querySelector(".list_name").classList.remove(actions_class, "pointer") : null;
       parent.classList.replace("border", "active_border");

       if (elem_.querySelector(".is_block_paginate")) {
         lenta = elem_.querySelector('.is_block_paginate');
         link = lenta.getAttribute("data-link");
         scrolled(document.body.querySelector('.is_block_paginate'))
       };
    }};
    request.send( null );
};

function check_span1(span1, uuid, response) {
  if (span1.classList.contains(uuid)){
    document.body.querySelector(".is_paginate").insertAdjacentHTML('afterBegin', response)
  }
};

function get_preview(response, type) {
  if (document.body.querySelector(".current_file_dropdown")){
    if (type == "doc") {
      pk = response.querySelector(".span_btn").getAttribute("data-pk");
      media_body = response.querySelector(".media-body");
      media_body.querySelector(".span_btn").remove(); media_body.querySelector(".small").remove();
      doc_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, media_body, pk)
    } else if (type == "track") {
      response.querySelector(".span_btn").remove(); response.querySelector(".small").remove();
      track_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, response)
    }
  } else if (document.body.querySelector(".attach_block")){
    if (type == "doc") {
      pk = response.querySelector(".span_btn").getAttribute("data-pk");
      media_body = response.querySelector(".media-body");
      media_body.querySelector(".span_btn").remove(); media_body.querySelector(".small").remove();
      doc_post_attach(document.body.querySelector(".attach_block"), response.querySelector(".media-body"), pk)
    }
  } else if (document.body.querySelector(".message_attach_block")){
    if (type == "doc") {
      pk = response.querySelector(".span_btn").getAttribute("data-pk");
      media_body = response.querySelector(".media-body");
      media_body.querySelector(".span_btn").remove(); media_body.querySelector(".small").remove();
      doc_message_attach(document.body.querySelector(".message_attach_block"), response.querySelector(".media-body"), pk)
  }
  };
};

function post_and_load_object_page(form, url_post, url_1, url_2, stat_type) {
    form_data = new FormData(form);
    pk = document.body.querySelector(".pk_saver").getAttribute("data-pk");
    var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    ajax_link.open('POST', url_post + pk + "/", true);
    ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    ajax_link.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            elem_ = document.createElement('span');
            elem_.innerHTML = ajax_link.responseText;
            ajax = elem_.querySelector("#reload_block");
            rtr = document.getElementById('ajax');

            prev_container = rtr.querySelector(".main-container");
            next_container = ajax.querySelector(".main-container");
            add_list_in_all_stat("created_" + next_container.getAttribute("data-type"),next_container.getAttribute("data-pk"),prev_container.getAttribute("data-type"),prev_container.getAttribute("data-pk"))

            rtr.innerHTML = ajax.innerHTML;
            window.scrollTo(0, 0);
            document.title = elem_.querySelector('title').innerHTML;
            uuid = rtr.querySelector(".uuid_saver").getAttribute("data-uuid");
            window.history.pushState(null, "vfgffgfgf", url_1 + pk + url_2 + uuid + '/');
            get_document_opacity_1();
            add_list_in_all_stat(stat_type,pk,prev_container.getAttribute("data-type"),prev_container.getAttribute("data-pk"))
        }
    }
    ajax_link.send(form_data)
};

function edit_and_load_object_page(form, url_post, url_1, url_2) {
    form_data = new FormData(form);
    pk = form.getAttribute("data-pk");
    uuid = form.getAttribute("data-uuid");
    var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    ajax_link.open('POST', url_post + pk + "/" + uuid + "/", true);
    ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    ajax_link.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            elem_ = document.createElement('span');
            elem_.innerHTML = ajax_link.responseText;
            ajax = elem_.querySelector("#reload_block");
            rtr = document.getElementById('ajax');
            rtr.innerHTML = ajax.innerHTML;
            window.scrollTo(0, 0);
            document.title = elem_.querySelector('title').innerHTML;
            uuid = rtr.querySelector(".pk_saver").getAttribute("data-uuid");
            window.history.pushState(null, "vfgffgfgf", url_1 + pk + url_2 + uuid + '/')
        }
    }
    ajax_link.send(form_data)
};

function send_form_and_toast(url, form, toast) {
    form_data = new FormData(form);
    ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    ajax_link.open('POST', url, true);
    ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    ajax_link.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            toast_info(toast);
        }
    }
    ajax_link.send(form_data);
};

function send_form_and_close_window(url, form) {
    form_data = new FormData(form);
    ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    ajax_link.open('POST', url, true);
    ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    ajax_link.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            close_work_fullscreen();
        }
    }
    ajax_link.send(form_data);
};

function get_with_pk_and_reload(url) {
    pk = document.body.querySelector(".pk_saver").getAttribute("data-pk");
    link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    link_.open('GET', url + pk + "/", true);
    link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    link_.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            this_page_reload(document.location.href);
        }
    };
    link_.send();
};

function post_with_pk_and_reload(parent, url) {
    pk = document.body.querySelector(".pk_saver").getAttribute("data-pk");
    form_data = new FormData(parent);

    link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    link_.open('POST', url + pk + "/", true);
    link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    link_.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            this_page_reload(document.location.href);
        }
    };
    link_.send(form_data);
};

function send_change(span, _link, new_class, html) {
    parent = span.parentElement;
    item = span.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
    item.getAttribute("data-pk") ? pk = item.getAttribute("data-pk") : pk = item.getAttribute("good-pk");
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    link.open('GET', _link + pk + "/", true);
    link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    link.onreadystatechange = function() {
        if (link.readyState == 4 && link.status == 200) {
            new_span = document.createElement("span");
            new_span.classList.add(new_class, "dropdown-item");
            new_span.innerHTML = html;
            parent.innerHTML = "";
            parent.append(new_span)
        }
    };
    link.send(null)
};

function chat_send_change(span, _link, new_class, html) {
    parent = span.parentElement;
    item = parent.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    link.open('GET', _link + item.getAttribute("chat-pk") + "/", true);
    link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    link.onreadystatechange = function() {
        if (link.readyState == 4 && link.status == 200) {
            new_span = document.createElement("span");
            new_span.classList.add(new_class, "dropdown-item");
            new_span.innerHTML = html;
            parent.innerHTML = "";
            parent.append(new_span)
        }
    };
    link.send(null)
};

function mob_send_change(span, _link, new_class, html) {
    parent = span.parentElement;
    item = span.parentElement.parentElement.parentElement.parentElement.parentElement;
    item.getAttribute("data-pk") ? pk = item.getAttribute("data-pk") : pk = item.getAttribute("good-pk"); link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    link.open('GET', _link + pk + "/", true);
    link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    link.onreadystatechange = function() {
        if (link.readyState == 4 && link.status == 200) {
            new_span = document.createElement("span");
            new_span.classList.add(new_class, "dropdown-item");
            new_span.innerHTML = html;
            parent.innerHTML = "";
            parent.append(new_span)
        }
    };
    link.send(null)
};

function send_good_change(span, _link, new_class, html) {
    parent = span.parentElement;
    item = span.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
    pk = item.getAttribute("good-pk");
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    link.open('GET', _link + pk + "/", true);
    link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    link.onreadystatechange = function() {
        if (link.readyState == 4 && link.status == 200) {
            new_span = document.createElement("span");
            new_span.classList.add(new_class, "dropdown-item");
            new_span.innerHTML = html;
            parent.innerHTML = "";
            parent.append(new_span)
        }
    };
    link.send(null)
};
function send_mob_good_change(span, _link, new_class, html) {
    parent = span.parentElement;
    item = parent.parentElement.parentElement.parentElement.parentElement;
    pk = item.getAttribute("good-pk");
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    link.open('GET', _link + pk + "/", true);
    link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    link.onreadystatechange = function() {
        if (link.readyState == 4 && link.status == 200) {
            new_span = document.createElement("span");
            new_span.classList.add(new_class, "dropdown-item");
            new_span.innerHTML = html;
            parent.innerHTML = "";
            parent.append(new_span)
        }
    };
    link.send(null)
};

function send_photo_change(span, _link, new_class, html) {
    parent = span.parentElement;
    item = span.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
    photo_pk = item.getAttribute("data-pk");
    pk = item.getAttribute("owner-pk");
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    link.open('GET', _link + pk + "/" + photo_pk + "/", true);
    link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    link.onreadystatechange = function() {
        if (link.readyState == 4 && link.status == 200) {
            new_span = document.createElement("a");
            new_span.classList.add(new_class);
            new_span.innerHTML = html;
            parent.innerHTML = "";
            parent.append(new_span)
        }
    };
    link.send(null)
};

class ToastManager {
    constructor() {
        this.id = 0;
        this.toasts = [];
        this.icons = {
            'SUCCESS': "",
            'ERROR': '',
            'INFO': '',
            'WARNING': '',
        };
        var body = document.querySelector('#ajax');
        this.toastsContainer = document.createElement('div');
        this.toastsContainer.classList.add('toasts', 'border-0');
        body.appendChild(this.toastsContainer)
    }
    showSuccess(message) {
        return this._showToast(message, 'SUCCESS')
    }
    showError(message) {
        return this._showToast(message, 'ERROR')
    }
    showInfo(message) {
        return this._showToast(message, 'INFO')
    }
    showWarning(message) {
        return this._showToast(message, 'WARNING')
    }
    _showToast(message, toastType) {
        var newId = this.id + 1;
        var newToast = document.createElement('div');
        newToast.style.display = 'inline-block';
        newToast.classList.add(toastType.toLowerCase());
        newToast.classList.add('toast');
        newToast.innerHTML = `<progress max="100"value="0"></progress><h3>${message}</h3>`;
        var newToastObject = {
            id: newId,
            message,
            type: toastType,
            timeout: 4000,
            progressElement: newToast.querySelector('progress'),
            counter: 0,
            timer: setInterval(() => {
                newToastObject.counter += 1000 / newToastObject.timeout;
                newToastObject.progressElement.value = newToastObject.counter.toString();
                if (newToastObject.counter >= 100) {
                    newToast.parentElement.remove();
                    clearInterval(newToastObject.timer);
                    this.toasts = this.toasts.filter((toast) => {
                        return toast.id === newToastObject.id
                    })
                }
            }, 10)
        }
        newToast.addEventListener('click', () => {
            newToast.parentElement.remove();
            clearInterval(newToastObject.timer);
            this.toasts = this.toasts.filter((toast) => {
                return toast.id === newToastObject.id
            })
        });
        this.toasts.push(newToastObject);
        this.toastsContainer.appendChild(newToast);
        return this.id++
    }
};

function toast_success(text) {
  toasts = document.body.querySelectorAll(".toasts")
  for (var i = 0; i < toasts.length; i++){
    if (toasts[i].querySelector("h3").innerHTML = text) {
      return
    }
  };
  new ToastManager().showSuccess(text)
};

function toast_error(text) {
  toasts = document.body.querySelectorAll(".toasts")
  for (var i = 0; i < toasts.length; i++){
    if (toasts[i].querySelector("h3").innerHTML = text) {
      return
    }
  };
  new ToastManager().showError(text)
};

function toast_info(text) {
  toasts = document.body.querySelectorAll(".toasts")
  for (var i = 0; i < toasts.length; i++){
    if (toasts[i].querySelector("h3").innerHTML = text) {
      return
    }
  };
  new ToastManager().showInfo(text)
};

function toast_warning(text) {
  toasts = document.body.querySelectorAll(".toasts")
  for (var i = 0; i < toasts.length; i++){
    if (toasts[i].querySelector("h3").innerHTML = text) {
      return
    }
  };
  new ToastManager().showWarning(text)
};

function elementInViewport(el) {
    var bounds = el.getBoundingClientRect();
    return ((bounds.top + bounds.height > 0) && (window.innerHeight - bounds.top > 0));
};

function send_comment(form, block, link) {
  text_val = form.querySelector(".smile_supported");
  _val = format_text(text_val);
  _text = _val.innerHTML;
  if (_text.replace(/<(?!br)(?!img)\/?[a-z][^>]*(>|$)/gi, "").trim() == "" && !form.querySelector(".img_block").innerHTML) {
    toast_error("Напишите или прикрепите что-нибудь");
    return
  };

  $content_input = document.createElement("input");
  $content_input.setAttribute("name", "content");
  $content_input.setAttribute("type", "hidden");
  $content_input.classList.add("input_content");
  $content_input.value = _text;
  form.append($content_input);

  _attach_value = "";
  attach_list = form.querySelectorAll(".attach");
  for (var i = 0; i < attach_list.length; i++) {
    _attach_value += attach_list[i].value + ","
  };

  $attach_input = document.createElement("input");
  $attach_input.setAttribute("name", "attach");
  $attach_input.setAttribute("type", "hidden");
  $attach_input.classList.add("input_attach");
  $attach_input.value = _attach_value.slice(0,-1);
  form.append($attach_input);

  form_comment = new FormData(form);
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  link_.open('POST', link, true);
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
          form.querySelector(".comment_text").innerHTML = "";
          elem = link_.responseText;
          new_post = document.createElement("span");
          new_post.innerHTML = elem;
          block.append(new_post);
          toast_success(" Комментарий опубликован");
          form.querySelector(".img_block").innerHTML = "";
          form.querySelector(".input_content").remove();
          form.querySelector(".input_attach").remove();
          try {
              form_dropdown = form.querySelector(".current_file_dropdown");
              form_dropdown.classList.remove("current_file_dropdown");
              form_dropdown.parentElement.parentElement.classList.remove("files_one", "files_two");
              form_dropdown.parentElement.parentElement.classList.add("files_null")
          } catch {
              null
          }
      }
  };
  link_.send(form_comment)
};

function load_chart() {
    try {
        var ctx = document.getElementById('canvas');
        var dates = ctx.getAttribute('dates').split(",");
        var data_1 = ctx.getAttribute('data_1').split(",");
        var data_2 = ctx.getAttribute('data_2').split(",");
        var label_1 = ctx.getAttribute('label_1');
        var label_2 = ctx.getAttribute('label_2');
        var config = {
            type: 'line',
            data: {
                labels: dates,
                datasets: [{
                    label: label_1,
                    backgroundColor: 'rgb(255, 99, 132)',
                    borderColor: 'rgb(255, 99, 132)',
                    data: data_1,
                    fill: false,
                }, {
                    label: label_2,
                    fill: false,
                    backgroundColor: 'rgb(54, 162, 235)',
                    borderColor: 'rgb(54, 162, 235)',
                    data: data_2,
                }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                title: {
                    display: true,
                    text: ''
                },
                tooltips: {
                    mode: 'index',
                    intersect: false,
                },
                hover: {
                    mode: 'nearest',
                    intersect: true
                },
                scales: {
                    xAxes: [{
                        display: true,
                        scaleLabel: {
                            display: true,
                            labelString: ''
                        }
                    }],
                    yAxes: [{
                        display: true,
                        scaleLabel: {
                            display: true,
                            labelString: ''
                        }
                    }]
                }
            }
        };
        ctx.getContext('2d');
        window.myLine = new Chart(ctx, config)
    } catch {
        return
    }
};

function addStyleSheets(href) {
    $head = document.head, $link = document.createElement('link');
    $link.rel = 'stylesheet';
    $link.classList.add("my_color_settings");
    $link.href = href;
    $head.appendChild($link)
};

function get_document_opacity_0() {
  document.body.style.overflow = "hidden";
  document.body.style.marginRight = "4px";
  overlay = document.body.querySelector(".body_overlay");
  overlay.style.visibility = "unset";
  overlay.style.opacity = "1";
};
function get_document_opacity_1() {
  document.body.style.overflow = "scroll";
  document.body.style.marginRight = "0";
  overlay = document.body.querySelector(".body_overlay");
  overlay.style.visibility = "hidden";
  overlay.style.opacity = "0";
};

function post_update_votes(post, uuid) {
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  link_.open('GET', "/posts/user_progs/update_votes/" + uuid + "/", true);
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
          jsonResponse = JSON.parse(link_.responseText);
          post.querySelector(".likes_count").innerHTML = jsonResponse.like_count;
          post.querySelector(".dislikes_count").innerHTML = jsonResponse.dislike_count;
      }
  };

  link_.send();
};

function send_reaction(item, pk, _link) {
    reactions_block = item.querySelector(".react_items");

    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    link.overrideMimeType("application/json");
    link.open('GET', _link, true);
    link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    link.onreadystatechange = function() {
        if (link.readyState == 4 && link.status == 200) {
          jsonResponse = JSON.parse(link.responseText).data;
          console.log(jsonResponse);
          item.querySelector(".all_reactions").innerHTML = jsonResponse[0];
            userpic = document.body.querySelector(".userpic");
            userpic.querySelector("img") ? (
              user_src = userpic.querySelector("img").getAttribute("src"),
              $img = document.createElement("img"),
              $img.src = user_src,
              $img.style.borderRadius = "50%",
              $img.style.width = "50px"
            ) :
            ($img = document.createElement("span"),
               $img.innerHTML = '<svg fill="currentColor" class="svg_default svg_default_50" viewBox="0 0 24 24"><path d="M0 0h24v24H0V0z" fill="none"></path><path d="M12 5.9c1.16 0 2.1.94 2.1 2.1s-.94 2.1-2.1 2.1S9.9 9.16 9.9 8s.94-2.1 2.1-2.1m0 9c2.97 0 6.1 1.46 6.1 2.1v1.1H5.9V17c0-.64 3.13-2.1 6.1-2.1M12 4C9.79 4 8 5.79 8 8s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm0 9c-2.67 0-8 1.34-8 4v3h16v-3c0-2.66-5.33-4-8-4z"></path></svg>'
            );
            user_name = userpic.getAttribute("data-name");
            user_pk = document.body.querySelector(".userpic").getAttribute("data-pk");

            react_list = item.querySelectorAll(".react_window_toggle");
            // пройдемся по всему списку допустимых реакций списка.
            for (var i = 0; i < react_list.length; i++) {
              id = react_list[i].getAttribute("data-pk");
              count = jsonResponse[id];
              reactions_block_exists = reactions_block.querySelector('[data-react=' + '"' + id + '"' + ']');
              // если такая реакция уже есть у объекта
              if (reactions_block_exists) {
                 cur_block = reactions_block.querySelector('[data-react=' + '"' + id + '"' + ']');

                 // если пользователь уже ставил эту реакцию
                 if (cur_block.querySelector(".border_radius")) {
                   cur_block.querySelector(".like").classList.remove("border_radius");

                   if (cur_block.querySelector(".like_pop")) {
                       pop = cur_block.querySelector(".like_pop");
                       pop.querySelector('[href=' + '"' + user_pk + '"' + ']').remove();
                     }
                 }
                 else if (pk == id){
                   cur_block.querySelector(".like").classList.add("border_radius");
                   $a = document.createElement("a");
                   $a.style.paddingRight = "10px";
                   $a.setAttribute("href", user_pk);
                   $figure = document.createElement("figure");
                   $figure.style.margin = "0";
                   $figure.title = user_name;
                   $figure.append($img)
                   $a.append($figure);
                   cur_block.querySelector(".like_list").append($a);
                 }

                 console.log("id item", id);
                 console.log("count item", count);
                 if (count == 0) {
                   cur_block.remove();
                 }
                 else {
                   cur_block.querySelector(".reactions_count").innerHTML = count;
                   cur_block.querySelector('[data-count="like"]').innerHTML = count;
                }

              }
              else if (!reactions_block_exists && count > 0){
                // если такой реакции еще нет у объекта...
                console.log("создаем блок реакций");
                count = jsonResponse[pk];
                // создаем главный блок react
                $react = document.createElement("div");
                $react.classList.add("react");
                $react.setAttribute("data-react", pk);

                // создаем иконку и счетчик
                $like = document.createElement("div");
                $like.classList.add("like", "border_radius", "send_react");
                $like_img = document.createElement("img");
                $like_img.src = "/static/images/reactions/" + pk + ".png";
                if (reactions_block.classList.contains("comments")) {
                  $like_img.style.width = "15px";
                  $like_img.style.marginRight = "4px";
                }
                else {
                  $like_img.style.width = "17px";
                  $like_img.style.marginRight = "7px";
                }
                $like_span = document.createElement("span");
                $like_span.innerHTML = "<span class='reactions_count'>" + count + "</span>"
                $like.append($like_img);
                $like.append($like_span);

                // создаем сегмент like_window
                $like_window = document.createElement("div");
                $like_window.classList.add("like_window");
                $div = document.createElement("div");
                $div.classList.add("like_pop");

                $a = document.createElement("a");
                $a.style.paddingRight = "10px";
                $a.setAttribute("href", user_pk);
                $span1 = document.createElement("span");
                $span1.classList.add("item_reactions", "pointer");
                $span1.innerHTML = "Отреагировал <span data-count='like'>1</span> человек";
                $span2 = document.createElement("span");
                $span2.style.display = "flex";
                $span2.style.marginTop = "10px";
                $figure = document.createElement("figure");
                $figure.style.margin = "0";
                $figure.title = user_name;
                $figure.append($img)
                $a.append($figure);
                $span2.append($a);
                $div.append($span1);
                $div.append($span2);
                $div.style.margin = "15px";
                $like_window.append($div);

                $react.append($like);
                $react.append($like_window);
                reactions_block.append($react);
              }
            }
        }
    };
    link.send(null)
};

function get_image_priview(ggg, img) {
    entrou = false;
    img.click();
    img.onchange = function() {
        if (!entrou) {
            imgPath = img.value;
            extn = imgPath.substring(imgPath.lastIndexOf(".") + 1).toLowerCase();
            if (extn == "gif" || extn == "png" || extn == "jpg" || extn == "jpeg") {
                if (typeof FileReader != "undefined") {
                    if (ggg) {}
                    ggg.innerHTML = "";
                    reader = new FileReader();
                    reader.onload = function(e) {
                        $img = document.createElement("img");
                        $img.src = e.target.result;
                        $img.class = "thumb-image";
                        $img.style.width = "100%";
                        ggg.innerHTML = '<a href="#" style="right:15px;top: 0;" class="delete_thumb">Удалить</a>'
                        ggg.append($img)
                    };
                    reader.readAsDataURL(img.files[0])
                }
            } else {
                this.value = null
            }
        }
        entrou = true;
        setTimeout(function() {
            entrou = false
        }, 1000)
    }
};

/// ФУНКЦИИ СМЕНЫ ПОРЯДКА ЭЛЕМЕНТОВ
function change_position(el) {
  parent = el.parentElement;
  array = parent.querySelectorAll(".drag_item");
  len = array.length + 1;
  token = document.body.getAttribute("data-csrf");
  post_array = []
  for (var i=0; i<array.length; i++) {
    count = len -= 1;
    post_array.push({key:array[i].getAttribute("data-pk"),value: count});
  };
  var xmlhttp = new XMLHttpRequest();
  xmlhttp.open("POST", "/users/progs/change_position/" + document.body.querySelector(".pk_saver").getAttribute("data-pk") + "/?types=" + parent.getAttribute("data-type") + "&community_id=" + parent.getAttribute("data-community-id"));
  xmlhttp.setRequestHeader("Content-Type", "application/json;charset=UTF-8");
  xmlhttp.setRequestHeader('X-CSRFToken', token);
  xmlhttp.send(JSON.stringify(post_array));
};

function minus_new_followers() {
    if (document.body.querySelector(".new_followers_bagde")) {
        new_followers = document.body.querySelector(".new_followers_bagde"),
            count = new_followers.innerHTML,
            count * 1,
            count -= 1,
            count > 0 ? new_followers.innerHTML = count : new_followers.innerHTML = ""
    }
};
