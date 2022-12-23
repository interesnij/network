on('#ajax', 'click', '.track_edit', function() {
  parent = this.parentElement.parentElement.parentElement;
  blocks = document.body.querySelectorAll('.col-sm-12');
  for (var i = 0; i < blocks.length; i++) {blocks[i].classList.remove("edited_track")}

  parent.parentElement.parentElement.parentElement.classList.add("edited_track")
  create_fullscreen("/music/edit_track/" + parent.getAttribute("data-pk") +"/", "item_fullscreen", false, true);
});

on('#ajax', 'click', '.load_attach_music_list', function() {
  profile_list_block_attach(this, "/playlist/", "load_attach_music_list");
});

on('#ajax', 'click', '.load_music_list', function() {
  parent = this.parentElement.parentElement.parentElement;
  playlist_pk = parent.getAttribute("playlist-pk");
  create_fullscreen("/music/load_list/" + playlist_pk + "/", "worker_fullscreen", false, true);
});
