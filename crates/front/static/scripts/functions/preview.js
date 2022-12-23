function check_photo_in_block(block, _this, pk) {
    if (block.querySelector('[photo-pk=' + '"' + pk + '"' + ']')) {
        _this.parentElement.parentElement.setAttribute("tooltip", "Изображение уже выбрано");
        _this.parentElement.parentElement.setAttribute("flow", "up");
        return true
    } else {
        return false
    }
};
function check_photo_list_in_block(block, _this, pk) {
    if (block.querySelector('[photolist-pk=' + '"' + pk + '"' + ']')) {
        toast_info("Альбом уже прикреплён")
        return true
    } else {
        return false
    }
};
function check_video_in_block(block, _this, pk) {
    if (block.querySelector('[video-pk=' + '"' + pk + '"' + ']')) {
        _this.parentElement.parentElement.setAttribute("tooltip", "Видеоролик уже выбран");
        _this.parentElement.parentElement.setAttribute("flow", "up");
        return true
    } else {
        return false
    }
};
function check_video_list_in_block(block, _this, pk) {
    if (block.querySelector('[videolist-pk=' + '"' + pk + '"' + ']')) {
        toast_info("Видеоальбом уже прикреплён")
        return true
    } else {
        return false
    }
};
function check_music_in_block(block, _this, pk) {
    if (block.querySelector('[track-pk=' + '"' + pk + '"' + ']')) {
        _this.parentElement.setAttribute("tooltip", "Аудиозапись уже выбрана");
        _this.parentElement.setAttribute("flow", "up");
        return true
    } else {
        return false
    }
};
function check_playlist_in_block(block, _this, pk) {
    if (block.querySelector('[playlist-pk=' + '"' + pk + '"' + ']')) {
        toast_info("Плейлист уже прикреплён")
        return true
    } else {
        return false
    }
};
function check_doc_in_block(block, _this, pk) {
    if (block.querySelector('[doc-pk=' + '"' + pk + '"' + ']')) {
        _this.parentElement.parentElement.setAttribute("tooltip", "Документ уже выбран");
        _this.parentElement.parentElement.setAttribute("flow", "up");
        return true
    } else {
        return false
    }
};
function check_doc_list_in_block(block, _this, pk) {
    if (block.querySelector('[doclist-pk=' + '"' + pk + '"' + ']')) {
        toast_info("Список документов уже прикреплён")
        return true
    } else {
        return false
    }
};
function check_good_in_block(block, _this, pk) {
    if (block.querySelector('[good-pk=' + '"' + pk + '"' + ']')) {
        _this.parentElement.setAttribute("tooltip", "Товар уже выбран");
        _this.parentElement.setAttribute("flow", "up");
        return true
    } else {
        return false
    }
};
function check_good_list_in_block(block, _this, pk) {
    if (block.querySelector('[goodlist-pk=' + '"' + pk + '"' + ']')) {
        toast_info("Список товаров уже прикреплён")
        return true
    } else {
        return false
    }
};
function check_survey_in_block(block, _this, pk) {
    if (block.querySelector('[survey-pk=' + '"' + pk + '"' + ']')) {
      _this.setAttribute("tooltip", "Опрос уже выбран");
      _this.setAttribute("flow", "up");
      return true
    } else {
        return false
    }
};

function photo_preview_delete(){
  $span = document.createElement("span");
  $span.classList.add("photo_preview_delete");
  $span.innerHTML = '<svg fill="#FF0000" viewBox="0 0 24 24"><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/><path d="M0 0h24v24H0z" fill="none"/></svg>';
  $span.setAttribute("tooltip", "Не прикреплять");
  $span.setAttribute("flow", "up");
  return $span
};
function list_preview_delete(){
  $span = document.createElement("span");
  $span.classList.add("list_preview_delete");
  $span.innerHTML = '<svg fill="#FF0000" viewBox="0 0 24 24"><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/><path d="M0 0h24v24H0z" fill="none"/></svg>';
  $span.setAttribute("tooltip", "Не прикреплять");
  $span.setAttribute("flow", "up");
  return $span
};

function video_preview_delete(){
  $span = document.createElement("span");
  $span.classList.add("video_preview_delete");
  $span.innerHTML = '<svg fill="#FF0000" viewBox="0 0 24 24"><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/><path d="M0 0h24v24H0z" fill="none"/></svg>';
  $span.setAttribute("tooltip", "Не прикреплять");
  $span.setAttribute("flow", "up");
  return $span
};
function music_preview_delete(){
  $span = document.createElement("span");
  $span.classList.add("music_preview_delete");
  $span.innerHTML = '<svg fill="#FF0000" viewBox="0 0 24 24"><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/><path d="M0 0h24v24H0z" fill="none"/></svg>';
  $span.setAttribute("tooltip", "Не прикреплять");
  $span.setAttribute("flow", "up");
  return $span
};
function doc_preview_delete(){
  $span = document.createElement("span");
  $span.classList.add("doc_preview_delete");
  $span.innerHTML = '<svg fill="#FF0000" viewBox="0 0 24 24"><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/><path d="M0 0h24v24H0z" fill="none"/></svg>';
  $span.setAttribute("tooltip", "Не прикреплять");
  $span.setAttribute("flow", "up");
  return $span
};
function good_preview_delete(){
  $span = document.createElement("span");
  $span.classList.add("good_preview_delete");
  $span.innerHTML = '<svg fill="#FF0000" viewBox="0 0 24 24"><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/><path d="M0 0h24v24H0z" fill="none"/></svg>';
  $span.setAttribute("tooltip", "Не прикреплять");
  $span.setAttribute("flow", "up");
  return $span
};
function article_preview_delete(){
  $span = document.createElement("span");
  $span.classList.add("article_preview_delete");
  $span.innerHTML = '<svg fill="#FF0000" viewBox="0 0 24 24"><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/><path d="M0 0h24v24H0z" fill="none"/></svg>';
  $span.setAttribute("tooltip", "Не прикреплять");
  $span.setAttribute("flow", "up");
  return $span
};

function create_preview_commmunity(_this){
  $name = _this.querySelector("h6").innerHTML;

  $div = document.createElement("div");
  $div.style.display = "inline-block";
  $div.style.margin = "5px";
  $div.setAttribute("data-pk", _this.getAttribute("data-pk"));
  $div.classList.add("preview_item_delete", "pointer");
  $div.setAttribute("tooltip", $name);
  $div.setAttribute("flow", "up");

  $div_flex = document.createElement("div");

  _this.querySelector("img") ? ($img = document.createElement("img"), $img.setAttribute("src", _this.querySelector("img").getAttribute("src")),$img.style.width = "50px",$img.style.heigth = "auto",$img.style.borderRadius = "50%")
                             : ($img = document.createElement("span"), $img.innerHTML = '<svg fill="currentColor" class="svg_default svg_default_50" viewBox="0 0 24 24"><path d="M0 0h24v24H0z" fill="none"/><path d="M16 11c1.66 0 2.99-1.34 2.99-3S17.66 5 16 5c-1.66 0-3 1.34-3 3s1.34 3 3 3zm-8 0c1.66 0 2.99-1.34 2.99-3S9.66 5 8 5C6.34 5 5 6.34 5 8s1.34 3 3 3zm0 2c-2.33 0-7 1.17-7 3.5V19h14v-2.5c0-2.33-4.67-3.5-7-3.5zm8 0c-.29 0-.62.02-.97.05 1.16.84 1.97 1.97 1.97 3.45V19h6v-2.5c0-2.33-4.67-3.5-7-3.5z"/></svg>');
  $figure = document.createElement("figure");
  $figure.append($img);

  $input = document.createElement("span");
  $input.innerHTML = '<input type="hidden" class="community" name="communities" value="' + pk + '">';

  $div_flex.append($figure);
  $div.append($input);
  $div.append($div_flex);
  return $div
};
function create_preview_chat_item(_this){
  $name = _this.querySelector("p").innerHTML;

  $div = document.createElement("div");
  $div.style.display = "inline-block";
  $div.style.margin = "5px";
  $div.setAttribute("data-pk", _this.getAttribute("data-pk"));
  $div.classList.add("preview_item_delete", "pointer");
  $div.setAttribute("tooltip", $name);
  $div.setAttribute("flow", "up");

  $div_flex = document.createElement("div");

  _this.querySelector("img") ? (
    $img = document.createElement("img"),
    $img.setAttribute("src", _this.querySelector("img").getAttribute("src")),
    $img.style.width = "50px",
    $img.style.heigth = "auto",
    $img.style.borderRadius = "50%")
                             : (
                               $img = document.createElement("span"),
                               $img.innerHTML = '<svg fill="currentColor" class="svg_default svg_default_50" viewBox="0 0 24 24"><path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/><path d="M0 0h24v24H0z" fill="none"/></svg>');
  $figure = document.createElement("figure");
  $figure.append($img);

  $input = document.createElement("span");
  $input.innerHTML = '<input type="hidden" class="chat" name="chat_items" value="' + pk + '">';

  $div_flex.append($figure);
  $div.append($input);
  $div.append($div_flex);
  return $div
};

function create_preview_survey(pk, src, container_html){
  $div = document.createElement("div");
  $div.classList.add("card");
  $div.style.flex = "0 0 100%";
  $div.setAttribute("survey-pk", pk);

  $box_shadow = document.createElement("div");
  $div.classList.add("mb-3", "border", "text-center", "position-relative", "box-shadow");

  $input = document.createElement("span");
  $input.innerHTML = '<input class="attach" type="hidden" name="attach_items" value="sur' + pk + '">';

  $figure = document.createElement("div");
  $figure.classList.add("background-img");
  if (src) {
    $img = document.createElement("img");
    $img.setAttribute("src", src);
    $figure.append($img);
  };

  $a = document.createElement("a");
  $a.classList.add("survey_attach_remove", "pointer", "position-relative");
  $a.innerHTML = "Открепить";

  $container = document.createElement("div");
  $container.classList.add("container");
  $container.innerHTML = container_html;

  $div.append($input);
  $box_shadow.append($figure);
  $box_shadow.append($a);
  $box_shadow.append($container);
  $div.append($box_shadow);
  return $div
};

function create_preview_photo(img_src, photo_pk, user_pk){
  $div = document.createElement("div");
  $div.classList.add("photo");
  $input = document.createElement("span");
  $input.innerHTML = '<input class="attach" type="hidden" name="attach_items" value="pho' + photo_pk + '">';
  $img = document.createElement("img");
  $img.classList.add("detail_photo", "image_fit", "pointer", "handle");
  $img.setAttribute("src", img_src);
  $img.setAttribute('photo-pk', photo_pk);
  $img.setAttribute('data-pk', user_pk);
  $div.append(photo_preview_delete());
  $div.append($input);
  $div.append($img);
  return $div
};
function create_preview_message_photo(img_src, photo_pk, user_pk){
  $div = document.createElement("div");
  $div.classList.add("photo", "handle");
  $input = document.createElement("span");
  $input.innerHTML = '<input class="attach" type="hidden" name="attach_items" value="pho' + photo_pk + '">';
  $img = document.createElement("img");
  $img.classList.add("u_photo_priview", "image_fit", "pointer");
  $img.setAttribute("src", img_src);
  $img.setAttribute('photo-pk', photo_pk);
  $img.setAttribute('data-pk', user_pk);
  $div.append(photo_preview_delete());
  $div.append($input);
  $div.append($img);
  return $div
};
function create_preview_photo_list(src, title, pk, count){
  $div = document.createElement("div");
  $div.classList.add("bg-dark", "position-relative", "text-center", "big_mobile_element", "handle");
  $div.setAttribute("photolist-pk", pk);

  $input = document.createElement("span");
  $input.innerHTML = '<input class="attach" type="hidden" name="attach_items" value="lph' + pk + '">';

  $img = document.createElement("img");
  $img.setAttribute("src", src);

  $figure = document.createElement("figure");
  $figure.classList.add("background-img");
  $figure.append($img);

  $h6 = document.createElement("h6");
  $h6.innerHTML = title;
  $h6.classList.add("load_photo_list", "text-white", "pointer", "mb-2", "nowrap");

  $span = document.createElement("p");
  $span.classList.add("photo_attach_list_remove", "underline", "pointer", "text-white");
  $span.innerHTML = "Открепить";

  $hr = document.createElement("hr");
  $hr.classList.add("my-3");

  $a = document.createElement("a");
  $a.classList.add("load_photo_list", "pointer", "text-white");
  $a.innerHTML = count;

  $container = document.createElement("div");
  $container.classList.add("container", "p-3");

  $container.append($h6);
  $container.append($span);
  $container.append($hr);
  $container.append($a);

  $div.append($figure); $div.append($container); $div.append($input);
  return $div
};

function create_preview_video(img_src, pk, counter){
  $div = document.createElement("div");
  $div.classList.add("video", "handle");
  $input = document.createElement("span");
  $input.innerHTML = '<input class="attach" type="hidden" name="attach_items" value="vid' + pk + '">';
  $img = document.createElement("img");
  $icon_div = document.createElement("span");
  $img.classList.add("image_fit");
  $img.src = img_src;
  $icon_div.classList.add("video_icon_play_v2", "video_list_detail");
  $icon_div.setAttribute("video-pk", pk);

  $div.append(video_preview_delete());
  $div.append($input);
  $div.append($img);
  $div.append($icon_div);
  return $div
};

function create_preview_video_list(name, pk, count){
  $div = document.createElement("div");
  $div.classList.add("handle");
  $div.style.flexBasis = "100%";
  $div.style.position = "relative";
  $div.setAttribute("videolist-pk", pk);
  $div.innerHTML = '<div style="display:flex"><figure><a class="load_video_list pointer"><svg fill="currentColor" class="svg_default" style="width:60px;height:88px;" viewBox="0 0 24 24"><path d="M0 0h24v24H0z" fill="none"></path><path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"></path></svg></a></figure><div class="media-body" style="margin-left: 10px;margin-top: 15px;"><h6 class="my-0 mt-1 load_video_list pointer">' + name + '</h6><p>Роликов: ' + count + '</p></div></div>'

  $input = document.createElement("span");
  $input.innerHTML = '<input class="attach" type="hidden" name="attach_items" value="lvi' + pk + '">';
  $div.append($input);
  $div.append(list_preview_delete());
  return $div
};

function create_preview_music(title, img_src, track_pk, list_pk){
  $div = document.createElement("div");

  $input = document.createElement("span");
  $img = document.createElement("img");
  $figure = document.createElement("figure");
  $media = document.createElement("span");
  $progress2 = document.createElement("div");
  $h6 = document.createElement("h6");

  $div.classList.add("music", "handle", "track");
  $div.setAttribute("track-pk", track_pk);
  $div.setAttribute("playlist-pk", list_pk);
  $div.style.display = "flex";
  $div.style.margin = "5px";
  $div.style.position = "relative";
  $div.style.flexBasis = "100%";

  $input.innerHTML = '<input class="attach" type="hidden" name="attach_items" value="mus' + track_pk + '">';

  $img.src = img_src;
  $img.style.width = "30px";
  $figure.append($img);

  $media.style.marginLeft = "10px";
  $media.style.marginRight = "40px";
  $media.style.overflow = "hidden";
  $media.style.zIndex = "1";
  $h6.classList.add("music_list_item", "pointer", "music_title");
  $h6.innerHTML = title;

  $h6_wrapper = document.createElement("span");
  $h6_wrapper.append($h6);
  $media.append($h6_wrapper);

  $progress2.classList.add("progress2");

  $div.append(music_preview_delete());
  $div.append($input);
  $div.append($figure);
  $div.append($media);
  $div.append($progress2);
  return $div
};
function create_preview_playlist(name, img_src, pk, track_pk, count){
  // первый блок
  $div = document.createElement("div");
  $div.classList.add("playlist", "card");
  $div.style.flexBasis = "100%";
  $div.style.position = "relative";
  $div.setAttribute("playlist-pk", pk);
  $div.setAttribute("data-pk", pk);

  // прокладка - сначала в него, потом его в главный блок
  $wrapper_div = document.createElement("div");
  $wrapper_div.style.display = "flex";
  $wrapper_div.setAttribute("playlist-pk", pk);

  // элементы в figure
  $figure = document.createElement("figure");
  $figure.classList.add("position-relative");
  $figure_a = document.createElement("a");
  $figure_a.classList.add("load_music_list", "pointer");
  $img = document.createElement("img");
  $img.src = img_src;
  $img.classList.add("image_fit_120");
  $figure_a.append($img);
  $play_div = document.createElement("div");
  $play_div.classList.add("play_list_mode", "music_list_item");
  $play_div.setAttribute("track-pk", track_pk);
  $wrapper_play = document.createElement("span");
  $wrapper_play.append($play_div);
  $figure.append($figure_a);
  $figure.append($wrapper_play);

  // $media_body
  $media_body = document.createElement("div");
  $media_body.style.marginLeft = "10px";
  $media_body.classList.add("handle");
  $h6 = document.createElement("h6");
  $h6.classList.add("my-0", "mt-1", "load_music_list", "pointer");
  $h6.innerHTML = name;
  $span = document.createElement("span");
  $count_p = document.createElement("p");
  $count_p.classList.add("mt-2");
  $count_p.innerHTML = count;
  $span.append($count_p);

  $media_body.append($h6);
  $media_body.append($span);

  $input = document.createElement("span");
  $input.innerHTML = '<input class="attach" type="hidden" name="attach_items" value="lmu' + pk + '">';

  $wrapper_div.append($figure);
  $wrapper_div.append($media_body);

  $div.append($input);
  $div.append($wrapper_div);
  $div.append(list_preview_delete());
  return $div
};

function create_preview_doc(media_body, pk){
  $div = document.createElement("div");
  $input = document.createElement("span");
  $span = document.createElement("span");
  $figure = document.createElement("figure");
  $media = document.createElement("span");

  $div.classList.add("doc", "handle");
  $div.setAttribute("data-pk", pk);
  $div.style.display = "flex";
  $div.style.margin = "5px";
  $div.style.flexBasis = "100%";
  $div.style.position = "relative";

  $input.innerHTML = '<input class="attach" type="hidden" name="attach_items" value="doc' + pk + '">';

  $span.innerHTML = '<svg fill="currentColor" style="width:35px;heigth:35px" class="svg_default" viewBox="0 0 24 24"><path d="M0 0h24v24H0z" fill="none"/><path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"/></svg>';
  $figure.append($span);

  $media.innerHTML = media_body.innerHTML;
  $media.style.marginLeft = "10px";
  $media.style.marginRight = "40px";
  $media.style.overflow = "hidden";
  h6 = $media.querySelector("h6");
  h6.style.paddingTop = "8px";

  $div.append(doc_preview_delete());
  $div.append($input);
  $div.append($figure);
  $div.append($media);
  return $div
};
function create_preview_doc_list(name, pk, count){
  $div = document.createElement("div");
  $div.classList.add("handle");
  $div.style.flexBasis = "100%";
  $div.style.position = "relative";
  $div.setAttribute("doclist-pk", pk);
  $div.innerHTML = '<div style="display:flex;"><figure><a class="load_doc_list pointer"><svg fill="currentColor" class="svg_default" style="width:60px;height:88px;" viewBox="0 0 24 24"><path d="M0 0h24v24H0z" fill="none"></path><path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"></path></svg></a></figure><div class="media-body" style="margin-left: 10px;margin-top: 15px;"><h6 class="my-0 mt-1 load_doc_list pointer">' + name + '</h6><p>Документов: ' + count + '</p></div></div>'

  $input = document.createElement("span");
  $input.innerHTML = '<input class="attach" type="hidden" name="attach_items" value="ldo' + pk + '">';
  $div.append($input);
  $div.append(list_preview_delete());
  return $div
};

function create_preview_good(img_src, pk, uuid, title){
  $div = document.createElement("div");
  $div.classList.add("good_detail", "good", "handle");
  $div.setAttribute('good-pk', pk);
  $div.style.cursor = "pointer";

  $input = document.createElement("span");
  $title = document.createElement("span");
  $title.innerHTML = '<span class="badge badge-info mb-2" style="position: absolute;bottom:-8px;"><svg style="padding-bottom: 1px" height="13" fill="#FFFFFF" viewBox="0 0 24 24" width="13"><path d="M0 0h24v24H0z" fill="none"/><path d="M17.21 9l-4.38-6.56c-.19-.28-.51-.42-.83-.42-.32 0-.64.14-.83.43L6.79 9H2c-.55 0-1 .45-1 1 0 .09.01.18.04.27l2.54 9.27c.23.84 1 1.46 1.92 1.46h13c.92 0 1.69-.62 1.93-1.46l2.54-9.27L23 10c0-.55-.45-1-1-1h-4.79zM9 9l3-4.4L15 9H9zm3 8c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2z"/></svg>' + title + '</span>';
  $input.innerHTML = '<input class="attach" type="hidden" name="attach_items" value="goo' + pk + '">';
  $img = document.createElement("img");
  $img.classList.add("image_fit");
  $img.src = img_src;

  $div.append(good_preview_delete());
  $div.append($input);
  $div.append($title);
  $div.append($img);
  return $div
};
function create_preview_good_list(name, pk, count){
  $div = document.createElement("div");
  $div.classList.add("handle");
  $div.style.flexBasis = "100%";
  $div.style.position = "relative";
  $div.setAttribute("goodlist-pk", pk);
  $div.innerHTML = '<div style="display:flex;"><figure><a class="load_good_list pointer"><svg fill="currentColor" class="svg_default" style="width:60px;height:88px;" viewBox="0 0 24 24"><path d="M0 0h24v24H0z" fill="none"></path><path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"></path></svg></a></figure><div class="media-body" style="margin-left: 10px;margin-top: 15px;"><h6 class="my-0 mt-1 load_good_list pointer">' + name + '</h6><p>Товаров: ' + count + '</p></div></div>'

  $input = document.createElement("span");
  $input.innerHTML = '<input class="attach" type="hidden" name="attach_items" value="lgo' + pk + '">';
  $div.append($input);
  $div.append(list_preview_delete());
  return $div
};

function create_preview_article(img_src, pk, title){
  $div = document.createElement("div");
  $div.classList.add("article", "handle");
  $title = document.createElement("span");
  $div.setAttribute('data-pk', pk);
  $div.style.cursor = "pointer";

  $input = document.createElement("span");
  $input.innerHTML = '<input class="attach" type="hidden" name="attach_items" value="art' + pk + '">';

  $img = document.createElement("img");
  $img.style.width = "100%";
  $img.classList.add("image_fit");
  $p = document.createElement("p");
  $figure = document.createElement("figure");
  $figure.classList.add("u_article_detail");

  $img.src = img_src;
  $figure.append($img);
  $title.innerHTML = '<span class="badge badge-info mb-2" style="position: absolute;bottom:-8px;"><svg style="padding-bottom: 1px" height="13" fill="#FFFFFF" viewBox="0 0 24 24" width="13"><path d="M0 0h24v24H0z" fill="none"/><path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-5 14H7v-2h7v2zm3-4H7v-2h10v2zm0-4H7V7h10v2z"/></svg>' + title + '</span>'

  $div.append(article_preview_delete());
  $div.append($input);
  $div.append($figure);
  $div.append($title);
  return $div
};
function create_preview_article_list(name, pk, count){
  $div = document.createElement("div");
  $div.classList.add("handle");
  $div.style.flexBasis = "100%";
  $div.style.position = "relative";
  $div.setAttribute("articlelist-pk", pk);
  $div.innerHTML = '<div style="display:flex;"><figure><a class="load_article_list pointer"><svg fill="currentColor" class="svg_default" style="width:60px;height:88px;" viewBox="0 0 24 24"><path d="M0 0h24v24H0z" fill="none"></path><path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"></path></svg></a></figure><div class="media-body" style="margin-left: 10px;margin-top: 15px;"><h6 class="my-0 mt-1 load_article_list pointer">' + name + '</h6><p>Статей: ' + count + '</p></div></div>'

  $input = document.createElement("span");
  $input.innerHTML = '<input class="attach" type="hidden" name="attach_items" value="lgo' + pk + '">';
  $div.append($input);
  $div.append(list_preview_delete());
  return $div
};

on('#ajax', 'click', '.photo_preview_delete', function() {
  parent = this.parentElement;
  block = parent.parentElement;
  if (block.classList.contains("attach_block")){
    remove_file_attach(), is_full_attach()
  } else if (block.classList.contains("img_block")){
    remove_file_dropdown(); is_full_dropdown()
  } else if (block.classList.contains("message_attach_block")){
    remove_file_message_attach(); is_full_message_attach(); check_message_form_btn()
  }
  parent.remove();
});
on('#ajax', 'click', '.doc_preview_delete', function() {
  parent = this.parentElement;
  block = parent.parentElement;
  if (block.classList.contains("attach_block")){
    remove_file_attach(), is_full_attach()
  } else if (block.classList.contains("img_block")){
    remove_file_dropdown(); is_full_dropdown()
  } else if (block.classList.contains("message_attach_block")){
    remove_file_message_attach(); is_full_message_attach(); check_message_form_btn()
  }
  parent.remove();
  try{ remove_file_dropdown(); is_full_dropdown()} catch { remove_file_attach(), is_full_attach()}
});
on('#ajax', 'click', '.video_preview_delete', function() {
  parent = this.parentElement;
  block = parent.parentElement;
  if (block.classList.contains("attach_block")){
    remove_file_attach(), is_full_attach()
  } else if (block.classList.contains("img_block")){
    remove_file_dropdown(); is_full_dropdown()
  } else if (block.classList.contains("message_attach_block")){
    remove_file_message_attach(); is_full_message_attach(); check_message_form_btn()
  }
  parent.remove();
});
on('#ajax', 'click', '.list_preview_delete', function() {
  parent = this.parentElement;
  block = parent.parentElement;
  if (block.classList.contains("attach_block")){
    remove_file_attach(), is_full_attach()
  } else if (block.classList.contains("img_block")){
    remove_file_dropdown(); is_full_dropdown()
  } else if (block.classList.contains("message_attach_block")){
    remove_file_message_attach(); is_full_message_attach(); check_message_form_btn()
  }
  parent.remove();
});
on('#ajax', 'click', '.music_preview_delete', function() {
  parent = this.parentElement;
  block = parent.parentElement;
  if (block.classList.contains("attach_block")){
    remove_file_attach(), is_full_attach()
  } else if (block.classList.contains("img_block")){
    remove_file_dropdown(); is_full_dropdown()
  } else if (block.classList.contains("message_attach_block")){
    remove_file_message_attach(); is_full_message_attach(); check_message_form_btn()
  }
  parent.remove();
});

on('#ajax', 'click', '.preview_item_delete', function() {
  _this = this;
  parent = _this.parentElement;
  parent.querySelector(".preview_item_delete") ? null : parent.innerHTML = "";
  _this.remove();
});

on('#ajax', 'click', '.good_preview_delete', function() {
  parent = this.parentElement;
  block = parent.parentElement;
  if (block.classList.contains("attach_block")){
    remove_file_attach(), is_full_attach()
  } else if (block.classList.contains("img_block")){
    remove_file_dropdown(); is_full_dropdown()
  } else if (block.classList.contains("message_attach_block")){
    remove_file_message_attach(); is_full_message_attach(); check_message_form_btn()
  }
  parent.remove();
});
on('#ajax', 'click', '.article_preview_delete', function() {
  parent = this.parentElement;
  block = parent.parentElement;
  if (block.classList.contains("attach_block")){
    remove_file_attach(), is_full_attach()
  } else if (block.classList.contains("img_block")){
    remove_file_dropdown(); is_full_dropdown()
  } else if (block.classList.contains("message_attach_block")){
    remove_file_message_attach(); is_full_message_attach(); check_message_form_btn()
  }
  parent.remove();
});

function like_reload(like_block, dislike_block, _class){
  userpic = document.body.querySelector(".userpic");
  userpic.querySelector("img") ? (user_src = userpic.querySelector("img").getAttribute("src"),$img = document.createElement("img"),$img.src = user_src,$img.style.borderRadius = "50%",$img.style.width = "50px") : $img = document.createElement("span"), $img.innerHTML = '<svg fill="currentColor" class="svg_default svg_default_50" viewBox="0 0 24 24"><path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/><path d="M0 0h24v24H0z" fill="none"/></svg>';
  user_name = userpic.getAttribute("data-name");
  user_pk = document.body.querySelector(".userpic").getAttribute("data-pk");

  if (!like_block.querySelector('figure')){
    console.log("создаем блок лайков");
    $div = document.createElement("div");
    $a = document.createElement("a");
    $a.style.paddingRight = "10px";
    $a.setAttribute("data-pk", user_pk);
    $span1 = document.createElement("span");
    $span1.classList.add(_class, "pointer");
    $span1.innerHTML = "Одобрил 1 человек";
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
    like_block.append($div)
  }
  else if (like_block.querySelector( '[data-pk=' + '"' + user_pk + '"' + ']' )){
        like_block.querySelector( '[data-pk=' + '"' + user_pk + '"' + ']' ).remove()
      if (!like_block.querySelector('figure')) {
        like_block.innerHTML = ""
        console.log("удаляем блок лайков");
      } else {
        value = like_block.querySelector('[data-count=like]').innerHTML;
        value = value*1;
        value -= 1;
        like_block.querySelector('.pointer').innerHTML = "Всего одобрили :<span data-count='like'> " + value + "</span>";
        console.log("удаляем пользователя из лайков");
      }
  }
  else {
      all_likes = like_block.querySelector('.pointer');
      $a = document.createElement("a");
      $a.style.paddingRight = "10px";
      $a.setAttribute("data-pk", user_pk);
      $figure = document.createElement("figure");
      $figure.style.margin = "0";
      $figure.title = user_name;
      $figure.append($img)
      $a.append($figure);
      all_likes.nextElementSibling.prepend($a);
      value = all_likes.querySelector('[data-count=like]').innerHTML;
      value = value*1;
      value += 1;
      like_block.querySelector('.pointer').innerHTML = "Всего одобрили :<span data-count='like'> " + value + "</span>";
      console.log("создаем пользователя в лайках")
  }
  if (dislike_block.querySelector( '[data-pk=' + '"' + user_pk + '"' + ']' )){
    dislike_block.querySelector( '[data-pk=' + '"' + user_pk + '"' + ']' ).remove();
    if (!dislike_block.querySelector('figure')){
      dislike_block.innerHTML = ""
    } else {
    value = dislike_block.querySelector('[data-count=dislike]').innerHTML;
    value = value*1;
    value -= 1;
    dislike_block.querySelector('.pointer').innerHTML = "Всего не одобрили :<span data-count='dislike'> " + value + "</span>";
  }
    console.log("удаляем пользователя из дизлайков")
  }
};

on('body', 'click', '.photo_attach_list', function() {
  _this = this;
  src = _this.parentElement.previousElementSibling.querySelector("img").getAttribute("src");
  title = _this.parentElement.querySelector(".nowrap").innerHTML;
  pk = _this.getAttribute('data-pk');
  count = _this.parentElement.querySelector(".count").innerHTML;
  if (document.body.querySelector(".current_file_dropdown")){
    check_photo_list_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, pk) ? null : (photo_list_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, src, title, pk, count), close_work_fullscreen())
  } else if (document.body.querySelector(".attach_block")){
    check_photo_list_in_block(document.body.querySelector(".attach_block"), _this, pk) ? null : (photo_list_post_attach(document.body.querySelector(".attach_block"), src, title, pk, count), close_work_fullscreen())
  } else if (document.body.querySelector(".message_attach_block")){
    check_photo_list_in_block(document.body.querySelector(".message_attach_block"), _this, pk) ? null : (photo_list_message_attach(document.body.querySelector(".message_attach_block"), src, title, pk, count), close_work_fullscreen())
  }
});
