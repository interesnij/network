function clear_attach_block(){
  if (document.body.querySelector(".attach_block")){
    a_b = document.body.querySelector(".attach_block"),
    a_b.innerHTML = "";
    a_b.classList = "";
    a_b.classList.add("files_0");
  }
};

function is_full_attach(){
  files_block = document.body.querySelector(".attach_block");
  if (files_block.classList.contains("files_10")){
    files_block.parentElement.querySelector(".attach_panel").style.display = "none";
    close_work_fullscreen();
  }
  else {
    files_block.parentElement.querySelector(".attach_panel").style.display = "block"
}
};
function add_file_attach(){
  files_block = document.body.querySelector(".attach_block");
  if (files_block.classList.contains("files_0")){ files_block.classList.add("files_1"), files_block.classList.remove("files_0")}
  else if (files_block.classList.contains("files_1")){ files_block.classList.add("files_2"), files_block.classList.remove("files_1")}
  else if (files_block.classList.contains("files_2")){ files_block.classList.add("files_3"), files_block.classList.remove("files_2")}
  else if (files_block.classList.contains("files_3")){ files_block.classList.add("files_4"), files_block.classList.remove("files_3")}
  else if (files_block.classList.contains("files_4")){ files_block.classList.add("files_5"), files_block.classList.remove("files_4")}
  else if (files_block.classList.contains("files_5")){ files_block.classList.add("files_6"), files_block.classList.remove("files_5")}
  else if (files_block.classList.contains("files_6")){ files_block.classList.add("files_7"), files_block.classList.remove("files_6")}
  else if (files_block.classList.contains("files_7")){ files_block.classList.add("files_8"), files_block.classList.remove("files_7")}
  else if (files_block.classList.contains("files_8")){ files_block.classList.add("files_9"), files_block.classList.remove("files_8")}
  else if (files_block.classList.contains("files_9")){ files_block.classList.add("files_10"), files_block.classList.remove("files_9")}
};
function remove_file_attach(){
  files_block = document.body.querySelector(".attach_block");
  if (files_block.classList.contains("files_1")){ files_block.classList.add("files_0"), files_block.classList.remove("files_1")}
  else if (files_block.classList.contains("files_2")){ files_block.classList.add("files_1"), files_block.classList.remove("files_2")}
  else if (files_block.classList.contains("files_3")){ files_block.classList.add("files_2"), files_block.classList.remove("files_3")}
  else if (files_block.classList.contains("files_4")){ files_block.classList.add("files_3"), files_block.classList.remove("files_4")}
  else if (files_block.classList.contains("files_5")){ files_block.classList.add("files_4"), files_block.classList.remove("files_5")}
  else if (files_block.classList.contains("files_6")){ files_block.classList.add("files_5"), files_block.classList.remove("files_6")}
  else if (files_block.classList.contains("files_7")){ files_block.classList.add("files_6"), files_block.classList.remove("files_7")}
  else if (files_block.classList.contains("files_8")){ files_block.classList.add("files_7"), files_block.classList.remove("files_8")}
  else if (files_block.classList.contains("files_9")){ files_block.classList.add("files_8"), files_block.classList.remove("files_9")}
  else if (files_block.classList.contains("files_10")){ files_block.classList.add("files_9"), files_block.classList.remove("files_10")}
};

function create_drag_attach_post(block){
  if (!block.classList.contains("attach_drag_post")) {
    block.classList.add("attach_drag_post");
    get_dragula(".attach_drag_post");
  }
};

function photo_list_post_attach(block, src, title, pk, count) {
  is_full_attach();
  div = create_preview_photo_list(src, title, pk, count);
  block.append(div);
  create_drag_attach_post(block);
  add_file_attach();
  is_full_attach();
};
function doc_list_post_attach(block, title, pk, count) {
  is_full_attach();
  div = create_preview_doc_list(title, pk, count);
  block.append(div);
  create_drag_attach_post(block);
  add_file_attach();
  is_full_attach();
};
function playlist_post_attach(block, title, img_src, pk, track_pk, count) {
  is_full_attach();
  div = create_preview_playlist(title, img_src, pk, track_pk, count);
  block.append(div);
  create_drag_attach_post(block);
  add_file_attach();
  is_full_attach();
};
function video_list_post_attach(block, title, pk, count) {
  is_full_attach();
  div = create_preview_video_list(title, pk, count);
  block.append(div);
  create_drag_attach_post(block);
  add_file_attach();
  is_full_attach();
};

function photo_post_attach(block, photo_pk, user_pk, src) {
  is_full_attach();
  div = create_preview_photo(src, photo_pk, user_pk)
  block.append(div);
  create_drag_attach_post(block);
  add_file_attach();
  is_full_attach();
};

function photo_post_upload_attach(photo_list, block){
  is_full_attach();
  for (var i = 0; i < photo_list.length; i++){
    parent = photo_list[i];
    div = create_preview_photo(parent.querySelector(".progressive").getAttribute('data-href'), parent.getAttribute("photo-pk"), parent.getAttribute("data-pk"));
    add_file_attach();
    block.append(div);
    is_full_attach();
  };
  create_drag_attach_post(block);
  close_work_fullscreen();
};

function video_post_attach(block, pk, counter, src) {
  is_full_attach();
  div = create_preview_video(src, pk, counter)
  block.append(div);
  create_drag_attach_post(block);
  add_file_attach();
  is_full_attach();
};

function music_post_attach(block, title, track_pk, list_pk, src) {
  is_full_attach();
  div = create_preview_music(title, src, track_pk, list_pk)
  block.append(div);
  create_drag_attach_post(block);
  add_file_attach();
  is_full_attach();
};

function doc_post_attach(block, media_block, pk) {
  is_full_attach();
  div = create_preview_doc(media_block, pk)
  block.append(div);
  create_drag_attach_post(block);
  add_file_attach();
  is_full_attach();
};

function good_post_attach(block, src, pk, uuid, title) {
  is_full_attach();
  div = create_preview_good(src, pk, uuid, title);
  block.append(div);
  create_drag_attach_post(block);
  add_file_attach();
  is_full_attach();
};

function survey_post_attach(block, pk, src, container_html) {
  is_full_attach();
  div = create_preview_survey(pk, src, container_html);
  block.append(div);
  create_drag_attach_post(block);
  add_file_attach();
  is_full_attach();
};

function article_post_attach(_this, block) {
  is_full_attach();
  _this.parentElement.classList.add("attach_toggle");
  title = _this.parentElement.querySelector(".article_title").innerHTML;
  div = create_preview_article(_this.querySelector("img").getAttribute('data-src'), uuid, title);
  block.append(div);
  create_drag_attach_post(block);
  add_file_attach();
  is_full_attach();
};

function commmunity_form_selected(_this, block) {
  pk = _this.getAttribute('data-pk');
  if (block.querySelector( '[data-pk=' + '"' + pk + '"' + ']' )){
    _this.setAttribute("tooltip", "Сообщество уже выбрано");
    _this.setAttribute("flow", "up");
    return
  }

  div = create_preview_commmunity(_this);
  block.append(div);
};
function chat_item_form_selected(_this, block) {
  pk = _this.getAttribute('data-pk');
  member_pk = _this.parentElement.parentElement.getAttribute("data-pk");
  if (block.querySelector( '[data-pk=' + '"' + pk + '"' + ']' ) || member_pk == pk){
    _this.setAttribute("tooltip", "Друг уже выбран");
    _this.setAttribute("flow", "up");
    return
  }
  div = create_preview_chat_item(_this);
  block.append(div);
  _this.parentElement.parentElement.style.display = "none"
};
