function clear_comment_dropdown(){
  try{
  dropdowns = document.body.querySelectorAll(".current_file_dropdown");
  for (var i = 0; i < dropdowns.length; i++) {
    btn = dropdowns[i].parentElement.parentElement;
    btn.classList.remove("files_two", "files_one");
    btn.classList.add("files_null");
    btn.style.display = "block";
    dropdowns[i].classList.remove("current_file_dropdown");
  }} catch { null }
  try{
  img_blocks = document.body.querySelectorAll(".img_block");
  for (var i = 0; i < img_blocks.length; i++) {
    img_blocks[i].innerHTML = "";
  }} catch { null }
};
function is_full_dropdown(){
  dropdown = document.body.querySelector(".current_file_dropdown").parentElement.parentElement;
  if (dropdown.classList.contains("files_two")){
    dropdown.style.display = "none";
    close_work_fullscreen();
  }
  if (dropdown.classList.contains("files_one") || dropdown.classList.contains("files_null")){
    dropdown.style.display = "block"
  }
};
function add_file_dropdown(){
  dropdown = document.body.querySelector(".current_file_dropdown").parentElement.parentElement;
  if (dropdown.classList.contains("files_null")){
    dropdown.classList.add("files_one");
    dropdown.classList.add("files_null")
    }
  else if(dropdown.classList.contains("files_one")){
    dropdown.classList.add("files_two");
    dropdown.classList.remove("files_one")
  };
};
function remove_file_dropdown(){
  dropdown = document.body.querySelector(".current_file_dropdown").parentElement.parentElement;
  if (dropdown.classList.contains("files_one")){
    dropdown.classList.add("files_null"); dropdown.classList.remove("files_one")}
  else if(dropdown.classList.contains("files_two")){
    dropdown.classList.add("files_one"); dropdown.classList.remove("files_two")};
};

function create_drag_attach_comment(block){
  if (!block.classList.contains("attach_drag_comment")) {
    block.classList.add("attach_drag_comment");
    get_dragula(".attach_drag_comment");
  }
};

function photo_comment_attach(dropdown, photo_pk, user_pk, src) {
  is_full_dropdown();
  img_block = dropdown.parentElement.parentElement.previousElementSibling;
  div = create_preview_photo(src, photo_pk, user_pk);
  img_block.append(div);
  create_drag_attach_comment(block);
  add_file_dropdown()
  is_full_dropdown();
};
function photo_list_comment_attach(dropdown, title, pk, count) {
  is_full_dropdown();
  img_block = dropdown.parentElement.parentElement.previousElementSibling;
  div = create_preview_photo_list(title, pk, count);
  img_block.append(div);
  create_drag_attach_comment(block);
  add_file_dropdown();
  is_full_dropdown();
};

function photo_comment_upload_attach(photo_list, dropdown){
  is_full_dropdown();

  img_block = dropdown.parentElement.parentElement.previousElementSibling;
  for (var i = 0; i < photo_list.length; i++){
    parent = photo_list[i];
    div = create_preview_photo(parent.querySelector(".progressive").getAttribute('data-href'), parent.getAttribute("photo-pk"), parent.getAttribute("data-pk"));
    block.append(div);
    img_block.append(div);
    add_file_dropdown();
    is_full_dropdown();
  };
  create_drag_attach_comment(block);
  close_work_fullscreen();
};

function survey_message_attach(block, pk, src, container_html) {
  is_full_dropdown();
  div = create_preview_survey(pk, src, container_html);
  block.append(div);
  create_drag_attach_comment(block);
  add_file_dropdown();
  is_full_dropdown();
};

function video_comment_attach(dropdown, pk, counter, src){
  is_full_dropdown(dropdown);
  img_block = dropdown.parentElement.parentElement.previousElementSibling;
  div = create_preview_video(src, pk, counter)
  img_block.append($div);
  create_drag_attach_comment(block);
  add_file_dropdown();
  is_full_dropdown();
};
function video_list_comment_attach(dropdown, title, pk, count) {
  is_full_dropdown();
  img_block = dropdown.parentElement.parentElement.previousElementSibling;
  div = create_preview_video_list(title, pk, count);
  img_block.append(div);
  create_drag_attach_comment(block);
  add_file_dropdown();
  is_full_dropdown();
};

function music_comment_attach(dropdown, title, track_pk, list_pk, src){
  is_full_dropdown(dropdown);
  img_block = dropdown.parentElement.parentElement.previousElementSibling;
  div = create_preview_music(title, src, track_pk, list_pk)
  add_file_dropdown();
  img_block.append(div);
  create_drag_attach_comment(block);
  is_full_dropdown();
};
function playlist_comment_attach(dropdown, title, img_src, pk, track_pk, count) {
  is_full_dropdown();
  img_block = dropdown.parentElement.parentElement.previousElementSibling;
  div = create_preview_playlist(title, pk, count);
  img_block.append(div);
  add_file_dropdown();
  is_full_dropdown();
};

function doc_comment_attach(dropdown, media_block, pk){
  is_full_dropdown(dropdown);
  img_block = dropdown.parentElement.parentElement.previousElementSibling;
  div = create_preview_doc(media_block, pk)
  add_file_dropdown();
  img_block.append(div);
  create_drag_attach_comment(block);
  is_full_dropdown();
};
function doc_list_comment_attach(dropdown, title, pk, count) {
  is_full_dropdown();
  img_block = dropdown.parentElement.parentElement.previousElementSibling;
  div = create_preview_doc_list(title, pk, count);
  img_block.append(div);
  create_drag_attach_comment(block);
  add_file_dropdown();
  is_full_dropdown();
};

function good_comment_attach(dropdown, src, pk, uuid, title){
  is_full_dropdown();
  img_block = dropdown.parentElement.parentElement.previousElementSibling;
  div = create_preview_good(src, pk, uuid, title)
  img_block.append(div);
  create_drag_attach_comment(block);
  add_file_dropdown();
  is_full_dropdown();
};
function good_list_comment_attach(dropdown, title, pk, count) {
  is_full_dropdown();
  img_block = dropdown.parentElement.parentElement.previousElementSibling;
  div = create_preview_good_list(title, pk, count);
  img_block.append(div);
  create_drag_attach_comment(block);
  add_file_dropdown();
  is_full_dropdown();
};

function article_comment_attach(_this, dropdown){
  is_full_dropdown(dropdown);
  uuid = _this.getAttribute('data-uuid');
  img_block = dropdown.parentElement.parentElement.previousElementSibling;
  div = create_preview_article(_this.querySelector("img").getAttribute('data-src'), uuid, _this.parentElement.querySelector(".article_title").innerHTML)
  img_block.append(div);
  create_drag_attach_comment(block);
  add_file_dropdown();
  is_full_dropdown();
};
function article_list_comment_attach(dropdown, title, pk, count) {
  is_full_dropdown();
  img_block = dropdown.parentElement.parentElement.previousElementSibling;
  div = create_preview_article_list(title, pk, count);
  img_block.append(div);
  create_drag_attach_comment(block);
  add_file_dropdown();
  is_full_dropdown();
};
