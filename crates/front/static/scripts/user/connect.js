on('#ajax', 'click', '.follow_create', function() {
  _this = this;
  document.body.querySelector(".pk_saver") ?  pk = document.body.querySelector(".pk_saver").getAttribute("data-pk") : pk = _this.parentElement.parentElement.parentElement.getAttribute("data-pk");
  form_data = new FormData();
  form_data.append("id", pk);
  
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.timeout = 30000;
  link_.open( 'POST', "/users/progs/follow", true );
  link_.setRequestHeader('Content-Type', 'application/json');
  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    document.body.querySelector(".pk_saver") ? this_page_reload('/id' + pk + '/')
     : (a = document.createElement("a"), a.classList.add("small", "follow_delete", "pointer"), a.innerHTML = 'Отписаться', _this.parentElement.append(a), _this.remove())
  }};
  link_.send(JSON.stringify(form_data));
});
on('#ajax', 'click', '.follow_delete', function() {
  _this = this;
  document.body.querySelector(".pk_saver") 
    ? pk = document.body.querySelector(".pk_saver").getAttribute("data-pk")
    : pk = _this.parentElement.parentElement.parentElement.getAttribute("data-pk");
  
    form_data = new FormData();
  form_data.append("id", pk);
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.timeout = 30000;
  link_.addEventListener('loadstart', _loadstart);
  link_.open( 'POST', "/users/progs/unfollow", true );
  link_.setRequestHeader('Content-Type', 'application/json');
  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    document.body.querySelector(".pk_saver") ? this_page_reload('/id' + pk + '/')
          : (a = document.createElement("a"), a.classList.add("small", "follow_create", "pointer"), a.innerHTML = 'Подписаться', _this.parentElement.append(a), _this.remove())
  }};
  link_.ontimeout = function() {alert( 'Извините, запрос превысил максимальное время' )}

  function _loadstart() {console.log("Запрос начат")}
  link_.send(JSON.stringify(form_data));
});
on('#ajax', 'click', '.follow_view', function() {
  _this = this;
  document.body.querySelector(".pk_saver") ?  pk = document.body.querySelector(".pk_saver").getAttribute("data-pk") : null
  form_data = new FormData();
  form_data.append("id", pk);

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/users/progs/follow_view", true );
  link_.setRequestHeader('Content-Type', 'application/json');
  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    _this.remove();
    toast_info("Пользователь оставлен в подписчиках");
    minus_new_followers();
  }};
  link_.send(JSON.stringify(form_data));
});

on('#ajax', 'click', '.connect_create', function() {
  _this = this;
  document.body.querySelector(".pk_saver") 
    ? pk = document.body.querySelector(".pk_saver").getAttribute("data-pk")
    : pk = this.parentElement.parentElement.parentElement.getAttribute("data-pk");
  form_data = new FormData();
  form_data.append("id", pk);
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );

  link_.open( 'POST', "/users/progs/friend", true );
  link_.setRequestHeader('Content-Type', 'application/json');
  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    minus_new_followers();
    document.body.querySelector(".pk_saver") ? this_page_reload('/id')
        : (a = document.createElement("a"), a.classList.add("small", "connect_delete", "pointer"), a.innerHTML = 'Убрать из друзей', _this.parentElement.append(a), _this.remove())
  }}
  link_.send(JSON.stringify(form_data));
});
on('#ajax', 'click', '.connect_delete', function() {
  _this = this;
  document.body.querySelector(".pk_saver") ?  pk = document.body.querySelector(".pk_saver").getAttribute("data-pk") : pk = this.parentElement.parentElement.parentElement.getAttribute("data-pk");
  form_data = new FormData();
  form_data.append("id", pk);
  
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/users/progs/unfriend", true );
  link_.setRequestHeader('Content-Type', 'application/json');
  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    document.body.querySelector(".pk_saver") ? this_page_reload('/id' + pk)
      : (a = document.createElement("a"), a.classList.add("small", "connect_create", "pointer"), a.innerHTML = 'Добавить в друзья', _this.parentElement.append(a), _this.remove())
  }};
  link_.send(JSON.stringify(form_data));
});
on('#ajax', 'click', '.user_block', function() {
  _this = this;
  document.body.querySelector(".pk_saver") ?  pk = document.body.querySelector(".pk_saver").getAttribute("data-pk") : pk = this.parentElement.parentElement.parentElement.getAttribute("data-pk");
  form_data = new FormData();
  form_data.append("id", pk);
  
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/users/progs/block", true );
  link_.setRequestHeader('Content-Type', 'application/json');
  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    document.body.querySelector(".pk_saver") ? this_page_reload('/id' + pk)
    : (a = document.createElement("a"), a.classList.add("small", "user_unblock", "pointer"), a.innerHTML = 'Разблокировать', _this.parentElement.append(a), _this.remove())
  }};
  link_.send(JSON.stringify(form_data));
});
on('#ajax', 'click', '.user_unblock', function() {
  _this = this;
  document.body.querySelector(".pk_saver") ?  pk = document.body.querySelector(".pk_saver").getAttribute("data-pk") : pk = this.parentElement.parentElement.parentElement.getAttribute("data-pk");
  form_data = new FormData();
  form_data.append("id", pk);
  
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/users/progs/block", true );
  link_.setRequestHeader('Content-Type', 'application/json');
  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    document.body.querySelector(".pk_saver") ? this_page_reload('/id' + pk)
    : (a = document.createElement("a"), a.classList.add("small", "user_block", "pointer"), a.innerHTML = 'Заблокировать', _this.parentElement.append(a), _this.remove())
  }};
  link_.send(JSON.stringify(form_data));
});
