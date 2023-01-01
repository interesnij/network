
on('#ajax', 'click', '.community_member_create', function() {
    li = this.parentElement.parentElement.parentElement.parentElement;
    community_pk = document.body.querySelector(".pk_saver").getAttribute("data-pk");
    user_pk = li.getAttribute("data-pk");
    form_data = new FormData();
    form_data.append("user_id", user_pk);
    form_data.append("community_id", community_pk);

    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/communities/progs/manager_add_member", true );
    link.setRequestHeader('Content-Type', 'application/json');
    link.onreadystatechange = function () {
      if ( this.readyState == 4 && this.status == 200 ) {
        li.remove()
      }};
  link.send( JSON.stringify(form_data) );
});
on('#ajax', 'click', '.community_member_delete', function() {
    li = this.parentElement.parentElement.parentElement.parentElement;
    community_pk = document.body.querySelector(".pk_saver").getAttribute("data-pk");
    user_pk = li.getAttribute("data-pk");
    form_data = new FormData();
    form_data.append("user_id", user_pk);
    form_data.append("community_id", community_pk);

    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'GET', "/communities/progs/manager_delete_member", true );
    link.setRequestHeader('Content-Type', 'application/json');
    link.onreadystatechange = function () {
      if ( this.readyState == 4 && this.status == 200 ) {
        li.remove()
      }};
  link.send( JSON.stringify(form_data) );
});

on('#ajax', 'click', '#community_private_post_btn', function() {
  send_form_and_toast('/communities/manage/private_post', document.body.querySelector("#community_private_post_form"), "Изменения приняты!")
});
on('#ajax', 'click', '#community_private_photo_btn', function() {
  send_form_and_toast('/communities/manage/private_photo', document.body.querySelector("#community_private_photo_form"), "Изменения приняты!")
});
on('#ajax', 'click', '#community_private_good_btn', function() {
  send_form_and_toast('/communities/manage/private_good', document.body.querySelector("#community_private_good_form"), "Изменения приняты!")
});
on('#ajax', 'click', '#community_private_video_btn', function() {
  send_form_and_toast('/communities/manage/private_video', document.body.querySelector("#community_private_video_form"), "Изменения приняты!")
});
on('#ajax', 'click', '#community_private_music_btn', function() {
  send_form_and_toast('/communities/manage/private_music', document.body.querySelector("#community_private_music_form"), "Изменения приняты!")
});

on('#ajax', 'click', '#community_notify_post_btn', function() {
  send_form_and_toast('/communities/manage/notify_post', document.body.querySelector("#community_notify_post_form"), "Изменения приняты!")
});
on('#ajax', 'click', '#community_notify_photo_btn', function() {
  send_form_and_toast('/communities/manage/notify_photo', document.body.querySelector("#community_notify_photo_form"), "Изменения приняты!")
});
on('#ajax', 'click', '#community_notify_good_btn', function() {
  send_form_and_toast('/communities/manage/notify_good', document.body.querySelector("#community_notify_good_form"), "Изменения приняты!")
});
on('#ajax', 'click', '#community_notify_video_btn', function() {
  send_form_and_toast('/communities/manage/notify_video', document.body.querySelector("#community_notify_video_form"), "Изменения приняты!")
});
on('#ajax', 'click', '#community_notify_music_btn', function() {
  send_form_and_toast('/communities/manage/notify_music', document.body.querySelector("#community_notify_music_form"), "Изменения приняты!")
});

on('#ajax', 'click', '#community_sections_btn', function() {
  send_form_and_toast('/communities/manage/sections', document.body.querySelector("#community_sections_form"), "Изменения приняты!")
});
on('#ajax', 'click', '#community_general_info_btn', function() {
  send_form_and_toast('/communities/manage/general', document.body.querySelector("#community_general_info_form"), "Изменения приняты!")
});

on('#ajax', 'click', '#add_community_exclude_users_btn', function() {
  form = this.parentElement.parentElement;
  post_include_exclude_users(form, '/communities/manage/load_exclude_users')
});
on('#ajax', 'click', '#add_community_include_users_btn', function() {
  form = this.parentElement.parentElement;
  post_include_exclude_users(form, '/communities/manage/load_include_users')
});
