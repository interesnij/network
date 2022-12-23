function get_dragula(block) {
  // функция инициирует библиотеку dragula.js
  _block = document.body.querySelector(block)
  dragula([_block], {
    moves: function (el, container, handle) {
      return handle.classList.contains('handle')
    }})
    //.on('drag', function (el) {console.log("drag!");})
    .on('drop', function (el) {change_position(el)})
    //.on('over', function (el, container) {console.log("over!"); over = true;})
    //.on('out', function (el, container) {console.log("over!");;});
};

function create_tracks_keys() {
  var playlist = document.body.querySelector("#saved_playlist");
  var tracks = playlist.querySelectorAll("li");
  list = [];
  for (var i = 0; i < tracks.length; i++) {
    list.push(tracks[i].getAttribute("data-track-id"));
  }
  playlist.setAttribute("data-ids", list);
}
create_tracks_keys();

var $serf_history = [], $new_window_list = [], $new_elements = [];
var user_info = document.body.querySelector(".userpic");
var $request_user_id = user_info.getAttribute("data-id");
var $user_device = user_info.getAttribute("data-device");
page_time = false, $new_time = 0;

get_dragula(".drag_container");
get_dragula(".drag_list");
document.title = document.querySelector(".main-container").getAttribute("data-title");

function create_window_stat_list(block) {
  if ($new_window_list.length) {
    push_window_stat_list()
  };
  if (block.querySelector(".is_stat_list")) {
    item = block.querySelector(".is_stat_list");
    main_container = document.body.querySelector(".main-container");
    $new_window_list = [item.getAttribute("data-title"),"url",0,0, main_container.getAttribute("data-title"),"url",$request_user_id, $user_device, new Date().toLocaleString().replace(",", "")]
  }
};

function add_list_in_all_stat(item_type,item_pk,main_type,main_pk) {
  $all_stat.push(item_type + ";" + item_pk + ";;;" + main_type + ";" + main_pk + ";" + $request_user_id + ";" + $user_device + ";" + new Date().toLocaleString().replace(",", ""));
};

function push_window_stat_list() {
  el_list_stat = $new_window_list[0] + ";" + $new_window_list[1] + ";" + $new_window_list[2] + ";" + $new_window_list[3] + ";" + $new_window_list[4] + ";" + $new_window_list[5] + ";" + $new_window_list[6] + ";" + $new_window_list[7] + ";" + $new_window_list[8];
  $all_stat.push(el_list_stat);
  $new_window_list = [];
};

function close_fullscreen() {
  container = document.body.querySelector("#fullscreens_container");
  video_container = document.body.querySelector("#video_loader");
  if (!container.innerHTML && video_container.innerHTML) {
    video_container.innerHTML == "";
    video_container.parentElement.style.display = "none";
    get_document_opacity_1();

    return
  };
  toggle_active_select = false;
  _window = container.querySelector(".card_fullscreen");
  try {
    if (_window.querySelector(".cool_private_form") && !_window.querySelector(".remove_user_input")) {
      toggle_active_select = true;
    };
    _window.remove();
    if (toggle_active_select) {
      settings_window = container.querySelector(".card_fullscreen");
      collector_active = settings_window.querySelector(".collector_active");
      select = collector_active.previousElementSibling;
      select.value = select.getAttribute("data-value");
    }
  }
  catch { _window.remove() }
  if (!container.innerHTML) {
    get_document_opacity_1();
    push_window_stat_list();
    if (document.body.querySelector(".main-container")) {
      document.title = document.body.querySelector(".main-container").getAttribute("data-title");
    }
  } else {
    create_window_stat_list(container.querySelector(".card_fullscreen"));
    // включаем видимость кнопок нового первого окна.
    prev_window = container.querySelector(".card_fullscreen");
    prev_window.classList.remove("hide");

    if (container.querySelector(".data-title")) {
      document.title = container.querySelector(".data-title").getAttribute("data-title");
    }
  };
  window.history.replaceState(null, null, window.location.pathname);
  all_windows = container.querySelectorAll(".card_fullscreen");
  for (var i = 0; i < all_windows.length; i++) {
    if (!all_windows[i].querySelector("#fullscreen_loader").firstChild) {
      all_windows[i].remove();
    }
  }
  if (!container.firstChild) {
    get_document_opacity_1();
  }
};
function close_work_fullscreen() {
  toggle_active_select = false;
  container = document.body.querySelector("#fullscreens_container");
  _window = container.querySelector(".card_fullscreen");
  try{
  if (_window.querySelector(".cool_private_form") && !_window.querySelector(".remove_user_input")) {
    toggle_active_select = true;
  };

  _window.remove();
  if (container.querySelector(".data-title")) {
    document.title = container.querySelector(".data-title").getAttribute("data-title");
  }
  if (toggle_active_select) {
    settings_window = container.querySelector(".card_fullscreen");
    collector_active = settings_window.querySelector(".collector_active");
    try{
      select = collector_active.previousElementSibling;
      select.value = select.getAttribute("data-value")
    } catch { null }
  }
  } catch { _window.remove() }

  if (!container.innerHTML) {
    get_document_opacity_1();
  };
  window.history.replaceState(null, null, window.location.pathname);
  all_windows = container.querySelectorAll(".card_fullscreen");
  for (var i = 0; i < all_windows.length; i++) {
    if (!all_windows[i].querySelector("#fullscreen_loader").firstChild) {
      all_windows[i].remove();
    }
  }
  if (!container.firstChild) {
    get_document_opacity_1();
  }
};

function view_timer(count, list) {
    var i = 0;
    setInterval(() => {
      if (i == count && document.body.querySelector(".card_fullscreen")) {
        document.body.querySelector(".card_fullscreen").classList.add("count_done");
        return;
      };
    list[3] += 1;
    }, 1000);
};

function create_fullscreen(url, type_class, need_drag_items, need_replace_history) {
  container = document.body.querySelector("#fullscreens_container");

  try {
    if (document.body.querySelector(".video_fullscreen").style.display =="none") {
      count_items = container.querySelectorAll(".card_fullscreen").length
    } else {
      count_items = container.querySelectorAll(".card_fullscreen").length + 1
    }
  }
  catch {count_items = 0};

  $parent_div = document.createElement("div");
  $parent_div.classList.add("card_fullscreen", "mb-3", "border", type_class);
  $parent_div.style.zIndex = 100 + count_items;
  $parent_div.style.opacity = "0";

  if (document.body.querySelector(".desctop_nav")) {
    hide_svg = '<svg class="svg_default" style="position:fixed;" width="30" height="30" fill="currentColor" viewBox="0 0 24 24"><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/><path d="M0 0h24v24H0z" fill="none"/></svg>'
  } else { hide_svg = "" };
  $hide_span = document.createElement("span");
  $hide_span.classList.add("this_fullscreen_hide");

  _page_time = false;

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  link.open('GET', url, true);
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
        if (container.innerHTML) {
          prev_window = container.querySelector(".card_fullscreen");
          prev_window.classList.add("hide");
        };

        $loader = document.createElement("div");

        $loader.setAttribute("id", "fullscreen_loader");
        $hide_span.innerHTML = hide_svg;
        $parent_div.append($hide_span);
        $parent_div.append($loader);
        $parent_div.append(create_gif_loading ());
        container.prepend($parent_div);

          $load_div.remove();

          elem = link.responseText;

          $loader.innerHTML = elem;
          height = $loader.scrollHeight*1 + 30;
          if (height < 500 && !$loader.querySelector(".data_display")) {
            $parent_div.style.height = height + "px";
            $loader.style.overflowY = "unset";

            _height = (window.innerHeight - height - 50) / 2;
            $parent_div.style.top = _height + "px";
            prev_next_height = _height*1 + 50 + "px";
            try {$loader.querySelector(".prev_item").style.top = "-" + prev_next_height}catch {null};
            try {$loader.querySelector(".next_item").style.top = "-" + prev_next_height}catch {null}
          } else {
            $parent_div.style.height = "100%";
            $parent_div.style.top = "15px";
            $loader.style.overflowY = "auto";
          };
          $parent_div.style.opacity = "1";
          if ($loader.querySelector(".data_display")) {
            $loader.style.overflowY = "unset";
          }

          get_document_opacity_0();

          // создаем временный список или элемент окна
          create_window_stat_list($loader);

          // добавляем все элементы списка, как и все на основной странице, таким же путем
          append_items_in_stat_list($loader, $new_elements);
          if (!_page_time) {
            view_timer(120, $new_window_list)
            _page_time = true;
          };
          offset = 0;

          $loader.onscroll = function() {
            window_scrollStopper();
            if ($loader.parentElement.classList.contains("count_done")) {
              $loader.parentElement.classList.remove("count_done");
              _page_time = false;
              view_timer(120, $new_window_list)
              _page_time = true;
            };

            if ($loader.querySelector(".next_page_list")) {
              box = $loader.querySelector('.next_page_list');
              if (box && box.classList.contains("next_page_list")) {
                  inViewport = elementInViewport(box);
                  if (inViewport) {
                      box.remove();
                      var link_3 = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
                      link_3.open('GET', location.protocol + "//" + location.host + box.getAttribute("data-link"), true);
                      link_3.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

                      link_3.onreadystatechange = function() {
                          if (this.readyState == 4 && this.status == 200) {
                              var elem = document.createElement('span');
                              elem.innerHTML = link_3.responseText;
                              $loader.querySelector(".is_block_paginate").insertAdjacentHTML('beforeend', elem.querySelector(".is_block_paginate").innerHTML);
                            }
                      }
                      link_3.send();
                  }
              };
            };

            if ($loader.scrollHeight  > offset) {
              offset = $loader.scrollHeight;
              $new_window_list[3] = parseFloat(offset * 0.000264).toFixed(2);
            }
          };
          if (need_drag_items) {
            console.log("need_drag_items!");
            get_dragula(".worker_drag_container")
          }
          get_music_player_support($loader);

          if ($loader.querySelector(".data-title")) {
            document.title = $loader.querySelector(".data-title").getAttribute("data-title");
          }
          if (need_replace_history) {
            window.history.pushState(null, "vfgffgfgf", window.location.href + "?url=" + url + "&screen=" + type_class);
          }
      }
  };
  link.send();
};

function change_this_fullscreen(_this, type_class) {
  _this.parentElement.classList.contains("col") ?
    $loader = _this.parentElement.parentElement.parentElement.parentElement
    : $loader = _this.parentElement.parentElement.parentElement;
  $loader.innerHTML = "";
  $parent_div.style.opacity = "0";
  $parent_div.style.height = "35px";
  url = _this.getAttribute("href");
  $serf_history.push(url);

  _page_time = false;

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  link.open('GET', url, true);
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
          elem = link.responseText;
          $loader.innerHTML = elem;
          height = $loader.scrollHeight*1 + 30;
          $parent_div = $loader.parentElement
          if (height < 500 && !$loader.querySelector(".data_display")){
            $parent_div.style.height = height + "px";
            _height = (window.innerHeight - height - 50) / 2;
            $parent_div.style.top = _height + "px";
            prev_next_height = _height*1 + 50 + "px";
            $loader.style.overflowY = "unset";
            try {$loader.querySelector(".prev_item").style.top = "-" + prev_next_height}catch {null};
            try {$loader.querySelector(".next_item").style.top = "-" + prev_next_height}catch {null}
          } else {
            $parent_div.style.height = "100%";
            $parent_div.style.top = "15px";
            $loader.style.overflowY = "auto";
          };
          $parent_div.style.opacity = "1";
          $parent_div.style.opacity = "1";
          if ($loader.querySelector(".data_display")) {
            $loader.style.overflowY = "unset";
          };

          get_document_opacity_0();

          create_window_stat_list($loader);
          append_items_in_stat_list($loader, $new_elements);
          get_music_player_support($loader);

          if ($loader.querySelector(".data-title")) {
            document.title = $loader.querySelector(".data-title").getAttribute("data-title");
          }

          if (!_page_time) {
            view_timer(120, $new_window_list)
            _page_time = true;
          };
          offset = 0;

          $loader.onscroll = function() {
            window_scrollStopper();
            if ($loader.parentElement.classList.contains("count_done")) {
              $loader.parentElement.classList.remove("count_done");
              _page_time = false;
              view_timer(120, $new_window_list)
              _page_time = true;
            };

            if ($loader.querySelector(".next_page_list")) {
              box = $loader.querySelector('.next_page_list');
              if (box && box.classList.contains("next_page_list")) {
                  inViewport = elementInViewport(box);
                  if (inViewport) {
                      box.remove();
                      var link_3 = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
                      link_3.open('GET', location.protocol + "//" + location.host + box.getAttribute("data-link"), true);
                      link_3.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

                      link_3.onreadystatechange = function() {
                          if (this.readyState == 4 && this.status == 200) {
                              var elem = document.createElement('span');
                              elem.innerHTML = link_3.responseText;
                              $loader.querySelector(".is_block_paginate").insertAdjacentHTML('beforeend', elem.querySelector(".is_block_paginate").innerHTML);
                            }
                      }
                      link_3.send();
                  }
              };
            };

            if ($loader.scrollHeight  > offset) {
              offset = $loader.scrollHeight;
              $new_window_list[3] = parseFloat(offset * 0.000264).toFixed(2);
            };
            window.history.pushState(null, "vfgffgfgf", window.location.href + "?url=" + url + "&screen=" + type_class);
          }
      }
  };
  link.send();
};

var delayedExec = function(after, fn) {
    var timer;
    return function() {
        timer && clearTimeout(timer);
        timer = setTimeout(fn, after);
    };
};

function append_items_in_stat_list(block, list) {
  _list = block.querySelectorAll('.pag');
  main_container = document.body.querySelector(".main-container");
  for (var i = 0; i < _list.length; i++) {
      if (!_list[i].classList.contains("showed")) {
          inViewport = elementInViewport(_list[i]);
          if (inViewport) {
            if (i == 1) {
              get_el_view_time(120);
            };

            pk = _list[i].getAttribute('data-pk');
            type = _list[i].getAttribute('data-type');
            if ($all_stat.indexOf(type + " " + pk) == -1 && $new_elements.indexOf(pk + " " + type) == -1) {
              list.push([
                type,
                pk,
                0,
                main_container.getAttribute("data-title"),
                "url",
                $request_user_id,
                $user_device,
                new Date().toLocaleString().replace(",", "")
              ]);
              console.log(list);
            };
            _list[i].classList.add("showed");
          }
      }
  };
};

var window_scrollStopper = delayedExec(3000, function() {
    try {
      append_items_in_stat_list(document.body.querySelector(".card_fullscreen"), $new_elements)
    } catch {null};
});

// высота экрана в переаводе на метры
$window_height = parseFloat(window.innerHeight * 0.000264).toFixed(2);

// $all_stat = список, в который попадают все элементы статистики для отправки на сервер
$all_stat = [];

// $page_stat = список страницы.
$page_stat = [];

// $page_stat = список списка полльзователя или сообщества в их разделах,
// так как прочие варианты отображения обработаем отдельно в окнах.
$list_stat = [];

// инициализируем временные списки для сбора статистики
init_stat_lists(document.body.querySelector(".main-container"), '', '');

function init_stat_lists(next_block, prev_type, prev_pk) {
  if ($page_stat.length) {
  el_page_stat = $page_stat[0] + ";" + $page_stat[1] + ";" + $page_stat[2] + ";" + $page_stat[3] + ";" + $page_stat[4] + ";" + $page_stat[5] + ";" + $page_stat[6] + ";" + $page_stat[7] + ";" + $page_stat[8];
  $all_stat.push(el_page_stat);
  };
  if ($list_stat.length) {
    el_list_stat = $list_stat[0] + ";" + $list_stat[1] + ";" + $list_stat[2] + ";" + $list_stat[3] + ";" + $list_stat[4] + ";" + $list_stat[5] + ";" + $list_stat[6] + ";" + $list_stat[7] + ";" + $list_stat[8];
    $all_stat.push(el_list_stat);
  };

  $page_stat = [next_block.getAttribute("data-type"), next_block.getAttribute("data-pk"), $window_height, 0, $request_user_id, prev_type, prev_pk, $user_device, new Date().toLocaleString().replace(",", "")];
  $list_stat = [];

  append_items_in_stat_list(next_block, $new_elements);

  console.log("Обнуляем списки и обновляем основной список стата");
  get_page_view_time(120);
  page_time = true;
};

function get_page_view_time(count) {
  // считаем время нахождения на странице, до 2х минут. При скролле перезапускаем.
  if (page_time) {
    return
  }
  console.log("Общее время страницы работает");
  i = 0;
  if (i < count) {
    setInterval(() => append_page_time_in_lists(), 1000);
    i += 1
  } else {page_time = false;};
};

function append_page_time_in_lists() {
  // добавляем секунды просмотра страницы и списка, если он есть
  if ($page_stat.length) {
    $page_stat[3] += 1
  };
  if ($list_stat.length) {
    $list_stat[3] += 1
  }
};

function get_el_view_time(count) {
  console.log("Счетчик времени элементов запущен");
  t = 0;
  if (t < count) {
    setInterval(() => $new_time +=1 , 1000);
    t += 1
  }
};

window.onbeforeunload = function() {
  console.log("Reload");
};

function reload_list_stat() {
  if ($list_stat.length) {
    el_list_stat = $list_stat[0] + ";" + $list_stat[1] + ";" + $list_stat[2] + ";" + $list_stat[3] + ";" + $list_stat[4] + ";" + $list_stat[5] + ";" + $list_stat[6] + ";" + $list_stat[7] + ";" + $list_stat[8];
    $all_stat.push(el_list_stat)
  };
  block = document.body.querySelector(".main-container");
  list = block.querySelector(".is_stat_list");
  //$list_stat = [list.getAttribute("data-type"), list.getAttribute("data-pk"), 0, 0, block.getAttribute("data-pk"), block.getAttribute("data-type"),$request_user_id, $user_device, new Date().toLocaleString().replace(",", "")];
};

var scrollStopper = delayedExec(3000, function() {
    try {
      main_container = document.body.querySelector(".main-container");
      if (main_container.querySelector(".is_stat_list") && !$list_stat.length) {
        pag_list = main_container.querySelector(".is_stat_list");
        $list_stat = [pag_list.getAttribute("data-title"), "url", 0, 0, main_container.getAttribute("data-title"), "url", $request_user_id, $user_device, new Date().toLocaleString().replace(",", "")];
      };

          list = main_container.querySelectorAll('.pag');
          for (var i = 0; i < list.length; i++) {
              if (!list[i].classList.contains("showed")) {
                  inViewport = elementInViewport(list[i]);
                  if (inViewport) {
                    if (i == 1) {
                      get_el_view_time(120);
                    };

                    pk = list[i].getAttribute('data-pk');
                    type = list[i].getAttribute('data-type');
                    if ($all_stat.indexOf(type + " " + pk) == -1 && $new_elements.indexOf(pk + " " + type) == -1 && type != null) {
                      // "user_post", object.pk, height, time, owner_pk, owner_type, request.user.pk, mobile/desctop, datetime
                      $new_elements.push([type,pk,0,0,main_container.getAttribute("data-title"), "url", $request_user_id, $user_device, new Date().toLocaleString().replace(",", "")]);
                      console.log($new_elements);
                    };
                    list[i].classList.add("showed");
                  }
              }
          };
          console.log($list_stat);
    } catch {null};
});


function scrolled(_block) {
    offset = 0;
    window.onscroll = function() {
      console.log("paginate");
      // программа отслеживает окончание прокрутки
      scrollStopper();
      // программа считает секунды для внесения в стат страницы и списка, если он есть.
      if (!page_time) {
        get_page_view_time(120);
        page_time = true;
      };
      if ($new_elements.length) {
        for (var i = 0; i < $new_elements.length; i++){
          $new_elements[i][3] = 3 + $new_time;
          el = $new_elements[i][0] + ";" + $new_elements[i][1] + ";" + $new_elements[i][2] + ";" + $new_elements[i][3] + ";" + $new_elements[i][4] + ";" + $new_elements[i][5] + ";" + $new_elements[i][6] + ";" + $new_elements[i][7] + ";" + $new_elements[i][8]
          $all_stat.push(el);
        };
        $new_elements = [];
        $new_time = 0;
      };

      // программа останавливает отчет времени просмотра элементов, на которых остановился
      // пользователь, записывает его всем новым элементам pag, затем их добавляет в основной
      // список стата, обнуляет счетчик и очищает список новых элементов.
      if ((window.innerHeight + window.pageYOffset) > offset) {
        offset = window.innerHeight + window.pageYOffset;
        $page_stat[2] = parseFloat(offset * 0.000264).toFixed(2);
        if ($list_stat.length) {
          $list_stat[2] = parseFloat(window.pageYOffset * 0.000264).toFixed(2);
        };
      };

      try {
          box = _block.querySelector('.next_page_list');
          if (box && box.classList.contains("next_page_list")) {
              inViewport = elementInViewport(box);
              if (inViewport) {
                  box.classList.remove("next_page_list");
                  paginate(box);
              }
          };
      } catch {return}
    }
};

function open_video_fullscreen(url) {
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    link.open('GET', url, true);
    link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

    link.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            elem = link.responseText;
            block = document.body.querySelector("#video_loader");
            try {
              count_items = container.querySelectorAll(".card_fullscreen").length
            } catch {count_items = 0};
            block.parentElement.style.zIndex = 100 + count_items;
            block.parentElement.style.display = "block";
            block.innerHTML = elem;
        }
    };
    link.send();
};

function paginate(block) {
        var link_3 = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
        link_3.open('GET', location.protocol + "//" + location.host + block.getAttribute("data-link"), true);
        link_3.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

        link_3.onreadystatechange = function() {
            if (this.readyState == 4 && this.status == 200) {
                var elem = document.createElement('span');
                elem.innerHTML = link_3.responseText;
                if (document.body.querySelector(".chat_container")){
                  block.parentElement.insertAdjacentHTML('afterbegin', elem.querySelector(".is_paginate").innerHTML);
                  window.scrollTo({
                    top: elem.querySelector(".is_paginate").scrollHeight,
                    behavior: "smooth"
                  });
                }
                else if (!document.body.querySelector(".chat_container")){
                  block.parentElement.insertAdjacentHTML('beforeend', elem.querySelector(".is_paginate").innerHTML)
                };
                block.remove()
            }
        }
        link_3.send();
};

function create_pagination(block) {
  if (block.querySelector('.is_paginate')) {
    scrolled(block.querySelector('.is_paginate'));
    console.log("Работает пагинация для списка не постов")
  }
};

function load_item_window() {
  // подгружаем окно при загрузке страницы, если есть параметры ссылки на него
  params = window.location.search.replace( '?', '').split('&');
  if (params) {
    if (params[0].split("=")[0] == "url") {
      try {
        url = params[0].split("=")[1];
        fullscreen = params[1].split("=")[1];
        setTimeout(create_fullscreen(url, fullscreen, false), 3000)
      } catch { null };
    }
  }
};

function if_list(block) {
  // прога подгружает списки чего либо при подгрузке страницы, а также подтягивает окна
    if (block.querySelector('.load_block') && block.querySelector('.load_block').getAttribute) {
        _block = block.querySelector('.load_block');
        list_block_load(_block, ".load_block", _block.getAttribute("data-link"));
        scrolled(_block.querySelector('.is_paginate'));
    } else if (block.querySelector('.is_block_paginate')) {
        lenta = block.querySelector('.is_block_paginate');
        link = lenta.getAttribute("data-link");
        list_load(block.querySelector(".is_block_paginate"), link);
        scrolled(lenta.querySelector('.list_pk'));
    };
    load_item_window();

    // проверим, не страница ли чата.
    if (block.querySelector(".chat_container")) {
      width = block.querySelector(".main_chat_block").offsetWidth - 14;
      block.querySelector(".fixed_header_chat").style.width = width + "px";
      window.scrollTo( 0, 3000 );
    }
};

if_list(document.getElementById('ajax'));
create_pagination(document.getElementById('ajax'));
get_dragula(".drag_container");

function list_load(block, link) {
    var request = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    request.open('GET', link, true);
    request.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    request.onreadystatechange = function() {
        if (request.readyState == 4 && request.status == 200) {
            block.innerHTML = request.responseText;
            get_dragula(".drag_container");
            get_dragula(".drag_list");
            create_pagination(block);
            try{ fullscreen_resize() } catch { null };
        }
    };
    request.send(null);
};

function list_block_load(target_block, response_block, link) {
  // грузим блок response_block по ссылке link в блок target_block
  var request = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  request.open( 'GET', link, true );
  request.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  request.onreadystatechange = function () {
    if ( request.readyState == 4 && request.status == 200 ){
        elem_ = document.createElement('span');
        elem_.innerHTML = request.responseText;
       target_block.innerHTML = elem_.querySelector(response_block).innerHTML;
       get_dragula(".drag_container");
       get_dragula(".drag_list");
       create_pagination(target_block);
       get_music_player_support(target_block);
    }};
    request.upload.onprogress = function(event) {
      console.log( 'начало работы');
    };
    request.upload.onload = function() {
      alert( 'конец работы!' );
    };
    request.send( null );
};

// Работаем с историей, создавая свою! Всё, что меняет адреса, отправляем сюда!

function this_page_reload(url) {
    var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    ajax_link.open('GET', url, true);
    ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    ajax_link.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            elem_ = document.createElement('span');
            elem_.innerHTML = ajax_link.responseText;
            ajax = elem_.querySelector("#reload_block");
            rtr = document.getElementById('ajax');

            prev = rtr.querySelector(".main-container");
            next = ajax.querySelector(".main-container");
            try{
              init_stat_lists(next.getAttribute("data-type"), next.getAttribute("data-pk"), prev.getAttribute("data-type"), prev.getAttribute("data-pk"))
            }catch { null };

            rtr.innerHTML = ajax.innerHTML;
            window.scrollTo(0, 0);
            if_list(rtr);
            create_pagination(rtr);
            get_document_opacity_1();
        }
    }
    ajax_link.send()
};

window.addEventListener('popstate', function (e) {
  e.preventDefault();

  var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  ajax_link.open('GET', $serf_history.slice(-1), true);
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  ajax_link.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
          ajax = elem_.querySelector("#reload_block");
          rtr = document.getElementById('ajax');

          prev = rtr.querySelector(".main-container");
          next = ajax.querySelector(".main-container");
          init_stat_lists(next.getAttribute("data-type"), next.getAttribute("data-pk"), prev.getAttribute("data-type"), prev.getAttribute("data-pk"));

          rtr.innerHTML = ajax.innerHTML;
          window.scrollTo(0, 0);
          title = elem_.querySelector('title').innerHTML;
          window.history.pushState(null, "vfgffgfgf", $serf_history.slice(-1));
          document.title = title;
          if_list(rtr);
          create_pagination(rtr);
          get_dragula(".drag_container");
          get_dragula(".drag_list");
          get_document_opacity_1();
          $serf_history.push(document.location.href);

      }
  }
  ajax_link.send()
});

function ajax_get_reload(url) {
  $serf_history.push(document.location.href);
    var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    ajax_link.open('GET', url + "?ajax=1", true);
    ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    ajax_link.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            elem_ = document.createElement('span');
            elem_.innerHTML = ajax_link.responseText;
            rtr = document.getElementById('ajax');

            prev = rtr.querySelector(".main-container");
            init_stat_lists(ajax.querySelector(".main-container"), prev.getAttribute("data-title"), document.location.href);

            rtr.innerHTML = elem_.innerHTML;
            window.scrollTo(0, 0);
            window.history.pushState(null, "vfgffgfgf", url);
            document.title = rtr.querySelector('.main-container').getAttribute("data-title");
            if_list(rtr);
            create_pagination(rtr);
            get_dragula(".drag_container");
            get_dragula(".drag_list");
            get_document_opacity_1();
            get_music_player_support(rtr);
            console.log("статистика",  $all_stat);
        }
    }
    ajax_link.send()
};

function search_ajax_get_reload(url) {
  $serf_history.push(document.location.href);
    var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    ajax_link.open('GET', url + "?ajax=1", true);
    ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    ajax_link.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            elem_ = document.createElement('span');
            elem_.innerHTML = ajax_link.responseText;
            rtr = document.body.querySelector(".load_search_container");

            prev = rtr.querySelector(".main-container");
            next = ajax.querySelector(".main-container");
            init_stat_lists(ajax.querySelector(".main-container"), prev.getAttribute("data-title"), document.location.href);

            rtr.innerHTML = elem_.innerHTML;
            window.scrollTo(0, 0);
            window.history.pushState(null, "vfgffgfgf", url);
            document.title = rtr.querySelector('.main-container').getAttribute("data-title");
            if_list(rtr);
            create_pagination(rtr);
            get_document_opacity_1();

            try{
              document.getElementById("user_height").innerHTML = elem_.querySelector("#user_height").innerHTML;
              document.getElementById("user_time").innerHTML = elem_.querySelector("#user_time").innerHTML
            } catch{null}
        }
    }
    ajax_link.send()
};

function create_gif_loading () {
  $load_gif = document.createElement("img");
  $load_gif.setAttribute("src", "/static/images/preloader.gif");
  $load_gif.style.width = "40px";
  $load_div = document.createElement("div");
  $load_div.classList.add("centered", "m-1");
  $load_div.append($load_gif);
  return $load_div
};
