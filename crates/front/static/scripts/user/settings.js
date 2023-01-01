
on('#ajax', 'click', '#add_profile_exclude_users_btn', function() {
  form = this.parentElement.parentElement;
  post_include_exclude_users(form, '/users/settings/load_exclude_users')
});
on('#ajax', 'click', '#add_profile_include_users_btn', function() {
  form = this.parentElement.parentElement;
  post_include_exclude_users(form, '/users/settings/load_include_users')
});

on('#ajax', 'click', '#info_user_btn', function() {
  send_form_and_toast('/users/settings/info', document.body.querySelector("#info_user_form"), "Изменения приняты!")
});

on('#ajax', 'click', '#user_notify_profile_btn', function() {
  send_form_and_toast('/users/settings/notify', document.body.querySelector("#user_notify_profile_form"), "Изменения приняты!")
});
on('#ajax', 'click', '#user_notify_post_btn', function() {
  send_form_and_toast('/users/settings/notify_post', document.body.querySelector("#user_notify_post_form"), "Изменения приняты!")
});
on('#ajax', 'click', '#user_notify_photo_btn', function() {
  send_form_and_toast('/users/settings/notify_photo', document.body.querySelector("#user_notify_photo_form"), "Изменения приняты!")
});
on('#ajax', 'click', '#user_notify_good_btn', function() {
  send_form_and_toast('/users/settings/notify_good', document.body.querySelector("#user_notify_good_form"), "Изменения приняты!")
});
on('#ajax', 'click', '#user_notify_video_btn', function() {
  send_form_and_toast('/users/settings/notify_video', document.body.querySelector("#user_notify_video_form"), "Изменения приняты!")
});
on('#ajax', 'click', '#user_notify_music_btn', function() {
  send_form_and_toast('/users/settings/notify_music', document.body.querySelector("#user_notify_music_form"), "Изменения приняты!")
});
on('#ajax', 'click', '#u_edit_name_btn', function() {
  form = document.body.querySelector("#u_edit_name_form");
  field1 = form.querySelector("#first_name"); field2 = form.querySelector("#last_name");
  if (!field1.value){
    field1.style.border = "1px #FF0000 solid";
    toast_error("Введите имя!"); return
  } else if (!field2.value){
    field2.style.border = "1px #FF0000 solid";
    toast_error("Введите фамилию!"); return
  };
  send_form_and_toast('/users/settings/edit_name', form, "Имя / фамилия изменены!");
  document.body.querySelector(".edit_user_name").innerHTML = field1.value + " " + field2.value;
  close_work_fullscreen()
});
on('#ajax', 'click', '#u_edit_password_btn', function() {
  form = document.body.querySelector("#u_edit_password_form");
  field1 = form.querySelector("#password1"); field2 = form.querySelector("#password2");
  if (!field1.value){
    field1.style.border = "1px #FF0000 solid";
    toast_error("Введите новый пароль!"); return
  } else if (!field2.value){
    field2.style.border = "1px #FF0000 solid";
    toast_error("Повторите новый пароль!"); return
  } else if (field1.value != field2.value){
    field2.value = '';
    toast_error("Пароли не совпадают!"); return
  };
  send_form_and_toast('/users/settings/edit_password', form, "Пароль изменён!");
  close_work_fullscreen()
});

on('#ajax', 'click', '.edit_user_name', function() {
  create_fullscreen("/users/settings/edit_name", "worker_fullscreen", false, true);
});
on('#ajax', 'click', '.edit_user_password', function() {
  create_fullscreen("/users/settings/edit_password", "worker_fullscreen", false, true);
});
on('#ajax', 'click', '.edit_user_email', function() {
  create_fullscreen("/users/settings/edit_email", "worker_fullscreen", false, true);
});
on('#ajax', 'click', '.edit_user_phone', function() {
  create_fullscreen("/users/settings/edit_phone", "worker_fullscreen", false, true);
});
on('#ajax', 'click', '.edit_user_custom_link', function() {
  create_fullscreen("/users/settings/edit_link", "worker_fullscreen", false, true);
});
on('#ajax', 'click', '.user_identified_send', function() {
  create_fullscreen("/users/settings/identify_send", "worker_fullscreen", false, true);
});
on('#ajax', 'click', '.remove_user_profile', function() {
  create_fullscreen("/users/settings/remove_profile", "worker_fullscreen", false, true);
});
