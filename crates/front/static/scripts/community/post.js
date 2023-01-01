on('#ajax', 'click', '#add_community_btn', function() {
  form = document.querySelector("#add_community_form");
  if (!form.querySelector("#id_name").value){
    form.querySelector("#id_name").style.border = "1px #FF0000 solid";
    toast_error("Название - обязательное поле!");
    return
  } else if (!form.querySelector("#sub_category").value || !form.querySelector("#subcat").firstChild){
    form.querySelector("#sub_category").style.border = "1px #FF0000 solid";
    toast_error("Тематика - обязательное поле!");
    return
  } else { this.disabled = true };

  	form_data = new FormData(form);
    var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      ajax_link.open( 'POST', '/communities/create_community', true );
      ajax_link.setRequestHeader('Content-Type', 'application/json');
      ajax_link.onreadystatechange = function () {
        if ( this.readyState == 4 && this.status == 200 ) {
            elem_ = document.createElement('span');
            elem_.innerHTML = ajax_link.responseText;
            ajax = elem_.querySelector("#reload_block");
            rtr = document.getElementById('ajax');
            rtr.innerHTML = ajax.innerHTML;
            pk = rtr.querySelector(".pk_saver").getAttribute("data-pk");
            window.scrollTo(0,0); 
            document.title = elem_.querySelector('title').innerHTML;
            close_work_fullscreen();
            if_list(rtr);
            window.history.pushState(null, "vfgffgfgf", "/public" + pk);

        } else { this.disabled = false }
      }
      ajax_link.send(JSON.stringify(form_data));
});

on('#ajax', 'click', '.member_create', function() {
  get_with_pk_and_reload("/communities/progs/add_member")
});
on('#ajax', 'click', '.member_delete', function() {
  get_with_pk_and_reload("/communities/progs/delete_member")
});

on('#ajax', 'click', '.member_follow_create', function() {
  get_with_pk_and_reload("/users/add_member")
});
on('#ajax', 'click', '.member_follow_delete', function() {
  get_with_pk_and_reload("/users/delete_member")
});
