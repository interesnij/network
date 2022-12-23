on('#ajax', 'click', '.add_video', function() {
  create_fullscreen("/video/add_video_in_list/" + this.parentElement.parentElement.getAttribute("data-pk") + "/", "worker_fullscreen", false, true);
});
on('#ajax', 'click', '.uri_click', function() {
  this.nextElementSibling.style.display = "block";
  this.parentElement.nextElementSibling.querySelector("#add_video_btn").style.display = "block";
  fullscreen_resize()
});

on('#ajax', 'click', '.load_attach_video_list', function() {
  profile_list_block_attach(this, "/videolist/", "load_attach_video_list");
});

on('#ajax', 'click', '.load_video_list', function() {
  parent = this.parentElement.parentElement.parentElement;
  videolist_pk = parent.getAttribute("videolist-pk");
  create_fullscreen("/video/load_list/" + videolist_pk + "/", "item_fullscreen", false, true);
});

on('#ajax', 'click', '.video_list_detail', function() {
  videos = this.parentElement.querySelectorAll(".video_list_detail");
  video_pk = this.getAttribute("video-pk");
  for (var i = 0; i < videos.length; i++) {
    if (video_pk == videos[i].getAttribute("video-pk")) {
      counter = i;
      break
    }
  }

  play_video_list("/video/load_video/" + video_pk + "/", counter, video_pk)
});

on('#ajax', 'click', '.post_video', function() {
  videos = this.parentElement.querySelectorAll(".post_video");
  for (var i = 0; i < videos.length; i++) {
    videos[i].classList.remove("play")
  };
  this.classList.add("play");
  counter = 0;
  count = 0;
  for (var i = 0; i < videos.length; i++) {
    if (videos[i].classList.contains("play")) {
      counter = count;
      break
    }
    count += 1;
  }
  video_pk = this.getAttribute("video-pk");
  pk = this.parentElement.parentElement.parentElement.getAttribute("data-pk");
  counter = this.getAttribute('video-counter') - 1;
  play_video_list("/video/load_post_video/" + pk + "/", counter, video_pk)
});

on('#ajax', 'click', '.message_video', function() {
  videos = this.parentElement.querySelectorAll(".message_video");
  for (var i = 0; i < videos.length; i++) {
    videos[i].classList.remove("play")
  };
  this.classList.add("play");
  counter = 0;
  count = 0;
  for (var i = 0; i < videos.length; i++) {
    if (videos[i].classList.contains("play")) {
      counter = count;
      break
    }
    count += 1;
  }
  video_pk = this.getAttribute("video-pk");
  uuid = this.parentElement.parentElement.parentElement.parentElement.getAttribute("data-uuid");
  counter = this.getAttribute('video-counter') - 1;
  play_video_list("/video/load_message_video/" + uuid + "/", counter, video_pk)
});

on('#ajax', 'click', '.play_comment_video', function() {
  videos = this.parentElement.querySelectorAll(".play_comment_video");
  for (var i = 0; i < videos.length; i++) {
    videos[i].classList.remove("play")
  };
  this.classList.add("play");
  count = 0;
  for (var i = 0; i < videos.length; i++) {
    if (videos[i].classList.contains("play")) {
      counter = count;
      break
    }
    count += 1;
  }
  comment_pk = this.getAttribute("comment-pk");
  video_pk = this.getAttribute("video-pk");
  counter = this.getAttribute('video-counter') - 1;
  play_video_list("/video/load_comment_video/" + video_pk + "/", counter, video_pk);
});

on('body', 'click', '.video_fullscreen_resize', function() {
  video_window = document.querySelector(".video_fullscreen");
  video_window.classList.add("video_fullscreen_resized", "draggable");
  document.body.querySelector(".video_btn_big").style.display = "none";
  document.body.querySelector(".video_btn_small").style.display = "block";
  get_resize_screen();
  dragElement(document.querySelector(".draggable"));

});
on('body', 'click', '.video_fullscreen_normal', function() {
  video_window = document.querySelector(".video_fullscreen");
  video_window.style.top = "0"; video_window.style.left = "auto";
  video_window.classList.remove("video_fullscreen_resized", "draggable");
  document.body.querySelector(".video_btn_small").style.display = "none";
  document.body.querySelector(".video_btn_big").style.display = "block";
  get_normal_screen()
});

on('body', 'click', '#video_holder', function() {
  ggg = this;
  img = this.previousElementSibling.querySelector("#id_image");
  get_image_priview(ggg, img)
});
