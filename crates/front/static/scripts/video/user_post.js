
on('#video_loader', 'click', '.video_off_comment', function() {
  send_photo_change(this, "/video/off_comment", "video_on_comment", "Вкл. комментарии");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".load_video_comments").style.display = "none"
});
on('#video_loader', 'click', '.video_on_comment', function() {
  send_photo_change(this, "/video/on_comment", "video_off_comment", "Выкл. комментарии");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".load_video_comments").style.display = "unset"
});

on('#video_loader', 'click', '.video_off_votes', function() {
  send_photo_change(this, "/video/off_votes", "video_on_votes", "Вкл. реакции");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".like").style.display = "none";
  post.querySelector(".dislike").style.display = "none";
});
on('#video_loader', 'click', '.video_on_votes', function() {
  send_photo_change(this, "/video/on_votes", "video_off_votes", "Выкл. реакции");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".like").style.display = "unset";
  post.querySelector(".dislike").style.display = "unset";
});

on('body', 'click', '.video_remove', function() {
  send_photo_change(this, "/video/delete", "video_restore", "Отмена");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  this.parentElement.parentElement.nextElementSibling.style.display = "none";
  post.querySelector(".order-2").style.display = "none";
  post.querySelector(".card").style.opacity = "0.5";
  this.style.color = "#FF0000";
});
on('body', 'click', '.video_restore', function() {
  send_photo_change(this, "/video/restore", "user_video_remove", "Удалить");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  this.parentElement.parentElement.nextElementSibling.style.display = "unset";
  post.querySelector(".order-2").style.display = "unset";
  post.querySelector(".card").style.opacity = "1";
});


on('#ajax', 'click', '#edit_video_btn', function() {
  form_post = this.parentElement.parentElement.parentElement;
  if (!form_post.querySelector("#id_title").value) {
    form_post.querySelector("#id_title").style.border = "1px #FF0000 solid";
    toast_error("Назвите видеозапись!");
    return
  }
  else if (!form_post.querySelector(".smile_supported").innerHTML) {
    form_post.querySelector(".smile_supported").style.border = "1px #FF0000 solid";
    toast_error("Опишите видеозапись!");
    return
  } else { this.disabled = true };

  text_val = form_post.querySelector(".smile_supported");
  _val = format_text(text_val);
  _text = _val.innerHTML;

  $input = document.createElement("input");
  $input.setAttribute("name", "description");
  $input.setAttribute("type", "hidden");
  $input.classList.add("input_text");
  $input.value = _text;
  form_post.append($input);
  form_data = new FormData(form_post);
  
  _case = form_post.querySelector("#upload_video_pk");
  pk = _case.value;
  form_data.append("id", pk);

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/video/edit_video", true );
  link_.setRequestHeader('Content-Type', 'application/json');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    if (_case.classList.contains("new")) {
      elem = link_.responseText;
      new_post = document.createElement("span");
      new_post.innerHTML = elem;

      lenta_load = document.body.querySelector(".is_paginate");
      lenta_load.insertAdjacentHTML('afterBegin', new_post.innerHTML);
      lenta_load.querySelector(".items_empty") ? lenta_load.querySelector(".items_empty").style.display = "none" : null;
    }
    close_work_fullscreen();
    //main_container = document.body.querySelector(".main-container");
    //add_list_in_all_stat("created_user_post",new_post.querySelector(".pag").getAttribute("data-pk"),main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"))
  } else {
        new_post = document.createElement("span");
        new_post.innerHTML = link_.responseText;
        if (new_post.querySelector(".exception_value")){
          text = new_post.querySelector(".exception_value").innerHTML;
          toast_info(text)
        }
    }
  };

  link_.send(JSON.stringify(form_data));
});

on('#ajax', 'click', '#add_video_btn', function() {
  form_post = this.parentElement.parentElement;
  block = form_post.parentElement.parentElement.parentElement;
  if (!form_post.querySelector("#id_uri").value) {
    form_post.querySelector("#id_uri").style.border = "1px #FF0000 solid";
    toast_error("Введите адрес видеозаписи!");
    return
  }
  else { this.disabled = true };
  form_data = new FormData(form_post);
  form_data.append("id", form_post.getAttribute("data-pk"));

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/video/add_video_in_list", true );
  link_.setRequestHeader('Content-Type', 'application/json');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    new_post = document.createElement("span");
    new_post.innerHTML = link_.responseText;
    block.innerHTML = new_post.innerHTML;
    fullscreen_resize();
    //main_container = document.body.querySelector(".main-container");
    //add_list_in_all_stat("created_user_post",new_post.querySelector(".pag").getAttribute("data-pk"),main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"))
  } else {
        new_post = document.createElement("span");
        new_post.innerHTML = link_.responseText;
        if (new_post.querySelector(".exception_value")){
          text = new_post.querySelector(".exception_value").innerHTML;
          toast_info(text)
        }
    }
  };

  link_.send(JSON.stringify(form_data));
});
