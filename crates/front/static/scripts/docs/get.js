
on('#ajax', 'click', '.load_attach_doc_list', function() {
  profile_list_block_attach(this, "/doclist/", "load_attach_doc_list");
});

on('body', 'click', '.doc_edit', function() {
  parent = this.parentElement.parentElement.parentElement;
  blocks = document.body.querySelectorAll('.col-sm-12');
  for (var i = 0; i < blocks.length; i++) {blocks[i].classList.remove("edited_doc")}

  parent.parentElement.parentElement.parentElement.classList.add("edited_doc")
  create_fullscreen("/docs/edit_doc/" + parent.getAttribute("data-pk") +"/", "worker_fullscreen", false, true);
});

on('#ajax', 'click', '.load_doc_list', function() {
  parent = this.parentElement.parentElement.parentElement;
  doclist_pk = parent.getAttribute("doclist-pk");
  create_fullscreen("/docs/load_list/" + doclist_pk + "/", "item_fullscreen", false, true);
});
