
on('#ajax', 'keydown', '.search_main_form', function(e) {
  if (e.keyCode == 13) {
    e.preventDefault();
    section = "";
    value = this.value.replace("#", "%23");
    left_panel = document.body.querySelector(".search_panel");
    left_panel_options = left_panel.querySelectorAll(".search_ajax");
    for (var i = 0; i < left_panel_options.length; i++){
      url = left_panel_options[i].getAttribute("href");
      params = url.replace( '?', '').split('&');
      new_url = url.replace(params[1].split("=")[1], value);
      left_panel_options[i].setAttribute("href", new_url);
      if (left_panel_options[i].classList.contains(".active")) {
        section = params[0].split("=")[1];
      }
    };
    window.history.replaceState(null, null, new_url);

    var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    ajax_link.open('GET', '/search/?s=' + section + '&q=' + value, true);
    ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    ajax_link.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            elem_ = document.createElement('span');
            elem_.innerHTML = ajax_link.responseText;
            container = document.body.querySelector(".load_search_container");
            container.innerHTML = elem_.querySelector(".load_search_container").innerHTML;
        }
    }
    ajax_link.send()
}
});

on('#ajax', 'click', '.load_next_list_comments', function() {
  _this = this;
    var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    ajax_link.open('GET', _this.getAttribute("data-link"), true);
    ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    ajax_link.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            elem_ = document.createElement('span');
            elem_.innerHTML = ajax_link.responseText;

            elem_2 = document.createElement('span');
            elem_2.innerHTML = elem_.querySelector(".stream_comments").innerHTML
            _this.parentElement.append(elem_2);
            _this.remove();
            fullscreen_resize();
        }
    }
    ajax_link.send()
});
on('#ajax', 'click', '.toggle_visible_owner_lists', function() {
  this.parentElement.nextElementSibling.querySelector(".owner_lists").classList.toggle("hide");
})

on('#ajax', 'click', '.show_post_text', function() {
  shower = this.parentElement.querySelector(".show_post_text");
  shower.nextElementSibling.nextElementSibling.style.display = "unset";
  shower.nextElementSibling.remove();
  shower.previousElementSibling.remove();
  shower.remove();
});

on('#ajax', 'click', '.select_tab_content_universal', function() {
  _this = this;
  if (_this.classList.contains("active")) {
    return
  }
  parent = _this.parentElement.parentElement;
  nav_items = parent.querySelectorAll(".nav-link");
  for (var i = 0; i < nav_items.length; i++) {
    nav_items[i].classList.remove("active");
  }

  var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  ajax_link.open('GET', _this.getAttribute("data-link"), true);
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  ajax_link.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;

          parent.nextElementSibling.innerHTML = elem_.querySelector(".load_block").innerHTML;
          _this.classList.add("active");
      }
  }
  ajax_link.send();
});

on('#ajax', 'click', '.hide_comment_form', function() {
  block = this.parentElement.parentElement.parentElement.parentElement.parentElement
  block.querySelector(".col").style.display = "block";
  block.querySelector(".comment_text").style.display = "block";
  block.querySelector(".attach_container") ? block.querySelector(".attach_container").style.display = "block" : null;
  this.parentElement.parentElement.parentElement.remove();
});

on('#ajax', 'resize', '#fullscreen_loader', function() {
  console.log("resize!");
});

on('#ajax', 'click', '.next_shower_btn', function() {
  this.nextElementSibling.classList.toggle("hide");
});
on('#ajax', 'click', '.select_item_reaction', function() {
  btn = this.parentElement.previousElementSibling;
  counter_ru = btn.querySelector(".reactions_count_ru");
  counter = btn.querySelector(".reactions_count");
  count = counter.innerHTML*1;
  react_input = this.querySelector(".switch-sm");
  reactions_input = btn.previousElementSibling;
  if (react_input.checked == 1){
      count -= 1;
      counter.innerHTML = count;
      counter_ru.innerHTML = get_count_ru_alt(count, "реакция", "реакции", "реакций");
      react_input.removeAttribute('checked');
    } else {
      count += 1;
      counter.innerHTML = count;
      counter_ru.innerHTML = get_count_ru_alt(count, "реакция", "реакции", "реакций");
      react_input.setAttribute('checked', 'true');
    }
});

on('#ajax', 'click', '.smile_dropdown', function() {
  block = this.nextElementSibling;
  if (!block.querySelector(".card")) {
    list_load(block, "/users/load/smiles/")
  };
  block.classList.toggle("show");
});

on('body', 'click', '.comment_btn', function() {
  form = this.parentElement.parentElement.parentElement;
  type = this.getAttribute('data-types');
  pk = type.slice(3);
  if (type.indexOf('pos') !== -1) {
    url = "/posts/add_comment/" + pk + "/";
  }
  else if (type.indexOf('goo') !== -1) {
    url = "/goods/add_comment/" + pk + "/";
  }
  else if (type.indexOf('pho') !== -1) {
    url = "/photos/add_comment/" + pk + "/";
  }
  else if (type.indexOf('vid') !== -1) {
    url = "/video/add_comment/" + pk + "/";
  }
  send_comment(form, form.parentElement.previousElementSibling, url);
});

on('body', 'click', '.reply_comment_btn', function() {
  form = this.parentElement.parentElement.parentElement;
  block = form.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement
  type = this.getAttribute('data-types');
  pk = type.slice(3);
  if (type.indexOf('cpo') !== -1) {
    url = "/posts/add_reply/" + pk + "/";
  }
  else if (type.indexOf('cgo') !== -1) {
    url = "/goods/add_reply/" + pk + "/";
  }
  else if (type.indexOf('cph') !== -1) {
    url = "/photos/add_reply/" + pk + "/";
  }
  else if (type.indexOf('cvi') !== -1) {
    url = "/video/add_reply/" + pk + "/";
  }

  send_comment(form, block, url);

  form.parentElement.style.display = "none";
  block.classList.add("replies_open")
});

on('body', 'click', '.reply_parent_btn', function() {
  form = this.parentElement.parentElement.parentElement.parentElement;
  block = form.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  send_comment(form, block, '/users/progs/reply_comment/')

  form.parentElement.style.display = "none";
  block.classList.add("replies_open");
});

on('#ajax', 'click', '.tab_smiles', function() {
  if (!this.classList.contains("active")) {
    parent = this.parentElement.parentElement.parentElement;
    parent.querySelector(".stickers_panel").classList.remove("active", "show");
    parent.querySelector(".smiles_panel").classList.add("active", "show");
    this.classList.add("active");
    this.parentElement.querySelector(".tab_stickers").classList.remove("active");
  }
});
on('#ajax', 'click', '.tab_stickers', function() {
  if (!this.classList.contains("active")) {
    parent = this.parentElement.parentElement.parentElement;
    parent.querySelector(".smiles_panel").classList.remove("active", "show");
    parent.querySelector(".stickers_panel").classList.add("active", "show");
    this.classList.add("active");
    this.parentElement.querySelector(".tab_smiles").classList.remove("active");
  }
});

on('#ajax', 'click', '.previous_click', function() {
  this.previousElementSibling.click();
});
on('body', 'click', '.menu_drop', function() {
  block = this.nextElementSibling;
  if (block.classList.contains("show")) { block.classList.remove("show") }
  else {
  all_drop = document.body.querySelectorAll(".dropdown-menu");
  for(i=0; i<all_drop.length; i++) {
    all_drop[i].classList.remove("show")
  } block.classList.add("show")}
});
on('body', 'click', '.menu_drop_2', function() {
  block = this.nextElementSibling.nextElementSibling;
  if (block.classList.contains("show")) { block.classList.remove("show") }
  else {
  all_drop = document.body.querySelectorAll(".dropdown-menu");
  for(i=0; i<all_drop.length; i++) {
    all_drop[i].classList.remove("show")
  } block.classList.add("show")}
});

on('body', 'click', '.user_nav_button', function() {
  document.body.querySelector(".settings_block_hide") ? (settings_block = document.body.querySelector(".settings_block_hide"),settings_block.classList.add("settings_block_show"),settings_block.classList.remove("settings_block_hide"))
  : (settings_block = document.body.querySelector(".settings_block_show"),settings_block.classList.add("settings_block_hide"),settings_block.classList.remove("settings_block_show"))
});
on('body', 'click', '.search_ajax', function(event) {
  event.preventDefault();
  var url = this.getAttribute('href');
  if (url != window.location.href){
    search_ajax_get_reload(url);
    search_panel = document.body.querySelector(".search_panel");
    items = search_panel.querySelectorAll(".search_ajax");
    for (var i = 0; i < items.length; i++){
      items[i].classList.remove("active")
    };
    if (this.getAttribute("data-left-a")) {
      search_panel.querySelector("." + this.getAttribute("data-left-a")).classList.add("active")
    } else {this.classList.add("active")}
  }
  else {toast_info("Список уже получен...")}
});

on('body', 'click', '.ajax', function(event) {
  event.preventDefault();
  var url = this.getAttribute('href');
  if (url != window.location.pathname){
    ajax_get_reload(url);
  } else {toast_info("Вы уже на этой странице")}
});
on('body', 'click', '.notify_ajax', function(event) {
  event.preventDefault();
  _this = this;
  url = _this.getAttribute('href');

  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  ajax_link.open('GET', url, true);
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  ajax_link.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
          // если есть блок с классом "user_notify_block", то пользователь на странице видит блоки уведомлений.
          // и, если есть у блока (в который переходит пользователь) непрочитанные уведомления, нужно убавить общий счетчик уведомлений на число этого блока
          if (document.body.querySelector(".user_notify_block")){
            _this.parentElement.classList.contains("card-body") && _this.querySelector(".tab_badge") ? (_count = _this.querySelector(".tab_badge").innerHTML.replace(/\s+/g, ''),
                                                             _count = _count*1,
                                                             notify = document.body.querySelector(".new_unread_notify"),
                                                             all_count = notify.querySelector(".tab_badge").innerHTML.replace(/\s+/g, ''),
                                                             all_count = all_count*1,
                                                             result = all_count - _count,
                                                             result > 0 ? notify.querySelector(".tab_badge").innerHTML = result : notify.innerHTML = '',
                                                             console.log("Вычитаем основной счетчик")
                                                           ) : null;
          }
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
          ajax = elem_.querySelector("#reload_block");
          rtr = document.getElementById('ajax');
          rtr.innerHTML = ajax.innerHTML;
          window.scrollTo(0, 0);
          title = elem_.querySelector('title').innerHTML;
          window.history.pushState(null, "vfgffgfgf", url);
          document.title = title;
          loaded = false;
          create_pagination(rtr);
          if (rtr.querySelector(".user_all_notify_container")) {
            document.body.querySelector(".new_unread_notify").innerHTML = "";
            console.log("Обнуляем основной счетчик")
          }
      }
  }
  ajax_link.send()
});


on('body', 'click', '.next_item', function(event) {
  event.preventDefault();
  this.style.display = "none";
  change_this_fullscreen(this, document.getElementById('item_loader'));
});
on('body', 'click', '.prev_item', function(event) {
  event.preventDefault();
  this.style.display = "none";
  change_this_fullscreen(this, document.getElementById('item_loader'));
});

on('#ajax', 'click', '.item_stat_f', function() {
  var parent, pk, uuid, loader
  parent = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  uuid = parent.getAttribute("data-uuid");
  create_fullscreen("/stat/item/" + uuid + "/", "item_fullscreen", false, true);
});

on('#ajax', 'click', '.item_fullscreen_hide', function() {
  close_work_fullscreen();
});
on('body', 'click', '.video_fullscreen_hide', function() {get_document_opacity_1();document.querySelector(".video_fullscreen").style.display = "none";document.getElementById("video_loader").innerHTML=""});
on('body', 'click', '.small_video_fullscreen_hide', function() {
  document.querySelector(".video_fullscreen").style.display = "none";
  video_window = document.querySelector(".video_fullscreen");
  get_document_opacity_1();
  video_window.classList.remove("video_fullscreen_resized", "draggable");
  document.body.querySelector(".video_btn_small").style.display = "none";
  document.body.querySelector(".video_btn_big").style.display = "block";
  document.getElementById("video_loader").innerHTML=""
});

on('#ajax', 'click', '.show_replies', function() {
  this.nextElementSibling.classList.toggle('replies_open');
});

on('body', 'click', '.reply_comment', function() {
  div = this.nextElementSibling.nextElementSibling;
  input = div.querySelector(".comment_text");
  input.innerHTML = this.previousElementSibling.innerHTML + ',&nbsp;';
  div.style.display = "block";
  focus_block(input)
});
function focus_block(value) {
  range = document.createRange();
  range.selectNodeContents(value);
  range.collapse(false);
  sel = window.getSelection();
  sel.removeAllRanges();
  sel.addRange(range);
};

function private_users_send(form_post, url, action, val) {
  form = new FormData(form_post);
  form.append("action", action);
  form.append("value", val);
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', url, true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    form_post.querySelector(".collector_active").innerHTML = "";
    toast_success("Настройки изменены")
  }};
  link_.send(form);
}

on('#ajax', 'click', '.select_perm_dropdown', function() {
  val = this.getAttribute("data-value"), _this = this, is_new_value = true;
  action = this.parentElement.getAttribute("data-action");
  _this.parentElement.classList.remove("show");

  form_post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  collectors = form_post.querySelectorAll(".collector");
  for (var i = 0; i < collectors.length; i++){
    collectors[i].classList.remove("collector_active")
  };
  parent_2 = this.parentElement.parentElement;
  collector = parent_2.querySelector(".collector");
  current_option = parent_2.querySelector(".menu_drop_2");
  current_option.innerHTML = _this.innerHTML;
  collector.classList.add("collector_active");

  if (form_post.classList.contains("case_edit")) {
    // если мы имеем дело с изменением приватности элемента, который
    // уже есть, поэтому можем сразу сохранять приватность

    if (form_post.classList.contains("chat_edit")) {
      // работаем с приватностью пользовательского чата
      if (val == 'd') {
        create_fullscreen("/chat/user_progs/load_exclude_users/" + form_post.getAttribute("data-pk") + "/?action=" + action, "worker_fullscreen");
      }
      else if (val == 'e') {
        create_fullscreen("/chat/user_progs/load_include_users/" + form_post.getAttribute("data-pk") + "/?action=" + action, "worker_fullscreen");
      }
      else {
        if (is_new_value) {
          private_users_send(form_post, "/chat/user_progs/private/" + form_post.getAttribute("data-pk") + "/", action, val)
        }
      }
  }
  else if (form_post.classList.contains("type_profile")) {
    // работаем с приватностью профиля пользователя
    if (val == 'e') {
      create_fullscreen("/users/settings/load_exclude_users/?action=" + action, "worker_fullscreen");
    }
    else if (val == 'f') {
      create_fullscreen("/users/settings/load_include_users/?action=" + action, "worker_fullscreen");
    }
    else {
      if (is_new_value) {
        private_users_send(form_post, "/users/settings/private/", action, val)
      }
    }
  }
  else if (form_post.classList.contains("type_community")) {
    // работаем с приватностью профиля пользователя
    c_pk = form_post.getAttribute("data-pk");
    if (val == 'h') {
      create_fullscreen("/communities/manage/load_exclude_users/" + c_pk + "/?action=" + action, "worker_fullscreen");
    }
    else if (val == 'i') {
      create_fullscreen("/communities/manage/load_include_users/" + c_pk + "/?action=" + action, "worker_fullscreen");
    }
    else {
      if (is_new_value) {
        private_users_send(form_post, "/communities/manage/private/" + c_pk + "/", action, val)
      }
    }
  }
}
  else if (form_post.classList.contains("case_create")) {
    // мы меняем приватность элемента, которого еще нет, поэтому не можем
    // на лету сохранять изменение приватности. Мы должны оформить исключения или
    // назначения в виде post полей и разобрать их при создании элемента
    // например, при создании списка записей

    input = parent_2.querySelector(".input");
    if (input.value == val) {
      is_new_value = false
    };
    input.setAttribute("value", val);

      if (val == 'e') {
        create_fullscreen("/users/load/list_exclude_users/?action=" + action + "&target=user&list=" + form_post.getAttribute("data-list"), "worker_fullscreen")
      }
      else if (val == 'f') {
        create_fullscreen("/users/load/list_include_users/?action=" + action + "&target=user&list=" + form_post.getAttribute("data-list"), "worker_fullscreen")
      }
      else if (val == 'h') {
        create_fullscreen("/users/load/list_exclude_users/?action=" + action + "&community_pk=" + form_post.getAttribute("community-pk") + "&list=" + form_post.getAttribute("data-list"), "worker_fullscreen")
      }
      else if (val == 'i') {
        create_fullscreen("/users/load/list_include_users/?action=" + action + "&community_pk=" + form_post.getAttribute("community-pk") + "&list=" + form_post.getAttribute("data-list"), "worker_fullscreen")
      }
      else {
        collector.innerHTML = ""
      }
  }
});

on('#ajax', 'click', '#add_list_selected_users_btn', function() {
  form = this.parentElement.parentElement;
  form.querySelector(".form_btn").disabled = true;
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
});

on('#ajax', 'click', '#add_list_selected_lists_btn', function() {
  form = this.parentElement;
  form.querySelector(".form_btn").disabled = true;
  collector = document.body.querySelector(".collector_active");
  users_block = form.querySelector(".card-header");
  users_list = users_block.querySelectorAll(".custom_color");
  final_list = "Выбраны списки: ";
  for (var i = 0; i < users_list.length; i++){
    users_list[i].querySelector("span").remove();
    users_list[i].querySelector(".list_pk").remove();
    final_list += users_list[i].innerHTML;
    final_list += '<input type="hidden" name="lists" value="' + users_list[i].getAttribute("data-pk") + '" />'
  };
  collector.innerHTML = "";
  collector.innerHTML = final_list;
  form.classList.remove("cool_private_form");
  close_work_fullscreen();
});

on('#ajax', 'click', '#add_list_selected_chats_items_btn', function() {
  form = this.parentElement.parentElement;
  form.querySelector(".form_btn").disabled = true;
  collector = document.body.querySelector(".collector_active");
  users_block = form.querySelector(".card-header");
  users_list = users_block.querySelectorAll(".custom_color");
  final_list = "Выбраны получатели: ";
  for (var i = 0; i < users_list.length; i++){
    users_list[i].querySelector("span").remove();
    users_list[i].querySelector(".list_pk").remove();
    final_list += users_list[i].innerHTML;
    final_list += '<input type="hidden" name="chat_items" value="' + users_list[i].getAttribute("data-pk") + '" />'
  };
  collector.innerHTML = "";
  collector.innerHTML = final_list;
  form.classList.remove("cool_private_form");
  close_work_fullscreen();
});

on('#ajax', 'click', '#add_list_selected_communities_btn', function() {
  form = this.parentElement.parentElement;
  form.querySelector(".form_btn").disabled = true;
  collector = document.body.querySelector(".collectorrr");
  users_block = form.querySelector(".card-header");
  users_list = users_block.querySelectorAll(".custom_color");
  final_list = "Выбраны сообщества: ";
  for (var i = 0; i < users_list.length; i++){
    users_list[i].querySelector("span").remove();
    users_list[i].querySelector(".list_pk").remove();
    final_list += users_list[i].innerHTML;
    final_list += '<input type="hidden" name="u_c" value="' + users_list[i].getAttribute("data-pk") + '" />'
  };
  collector.innerHTML = "";
  collector.innerHTML = final_list;
  form.classList.remove("cool_private_form");
  close_work_fullscreen();
});
