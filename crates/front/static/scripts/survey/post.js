on('#ajax', 'click', '#add_survey_btn', function() {
  form_post = this.parentElement.parentElement.parentElement;
  form_data = new FormData(form_post);

  answers = form_post.querySelector("#answers_container");
  selectedOptions = answers.querySelectorAll(".answer");
  val = false;
  for (var i = 0; i < selectedOptions.length; i++) {
    if(selectedOptions[i].value) {val = true}
  }
  if (!document.body.querySelector("#id_title").value){
    document.body.querySelector("#id_title").style.border = "1px #FF0000 solid";
    toast_error("Название - обязательное поле!");
  } else if (!val){
    for (var i = 0; i < selectedOptions.length; i++) {selectedOptions[i].style.border = "1px #FF0000 solid"};
    toast_error("Задайте варианты ответов!");
    return
  } else {this.disabled = true}
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/survey/add_survey_in_list/" + form_post.getAttribute("data-pk") + "/", true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    elem = link_.responseText;
    _new = document.createElement("div");
    _new.innerHTML = elem;
    if (document.querySelector(".attach_block")){
      document.body.querySelector(".attach_block").append(_new.querySelector(".load_pag"));
      add_file_attach();
      is_full_attach();
    } else if (document.querySelector(".message_attach_block")){
      document.body.querySelector(".message_attach_block").append(_new.querySelector(".load_pag"));
      add_file_attach();
      is_full_attach();
    }
    else {
        container = document.body.querySelector(".is_paginate");
        container.insertAdjacentHTML('afterBegin', _new.innerHTML);
        container.querySelector(".items_empty") ? container.querySelector(".items_empty").style.display = "none" : null;
  };
  close_work_fullscreen();
  toast_info("Опрос создан!")
  }};
  link_.send(form_data);
});

on('#ajax', 'click', '.survey_vote', function() {
  _this = this; is_have_vote = false;
  parent = _this.parentElement;
  answers = parent.querySelectorAll(".lite_color");

  if (parent.classList.contains("no_multiple")) {
    // один вариант ответа на опрос

     if (_this.querySelector(".vote_svg").innerHTML) {
       _this.querySelector(".vote_svg").innerHTML = '';
       if (parent.querySelector("input")) {
         is_have_vote = true;
       } else { is_have_vote = false; }
     } else {
       for (var i = 0; i < answers.length; i++) {
         answers[i].querySelector(".vote_svg").innerHTML = "";
        };
       _this.querySelector(".vote_svg").innerHTML = '<input type="hidden" name="votes" value="' + _this.getAttribute("data-pk") + '"><svg fill="currentColor" style="width:15px;height:15px;" class="svg_default" viewBox="0 0 24 24"><path fill="none" d="M0 0h24v24H0z"></path><path d="M9 16.2L4.8 12l-1.4 1.4L9 19 21 7l-1.4-1.4L9 16.2z"></path></svg>';
       is_have_vote = true
     };
  }

  else {
    // несколько вариантов ответа на опрос
    if (_this.querySelector(".vote_svg").innerHTML) {
      _this.querySelector(".vote_svg").innerHTML = '';
      if (parent.querySelector("input")) {
        is_have_vote = true;
      } else { is_have_vote = false; }
    } else {
      is_have_vote = true;
      _this.querySelector(".vote_svg").innerHTML = '<input type="hidden" name="votes" value="' + _this.getAttribute("data-pk") + '"><svg fill="currentColor" style="width:15px;height:15px;" class="svg_default" viewBox="0 0 24 24"><path fill="none" d="M0 0h24v24H0z"></path><path d="M9 16.2L4.8 12l-1.4 1.4L9 19 21 7l-1.4-1.4L9 16.2z"></path></svg>';
    };

  };

  footer = parent.nextElementSibling;
  if (is_have_vote) {
    footer.querySelector(".votes_remove").classList.remove("hidden");
    footer.querySelector(".float-right").classList.remove("hidden");
    footer.querySelector(".float-right").removeAttribute("disabled")
  } else {
    footer.querySelector(".votes_remove").classList.add("hidden");
    footer.querySelector(".float-right").classList.add("hidden")
  }
});

on('#ajax', 'click', '.votes_remove', function() {
  _this = this;
  block = _this.parentElement.previousElementSibling;
  answers = block.querySelectorAll(".lite_color");
  for (var i = 0; i < answers.length; i++) {
    answers[i].querySelector(".vote_svg").innerHTML = "";
  };
  _this.classList.add("hidden");
  _this.nextElementSibling.classList.add("hidden");
});

on('#ajax', 'click', '#add_vote_survey_btn', function() {
  _this = this;
  form_post = _this.parentElement.parentElement;
  block = form_post.parentElement
  form_data = new FormData(form_post);
  token = document.body.getAttribute("data-csrf");

  _this.disabled = true;
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/survey/vote/" + block.getAttribute("data-pk") + "/", true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  link_.setRequestHeader('X-CSRFToken', token);

  link_.onreadystatechange = function () {
  if ( link_.readyState == 4 && link_.status == 200 ) {
    elem = link_.responseText;
    toast_info("Вы проголосовали!");
    block = form_post.querySelector(".answers_container");
    answers = block.querySelectorAll(".lite_color");
    for (var i = 0; i < answers.length; i++) {
      answers[i].classList.remove("survey_vote", "pointer");
    };
    form_post.querySelector(".votes_remove").classList.add("hidden");
    form_post.querySelector(".float-right").classList.add("hidden");

    list = elem.split(";");
    for (var i = 0; i < list.length; i++) {
      values = list[i].split(",");
      if (block.querySelector('[data-pk=' + '"' + values[0] + '"' + ']')) {
        answer = block.querySelector('[data-pk=' + '"' + values[0] + '"' + ']');
        procent = values[2] + "%";
        answer.querySelector(".count").innerHTML = values[1];
        answer.querySelector(".progress2").style.width = procent;
        answer.querySelector(".procent").innerHTML = procent;
      };

    };
    if (!block.classList.contains("no_edited")) {
      dropdown_menu = form_post.previousElementSibling.querySelector(".dropdown-menu");
      $span = document.createElement("span");
      $span.classList.add("dropdown-item", "survey_unvote");
      $span.innerHTML = 'Удалить голос';
      dropdown_menu.prepend($span)
    }
  } else { this.disabled = false };

  };
  link_.send(form_data);
});

on('#ajax', 'click', '.survey_unvote', function() {
  _this = this;
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'GET', "/survey/unvote/" + this.parentElement.parentElement.parentElement.getAttribute("data-pk") + "/", true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( link_.readyState == 4 && link_.status == 200 ) {
    elem = link_.responseText;
    block = _this.parentElement.parentElement.parentElement.querySelector(".answers_container");
    list = elem.split(";");
    for (var i = 0; i < list.length; i++) {
      values = list[i].split(",");
      if (block.querySelector('[data-pk=' + '"' + values[0] + '"' + ']')) {
        answer = block.querySelector('[data-pk=' + '"' + values[0] + '"' + ']');

        procent = values[2] + "%";
        answer.querySelector(".count").innerHTML = values[1];
        answer.querySelector(".progress2").style.width = procent;
        answer.querySelector(".procent").innerHTML = procent;
        answer.querySelector(".vote_svg").innerHTML = "";
      };
    };
    answers = _this.parentElement.parentElement.parentElement.querySelectorAll(".lite_color");
    for (var i = 0; i < answers.length; i++) {
      answers[i].classList.add("survey_vote", "pointer");
    };
    toast_info("Ваш голос удален!");
    _this.remove();
  };

  };
  link_.send();
});


on('body', 'click', '.survey_remove', function() {
  _this = this;
  block = _this.parentElement.parentElement.parentElement;

  _this.disabled = true;
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'GET', "/survey/delete/" + block.getAttribute("data-pk") + "/", true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( link_.readyState == 4 && link_.status == 200 ) {
    p = document.createElement("div");
    p.classList.add("card", "mb-3");
    p.style.padding = "20px";
    p.style.flexBasis = "100%";
    p.innerHTML = "<span class='survey_restore pointer' data-pk='" + block.getAttribute("data-pk") + "'>Опрос удален. <span class='underline'>Восстановить</span></span>";

    block.parentElement.insertBefore(p, block);
    block.style.display = "none"
  }};
  link_.send();
});
on('body', 'click', '.survey_restore', function() {
  item = this.parentElement.nextElementSibling;
  pk = this.getAttribute("data-pk");
  block = this.parentElement;
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'GET', "/survey/restore/" + pk + "/", true );
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    block.remove();
    item.style.display = "block";
    //main_container = document.body.querySelector(".main-container");
    //add_list_in_all_stat("restored_user_post",pk,main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"));
  }};
  link.send();
});

on('#ajax', 'click', '#edit_survey_btn', function() {
  form_post = this.parentElement.parentElement.parentElement;
  form_data = new FormData(form_post);

  answers = form_post.querySelector("#answers_container");
  selectedOptions = answers.querySelectorAll(".answer");
  val = false;
  for (var i = 0; i < selectedOptions.length; i++) {
    if(selectedOptions[i].value) {val = true}
  }
  if (!document.body.querySelector("#id_title").value){
    document.body.querySelector("#id_title").style.border = "1px #FF0000 solid";
    toast_error("Название - обязательное поле!");
  } else if (!val){
    for (var i = 0; i < selectedOptions.length; i++) {selectedOptions[i].style.border = "1px #FF0000 solid"};
    toast_error("Задайте варианты ответов!");
    return
  } else {this.disabled = true}
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/survey/edit/" + form_post.getAttribute("data-pk") + "/", true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
  toast_info("Опрос изменен!");
  close_work_fullscreen()
  }};
  link_.send(form_data);
});
