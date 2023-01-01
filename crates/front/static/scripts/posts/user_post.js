on('#ajax', 'click', '#u_edit_link_btn', function() {
  form = this.parentElement.parentElement.parentElement;
  value = form.querySelector(".custom_link_input").value;
  form_data = new FormData(form);
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );

  link.open( 'POST', "/users/settings/edit_link", true )
  link.setRequestHeader('Content-Type', 'application/json');

  link.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    close_work_fullscreen();
    toast_info("Ссылка изменена!");
    d_value = "/" + value;
    document.body.querySelector(".edit_user_custom_link").innerHTML = "@" + value;
    document.body.querySelector(".userpic").setAttribute("data-pk", d_value);
    old_links = document.body.querySelectorAll(".request_link");
    for (var i = 0; i < old_links.length; i++) {
      old_links[i].setAttribute("href", location.protocol + '//' + location.host + d_value);
    };
  }};

  link.send(JSON.stringify(form_data));
});

on('body', 'click', '.comment_delete', function() {
  saver = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  type = this.parentElement.getAttribute("data-type");
  form_data = new FormData();
  form_data.append("types", type);

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/users/progs/delete_comment", true );
  link.setRequestHeader('Content-Type', 'application/json');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    div = document.createElement("div");
    div.classList.add("col-12");
    div.style.padding = "10px";
    div.style.display = "block";
    div.innerHTML = "Комментарий удалён. <span class='comment_recover pointer underline' data-type='" + type + "'>Восстановить</span>";
    saver.style.display = "none"; saver.parentElement.insertBefore(div, saver)
  }};
  link.send(JSON.stringify(form_data));
});
on('body', 'click', '.comment_recover', function() {
  type = this.getAttribute("data-type");
  form_data = new FormData();
  form_data.append("types", type);
  block = this.parentElement; next = block.nextElementSibling;
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/users/progs/recover_comment", true );
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    block.remove();
    next.style.display = "block";
  }};
  link.send(JSON.stringify(form_data));
});


on('#ajax', 'change', '.create_video_hide_file', function() {
  form = this.parentElement.parentElement.parentElement;
  pk = form.getAttribute("data-pk");
  form_data.append("id", pk);
  form_data = new FormData(form);

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );

  link.open( 'POST', "/video/add_video", true )
  link.setRequestHeader('Content-Type', 'application/json');

  link.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
  }};
  link.upload.onprogress = function(event) {
    count = event.loaded / event.total * 100;
    document.body.querySelector(".create_header").innerHTML = 'Загружено ' + Math.round(count) + '%';
  };
  link.upload.onload = function() {
    document.body.querySelector(".create_header").innerHTML = "Видеозапись загружена!"
  }
  link.send(JSON.stringify(form_data));
});

on('body', 'change', '.case_all_input', function() {
  _this = this, case_audio = false, case_video = false, id_video_upload_start = false, is_video_edit_window_loaded = true;
  if (this.classList.contains("add_photos_in_list")) {
    url = "/photos/add_photos_in_list"
  } else if (this.classList.contains("add_tracks_in_list")) {
    url = "/music/add_tracks_in_list";
    case_audio = true;
  } else if (this.classList.contains("add_docs_in_list")) {
    url = "/docs/add_docs_in_list"
  } else if (this.classList.contains("add_video_in_list")) {
    if (_this.files[0].type != "video/mp4") {
      toast_info("Пока работаем только с mp4");
      return
    };
    url = "/video/add_video_in_list";
    case_video = true;
  };

  form = this.parentElement.parentElement;
  form_data = new FormData(form);

  if (form.getAttribute("data-pk")) {
    form_data.append("id", form.getAttribute("data-pk"));
  };

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );

  link_.open( 'POST', url, true )
  link_.setRequestHeader('Content-Type', 'application/json');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
      if (case_video) {
        jsonResponse = JSON.parse(link_.responseText);
        document.body.querySelector("#upload_video_pk").setAttribute("value", jsonResponse.pk)
      }
      else {
        response = document.createElement("span");
        response.innerHTML = link_.responseText;
        lenta = form.parentElement.parentElement.parentElement.querySelector(".is_paginate");
        lenta.insertAdjacentHTML('afterBegin', response.innerHTML);
        lenta.querySelector(".items_empty") ? document.body.querySelector(".items_empty").style.display = "none" : null;
        if (case_audio) {
          type = "lis" + form.getAttribute("data-pk");
          playlist_type = document.body.querySelector("#saved_playlist").getAttribute("data-type");
          if (type == playlist_type) {
            add_html_tracks_in_player (response);
          }
        }
      }
  }};
  link_.upload.onprogress = function(event) {
    if (case_video) {
      if (!id_video_upload_start) {
        close_work_fullscreen();
        id_video_upload_start = true;
        create_fullscreen("/video/edit_new_video", "worker_fullscreen", false, true);
      };
      if (is_video_edit_window_loaded) {
        try {
          title = document.body.querySelector("#id_title");
          title.value = _this.files[0].name;
          title.select();
          is_video_edit_window_loaded = false
        } catch { null }
      }
    };
    count = event.loaded / event.total * 100;
    try {
      _this.parentElement.parentElement.querySelector("#onload_info").innerHTML = 'Загружено ' + Math.round(count) + '%'
    } catch { null }
  };
  link_.upload.onload = function() {
    try {
      btn = document.body.querySelector("#edit_video_btn");
      info = btn.parentElement.parentElement.querySelector("#onload_info");
      if (case_video) {
        info.innerHTML = "Видео загружено!";
        btn.classList.remove("hidden");
        fullscreen_resize();
      } else { info.innerHTML = "" }
    } catch { null }
  };
  link_.send(JSON.stringify(form_data));
});

on('body', 'click', '.photo_attach_list_remove', function() {
  block = this.parentElement.parentElement;
  if (block.parentElement.classList.contains("attach_block")){
    remove_file_attach(), 
    is_full_attach()
  } else if (block.classList.contains("comment_attach_block")){
    remove_file_dropdown(); 
    is_full_dropdown()
  } else if (block.classList.contains("message_attach_block")){
    remove_file_message_attach(); 
    is_full_message_attach()
  }
  block.remove();
});

on('#ajax', 'click', '#add_post_btn', function() {
  form_post = this.parentElement.parentElement.parentElement.parentElement;
  text_val = form_post.querySelector(".smile_supported");
  _val = format_text(text_val);
  _text = _val.innerHTML;
  if (_text.replace(/<(?!img)\/?[a-z][^>]*(>|$)/gi, "").trim() == "" && form_post.querySelector(".files_0")) {
    toast_error("Напишите или прикрепите что-нибудь"); return
  };

  $content_input = document.createElement("input");
  $content_input.setAttribute("name", "content");
  $content_input.setAttribute("type", "hidden");
  $content_input.classList.add("input_content");
  $content_input.value = _text;
  form_post.append($content_input);

  _attach_value = "";
  attach_list = form_post.querySelectorAll(".attach");
  for (var i = 0; i < attach_list.length; i++) {
    _attach_value += attach_list[i].value + ","
  };

  $attach_input = document.createElement("input");
  $attach_input.setAttribute("name", "attach");
  $attach_input.setAttribute("type", "hidden");
  $attach_input.classList.add("input_attach");
  $attach_input.value = _attach_value.slice(0,-1);
  form_post.append($attach_input);

  form_data = new FormData(form_post);

  lenta_load = form_post.parentElement.nextElementSibling;
  pk = form_post.parentElement.parentElement.getAttribute("data-uuid");
  form_data.append("list_id", pk);

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/posts/add_post_in_list", true );
  link_.setRequestHeader('Content-Type', 'application/json');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    clear_attach_block();

    elem = link_.responseText;
    new_post = document.createElement("span");
    new_post.innerHTML = elem;

    form_post.querySelector(".input_content").remove();
    form_post.querySelector(".input_attach").remove();
    form_post.querySelector(".smile_supported").innerHTML = "";

    drops = form_post.querySelectorAll(".dropdown-menu");
    for (var i = 0; i < drops.length; i++){drops[i].classList.remove("show")};

    lenta_load.insertAdjacentHTML('afterBegin', new_post.innerHTML);
    toast_info('Запись опубликована');
    lenta_load.querySelector(".items_empty") ? lenta_load.querySelector(".items_empty").style.display = "none" : null;
    main_container = document.body.querySelector(".main-container");
    add_list_in_all_stat("created_user_post",new_post.querySelector(".pag").getAttribute("data-pk"),main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"))
  } else {
        new_post = document.createElement("span");
        new_post.innerHTML = link_.responseText;
        if (new_post.querySelector(".exception_value")){
          text = new_post.querySelector(".exception_value").innerHTML;
          toast_info(text)
        }
    }
  };

  link_.send(JSON.stringify(form_data));
});

on('body', 'click', '.comment_edit', function() {
  _this = this;
  clear_comment_dropdown();

  type = _this.parentElement.getAttribute("data-type");
  form_data = new FormData();
  form_data.append("types", type);
  _this.parentElement.style.display = "none";
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/users/progs/edit_comment", true );
  link.setRequestHeader('Content-Type', 'application/json');

  link.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    elem = link.responseText;
    response = document.createElement("span");
    response.innerHTML = elem;
    parent = _this.parentElement.parentElement.parentElement;

    parent.parentElement.querySelector(".comment_text").style.display = "none";
    parent.parentElement.querySelector(".attach_container") ? parent.parentElement.querySelector(".attach_container").style.display = "none" : null;
    parent.append(response);
  }};
  link.send(JSON.stringify(form_data));
});

on('body', 'click', '.comment_edit_btn', function() {
  form = this.parentElement.parentElement.parentElement
  _text = form.querySelector(".smile_supported").innerHTML;
  if (_text.replace(/<[^>]*(>|$)|&nbsp;|&zwnj;|&raquo;|&laquo;|&gt;/g,'').trim() == "" && !form.querySelector(".img_block").firstChild){
    toast_error("Напишите или прикрепите что-нибудь");
    form.querySelector(".text-comment").style.border = "1px #FF0000 solid";
    form.querySelector(".dropdown").style.border = "1px #FF0000 solid";
    return
  };

  span_form = form.parentElement;
  block = span_form.parentElement.parentElement.parentElement;

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
  form_comment.append("types", type);
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  link_.open('POST', "/users/progs/edit_comment", true);
  link_.setRequestHeader('Content-Type', 'application/json');

  link_.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
          elem = link_.responseText;
          new_post = document.createElement("span");
          new_post.innerHTML = elem;
          block.querySelector(".media-body").innerHTML = new_post.querySelector(".media-body").innerHTML;
          toast_success("Комментарий изменен");
      }
  };
  link_.send(JSON.stringify(form_comment))
});

/*!
   item post scripts for user
  */
on('#ajax', 'click', '.post_remove', function() {
  item = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  pk = item.getAttribute("data-pk");
  form_data = new FormData();
  form_data.append("id", pk);

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/posts/delete_post", true );
  link.setRequestHeader('Content-Type', 'application/json');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    p = document.createElement("div");
    p.classList.add("card", "mb-3");
    p.style.padding = "20px";
    p.innerHTML = "<span class='post_restore pointer' data-pk='" + pk + "'>Запись удалена. <span class='underline'>Восстановить</span></span>";
    !document.querySelector(".post_detail") ? (item.parentElement.insertBefore(p, item), item.style.display = "none")
    : (document.querySelector(".item_fullscreen").style.display = "none",
    block = document.body.querySelector(".post_stream"),
    item = block.querySelector( '[data-pk=' + '"' + pk + '"' + ']' ),
    item.style.display = "none",
    p.style.display =  "block",
    item.parentElement.insertBefore(p, item));
    main_container = document.body.querySelector(".main-container");
    add_list_in_all_stat("deleted_user_post",pk,main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"));
  }};

  link.send(JSON.stringify(form_data));
});

on('#ajax', 'click', '.post_restore', function() {
  item = this.parentElement.nextElementSibling;
  pk = this.getAttribute("data-pk");
  form_data = new FormData();
  form_data.append("id", pk);

  block = this.parentElement;
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/posts/recover_post", true );
  link.setRequestHeader('Content-Type', 'application/json');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    block.remove();
    item.style.display = "block";
    main_container = document.body.querySelector(".main-container");
    add_list_in_all_stat("restored_user_post",pk,main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"));
  }};
  link.send(JSON.stringify(form_data));
});

on('#ajax', 'click', '.post_fixed', function() {
  item = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  pk = item.getAttribute("data-pk"); 
  send_change(this, "/posts/fixed", "post_unfixed", "Открепить");
  main_container = document.body.querySelector(".main-container");
  add_list_in_all_stat("fixed_user_post",pk,main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"));
});
on('#ajax', 'click', '.post_unfixed', function() {
  item = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  pk = item.getAttribute("data-pk");
  send_change(this, "/posts/unfixed", "post_fixed", "Закрепить");
  main_container = document.body.querySelector(".main-container");
  add_list_in_all_stat("unfixed_user_post",pk,main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"));
});

on('#ajax', 'click', '.post_off_comment', function() {
  send_change(this, "/posts/off_comment", "post_on_comment", "Вкл. комментарии");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".load_comments_list").style.display = "none";
  post.querySelector(".load_comments").style.setProperty('display', 'none', 'important');
  main_container = document.body.querySelector(".main-container");
  add_list_in_all_stat("off_comment_user_post",post.getAttribute("data-pk"),main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"));
})
on('#ajax', 'click', '.post_on_comment', function() {
  send_change(this, "/posts/on_comment", "post_off_comment", "Выкл. комментарии");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".load_comments_list").style.display = "unset"
  post.querySelector(".load_comments").style.setProperty('display', 'unset', 'important');
  main_container = document.body.querySelector(".main-container");
  add_list_in_all_stat("on_comment_user_post",post.getAttribute("data-pk"),main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"));
});

on('body', 'click', '.react_window_toggle', function() {
  react_section = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  pk = this.getAttribute("data-pk");
  send_reaction(react_section, pk, "/users/progs/send_reaction"
  );

  main_container = document.body.querySelector(".main-container");
  //add_list_in_all_stat("dislike_user_post_comment",comment_pk,main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"));
});
on('body', 'click', '.send_react', function() {
  react_section = this.parentElement.parentElement.parentElement;
  pk = this.parentElement.getAttribute("data-react");
  send_reaction(react_section, pk, "/users/progs/send_reaction");

  main_container = document.body.querySelector(".main-container");
  //add_list_in_all_stat("dislike_user_post_comment",comment_pk,main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"));
});

on('body', 'click', '.delete_list', function() {
  _this = this;
  _this.removeAttribute('tooltip');
  parent = _this.parentElement;
  type = parent.getAttribute('data-type');
  community_id = parent.getAttribute('data-community-id').trim();
  pk = type.slice(3);

  form_data = new FormData();
  form_data.append("list_id", pk);

  if (type.indexOf('lpo') !== -1) {
    if (community_id && community_id !== "") {
      url = "/posts/delete_community_list/" + community_id;
      form_data.append("community_id", community_id);
    } else {
      url = "/posts/delete_user_list";
    }
  }
  else if (type.indexOf('lph') !== -1) {
    if (community_id && community_id !== "") {
      url = "/photos/delete_community_list/" + community_id;
      form_data.append("community_id", community_id);
    } else {
      url = "/photos/delete_user_list";
    }
  }
  else if (type.indexOf('ldo') !== -1) {
    if (community_id && community_id !== "") {
      url = "/docs/delete_community_list/" + community_id;
      form_data.append("community_id", community_id);
    } else {
      url = "/docs/delete_user_list";
    }
  }
  else if (type.indexOf('lgo') !== -1) {
    if (community_id && community_id !== "") {
      url = "/goods/delete_community_list/" + community_id;
      form_data.append("community_id", community_id);
    } else {
      url = "/goods/delete_user_list";
    }
  }
  else if (type.indexOf('lmu') !== -1) {
    if (community_id && community_id !== "") {
      url = "/music/delete_community_list/" + community_id;
      form_data.append("community_id", community_id);
    } else {
      url = "/music/delete_user_list";
    }
  }
  else if (type.indexOf('lsu') !== -1) {
    if (community_id && community_id !== "") {
      url = "/survey/delete_community_list/" + community_id;
      form_data.append("community_id", community_id);
    } else {
      url = "/survey/delete_user_list";
    }
  }
  else if (type.indexOf('lvi') !== -1) {
    if (community_id && community_id !== "") {
      url = "/video/delete_community_list/" + community_id;
      form_data.append("community_id", community_id);
    } else {
      url = "/video/delete_user_list";
    }
  }

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', url, true );
  link_.setRequestHeader('Content-Type', 'application/json');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    hide_icons = parent.parentElement.querySelectorAll(".hide_delete");
    for (var i = 0; i < hide_icons.length; i++){
      hide_icons[i].style.display = "none";
    };
    parent.parentElement.querySelector(".second_list_name").innerHTML = "";
    //list = document.body.querySelector( '[data-pk=' + '"' + pk + '"' + ']' );
    //list.querySelector('.list_name').innerHTML = "Список удален";
    _this.classList.replace("delete_list", "recover_list");
    _this.innerHTML = "Восстановить список";
    //main_container = document.body.querySelector(".main-container");
    //add_list_in_all_stat(stat_class,type.slice(3),main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"))
  }}
  link_.send(JSON.stringify(form_data));
});
on('body', 'click', '.recover_list', function() {
  _this = this;
  _this.setAttribute('tooltip', 'Удалить список');
  parent = _this.parentElement;
  type = parent.getAttribute('data-type');
  community_id = parent.getAttribute('data-community-id').trim();
  pk = type.slice(3);

  form_data = new FormData();
  form_data.append("list_id", pk);

  if (type.indexOf('lpo') !== -1) {
    if (community_id && community_id !== "") {
      url = "/posts/recover_community_list/" + community_id;
      form_data.append("community_id", community_id);
    } else {
      url = "/posts/recover_user_list";
    }
  }
  else if (type.indexOf('lph') !== -1) {
    if (community_id && community_id !== "") {
      url = "/photos/recover_community_list/" + community_id;
      form_data.append("community_id", community_id);
    } else {
      url = "/photos/recover_user_list";
    }
  }
  else if (type.indexOf('ldo') !== -1) {
    if (community_id && community_id !== "") {
      url = "/docs/recover_community_list/" + community_id;
      form_data.append("community_id", community_id);
    } else {
      url = "/docs/recover_user_list";
    }
  }
  else if (type.indexOf('lgo') !== -1) {
    if (community_id && community_id !== "") {
      url = "/goods/recover_community_list/" + community_id;
      form_data.append("community_id", community_id);
    } else {
      url = "/goods/recover_user_list";
    }
  }
  else if (type.indexOf('lmu') !== -1) {
    if (community_id && community_id !== "") {
      url = "/music/recover_community_list/" + community_id;
      form_data.append("community_id", community_id);
    } else {
      url = "/music/recover_user_list";
    }
  }
  else if (type.indexOf('lsu') !== -1) {
    if (community_id && community_id !== "") {
      url = "/survey/recover_community_list/" + community_id;
      form_data.append("community_id", community_id);
    } else {
      url = "/survey/recover_user_list";
    }
  }
  else if (type.indexOf('lvi') !== -1) {
    if (community_id && community_id !== "") {
      url = "/video/recover_community_list/" + community_id;
      form_data.append("community_id", community_id);
    } else {
      url = "/video/recover_user_list";
    }
  }

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', url, true );
  link_.setRequestHeader('Content-Type', 'application/json');
  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    hide_icons = parent.parentElement.querySelectorAll(".hide_delete");
    for (var i = 0; i < hide_icons.length; i++){
      hide_icons[i].style.display = "unset";
    };
    second_list = document.body.querySelector('.second_list_name');
    name = second_list.getAttribute("data-name");
    second_list.innerHTML = name;
    //list = document.body.querySelector( '[data-pk=' + '"' + type.slice(3) + '"' + ']' );
    //list.querySelector('.list_name').innerHTML = name;
    _this.classList.replace("recover_list", "delete_list");
    _this.innerHTML = '<svg class="svg_info" fill="currentColor" viewBox="0 0 24 24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M16.5,10V9h-2v1H12v1.5h1v4c0,0.83,0.67,1.5,1.5,1.5h2c0.83,0,1.5-0.67,1.5-1.5v-4h1V10H16.5z M16.5,15.5h-2v-4h2V15.5z M20,6h-8l-2-2H4C2.89,4,2.01,4.89,2.01,6L2,18c0,1.11,0.89,2,2,2h16c1.11,0,2-0.89,2-2V8C22,6.89,21.11,6,20,6z M20,18H4V6h5.17 l2,2H20V18z"/></g></svg>';
    //main_container = document.body.querySelector(".main-container");
    //add_list_in_all_stat(stat_class,type.slice(3),main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"))
  }}
  link_.send(JSON.stringify(form_data));
});

on('#ajax', 'change', '#u_photo_post_attach', function() {
  form = this.parentElement;
  form_data = new FormData(form);
  input = form.querySelector(".upload_for_post_attach")
  if (input.files.length > 10) {
      toast_error("Не больше 10 фотографий");
      return;
  }
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/photos/add_attach_photo", true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    elem = link_.responseText;
    response = document.createElement("span");
    response.innerHTML = elem;
    photo_list = response.querySelectorAll(".pag");
    if (document.body.querySelector(".attach_block")){
      block = document.body.querySelector(".attach_block");
      photo_post_upload_attach(photo_list, block);
    } else if (document.body.querySelector(".message_attach_block")){
      block = document.body.querySelector(".message_attach_block");
      photo_message_upload_attach(photo_list, block);
    }
    }
    close_work_fullscreen();
  }
  link_.send(JSON.stringify(form_data));
});

on('#ajax', 'change', '#u_photo_post_comment_attach', function() {
  form = document.body.querySelector("#add_comment_photos");
  form_data = new FormData(form);
  input = form.querySelector("#u_photo_post_comment_attach")
  if (input.files.length > 2) {
      toast_error("Не больше 2 фотографий");
      return;
  }
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/photos/add_attach_photo", true );
  link_.setRequestHeader('Content-Type', 'application/json');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    elem = link_.responseText;
    response = document.createElement("span");
    response.innerHTML = elem;
    photo_list = response.querySelectorAll(".pag");
    photo_comment_upload_attach(photo_list, document.body.querySelector(".current_file_dropdown").parentElement.parentElement, photo_list.length);
    }
    close_work_fullscreen();
  }
  link_.send(JSON.stringify(form_data));
});

on('#ajax', 'click', '.photo_load_several', function() {
  previous = this.previousElementSibling
  _this = previous.querySelector("img");
  photo_pk = previous.getAttribute('photo-pk');
  user_pk = previous.getAttribute('data-pk');
  src = _this.parentElement.getAttribute("data-href");
  if (document.body.querySelector(".current_file_dropdown")){
    check_photo_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, photo_pk) ? null : (photo_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, photo_pk, user_pk, src), this.classList.add("active_svg"))
  } else if (document.body.querySelector(".attach_block")){
    check_photo_in_block(document.body.querySelector(".attach_block"), _this, photo_pk) ? null : (photo_post_attach(document.body.querySelector(".attach_block"), photo_pk, user_pk, src), this.classList.add("active_svg"))
  } else if (document.body.querySelector(".message_attach_block")){
    check_photo_in_block(document.body.querySelector(".message_attach_block"), _this, photo_pk) ? null : (photo_message_attach(document.body.querySelector(".message_attach_block"), photo_pk, user_pk, src), this.classList.add("active_svg"), show_message_form_send_btn())
  }
});

on('#ajax', 'click', '.photo_load_one', function() {
  _this = this;
  photo_pk = _this.parentElement.getAttribute('photo-pk');
  user_pk = _this.parentElement.getAttribute('data-pk');
  src = _this.parentElement.getAttribute("data-href");
  if (document.body.querySelector(".attach_block")){
    check_photo_in_block(document.body.querySelector(".attach_block"), _this, photo_pk) ? null : (photo_post_attach(document.body.querySelector(".attach_block"), photo_pk, user_pk, src), close_work_fullscreen())
  }
  else if (document.body.querySelector(".current_file_dropdown")){
    check_photo_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, photo_pk) ? null : (photo_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, photo_pk, user_pk, src), close_work_fullscreen())
  } else if (document.body.querySelector(".message_attach_block")){
    check_photo_in_block(document.body.querySelector(".message_attach_block"), _this, photo_pk) ? null : (close_work_fullscreen(), photo_message_attach(document.body.querySelector(".message_attach_block"), photo_pk, user_pk, src))
  }
});

on('#ajax', 'click', '.u_create_video_attach_btn', function() {
  form_data = new FormData(document.querySelector("#create_video_form"));
  user_pk = document.body.querySelector(".pk_saver").getAttribute("data-pk");
  form_data.append("user_id", user_pk);

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/video/create_video_attach", true );
  link_.setRequestHeader('Content-Type', 'application/json');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    elem_ = document.createElement('div');
    elem_.innerHTML = link_.responseText;

    dropdown = document.body.querySelector(".current_file_dropdown").parentElement.parentElement;
    video_comment_attach(elem_.querySelector("img"), dropdown);

    close_work_fullscreen();
  }};

  link_.send(JSON.stringify(form_data));
});

on('#ajax', 'click', '#create_repost_btn', function() {
  form_post = this.parentElement.parentElement;
  collector = form_post.querySelector(".collector");
  if (!collector.innerHTML) {
    collector.innerHTML = '<div class="response_text">⇠ <br>Выберите списки записей или получателей</div>';
    return
  }
  text_val = form_post.querySelector(".smile_supported");
  _val = format_text(text_val);
  _text = _val.innerHTML;

  $input = document.createElement("input");
  $input.setAttribute("name", "content");
  $input.setAttribute("type", "hidden");
  $input.classList.add("input_text");
  $input.value = _text;
  form_post.append($input);

  form_data = new FormData(form_post);

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/progs/create_repost", true );
  link_.setRequestHeader('Content-Type', 'application/json');

  link_.onreadystatechange = function () {
  if ( link_.readyState == 4 && link_.status == 200 ) {
    close_work_fullscreen();
    toast_info("Репост сделан!")
  }};

  link_.send(JSON.stringify(form_data));
});

on('#ajax', 'click', '#create_copy_btn', function() {
  form_post = this.parentElement.parentElement;
  collector = form_post.querySelector(".collector");
  if (!form_post.querySelector(".is_list") && !collector.innerHTML) {
    collector.innerHTML = '<div class="response_text">⇠ <br>Выберите списки</div>';
    return
  }
  else if (form_post.querySelector(".is_list") && form_post.querySelector(".copy_for_communities").checked && !collector.innerHTML) {
    collector.innerHTML = '<div class="response_text">⇠ <br>Выберите сообщества</div>';
    return
  }
  else if (form_post.querySelector(".is_list") && !form_post.querySelector(".copy_for_communities").checked && !form_post.querySelector(".copy_for_profile").checked) {
    collector.innerHTML = '<div class="response_text">Выберите, куда копировать объект</div>';
    return
  };

  form_data = new FormData(form_post);
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/progs/create_copy/", true );
  link_.setRequestHeader('Content-Type', 'application/json');

  link_.onreadystatechange = function () {
  if ( link_.readyState == 4 && link_.status == 200 ) {
    close_work_fullscreen();
    toast_info("Объект копирован!")
  }};
  link_.send(JSON.stringify(form_data));
});

on('#ajax', 'click', '#create_list_btn', function() {
  form_post = this.parentElement.parentElement.parentElement;
  form_data = new FormData(form_post);

  if (!form_post.querySelector("#id_name").value){
    form_post.querySelector("#id_name").style.border = "1px #FF0000 solid";
    toast_error("Название - обязательное поле!");
    return
  } else { this.disabled = true };
  community_id = form_post.getAttribute("community-pk");
  is_community = false;
  folder = form_post.getAttribute("data-folder");
  if (form_post.getAttribute("community-pk") && form_post.getAttribute("community-pk") !== "") {
    url = folder + "/add_community_list";
    form_data.append("community_id", community_id);
    is_community = true;
  } else {
    url = folder + "/add_user_list";
  }
  if (form_post.querySelector(".reactions_collector")) {
    react_value = form_post.querySelector(".reactions_collector");
    react_inputs = form_post.querySelectorAll(".switch-sm");
    for (var i = 0; i < react_inputs.length; i++) {
      if (react_inputs[i].checked == 1){
        react_value.value = react_value.value + react_inputs[i].value + ", ";
      }
    };
    react_value.value = react_value.value.slice(0, -2);
  }

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', url, true );
  link_.setRequestHeader('Content-Type', 'application/json');

  link_.onreadystatechange = function () {
  if ( link_.readyState == 4 && link_.status == 200 ) {
    jsonResponse = JSON.parse(link_.responseText);
    new_pk = jsonResponse.pk;
    new_name = jsonResponse.name;
    new_image = jsonResponse.image;
    userpic = document.body.querySelector(".userpic");
    user_id = userpic.getAttribute("data-id");

    if (folder == "/posts") {
      name = new_name;
      li = document.createElement("li");
      li.classList.add("date", "list", "pointer", "post_list_change");
      li.setAttribute("list-pk", new_pk);
      if (is_community) {
        li.setAttribute("data-pk", community_pk);
      }
      else {
        li.setAttribute("data-pk", user_id);
      }

      media = document.createElement("div");
      media.classList.add("media");

      media_body = document.createElement("div");
      media_body.classList.add("media-body");

      h6 = document.createElement("h6");
      h6.classList.add("my-0", "mt-1");
      h6.innerHTML = '<span class="list_name">' + name + '</span> (<span class="handle">0</span>)';

      figure = document.createElement("figure");

      if (userpic.querySelector("img")) {
        a = document.createElement("a");
        a.classList.add("ajax");
        a.setAttribute("href", userpic.getAttribute("data-pk"));
        img = document.createElement("img");
        img.setAttribute("src", userpic.querySelector("img").getAttribute("src"));
        img.style.borderRadius = "30px";
        img.style.width = "30px";
        figure.append(img);
        a.append(figure);
      } else {
        a = document.createElement("span");
        a.innerHTML = '<svg fill="currentColor" class="svg_default svg_default_30" viewBox="0 0 24 24"><path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"></path><path d="M0 0h24v24H0z" fill="none"></path></svg>'
        h6.classList.add("ml-2");
      };

      media_body.append(h6);
      media.append(a);
      media.append(media_body);
      li.append(media);
      document.body.querySelector(".date-list").prepend(li);
    }
    else {
      if (folder == "/docs") {
        _svg = "<svg class='pointer load_doc_list svg_default' width='50' height='50' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='1' stroke-linecap='round' stroke-linejoin='round'><path d='M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z'></path></svg>"
        _class = "load_doc_list";
        _data = 'doclist-pk="' + new_pk + '"';
      }
      else if (folder == "/goods") {
        _svg = "<svg class='pointer load_good_list svg_default' width='50' height='50' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='1' stroke-linecap='round' stroke-linejoin='round'><g><rect fill='none' height='24' width='24' /><path d='M18,6h-2c0-2.21-1.79-4-4-4S8,3.79,8,6H6C4.9,6,4,6.9,4,8v12c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V8C20,6.9,19.1,6,18,6z M12,4c1.1,0,2,0.9,2,2h-4C10,4.9,10.9,4,12,4z M18,20H6V8h2v2c0,0.55,0.45,1,1,1s1-0.45,1-1V8h4v2c0,0.55,0.45,1,1,1s1-0.45,1-1V8 h2V20z' /></g></svg>";
        _class = "load_good_list";
        _data = 'goodlist-pk="' + new_pk + '"';
      }
      else if (folder == "/music") {
        _svg = "<svg class='pointer load_music_list svg_default' width='50' height='50' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='1' stroke-linecap='round' stroke-linejoin='round'><path d='M9 18V5l12-2v13'></path><circle cx='6' cy='18' r='3'></circle><circle cx='18' cy='16' r='3'></circle></svg>";
        _class = "load_music_list";
        _data = 'playlist-pk="' + new_pk + '"';
      }
      else if (folder == "/photos") {
        _svg = "<svg class='pointer load_photo_list svg_default' width='50' height='50' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='1' stroke-linecap='round' stroke-linejoin='round'><rect x='3' y='3' width='18' height='18' rx='2' ry='2'></rect><circle cx='8.5' cy='8.5' r='1.5'></circle><polyline points='21 15 16 10 5 21'></polyline></svg>";
        _class = "load_photo_list";
        _data = 'photolist-pk="' + new_pk + '"';
      }
      else if (folder == "/surveys") {
        _svg = "<svg class='pointer load_survey_list svg_default' width='50' height='50' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='1' stroke-linecap='round' stroke-linejoin='round'><path d='M0 0h24v24H0V0z' fill='none'></path><path d='M18 9l-1.41-1.42L10 14.17l-2.59-2.58L6 13l4 4zm1-6h-4.18C14.4 1.84 13.3 1 12 1c-1.3 0-2.4.84-2.82 2H5c-.14 0-.27.01-.4.04-.39.08-.74.28-1.01.55-.18.18-.33.4-.43.64-.1.23-.16.49-.16.77v14c0 .27.06.54.16.78s.25.45.43.64c.27.27.62.47 1.01.55.13.02.26.03.4.03h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7-.25c.41 0 .75.34.75.75s-.34.75-.75.75-.75-.34-.75-.75.34-.75.75-.75zM19 19H5V5h14v14z'></path></svg>";
        _class = "load_survey_list";
        _data = 'surveylist-pk="' + new_pk + '"';
      }
      else if (folder == "/video") {
        _svg = "<svg class='pointer load_video_list svg_default' width='50' height='50' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='1' stroke-linecap='round' stroke-linejoin='round'><path d='M18 3v2h-2V3H8v2H6V3H4v18h2v-2h2v2h8v-2h2v2h2V3h-2zM8 17H6v-2h2v2zm0-4H6v-2h2v2zm0-4H6V7h2v2zm10 8h-2v-2h2v2zm0-4h-2v-2h2v2zm0-4h-2V7h2v2z' /><path d='M0 0h24v24H0z' fill='none' /></svg>";
        _class = "load_video_list";
        _data = 'videolist-pk="' + new_pk + '"';
      }
      if (is_community) {
        creator_name = form_post.getAttribute("community-name");
        creator_id = form_post.getAttribute("community-pk");
      }
      else {
        creator_name = userpic.getAttribute("data-name");
        creator_id = userpic.getAttribute("data-pk");
      }

      new_list = "<li class='list_item drag_item' data-pk='" + new_pk + "'><div class='card file-manager-item folder border'" + _data + "'><div class='card-img-top file-logo-wrapper'><div class='d-flex align-items-center justify-content-center w-100'>" + _svg + "</div></div><div class='card-body pt-0'><div class='content-wrapper'><p class='card-text file-name mb-0 list_name list_toggle pointer " + _class + "' style='text-align: left;' data-name='" + new_name + "'>" + new_name + "</p><p class='handle card-text file-size mb-0'>0</p></div><small class='file-accessed'><a class='ajax underline' href='" + creator_id + "'>" + creator_name + "</a></small></div></div></li>"
      if (document.body.querySelector(".drag_list")) {
        drag_list = document.body.querySelector(".drag_list");
        drag_list.innerHTML = new_list + drag_list.innerHTML;
      }
      else {
        new_block = "<div class='row no-gutters' style='overflow-x: auto;'><div class='col-12'><ul class='drag_list' style='width:max-content;list-style: none;'>" + new_list + "</ul></div></div>";
        document.body.querySelector(".new_ul_container").innerHTML = new_block;
      }
    };
    close_work_fullscreen();
  }};

  link_.send(JSON.stringify(form_data));
});

on('#ajax', 'click', '#edit_list_btn', function() {
  form = this.parentElement.parentElement.parentElement;

  if (form.querySelector(".reactions_collector")) {
    react_value = form.querySelector(".reactions_collector");
    react_inputs = form.querySelectorAll(".switch-sm");
    for (var i = 0; i < react_inputs.length; i++) {
      if (react_inputs[i].checked == 1){
        react_value.value = react_value.value + react_inputs[i].value + ", ";
      }
    }
    react_value.value = react_value.value.slice(0, -2);
  }

  form_data = new FormData(form);
  if (!form.querySelector("#id_name").value){
    form.querySelector("#id_name").style.border = "1px #FF0000 solid";
    toast_error("Название - обязательное поле!");
    return
  } else { this.disabled = true }
  pk = form.getAttribute("data-pk");
  form_data.append("list_id", pk);
  folder = form.getAttribute("data-folder");
  if (form.getAttribute("community-pk") && form.getAttribute("community-pk") !== "") {
    form_data.append("community_id", form.getAttribute("community-pk"));
    url = folder + "/edit_community_list";
  } else {
    url = folder + "/edit_user_list";
  }

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', url, true );
  link_.setRequestHeader('Content-Type', 'application/json');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    name = form.querySelector('#id_name').value;

    if (folder == "/posts") {
      lists = document.body.querySelector(".date-list");
      title = lists.querySelector( '[list-pk=' + '"' + pk + '"' + ']' );
      title.querySelector("h6").innerHTML = name;
    } else {
      list = document.body.querySelector( '[data-pk=' + '"' + pk + '"' + ']' );
      list.querySelector('.list_name') ? list.querySelector('.list_name').innerHTML = name : null;

    };
    document.body.querySelector('.second_list_name').innerHTML = name;
    close_work_fullscreen();
    toast_success("Список изменен");
    //main_container = document.body.querySelector(".main-container");
    //add_list_in_all_stat(stat_class,pk,main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"))
  }}
  link_.send(JSON.stringify(form_data));
});

on('#ajax', 'click', '#create_claim_btn', function() {
  form_post = this.parentElement.parentElement;

  text_val = form_post.querySelector(".smile_supported");
  _val = format_text(text_val);
  _text = _val.innerHTML;

  $input = document.createElement("input");
  $input.setAttribute("name", "description");
  $input.setAttribute("type", "hidden");
  $input.classList.add("input_text");
  $input.value = _text;
  form_post.append($input);

  form_data = new FormData(form_post);

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/progs/create_claim", true );
  link_.setRequestHeader('Content-Type', 'application/json');

  link_.onreadystatechange = function () {
  if ( link_.readyState == 4 && link_.status == 200 ) {
    close_work_fullscreen();
    toast_info("Жалоба отправлена!")
  }};

  link_.send(JSON.stringify(form_data));
});

on('#ajax', 'click', '.remove_list_in_user_collections', function() {
  _this = this;
  a = "u" + _this.getAttribute("data-pk");
  form = _this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  //input = form.querySelector(".item_type").value
  form_data = new FormData();
  form_data.append("types", form.querySelector(".item_type").value);

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/progs/uncopy_user_list", true );
  link_.setRequestHeader('Content-Type', 'application/json');

  link_.onreadystatechange = function () {
  if ( link_.readyState == 4 && link_.status == 200 ) {
    close_work_fullscreen()
  }};

  link_.send(JSON.stringify(form_data));
});
on('#ajax', 'click', '.remove_list_in_community_collections', function() {
  _this = this;
  pk = _this.getAttribute("data-pk");
  type = _this.getAttribute("data-type");
  form_data = new FormData();
  form_data.append("types", type);
  form_data.append("id", pk);

  block = _this.parentElement.parentElement.parentElement;
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/progs/uncopy_community_list", true );
  link_.setRequestHeader('Content-Type', 'application/json');

  link_.onreadystatechange = function () {
  if ( link_.readyState == 4 && link_.status == 200 ) {
    parent = _this.parentElement;
    parent.innerHTML = "";
    parent.innerHTML = "Сообщество";
    block.classList.add("communities_toggle", "pointer");
  }};

  link_.send(JSON.stringify(form_data));
});

on('#ajax', 'click', '.video_load_one', function() {
  _this = this;
  pk = _this.getAttribute('video-pk');
  counter = _this.getAttribute('video-counter');
  src = _this.getAttribute('src');
  if (document.body.querySelector(".current_file_dropdown")){
    check_video_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, pk) ? null : (video_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, pk, counter, src), close_work_fullscreen())
  } else if (document.body.querySelector(".attach_block")){
    check_video_in_block(document.body.querySelector(".attach_block"), _this, pk) ? null : (video_post_attach(document.body.querySelector(".attach_block"), pk, counter, src), close_work_fullscreen())
  } else if (document.body.querySelector(".message_attach_block")){
    check_video_in_block(document.body.querySelector(".message_attach_block"), _this, pk) ? null : (video_message_attach(document.body.querySelector(".message_attach_block"), pk, counter, src), close_work_fullscreen(), show_message_form_send_btn())
  }
});
on('#ajax', 'click', '.video_load_several', function() {
  previous = this.previousElementSibling
  _this = previous.querySelector("img");
  pk = _this.getAttribute('video-pk');
  counter = _this.getAttribute('video-counter');
  src = _this.getAttribute('src');
  if (document.body.querySelector(".current_file_dropdown")){
    check_video_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, pk) ? null : (video_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, pk, counter, src), this.classList.add("active_svg"))
  } else if (document.body.querySelector(".attach_block")){
    check_video_in_block(document.body.querySelector(".attach_block"), _this, pk) ? null : (video_post_attach(document.body.querySelector(".attach_block"), pk, counter, src), this.classList.add("active_svg"))
  } else if (document.body.querySelector(".message_attach_block")){
    check_video_in_block(document.body.querySelector(".message_attach_block"), _this, pk) ? null : (video_message_attach(document.body.querySelector(".message_attach_block"), pk, counter, src), this.classList.add("active_svg"), show_message_form_send_btn())
  }
});
on('body', 'click', '.video_attach_list', function() {
  _this = this;
  name = _this.parentElement.querySelector(".list_name").innerHTML;
  pk = _this.getAttribute('data-pk');
  count = _this.parentElement.querySelector(".count").innerHTML;
  if (document.body.querySelector(".current_file_dropdown")){
    check_video_list_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, pk) ? null : (video_list_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, name, pk, count), close_work_fullscreen())
  } else if (document.body.querySelector(".attach_block")){
    check_video_list_in_block(document.body.querySelector(".attach_block"), _this, pk) ? null : (video_list_post_attach(document.body.querySelector(".attach_block"), name, pk, count), close_work_fullscreen())
  } else if (document.body.querySelector(".message_attach_block")){
    check_video_list_in_block(document.body.querySelector(".message_attach_block"), _this, pk) ? null : (video_list_message_attach(document.body.querySelector(".message_attach_block"), name, pk, count), close_work_fullscreen())
  }
});

on('#ajax', 'click', '.music_load_several', function() {
  _this = this.previousElementSibling;

  title = _this.querySelector("h6").innerHTML;
  track_pk = _this.getAttribute('music-pk');
  list_pk = _this.parentElement.getAttribute('playlist-pk');
  _this.querySelector("img") ? src = _this.querySelector("img").getAttribute('src') : src = '/static/images/no_track_img.jpg'
  if (document.body.querySelector(".current_file_dropdown")){
    check_music_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, track_pk) ? null : (music_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, title, track_pk, list_pk, src), this.classList.add("active_svg"))
  } else if (document.body.querySelector(".attach_block")){
    check_music_in_block(document.body.querySelector(".attach_block"), _this, track_pk) ? null : (music_post_attach(document.body.querySelector(".attach_block"), title, track_pk, list_pk, src), this.classList.add("active_svg"))
  } else if (document.body.querySelector(".message_attach_block")){
    check_music_in_block(document.body.querySelector(".message_attach_block"), _this, track_pk) ? null : (music_message_attach(document.body.querySelector(".message_attach_block"), title, track_pk, list_pk, src), this.classList.add("active_svg"), show_message_form_send_btn())
  }
});
on('body', 'click', '.music_attach_list', function() {
  _this = this;
  name = _this.parentElement.querySelector(".load_attach_music_list").innerHTML;
  img_src = _this.parentElement.parentElement.querySelector(".image_fit_200").getAttribute("src");
  pk = _this.getAttribute('data-pk');
  track_pk = _this.getAttribute('track-pk');
  count = _this.parentElement.parentElement.querySelector(".count").innerHTML;
  if (document.body.querySelector(".current_file_dropdown")){
    check_playlist_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, pk) ? null : (playlist_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, name, img_src, pk, track_pk, count), close_work_fullscreen())
  } else if (document.body.querySelector(".attach_block")){
    check_playlist_in_block(document.body.querySelector(".attach_block"), _this, pk) ? null : (playlist_post_attach(document.body.querySelector(".attach_block"), name, img_src, pk, track_pk, count),  close_work_fullscreen())
  } else if (document.body.querySelector(".message_attach_block")){
    check_playlist_in_block(document.body.querySelector(".message_attach_block"), _this, pk) ? null : (playlist_message_attach(document.body.querySelector(".message_attach_block"), name, img_src, pk, track_pk, count), close_work_fullscreen())
  }
});

on('#ajax', 'click', '.doc_load_several', function() {
  _this = this.previousElementSibling;
  pk = _this.getAttribute('data-pk');
  media_block = _this.querySelector(".media-body")
  if (document.body.querySelector(".current_file_dropdown")){
    check_doc_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, pk) ? null : (doc_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, media_block, pk), this.classList.add("active_svg"))
  } else if (document.body.querySelector(".attach_block")){
    check_doc_in_block(document.body.querySelector(".attach_block"), _this, pk) ? null : (doc_post_attach(document.body.querySelector(".attach_block"), media_block, pk), this.classList.add("active_svg"))
  } else if (document.body.querySelector(".message_attach_block")){
    check_doc_in_block(document.body.querySelector(".message_attach_block"), _this, pk) ? null : (doc_message_attach(document.body.querySelector(".message_attach_block"), media_block, pk), this.classList.add("active_svg"), show_message_form_send_btn())
  }
});
on('body', 'click', '.doc_attach_list', function() {
  _this = this;
  name = _this.parentElement.querySelector(".list_name").innerHTML;
  pk = _this.getAttribute('data-pk');
  count = _this.parentElement.querySelector(".count").innerHTML;
  if (document.body.querySelector(".current_file_dropdown")){
    check_doc_list_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, pk) ? null : (doc_list_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, name, pk, count), close_work_fullscreen())
  } else if (document.body.querySelector(".attach_block")){
    check_doc_list_in_block(document.body.querySelector(".attach_block"), _this, pk) ? null : (doc_list_post_attach(document.body.querySelector(".attach_block"), name, pk, count), close_work_fullscreen())
  } else if (document.body.querySelector(".message_attach_block")){
    check_doc_list_in_block(document.body.querySelector(".message_attach_block"), _this, pk) ? null : (doc_list_message_attach(document.body.querySelector(".message_attach_block"), name, pk, count), close_work_fullscreen())
  }
});

on('#ajax', 'click', '.survey_attach_remove', function() {
  block = this.parentElement.parentElement;
  block.remove();
  if (document.body.querySelector(".message_attach_block")){
    remove_file_message_attach()
  }
  else if (document.body.querySelector(".current_file_dropdown")){
    remove_file_dropdown()
  }
  else if (document.body.querySelector(".attach_block")){
    remove_file_attach()
  }
});

on('#ajax', 'click', '.good_load_one', function() {
  _this = this;
  data_pk = _this.getAttribute('good-pk');
  data_uuid = _this.getAttribute('good-uuid');
  src = _this.querySelector("img").getAttribute('src');
  title = _this.querySelector(".good_title").innerHTML;

  if (document.body.querySelector(".current_file_dropdown")){
    check_good_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, data_pk) ? null : (good_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, src, data_pk, data_uuid, title), close_work_fullscreen())
  } else if (document.body.querySelector(".attach_block")){
    check_good_in_block(document.body.querySelector(".attach_block"), _this, data_pk) ? null : (good_post_attach(document.body.querySelector(".attach_block"), src, data_pk, data_uuid, title), close_work_fullscreen())
  } else if (document.body.querySelector(".message_attach_block")){
    check_good_in_block(document.body.querySelector(".message_attach_block"), _this, data_pk) ? null : (good_message_attach(document.body.querySelector(".message_attach_block"), src, data_pk, data_uuid, title), close_work_fullscreen(), show_message_form_send_btn())
  }
});
on('#ajax', 'click', '.good_load_several', function() {
  _this = this.previousElementSibling;
  data_pk = _this.getAttribute('good-pk');
  data_uuid = _this.getAttribute('good-uuid');
  src = _this.querySelector("img").getAttribute('src');
  title = _this.querySelector(".good_title").innerHTML;

  if (document.body.querySelector(".current_file_dropdown")){
    check_good_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, data_pk) ? null : (good_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, src, data_pk, data_uuid, title), this.classList.add("active_svg"))
  } else if (document.body.querySelector(".attach_block")){
    check_good_in_block(document.body.querySelector(".attach_block"), _this, data_pk) ? null : (good_post_attach(document.body.querySelector(".attach_block"), src, data_pk, data_uuid, title), this.classList.add("active_svg"))
  } else if (document.body.querySelector(".message_attach_block")){
    check_good_in_block(document.body.querySelector(".message_attach_block"), _this, data_pk) ? null : (good_message_attach(document.body.querySelector(".message_attach_block"), src, data_pk, data_uuid, title), this.classList.add("active_svg"), show_message_form_send_btn())
  }
});
on('body', 'click', '.good_attach_list', function() {
  _this = this;
  name = _this.parentElement.querySelector(".list_name").innerHTML;
  pk = _this.getAttribute('data-pk');
  count = _this.parentElement.querySelector(".count").innerHTML;
  if (document.body.querySelector(".current_file_dropdown")){
    check_good_list_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, pk) ? null : (good_list_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, name, pk, count), close_work_fullscreen())
  } else if (document.body.querySelector(".attach_block")){
    check_good_list_in_block(document.body.querySelector(".attach_block"), _this, pk) ? null : (good_list_post_attach(document.body.querySelector(".attach_block"), name, pk, count), close_work_fullscreen())
  } else if (document.body.querySelector(".message_attach_block")){
    check_good_list_in_block(document.body.querySelector(".message_attach_block"), _this, pk) ? null : (good_list_message_attach(document.body.querySelector(".message_attach_block"), name, pk, count), close_work_fullscreen())
  }
});

on('#ajax', 'click', '.commmunty_load_one', function() {
  _this = this;
  block = _this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  commmunity_form_selected(_this, block.querySelector("#selected_message_target_items"))
});
on('#ajax', 'click', '.chat_item_load_one', function() {
  _this = this;
  block = _this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  chat_item_form_selected(_this, block.querySelector("#selected_message_target_items"))
});
on('#ajax', 'click', '.chat_friends_load_one', function() {
  _this = this;
  block = this.parentElement.parentElement.nextElementSibling;
  chat_item_form_selected(_this, block)
});

on('#ajax', 'click', '.attach_survey', function() {
  _this = this;
  pk = _this.getAttribute('data-pk');
  figure = _this.querySelector(".background-img");
  figure.querySelector("src") ? img_src = figure.querySelector("src").getAttribute("src") : img_src = null;
  container_html = _this.querySelector(".container").innerHTML;
  if (document.body.querySelector(".current_file_dropdown")){
    check_survey_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, pk) ? null : (survey_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, pk, img_src, container_html), close_work_fullscreen())
  } else if (document.body.querySelector(".attach_block")){
    check_survey_in_block(document.body.querySelector(".attach_block"), _this, pk) ? null : (survey_post_attach(document.body.querySelector(".attach_block"), pk, img_src, container_html), close_work_fullscreen())
  } else if (document.body.querySelector(".message_attach_block")){
    check_survey_in_block(document.body.querySelector(".message_attach_block"), _this, pk) ? null : (survey_message_attach(document.body.querySelector(".message_attach_block"), pk, img_src, container_html), close_work_fullscreen())
  }
});
