on('#ajax', 'change', '.goods_category', function() {
  var val = this.value;
  if (val == '') {
    document.getElementById('subcat').innerHTML = "";
  } else {
    var link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'GET', "/goods/progs/cat/" + val, true );
    link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    link.onreadystatechange = function () {
      if ( link.readyState == 4 ) {
          if ( link.status == 200 ) {
              var sub = document.getElementById("subcat");
              sub.innerHTML = link.responseText;
          }
      }
  };
  link.send( null );
  };
});

function good_gallery(loader){
  thumb_list = loader.querySelectorAll(".thumb_list li");
  thumb = loader.querySelector(".big_img");
  thumb_list.forEach((item) => {
    item.addEventListener("mouseover", function () {
    image = item.children[0].src;
      thumb.src = image;
    });
  });
};

on('#ajax', 'click', '.good_edit', function() {
  this.parentElement.nextElementSibling.style.display = "block"
});

on('#ajax', 'click', '.good_off_votes', function() {
  send_good_change(this, "/goods/user_progs/off_votes", "good_on_votes", "Вкл. реакции");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".like").style.display = "none";
  post.querySelector(".dislike").style.display = "none";
});
on('#ajax', 'click', '.u_good_on_votes', function() {
  send_good_change(this, "/goods/on_votes", "good_off_votes", "Выкл. реакции");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".like").style.display = "unset";
  post.querySelector(".dislike").style.display = "unset";
});
on('#ajax', 'click', '.good_hide', function() {
  send_good_change(this, "/goods/hide", "good_unhide", "Товар не виден");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
});
on('#ajax', 'click', '.good_unhide', function() {
  send_good_change(this, "/goods/unhide", "good_hide", "Товар виден");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
});

on('#ajax', 'click', '.good_remove', function() {
  send_good_change(this, "/goods/delete", "good_restore", "Отмена");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  this.parentElement.parentElement.nextElementSibling.style.display = "none";
  post.querySelector(".order-2").style.display = "none";
  post.querySelector(".card").style.opacity = "0.5";
  this.style.color = "#FF0000";
});
on('#ajax', 'click', '.good_restore', function() {
  send_good_change(this, "/goods/restore", "good_remove", "Удалить");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  this.parentElement.parentElement.nextElementSibling.style.display = "unset";
  post.querySelector(".order-2").style.display = "unset";
  post.querySelector(".card").style.opacity = "1";
});


on('#ajax', 'click', '#good_image', function() {
  img = this.previousElementSibling.querySelector("#id_image")
  get_image_priview(this, img);
});
on('#ajax', 'click', '#good_image2', function() {
  img = this.previousElementSibling.querySelector(".image")
  get_image_priview(this, img);
});
on('#ajax', 'click', '#good_image3', function() {
  img = this.previousElementSibling.querySelector(".image")
  get_image_priview(this, img);
});
on('#ajax', 'click', '#good_image4', function() {
  img = this.previousElementSibling.querySelector(".image")
  get_image_priview(this, img);
});
on('#ajax', 'click', '#good_image5', function() {
  img = this.previousElementSibling.querySelector(".image")
  get_image_priview(this, img);
});
on('#ajax', 'click', '.good_add', function() {
  create_fullscreen('/goods/add_good_in_list/' + this.parentElement.parentElement.getAttribute("data-pk"), "item_fullscreen", false, true);
});

on('#ajax', 'click', '#add_good_btn', function() {
  form_post = this.parentElement.parentElement;
  form_data = new FormData(form_post);
  pk = form_post.getAttribute("data-pk");
  form_data.append("list_id", pk);

  if (!document.body.querySelector("#id_title").value){
    document.body.querySelector("#id_title").style.border = "1px #FF0000 solid";
    toast_error("Название - обязательное поле!"); return
  } else if (!document.body.querySelector("#category").value){
    document.body.querySelector("#category").style.border = "1px #FF0000 solid";
    toast_error("Категория - обязательное поле!"); return
  } else if (!document.body.querySelector("#id_description").value){
    document.body.querySelector("#id_description").style.border = "1px #FF0000 solid";
    toast_error("Описание товара - обязательное поле!"); return
  } else if (!document.body.querySelector("#id_image").value){
    document.body.querySelector("#good_image").style.border = "1px #FF0000 solid !important";
    toast_error("Фотография на обложку обязательна!"); return
  } else {this.disabled = true};

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/goods/add_good_in_list", true );
  link_.setRequestHeader('Content-Type', 'application/json');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    elem = link_.responseText;
    new_good = document.createElement("div");
    new_good.innerHTML = elem;
    good = new_good.querySelector(".good_detail");
    data_pk = good.getAttribute('good-pk');
    src = good.querySelector("img").getAttribute('src');
    title = good.querySelector(".good_title").innerHTML;

    if (document.body.querySelector(".current_file_dropdown")){
      check_good_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.previousElementSibling, _this, pk) ? null : (good_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, media_block, pk))
    } else if (document.body.querySelector(".attach_block")){
      check_good_in_block(document.body.querySelector(".attach_block"), _this, pk) ? null : (good_post_attach(document.body.querySelector(".attach_block"), media_block, pk))
    } else if (document.body.querySelector(".message_attach_block")){
      check_good_in_block(document.body.querySelector(".message_attach_block"), _this, pk) ? null : (good_message_attach(document.body.querySelector(".message_attach_block"), media_block, pk))
    }
    else {
      uuid = document.body.querySelector(".pk_saver").getAttribute("data-uuid");
      span1 = new_good.querySelector('.span1')
      if (span1.classList.contains(data_pk)){
        container = document.body.querySelector(".is_paginate");
        container.insertAdjacentHTML('afterBegin', new_good.innerHTML);
        container.querySelector(".items_empty") ? container.querySelector(".items_empty").style.display = "none" : null;
      }
  };
  close_work_fullscreen();
  toast_info("Товар создан!")
  }};
  link_.send(JSON.stringify(form_data));
});

on('#ajax', 'click', '.mob_good_off_comment', function() {
  send_mob_good_change(this, "/goods/off_comment", "mob_u_good_on_comment", "Вкл. комментарии");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".load_good_comments").style.display = "none"
})
on('#ajax', 'click', '.mob_good_on_comment', function() {
  send_mob_good_change(this, "/goods/on_comment", "mob_u_good_off_comment", "Выкл. комментарии");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".load_good_comments").style.display = "unset"
});
on('#ajax', 'click', '.mob_good_off_votes', function() {
  send_mob_good_change(this, "/goods/off_votes", "mob_u_good_on_votes", "Вкл. реакции");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".like").style.display = "none";
  post.querySelector(".dislike").style.display = "none";
});
on('#ajax', 'click', '.mob_good_on_votes', function() {
  send_mob_good_change(this, "/goods/on_votes", "mob_u_good_off_votes", "Выкл. реакции");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".like").style.display = "unset";
  post.querySelector(".dislike").style.display = "unset";
});
on('#ajax', 'click', '.mob_good_hide', function() {
  send_mob_good_change(this, "/goods/hide", "mob_u_good_unhide", "Товар не виден");
});
on('#ajax', 'click', '.mob_good_unhide', function() {
  send_mob_good_change(this, "/goods/unhide", "mob_u_good_hide", "Товар виден");
});
on('#ajax', 'click', '.mob_good_remove', function() {
  send_mob_good_change(this, "/goods/delete", "mob_user_good_restore", "Отмена");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".good_card").style.display = "none";
  post.querySelector(".card").style.opacity = "0.5";
  this.style.color = "#FF0000";
});
on('#ajax', 'click', '.mob_good_restore', function() {
  send_mob_good_change(this, "/goods/restore", "mob_user_good_remove", "Удалить");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".good_card").style.display = "unset";
  post.querySelector(".card").style.opacity = "1";
});
