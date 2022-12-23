

on('#ajax', 'click', '.photo_off_comment', function() {
  send_photo_change(this, "/photos/off_comment/", "photo_on_comment", "Вкл. комментарии");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".load_photo_comments").style.display = "none"
});
on('#ajax', 'click', '.photo_on_comment', function() {
  send_photo_change(this, "/photos/on_comment/", "photo_off_comment", "Выкл. комментарии");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".load_photo_comments").style.display = "unset"
});

on('#ajax', 'click', '.photo_edit', function() {
  this.parentElement.nextElementSibling.style.display = "block"
});

on('#ajax', 'click', '.photo_description', function() {
  form = this.parentElement.parentElement.parentElement;
  form_data = new FormData(form.querySelector(".photo_description_form"));
  pk = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.getAttribute("data-pk");

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/photos/edit_photo_description/" + pk + "/", true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {

    elem = link_.responseText;
    new_post = document.createElement("span");
    new_post.innerHTML = elem;
    form.previousElementSibling.innerHTML = new_post.innerHTML + '<br><br><span class="photo_edit pointer">Редактировать</span>';
    form.style.display = "none";
    form.querySelector('#id_description').value = new_post.innerHTML;
  }}
  link_.send(form_data);
});

on('#ajax', 'click', '.photo_off_votes', function() {
  send_photo_change(this, "/photos/off_votes/", "photo_on_votes", "Вкл. реакции");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".like").style.display = "none";
  post.querySelector(".dislike").style.display = "none";
});
on('#ajax', 'click', '.photo_on_votes', function() {
  send_photo_change(this, "/photos/on_votes/", "photo_off_votes", "Выкл. реакции");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".like").style.display = "unset";
  post.querySelector(".dislike").style.display = "unset";
});

on('#ajax', 'click', '.photo_remove', function() {
  send_photo_change(this, "/photos/delete/", "user_photo_restore", "Отмена");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  this.parentElement.parentElement.nextElementSibling.style.display = "none";
  post.querySelector(".order-2").style.display = "none";
  post.querySelector(".card").style.opacity = "0.5";
  this.style.color = "#FF0000";
});
on('#ajax', 'click', '.photo_restore', function() {
  send_photo_change(this, "/photos/restore/", "user_photo_remove", "Удалить");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  this.parentElement.parentElement.nextElementSibling.style.display = "unset";
  post.querySelector(".order-2").style.display = "unset";
  post.querySelector(".card").style.opacity = "1";
});

on('#ajax', 'change', '#user_avatar_upload', function() {
  parent = this.parentElement;
  post_with_pk_and_reload(parent, "/photos/add_user_avatar/")
});
on('#ajax', 'change', '#community_avatar_upload', function() {
  parent = this.parentElement;
  post_with_pk_and_reload(parent, "/photos/add_community_avatar/")
});

on('#ajax', 'change', '#photo_attach', function() {
  if (this.files.length > 10) {
      toast_error("Не больше 10 фотографий");return
  }
  form_data = new FormData(this.parentElement);
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/photos/add_attach_photo/", true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    elem = link_.responseText;
    response = document.createElement("span");
    response.innerHTML = elem;
    photo_list = response.querySelectorAll(".pag");
    photo_post_upload_attach(photo_list, document.body.querySelector(".attach_block"));
    }
    close_work_fullscreen();
  }
  link_.send(form_data);
});

on('#ajax', 'change', '#photo_comment_attach', function() {
  if (this.files.length > 2) {
      toast_error("Не больше 2 фотографий");return
  }
  form_data = new FormData(this.parentElement);
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/photos/add_attach_photo/", true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    elem = link_.responseText;
    response = document.createElement("span");
    response.innerHTML = elem;
    photo_list = response.querySelectorAll(".pag");
    photo_comment_upload_attach(photo_list, document.body.querySelector(".current_file_dropdown").parentElement.parentElement);
    }
    close_work_fullscreen();
  }
  link_.send(form_data);
});

on('#ajax', 'click', '.mob_photo_off_comment', function() {
  mob_send_change(this, "/photos/off_comment/", "mob_u_photo_on_comment", "Вкл. комментарии");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".load_photo_comments").style.display = "none"
});
on('#ajax', 'click', '.mob_photo_on_comment', function() {
  mob_send_change(this, "/photos/on_comment/", "mob_u_photo_off_comment", "Выкл. комментарии");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".load_photo_comments").style.display = "unset"
});

on('#ajax', 'click', '.mob_photo_off_votes', function() {
  mob_send_change(this, "/photos/off_votes/", "mob_u_photo_on_votes", "Вкл. реакции");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".like").style.display = "none";
  post.querySelector(".dislike").style.display = "none";
});
on('#ajax', 'click', '.mob_photo_on_votes', function() {
  mob_send_change(this, "/photos/on_votes/", "mob_u_photo_off_votes", "Выкл. реакции");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".like").style.display = "unset";
  post.querySelector(".dislike").style.display = "unset";
});
on('#ajax', 'click', '.mob_photo_remove', function() {
  mob_send_change(this, "/photos/delete/", "mob_user_photo_restore", "Отмена");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".content_block").style.display = "none";
  post.querySelector(".image_card").style.opacity = "0.5";
});
on('#ajax', 'click', '.mob_photo_restore', function() {
  mob_send_change(this, "/photos/restore/", "mob_user_photo_remove", "Удалить");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".content_block").style.display = "unset";
  post.querySelector(".image_card").style.opacity = "1";
});
