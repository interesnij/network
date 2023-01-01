
on('#ajax', 'click', '#edit_track_btn', function() {
  form = this.parentElement.parentElement.parentElement;
  pk = form.getAttribute("data-pk");
  form_data = new FormData(form);
  form_data.append("id", pk);

  if (!form.querySelector("#id_title").value){
    form.querySelector("#id_title").style.border = "1px #FF0000 solid";
    toast_error("Название - обязательное поле!");
    return
  } else { this.disabled = true };

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/music/edit_track", true );
  link_.setRequestHeader('Content-Type', 'application/json');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    toast_info("Аудиозапись изменена!")
    close_work_fullscreen();
    jsonResponse = JSON.parse(link_.responseText);
    track = document.body.querySelector(".edited_track");
    track.querySelector("h6").innerHTML = jsonResponse.title;
  }};

  link_.send(JSON.stringify(form_data));
});

on('#ajax', 'click', '.track_remove', function() {
  saver = this.parentElement.parentElement.parentElement;
  pk = saver.getAttribute("data-pk")
  form_data = new FormData();
  form_data.append("id", pk);
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/music/delete_track", true );
  link.setRequestHeader('Content-Type', 'application/json');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    div = document.createElement("div");
    div.classList.add("col-12");
    div.style.padding = "10px";
    div.style.display = "block";
    div.innerHTML = "Аудиозапись удалена. <span class='track_restore pointer underline' data-pk='" + pk + "'>Восстановить</span>";
    item = saver.parentElement.parentElement.parentElement;
    item.style.display = "none"; item.parentElement.insertBefore(div, item)
  }};
  link.send(JSON.stringify(form_data));
});
on('#ajax', 'click', '.track_restore', function() {
  pk = this.getAttribute("data-pk");
  form_data = new FormData();
  form_data.append("id", pk);
  block = this.parentElement; next = block.nextElementSibling;
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/music/recover_track", true ); 
  link.setRequestHeader('Content-Type', 'application/json');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    block.remove();
    next.style.display = "block";
  }};
  link.send(JSON.stringify(form_data));
});
