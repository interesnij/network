
on('#ajax', 'click', '.load_attach_good_list', function() {
  profile_list_block_attach(this, "/goodlist/", "load_attach_good_list");
});

on('#ajax', 'click', '.good_detail', function() {
  pk = this.getAttribute('good-pk');
  create_fullscreen('/goods/good/' + pk, "item_fullscreen", false, true);
  container = document.body.querySelector("#fullscreens_container");
  loader = container.querySelector(".card_fullscreen");
  setTimeout(function() {good_gallery(loader)}, 1000)
});

on('#ajax', 'click', '.load_good_list', function() {
  parent = this.parentElement.parentElement.parentElement;
  goodlist_pk = parent.getAttribute("goodlist-pk");
  create_fullscreen("/goods/load_list/" + goodlist_pk, "item_fullscreen", false, true);
});
