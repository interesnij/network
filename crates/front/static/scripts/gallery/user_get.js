
on('#ajax', 'click', '.load_attach_photo_list', function() {
  profile_list_block_attach(this, "/photolist/", "load_attach_photo_list");
});

on('#ajax', 'click', '.photo_priview', function() {
  pk = this.getAttribute('photo-pk');
  create_fullscreen("/photos/preview_photo/" + pk, "photo_fullscreen", false, true);
});
on('#ajax', 'click', '.photo_edit', function() {
  document.querySelector('#block_description_form').style.display =="none";
});

on('#ajax', 'click', '.detail_photo', function() {
  photo_pk = this.getAttribute('photo-pk');
  create_fullscreen("/photos/load_photo/" + photo_pk, "photo_fullscreen", false, true);
});

on('#ajax', 'click', '.comment_photo', function() {
  photo_pk = this.getAttribute('photo-pk');
  card = this.parentElement.parentElement.parentElement.querySelector(".react_style");
  type = card.getAttribute('data-type');
  create_fullscreen("/photos/load_comment_photo/" + type + "/" + photo_pk, "photo_fullscreen", false, true);
});

on('#ajax', 'click', '.post_photo', function() {
  photo_pk = this.getAttribute('photo-pk');
  card = this.parentElement.parentElement.parentElement;
  post_pk = card.getAttribute('data-pk');
  create_fullscreen("/photos/load_post_photo/" + post_pk + "/" + photo_pk, "photo_fullscreen", false, true);
});
on('body', 'click', '.chat_photo', function() {
  photo_pk = this.getAttribute('photo-pk');
  pk = this.parentElement.getAttribute('chat-pk');
  create_fullscreen("/photos/chat_photo/" + pk + "/" + photo_pk, "photo_fullscreen", false, true);
});

on('#ajax', 'click', '.load_photo_list', function() {
  parent = this.parentElement.parentElement;
  photolist_pk = parent.getAttribute("photolist-pk");
  create_fullscreen("/photos/load_list/" + photolist_pk, "item_fullscreen", false, true);
});
