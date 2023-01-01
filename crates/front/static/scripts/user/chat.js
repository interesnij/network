CURRENT_BLOB = null;
is_voise_sender_open = true;
function remove_voice_console(form) {
  form.querySelector('#my_audio').style.display = "none";
  form.querySelector('.delete_voice_btn').style.display = "none";
  form.querySelector('.mic_visual_canvas').style.display = "none";
  form.querySelector('.smile_supported').style.display = "block";
  form.querySelector('.file_dropdown_2').style.display = "contents";
  form.querySelector('.form_smilies').style.display = "block";
  form.querySelector('.voice_stop_btn').style.display = "none";
  show_message_form_voice_btn();
};

 async function get_record_stream() {
   if (!document.body.querySelector(".mic_visual_canvas")) {
     return
   };
   let TIMER_VALUE = 0;
   let leftchannel = [];
   let rightchannel = [];
   let recorder = null;
   let recording = false;
   let recordingLength = 0;
   let volume = null;
   let audioInput = null;
   let sampleRate = null;
   let AudioContext = window.AudioContext || window.webkitAudioContext;
   let context = null;
   let analyser = null;
   let canvas = document.body.querySelector('.mic_visual_canvas');
   let canvasCtx = canvas.getContext("2d");
   let visualSelect = "";
   let stream = null;
   let tested = false;
   let timer_block = document.body.querySelector(".smile_supported");

  try {
    window.stream = stream = await getStream();
    console.log('Есть поток');
  } catch(err) {
    console.log('Проблема с микрофоном', err);
  };

  const deviceInfos = await navigator.mediaDevices.enumerateDevices();
  var mics = [];
  for (let i = 0; i !== deviceInfos.length; ++i) {
    let deviceInfo = deviceInfos[i];
    if (deviceInfo.kind === 'audioinput') {
      mics.push(deviceInfo);
      let label = deviceInfo.label || 'микрофон ' + mics.length;
    }
  };
  setUpRecording();

  function setUpRecording() {
    context = new AudioContext();
    sampleRate = context.sampleRate;
    console.log(sampleRate);
    volume = context.createGain();
    audioInput = context.createMediaStreamSource(stream);
    analyser = context.createAnalyser();
    audioInput.connect(analyser);
    let bufferSize = 2048;
    let recorder = context.createScriptProcessor(bufferSize, 2, 2);
    analyser.connect(recorder);
    recorder.connect(context.destination);
    recorder.onaudioprocess = function(e) {
      if (!recording) return;
      console.log('Запись!');
      let left = e.inputBuffer.getChannelData(0);
      let right = e.inputBuffer.getChannelData(1);
      if (!tested) {
        tested = true;
        if ( !left.reduce((a, b) => a + b) ) {
          console.log("There seems to be an issue with your Mic");
          stop();
          stream.getTracks().forEach(function(track) {
            track.stop();
          });
          context.close();
        }
      }
      leftchannel.push(new Float32Array(left));
      rightchannel.push(new Float32Array(right));
      recordingLength += bufferSize;
    };
    visualize();
  };

  function getStream(constraints) {
    if (!constraints) {
      constraints = { audio: true, video: false };
    }
    return navigator.mediaDevices.getUserMedia(constraints);
  }

  function mergeBuffers(channelBuffer, recordingLength) {
    let result = new Float32Array(recordingLength);
    let offset = 0;
    let lng = channelBuffer.length;
    for (let i = 0; i < lng; i++){
      let buffer = channelBuffer[i];
      result.set(buffer, offset);
      offset += buffer.length;
    }
    return result;
  }

  function interleave(leftChannel, rightChannel){
    let length = leftChannel.length + rightChannel.length;
    let result = new Float32Array(length);
    let inputIndex = 0;
    for (let index = 0; index < length; ){
      result[index++] = leftChannel[inputIndex];
      result[index++] = rightChannel[inputIndex];
      inputIndex++;
    }
    return result;
  }

  function writeUTFBytes(view, offset, string){
    let lng = string.length;
    for (let i = 0; i < lng; i++){
      view.setUint8(offset + i, string.charCodeAt(i));
    }
  }

  function start() {
    recording = true;
    document.querySelector('.user_typed_box').style.visibility = 'visible'
    leftchannel.length = rightchannel.length = 0;
    recordingLength = 0;
    console.log('context: ', !!context);
    if (!context) {
      setUpRecording();
    };
    TIMER_VALUE = 603;
  }

  function stop() {
    console.log('Stop');
    recording = false;
    let leftBuffer = mergeBuffers ( leftchannel, recordingLength );
    let rightBuffer = mergeBuffers ( rightchannel, recordingLength );
    let interleaved = interleave ( leftBuffer, rightBuffer );
    let buffer = new ArrayBuffer(44 + interleaved.length * 2); // * 2
    let view = new DataView(buffer);
    writeUTFBytes(view, 0, 'RIFF');
    view.setUint32(4, 44 + interleaved.length * 2, true); // * 2
    writeUTFBytes(view, 8, 'WAVE');
    writeUTFBytes(view, 12, 'fmt ');
    view.setUint32(16, 16, true);
    view.setUint16(20, 1, true);
    view.setUint16(22, 2, true); // 1 - кол во каналов
    view.setUint32(24, sampleRate, true);
    view.setUint32(28, sampleRate * 4, true);
    view.setUint16(32, 4, true);
    view.setUint16(34, 16, true);
    writeUTFBytes(view, 36, 'data');
    view.setUint32(40, interleaved.length * 2, true); // * 2
    let lng = interleaved.length;
    let index = 44;
    let volume = 1;
    for (let i = 0; i < lng; i++){
        view.setInt16(index, interleaved[i] * (0x7FFF * volume), true);
        index += 2;
    };

    let blob = new Blob ( [ view ], { type : 'audio/wav' } );
    const audioUrl = URL.createObjectURL(blob);
    console.log('BLOB ', blob);
    console.log('URL ', audioUrl);
    document.querySelector('#my_audio').setAttribute('src', audioUrl);
    CURRENT_BLOB = blob;
    TIMER_VALUE = 0;
  }

  function visualize() {
    WIDTH = canvas.width;
    HEIGHT = canvas.height;
    CENTERX = canvas.width / 2;
    CENTERY = canvas.height / 2;
    analyser.fftSize = 2048;
    var bufferLength = analyser.fftSize;
    var dataArray = new Uint8Array(bufferLength);
    canvasCtx.clearRect(0, 0, WIDTH, HEIGHT);
    var draw = function() {
      drawVisual = requestAnimationFrame(draw);
      analyser.getByteTimeDomainData(dataArray);
      canvasCtx.fillStyle = 'rgb(200, 200, 200)';
      canvasCtx.fillRect(0, 0, WIDTH, HEIGHT);
      canvasCtx.lineWidth = 2;
      canvasCtx.strokeStyle = 'rgb(0, 0, 0)';
      canvasCtx.beginPath();
      var sliceWidth = WIDTH * 1.0 / bufferLength;
      var x = 0;
      for(var i = 0; i < bufferLength; i++) {
        var v = dataArray[i] / 128.0;
        var y = v * HEIGHT/2;
        if(i === 0) {
          canvasCtx.moveTo(x, y);
        } else {
          canvasCtx.lineTo(x, y);
        }
        x += sliceWidth;
      }
      canvasCtx.lineTo(canvas.width, canvas.height/6);
      canvasCtx.stroke();
    };
    draw();
  }

  function pause() {
    recording = false;
    context.suspend()
  }

  function resume() {
    recording = true;
    context.resume();
  }

  voice_timer = setInterval(function () {
    fake_value = TIMER_VALUE - 3;
    if (TIMER_VALUE >= 1) {
      if (TIMER_VALUE == 1) {
        console.log("TIMER_VALUE == 0");
        clearInterval(voice_timer);
        stop();
        form = document.querySelector(".customize_form");
        smile_supported = form.querySelector('.smile_supported');
        smile_supported.innerHTML = "";
        smile_supported.style.display = "none";
        smile_supported.setAttribute("contenteditable", "true");
        form.querySelector('#my_audio').style.display = "block";
        form.querySelector('.delete_voice_btn').style.display = "block";
        form.querySelector('.mic_visual_canvas').style.display = "none";
        form.querySelector('.voice_stop_btn').style.display = "none";
      };
      seconds = fake_value%60;
      minutes = fake_value/60%60;
      timer_block.setAttribute("contenteditable", "false");
      let strTimer = "<span style='color:red'>Запись!</span> Осталось: " + Math.trunc(minutes) + " мин. " + seconds + " сек." ;
      timer_block.innerHTML = strTimer;
    }
    else{ return };
    --TIMER_VALUE;
  }, 1000);

  on('#ajax', 'click', '.voice_stop_btn', function() {
    form = document.querySelector(".customize_form");
    smile_supported = form.querySelector('.smile_supported');
    smile_supported.innerHTML = "";
    smile_supported.style.display = "none";
    smile_supported.setAttribute("contenteditable", "true");
    form.querySelector('#my_audio').style.display = "block";
    form.querySelector('.delete_voice_btn').style.display = "block";
    form.querySelector('.mic_visual_canvas').style.display = "none";
    form.querySelector('.voice_stop_btn').style.display = "none";
    stop();
  });

  on('#ajax', 'click', '.delete_voice_btn', function() {
    stop();
    form = this.parentElement.parentElement;
    form.querySelector('.smile_supported').innerHTML = "";
    form.querySelector('.smile_supported').setAttribute("contenteditable", "true");
    remove_voice_console(form);
    form.querySelector('#voice_start_btn').style.display = "block";
    form.querySelector('#voice_post_btn').style.display = "none";
    form.querySelector("#my_audio").setAttribute("name", "no_voice");
  });

  on('#ajax', 'click', '#voice_post_btn', function() {
    if (!is_voise_sender_open) {
      return
    };
    stop();
    clearInterval(voice_timer);
    is_voise_sender_open = false;
    form_post = this.parentElement.parentElement.parentElement;
    form_post.querySelector('#voice_start_btn').style.display = "block";
    form_post.querySelector('#voice_post_btn').style.display = "none";

    remove_voice_console(form_post);
    if (document.body.querySelector(".chat_container")) {
      window.scrollTo({
        top: document.body.querySelector(".chat_container").scrollHeight,
        behavior: "smooth"
      })
    };
    new_post = document.createElement("div");
    new_post.classList.add("message", "new_message", "t_f", "pointer", "media", "p-1", "bg-light-secondary");

    user = document.body.querySelector(".userpic");
    link = user.getAttribute("data-pk");
    if (user.querySelector("img")) {
      src = user.querySelector("img").getAttribute("src");
      img = '<img style="border-radius:40px;width:40px;" src="' + src + '" />'
    } else {
      img = '<svg fill="currentColor" class="svg_default svg_default_30" viewBox="0 0 24 24"><path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"></path><path d="M0 0h24v24H0z" fill="none"></path></svg>'
    };

    figure = document.createElement("figure");
    figure.innerHTML = '<a href="' + link + '" class="ajax no_select">' + img + '</a>';
    media_body = document.createElement("div");
    media_body.classList.add("media-body", "t_f");
    media_body.innerHTML = '<h5 class="time-title mb-0"><a href="' + link + '" class="ajax creator_link"><span class="creator_name">' + user.getAttribute("data-name") + '</span></a><span class="favourite_icon"><svg width="18" height="18" fill="currentColor" enable-background="new 0 0 24 24" viewBox="0 0 24 24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm.5-13H11v6l5.25 3.15.75-1.23-4.5-2.67z"/></svg></span><small class="float-right small text-muted get_created t_f">Сейчас</small></h5><audio controls class="audio" src="' + URL.createObjectURL(CURRENT_BLOB) + '"></audio>';
    new_post.append(figure);
    new_post.append(media_body);

    message_load = form_post.parentElement.parentElement.parentElement.querySelector(".chatlist");
    message_load.append(new_post);
    message_load.querySelector(".items_empty") ? message_load.querySelector(".items_empty").style.display = "none" : null;
    if (document.querySelector(".chat_container")) {
      window.scrollTo({
        top: 12000,
        behavior: "smooth"
      })
    };

    message_text = form_post.querySelector(".message_text");
    message_text.classList.remove("border_red");
    message_text.setAttribute("contenteditable", "true");
    message_text.innerHTML = "";
    form_post.querySelector("#my_audio").setAttribute("name", "no_voice");

    form_post.querySelector(".hide_block_menu").classList.remove("show");
    form_post.querySelector(".message_dropdown").classList.remove("border_red");

    form_data = new FormData(form_post);
    form_data.append("voice", CURRENT_BLOB, 'fileName.wav');
    form_data.append("time", new Date().toLocaleString());

    pk = document.body.querySelector(".pk_saver").getAttribute("chat-pk");
    form_data.append("id", pk);

    link_2 = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link_2.open( 'POST', "/chat/user_progs/send_voice_message", true );
    link_2.setRequestHeader('Content-Type', 'application/json');

    link_2.onreadystatechange = function () {
    if ( this.readyState == 4 && this.status == 200 ) {
      jsonResponse = JSON.parse(link_2.responseText);
      id = jsonResponse.id;
      message = message_load.querySelector(".new_message");
      message.setAttribute("data-pk", id);
      message.classList.add("toggle_message", "is_have_edited");
      message.classList.remove("new_message");
      message.querySelector(".favourite_icon").innerHTML = "";
      CURRENT_BLOB = null;
      is_voise_sender_open = true;
    }};
    link_2.send(JSON.stringify(form_data));
  });
  start();
};

on('#ajax', 'click', '#voice_start_btn', function() {
    get_record_stream();
    console.log('Start recording');
    form = this.parentElement.parentElement;
    form.querySelector('.delete_voice_btn').style.display = "block";
    form.querySelector('.file_dropdown_2').style.display = "none";
    form.querySelector('.form_smilies').style.display = "none";
    form.parentElement.querySelector('.mic_visual_canvas').style.display = "block";
    form.querySelector('.voice_stop_btn').style.display = "block";

    form.querySelector('#voice_start_btn').style.display = "none";
    form.querySelector('#voice_post_btn').style.display = "block";
    form.querySelector("#my_audio").setAttribute("name", "voice");
  });

function get_toggle_messages() {
  list = document.body.querySelectorAll(".target_message");
  query = [];
  for (var i = 0; i < list.length; i++){
      query.push(list[i])
  };
  return query
};
function show_chat_console(message) {
  _console = document.body.querySelector(".console_btn_other");
  if (message.querySelector(".message_sticker") || message.querySelector(".audio") || !message.classList.contains("is_have_edited")) {
    _console.querySelector(".u_message_edit").style.display = "none"
  };

  favourite_btn = _console.querySelector(".toggle_messages_favourite");
  list_not_have_favourite_messages = true;
  for (var i = 0; i < get_toggle_messages().length; i++){
    if (!list[i].querySelector(".delete_favourite_message")) {
        list_not_have_favourite_messages = false;
      }
    };
  if (list_not_have_favourite_messages) {
    favourite_btn.innerHTML = '<path d="M0 0h24v24H0z" fill="none"/><path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z"/>'
    favourite_btn.parentElement.setAttribute("tooltip","Удалить из избранного");
    favourite_btn.classList.add("remove_favourite_messages");
    favourite_btn.classList.remove("create_favourite_messages")
  } else {
    favourite_btn.parentElement.setAttribute("tooltip","Отметить как важное");
    favourite_btn.innerHTML = '<path d="M12 7.13l.97 2.29.47 1.11 1.2.1 2.47.21-1.88 1.63-.91.79.27 1.18.56 2.41-2.12-1.28-1.03-.64-1.03.62-2.12 1.28.56-2.41.27-1.18-.91-.79-1.88-1.63 2.47-.21 1.2-.1.47-1.11.97-2.27M12 2L9.19 8.63 2 9.24l5.46 4.73L5.82 21 12 17.27 18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2z"/>'
    favourite_btn.classList.remove("remove_favourite_messages");
    favourite_btn.classList.add("create_favourite_messages")
  };

  _console.style.display = "unset";
  _console.previousElementSibling.style.display = "none";
  _console.previousElementSibling.style.left = "8px";
  _console.parentElement.parentElement.querySelector("h5").style.display = "none"
};

function edit_favourite_count(count, type) {
  if (document.body.querySelector(".favourite_block")) {
    block = document.body.querySelector(".favourite_block");
    try {
      _count = block.querySelector(".favourite_messages_count").innerHTML
    } catch {_count = 0};
    _count *= 1;
    if (type == "plus") {
      _count += count;
      block.parentElement.parentElement.classList.remove("hidden");
      block.querySelector(".favourite_messages_count").innerHTML = _count
    }
    else if (type == "minus") {
      _count -= count;
      block.querySelector(".favourite_messages_count").innerHTML = _count
      if (_count < 1) {
        block.parentElement.parentElement.classList.add("hidden");
      }
    }
  }
};

on('#ajax', 'click', '.u_message_unfixed', function() {
  message = this.parentElement.parentElement;
  form_data = new FormData();
  form_data.append("id", message.getAttribute("data-pk"));

  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    ajax_link.open( 'POST', "/chat/user_progs/unfixed_message", true );
		ajax_link.setRequestHeader('Content-Type', 'application/json');
    ajax_link.onreadystatechange = function () {
      if ( this.readyState == 4 && this.status == 200 ) {
        fix_span = message.parentElement.parentElement.parentElement.querySelector(".count_fixed_messages")
        fix_count = fix_span.innerHTML;
        fix_count *= 1;
        fix_count -= 1;
        fix_span.innerHTML = fix_count;
        message.remove();
      }
    }
    ajax_link.send(JSON.stringify(form_data));
});

on('#ajax', 'click', '.chat_search_btn', function() {
  value = this.parentElement.previousElementSibling;
  if (!value.value) {
    return
  }
  chat = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    ajax_link.open( 'GET', "/chat/" + chat.getAttribute("data-pk") + "/search?q=" + value.value, true );
		ajax_link.setRequestHeader('Content-Type', 'application/json');
    ajax_link.onreadystatechange = function () {
      if ( this.readyState == 4 && this.status == 200 ) {
        elem_ = document.createElement('span');
        elem_.innerHTML = ajax_link.responseText;
        chatview = chat.querySelector(".chatview");
        chatview.querySelector(".chatlist").style.display = "none";
        if (chatview.querySelector(".show_search_result")) {
          chatview.querySelector(".show_search_result").innerHTML = ""
        } else {
          span = document.createElement('span');
          span.classList.add("show_search_result");
          chatview.prepend(span);
        };
        span.innerHTML = elem_.innerHTML;
      }
    }
    ajax_link.send();
});

on('#ajax', 'click', '.delete_favourite_message', function() {
  pk = this.parentElement.parentElement.parentElement.parentElement.getAttribute("data-pk")
  form_data = new FormData();
  form_data.append("id", pk);
  
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'POST', "/chat/user_progs/unfavorite_messages", true );
	ajax_link.setRequestHeader('Content-Type', 'application/json');
  ajax_link.onreadystatechange = function () {
      if ( this.readyState == 4 && this.status == 200 ) {
        edit_favourite_count(1, "minus");
        messages = document.body.querySelectorAll( '[data-pk=' + '"' + pk + '"' + ']' );
        for (var i = 0; i < messages.length; i++){
          messages[i].querySelector(".favourite_icon").innerHTML = ""
        }
      }
  }
  ajax_link.send(JSON.stringify(form_data));
});
on('#ajax', 'click', '.create_favourite_messages', function() {
  hide_chat_console();
  messages = [];
  list = get_toggle_messages();
  for (var i = 0; i < list.length; i++){
    if (!list[i].querySelector(".delete_favourite_message")) {
        messages.push(list[i].getAttribute("data-pk"));
        list[i].querySelector(".favourite_icon").innerHTML = '<svg width="18" height="18" class="delete_favourite_message pointer" fill="currentColor" enable-background="new 0 0 24 24" viewBox="0 0 24 24"><path d="M12 7.13l.97 2.29.47 1.11 1.2.1 2.47.21-1.88 1.63-.91.79.27 1.18.56 2.41-2.12-1.28-1.03-.64-1.03.62-2.12 1.28.56-2.41.27-1.18-.91-.79-1.88-1.63 2.47-.21 1.2-.1.47-1.11.97-2.27M12 2L9.19 8.63 2 9.24l5.46 4.73L5.82 21 12 17.27 18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2z"/></svg>'
    };
    list[i].classList.remove("custom_color", "target_message")
  };

  form_data = new FormData();
  form_data.append("messages", list);
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    ajax_link.open( 'POST', "/chat/user_progs/favorite_messages", true );
		ajax_link.setRequestHeader('Content-Type', 'application/json');
    ajax_link.onreadystatechange = function () {
      if ( this.readyState == 4 && this.status == 200 ) {
        edit_favourite_count(messages.length, "plus")
      }
    }
    ajax_link.send(JSON.stringify(form_data));
});
on('#ajax', 'click', '.remove_favourite_messages', function() {
  hide_chat_console();
  messages = [];
  list = get_toggle_messages();
  for (var i = 0; i < list.length; i++){
    if (list[i].querySelector(".delete_favourite_message")) {
        messages.push(list[i].getAttribute("data-pk"));
        list[i].querySelector(".favourite_icon").innerHTML = ''
    };
    list[i].classList.remove("custom_color", "target_message")
  };
  form_data = new FormData();
  form_data.append("messages", list);

  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'POST', "/chat/user_progs/unfavorite_messages", true );
  ajax_link.setRequestHeader('Content-Type', 'application/json');
    ajax_link.onreadystatechange = function () {
      if ( this.readyState == 4 && this.status == 200 ) {
        edit_favourite_count(messages.length, "minus")
      }
    }
    ajax_link.send(JSON.stringify(form_data));
});

function hide_chat_console() {
  _console = document.body.querySelector(".console_btn_other");
  _console.querySelector(".u_message_edit").style.display = "unset";
  _console.style.display = "none";
  _console.previousElementSibling.style.display = "unset";
  _console.parentElement.parentElement.querySelector("h5").style.display = "unset"
};

on('#ajax', 'click', '.message_dropdown', function() {this.nextElementSibling.classList.toggle("show")});
on('#ajax', 'click', '.smile_sticker_dropdown', function() {
  block = this.nextElementSibling;
  if (!block.querySelector(".card")) {
    list_load(block, "/users/load/smiles_stickers")
  };
  block.classList.toggle("show");
});


on('#ajax', 'click', '.chat_search', function() {
  header = this.parentElement.parentElement.parentElement;
  input = header.nextElementSibling;
  input.style.display = "flex";
  header.style.display = "none";
  input.querySelector(".form-control").focus();
});
on('#ajax', 'click', '.hide_chat_search', function() {
  search = this.parentElement.parentElement;
  search.previousElementSibling.style.display = "flex";
  search.style.display = "none";
  if (document.body.querySelector(".show_search_result")) {
    document.body.querySelector(".show_search_result").innerHTML = "";
    document.body.querySelector(".chatlist").style.display = "block";
  }
});

on('#ajax', 'click', '.u_chat_info', function() {
  if (this.querySelector("a")) { return };
  pk = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.getAttribute("chat-pk")
  create_fullscreen("/chat/" + pk + "/info", "worker_fullscreen", false, true);
});
on('#ajax', 'click', '.favourite_messages_list', function() {
  create_fullscreen("/chat/favourites_messages", "worker_fullscreen");
});
on('#ajax', 'click', '.user_chat_settings', function() {
  pk = this.parentElement.parentElement.parentElement.parentElement.getAttribute("chat-pk")
  create_fullscreen("/chat/user_progs/edit/" + pk, "worker_fullscreen");
});
on('#ajax', 'click', '.user_chat_settings_private', function() {
  pk = this.parentElement.parentElement.parentElement.parentElement.getAttribute("chat-pk")
  create_fullscreen("/chat/user_progs/private/" + pk, "worker_fullscreen");
});
on('#ajax', 'click', '.show_attach_files', function() {
  pk = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.getAttribute("chat-pk")
  create_fullscreen("/chat/" + pk + "/collections", "item_fullscreen");
});
on('#ajax', 'click', '.select_chat_collections', function() {
  _this = this;
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/chat/" + this.parentElement.getAttribute("chat-pk") + "/collections?types=" + this.getAttribute("data-type"), true );
	ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  ajax_link.onreadystatechange = function () {
    if ( this.readyState == 4 && this.status == 200 ) {
      elem = document.createElement('span');
      elem.innerHTML = ajax_link.responseText;
      _this.parentElement.parentElement.parentElement.nextElementSibling.innerHTML = elem.querySelector(".load_block").innerHTML;
    }
  };
  ajax_link.send();
});

function create_user_input_card(name, pk, link) {
  $span = document.createElement("span");
  $span.setAttribute("data-pk", pk);
  $span.classList.add("btn","btn-sm","custom_color");
  $span.innerHTML = "<a href='" + link + "' target='_blank' >" + name + "</a><span class='remove_user_input pointer'>x<span>";
  $span.style.margin = "2px";
  $input = document.createElement("input");
  $input.classList.add("user_pk");
  $input.setAttribute("type", "hidden");
  $input.setAttribute("name", "users");
  $input.value = pk;
  $span.append($input);
  return $span
};
function create_list_input_card(name, pk, link) {
  $span = document.createElement("span");
  $span.setAttribute("data-pk", pk);
  $span.classList.add("btn","btn-sm","custom_color");
  $span.innerHTML = "<a class='pointer " + link + "' postlist-pk='" + pk + "'>" + name + "</a><span class='remove_list_input pointer'>x<span>";
  $span.style.margin = "2px";
  $input = document.createElement("input");
  $input.classList.add("list_pk");
  $input.setAttribute("type", "hidden");
  $input.setAttribute("name", "lists");
  $input.value = pk;
  $span.append($input);
  return $span
};

on('#ajax', 'click', '.remove_user_input', function() {
  parent = this.parentElement;
  header = parent.parentElement;
  parent.remove();
  container = header.parentElement;
  btn = container.querySelector(".form_btn");
  if (!header.querySelector(".remove_user_input")) {
    header.querySelector(".header_title").style.display = "block";
  };

  friend = container.querySelector('[data-pk=' + '"' + this.nextElementSibling.value + '"' + ']');
  friend.querySelector(".active_svg").classList.remove("active_svg");
  count = container.querySelectorAll(".active_svg").length;
  if (count > 1) {
    btn_text = "Выбрать пользователей" + " (" + count + ")";
    btn.disabled = false;
  } else if (count == 1) {
    btn_text = "Выбрать пользователя";
    btn.disabled = false;
  } else {
    btn_text = "Выбрать пользователей";
    btn.disabled = true;
  };
  btn.innerHTML = btn_text;
});
on('#ajax', 'click', '.remove_list_input', function() {
  parent = this.parentElement;
  header = parent.parentElement;
  parent.remove();
  container = header.parentElement;
  btn = container.querySelector(".form_btn");
  if (!header.querySelector(".remove_list_input")) {
    header.querySelector(".header_title").style.display = "block";
  };

  friend = container.querySelector('[data-pk=' + '"' + this.nextElementSibling.value + '"' + ']');
  friend.querySelector(".active_svg").classList.remove("active_svg");
  count = container.querySelectorAll(".active_svg").length;
  if (count > 1) {
    btn_text = "Выбрать списки" + " (" + count + ")";
    btn.disabled = false;
  } else if (count == 1) {
    btn_text = "Выбрать список";
    btn.disabled = false;
  } else {
    btn_text = "Выбрать списки";
    btn.disabled = true;
  };
  btn.innerHTML = btn_text;
});


function create_chat_items_input_card(name, pk, link) {
  $span = document.createElement("span");
  $span.setAttribute("data-pk", pk);
  $span.classList.add("btn","btn-sm","custom_color");
  $span.innerHTML = "<a href='" + link + "' target='_blank' >" + name + "</a><span class='remove_list_input pointer'>x<span>";
  $span.style.margin = "2px";
  $input = document.createElement("input");
  $input.classList.add("list_pk");
  $input.setAttribute("type", "hidden");
  $input.setAttribute("name", pk);
  $input.value = pk;
  $span.append($input);
  return $span
};
on('#ajax', 'click', '.add_member_chat_toggle', function() {
  container = this.parentElement.parentElement.parentElement;
  btn = container.querySelector(".form_btn");
  header = container.querySelector(".card-header");
  header_title = header.querySelector(".header_title");
  pk = this.getAttribute("data-pk");
  link = this.getAttribute("data-link");

  if (this.querySelector(".active_svg")) {
    input_svg = this.querySelector(".active_svg");
    input_svg.classList.remove("active_svg");
    input_svg.setAttribute("tooltip", "Выбрать пользователя")
    friend_input = header.querySelector('[data-pk=' + '"' + pk + '"' + ']');
    friend_input.remove();
    if (!header.querySelector(".remove_user_input")) {
      header.querySelector(".header_title").style.display = "block";
    }
  } else {
    input_svg = this.querySelector(".item_attach_circle");
    input_svg.classList.add("active_svg");
    input_svg.setAttribute("tooltip", "Отменить")
    header_title.style.display = "none";
    header.append(create_chat_items_input_card(this.querySelector("h6").innerHTML, pk, link))
  };

  count = container.querySelectorAll(".active_svg").length;
  if (count > 1) {
    btn_text = "Выбрать пользователей" + " (" + count + ")";
    btn.disabled = false;
  } else if (count == 1) {
    btn_text = "Выбрать пользователя";
    btn.disabled = false;
  } else {
    btn_text = "Выберите пользователей";
    btn.disabled = true;
  };
  btn.innerHTML = btn_text;
});

on('#ajax', 'click', '.items_lists_toggle', function() {
  container = this.parentElement.parentElement.parentElement;
  btn = container.querySelector(".form_btn");
  header = container.querySelector(".card-header");
  header_title = header.querySelector(".header_title");
  pk = this.getAttribute("data-pk");
  link = this.getAttribute("data-link");

  if (this.querySelector(".active_svg")) {
    input_svg = this.querySelector(".active_svg");
    input_svg.classList.remove("active_svg");
    input_svg.setAttribute("tooltip", "Выбрать список")
    friend_input = header.querySelector('[data-pk=' + '"' + pk + '"' + ']');
    friend_input.remove();
    if (!header.querySelector(".remove_list_input")) {
      header.querySelector(".header_title").style.display = "block";
    }
  } else {
    input_svg = this.querySelector(".item_attach_circle");
    input_svg.classList.add("active_svg");
    input_svg.setAttribute("tooltip", "Отменить")
    header_title.style.display = "none";
    header.append(create_list_input_card(this.querySelector("h6").innerHTML, pk, link))
  };

  count = container.querySelectorAll(".active_svg").length;
  if (count > 1) {
    btn_text = "Выбрать списки" + " (" + count + ")";
    btn.disabled = false;
  } else if (count == 1) {
    btn_text = "Выбрать список";
    btn.disabled = false;
  } else {
    btn_text = "Выберите списки";
    btn.disabled = true;
  };
  btn.innerHTML = btn_text;
});


on('#ajax', 'click', '.chat_items_toggle', function() {
  container = this.parentElement.parentElement.parentElement;
  btn = container.querySelector(".form_btn");
  header = container.querySelector(".card-header");
  header_title = header.querySelector(".header_title");
  pk = this.getAttribute("data-pk");
  link = this.getAttribute("data-link");

  if (this.querySelector(".active_svg")) {
    input_svg = this.querySelector(".active_svg");
    input_svg.classList.remove("active_svg");
    input_svg.setAttribute("tooltip", "Выбрать получателя")
    friend_input = header.querySelector('[data-pk=' + '"' + pk + '"' + ']');
    friend_input.remove();
    if (!header.querySelector(".remove_list_input")) {
      header.querySelector(".header_title").style.display = "block";
    }
  } else {
    input_svg = this.querySelector(".item_attach_circle");
    input_svg.classList.add("active_svg");
    input_svg.setAttribute("tooltip", "Отменить")
    header_title.style.display = "none";
    header.append(create_chat_items_input_card(this.querySelector("h6").innerHTML, pk, link))
  };

  count = container.querySelectorAll(".active_svg").length;
  if (count > 1) {
    btn_text = "Выбрать получателей" + " (" + count + ")";
    btn.disabled = false;
  } else if (count == 1) {
    btn_text = "Выбрать получателя";
    btn.disabled = false;
  } else {
    btn_text = "Выберите получателей";
    btn.disabled = true;
  };
  btn.innerHTML = btn_text;
});

on('#ajax', 'input', '.smile_supported', function() {
  _this = this;
  console.log(_this.innerHTML);
  if (_this.innerHTML == "<br>") {
    _this.innerHTML = "";
  }
  if (_this.classList.contains("chat_message_text")){
    if (document.body.querySelector(".chatlist")) {
      check_message_form_btn()
    };
    if (!_this.classList.contains("draft_created")) {
        _this.classList.add("draft_created");
        remove_class_timeout(_this);
        setTimeout(function(){
          form = _this.parentElement.parentElement;
          send_draft_message (form, "/chat/user_progs/save_draft_message");
      }, 2000)
    }
  };
});

on('#ajax', 'input', '.custom_link_input', function() {
  _this = this;
  value = _this.value.replace(/[^a-zA-Z0-9_]/g, "").trim();
  _this.value = value;
  btn = _this.parentElement.parentElement.parentElement.parentElement.querySelector("#u_edit_link_btn");
  if (value == "" || value[0] == "_") {
    btn.setAttribute("disabled", true);
    btn.innerHTML = "Изменить";
    _this.value = "";
  }
  else if (value.slice(-1) == "_") {
    btn.setAttribute("disabled", true);
  }

  else if (value.length < 5) {
    btn.setAttribute("disabled", true);
    btn.innerHTML = "Слишком короткая ссылка";
  }
  else if (value.length > 32) {
    btn.setAttribute("disabled", true);
    btn.innerHTML = "Слишком длинная ссылка";
  }
  else {
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    link.open('GET', "/progs/check_custom_link/" + value, true);
    link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    link.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
          span = document.createElement("span");
          elem = link.responseText;
          span.innerHTML = elem;
          bool = span.querySelector("#bool").innerHTML;
          string = span.querySelector("#string").innerHTML;

          btn.innerHTML = string;
          if (bool == "1") {
            btn.removeAttribute("disabled");
          }
          else {
            btn.setAttribute("disabled", true);
          }
        }
      }
      link.send();
  }
});

on('#ajax', 'click', '.show_chat_fixed_messages', function() {
  pk = this.parentElement.parentElement.parentElement.parentElement.parentElement.getAttribute('chat-pk');
  create_fullscreen("/chat/" + pk + "/fixed_messages", "item_fullscreen");
});

on('#ajax', 'click', '.classic_smile_item', function() {
  input = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.querySelector(".smile_supported");
  $img = document.createElement("img");
  $img.src = this.getAttribute("src");
  input.append($img);
  if (document.body.querySelector(".chatlist")) {
    show_message_form_send_btn();
    form = input.parentElement.parentElement;
    send_draft_message (form, "/chat/user_progs/save_draft_message");
  };
  setEndOfContenteditable(input);
});

function send_comment_sticker(form_post, value) {
  comment_form = false, reply_form = false, parent_form = false;
  $sticker = document.createElement("input");
  $sticker.setAttribute("name", "sticker");
  $sticker.setAttribute("type", "hidden");
  $sticker.classList.add("sticker");
  $sticker.value = value;
  form_post.append($sticker);
  form = new FormData(form_post);
  if (form_post.querySelector(".comment_form")){
    if (form_post.classList.contains("u_post_comment")) {url = '/posts/user_progs/add_comment'}
    else if (form_post.classList.contains("c_post_comment")) {url = '/posts/community_progs/add_comment'}
    else if (form_post.classList.contains("u_video_comment")) {url = '/video/user_progs/add_comment'}
    else if (form_post.classList.contains("c_video_comment")) {url = '/video/community_progs/add_comment'}
    else if (form_post.classList.contains("u_photo_comment")) {url = '/photos/user_progs/add_comment'}
    else if (form_post.classList.contains("c_photo_comment")) {url = '/photos/community_progs/add_comment'}
    else if (form_post.classList.contains("u_good_comment")) {url = '/goods/user_progs/add_comment'}
    else if (form_post.classList.contains("c_good_comment")) {url = '/goods/community_progs/add_comment'};
    comment_form = true
  }
  else if (form_post.querySelector(".reply_form") || form_post.querySelector(".parent_form")) {
    if (form_post.classList.contains("u_post_comment")) {url = '/posts/user_progs/reply_comment'}
    else if (form_post.classList.contains("c_post_comment")) {url = '/posts/community_progs/reply_comment'}
    else if (form_post.classList.contains("u_video_comment")) {url = '/video/user_progs/reply_comment'}
    else if (form_post.classList.contains("c_video_comment")) {url = '/video/community_progs/reply_comment'}
    else if (form_post.classList.contains("u_photo_comment")) {url = '/photos/user_progs/reply_comment'}
    else if (form_post.classList.contains("c_photo_comment")) {url = '/photos/community_progs/reply_comment'}
    else if (form_post.classList.contains("u_good_comment")) {url = '/goods/user_progs/reply_comment'}
    else if (form_post.classList.contains("c_good_comment")) {url = '/goods/community_progs/reply_comment'}
  };
  if (form_post.querySelector(".reply_form")) {
    reply_form = true
  } else {parent_form = true};

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  link_.open('POST', url, true);
  link_.setRequestHeader('Content-Type', 'application/json');
  link_.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
          form_post.querySelector(".comment_text").innerHTML = "";
          elem = link_.responseText;
          new_post = document.createElement("span");
          new_post.innerHTML = elem;
          if (comment_form) {
            block = form_post.parentElement.previousElementSibling
          } else if (reply_form) {
            block = form_post.parentElement.parentElement.querySelector(".stream_reply_comments")
          } else if (parent_form) {
            block = form_post.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement
          }

          form_post.querySelector(".comment_text").classList.remove("border_red");
          form_post.querySelector(".hide_block_menu").classList.remove("show");
          form_post.querySelector(".dropdown").classList.remove("border_red");
          form_post.querySelector(".sticker").remove();
          block.append(new_post);

      }
  };
  link_.send(JSON.stringify(form))
};

on('#ajax', 'click', '.classic_sticker_item', function() {
  if (document.body.querySelector(".chatlist")){
    url = "/chat/user_progs/send_message";
    send_message_sticker(url, this.getAttribute("data-pk"), document.body.querySelector(".pk_saver").getAttribute("chat-pk"))
  } else if (document.body.querySelector("#send_page_message_btn")){
    url = '/chat/user_progs/send_page_message'
    send_message_sticker(url, this.getAttribute("data-pk"), document.body.querySelector("#send_page_message_btn").getAttribute("data-pk"))
  } else if (this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.querySelector(".check_mesage_form")){
    form = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
    send_comment_sticker(form, this.getAttribute("data-pk"))
  }
});

function send_message_sticker(url, id, sticker) {
  is_chat = false; is_page = false;
  console.log(sticker);
  if (document.body.querySelector(".chatlist")){is_chat = true} else {is_page = true};
  if (is_chat) {
    form_post = document.body.querySelector(".customize_form")
  } else {
    form_post = document.body.querySelector(".page_message_form")
  }
  $sticker = document.createElement("input");
  $sticker.setAttribute("name", "sticker");
  $sticker.setAttribute("type", "text");
  $sticker.classList.add("sticker");
  $sticker.value = sticker;
  form_post.append($sticker);
  form_post.append("id", id);
  form_data = new FormData(form_post);
  if (document.body.querySelector(".chatlist")){is_chat = true} else {is_page = true};

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', url, true );
  link_.setRequestHeader('Content-Type', 'application/json');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    if (is_chat) {
      elem = link_.responseText;
      message_load = form_post.parentElement.parentElement.querySelector(".chatlist");
      new_post = document.createElement("span");
      new_post.innerHTML = elem;
      message_load.append(new_post);
      message_load.querySelector(".items_empty") ? message_load.querySelector(".items_empty").style.display = "none" : null;
      form_post.querySelector(".message_text").classList.remove("border_red");
      form_post.querySelector(".hide_block_menu").classList.remove("show");
      form_post.querySelector(".message_dropdown").classList.remove("border_red");
      form_post.querySelector(".sticker").remove();
      if (document.querySelector(".chat_container")) {
        window.scrollTo({
          top: message_load.scrollHeight,
          behavior: "smooth"
        })
      };
    } else {
      document.querySelector(".item_fullscreen").style.display = "none";
      document.getElementById("item_loader").innerHTML="";
    }
  }};
  link_.send(JSON.stringify(form_data));
};

on('#ajax', 'click', '.user_create_chat', function() {
  create_fullscreen("/chats/create_chat/" + this.getAttribute("data-pk"), "worker_fullscreen", false, true);
});
on('#ajax', 'click', '.user_send_page_message', function() {
  create_fullscreen("/chats/create_message/" + this.getAttribute("data-pk"), "worker_fullscreen", false, true);
});

on('#ajax', 'click', '.u_chat_photo', function() {
  photo_pk = this.getAttribute('photo-pk');
  pk = document.body.querySelector(".pk_saver").getAttribute('chat-pk')
  create_fullscreen("/photos/user/chat_photo/" + pk + "/" + photo_pk, "photo_fullscreen");
});
on('#ajax', 'click', '.c_chat_photo', function() {
  photo_pk = this.getAttribute('photo-pk');
  pk = document.body.querySelector(".pk_saver").getAttribute('chat-pk')
  create_fullscreen("/photos/community/chat_photo/" + pk + "/" + photo_pk, "photo_fullscreen");
});

on('#ajax', 'click', '.user_add_members', function() {
  block = this.nextElementSibling.querySelector("#chat_members");
  if (!block.querySelector(".load_pag")){
  block.classList.add("mt-4");
  list_load(block, "/users/load/friends")
} else { block.style.display = "block"}
});

on('#ajax', 'click', '#add_chat_btn', function() {
  form = this.parentElement.parentElement.parentElement;
  this.disabled = true;
  pk = this.getAttribute("data-pk");
  form_data = new FormData(form);
  form_data.append("id", pk);

    var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      ajax_link.open( 'POST', '/chat/user_progs/create_chat', true );
      ajax_link.setRequestHeader('Content-Type', 'application/json');
      ajax_link.onreadystatechange = function () {
        if ( this.readyState == 4 && this.status == 200 ) {
            elem_ = document.createElement('span');
            elem_.innerHTML = ajax_link.responseText;
            ajax = elem_.querySelector("#reload_block");
            rtr = document.getElementById('ajax');
            rtr.innerHTML = ajax.innerHTML;
            pk = rtr.querySelector(".pk_saver").getAttribute("data-pk");
            document.title = elem_.querySelector('title').innerHTML;
            window.history.pushState(null, "vfgffgfgf", "/chat/" + pk);
            get_document_opacity_1();
        }
      }
      ajax_link.send(JSON.stringify(form_data));
});

on('#ajax', 'click', '#send_page_message_btn', function() {
  form = this.parentElement.parentElement.parentElement;
  _text = form.querySelector(".page_message_text").innerHTML;
  if (_text.replace(/<[^>]*(>|$)|&nbsp;|&zwnj;|&raquo;|&laquo;|&gt;/g,'').trim() == "" && form.querySelector(".files_0")){
    toast_error("Напишите или прикрепите что-нибудь");
    form.querySelector(".page_message_text").classList.add("border_red");
    return
  };

  this.disabled = true;
  form.querySelector(".type_hidden").value = form.querySelector(".page_message_text").innerHTML;
  form_data = new FormData(form);
  form_data.append("id", this.getAttribute("data-pk"));

    var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      ajax_link.open( 'POST', '/chat/user_progs/send_page_message', true );
      ajax_link.setRequestHeader('Content-Type', 'application/json');
      ajax_link.onreadystatechange = function () {
        if ( this.readyState == 4 && this.status == 200 ) {
            toast_success("Сообщение отправлено");
            close_work_fullscreen();
        } else {this.disabled = false}
      }
      ajax_link.send(JSON.stringify(form_data));
});

function send_message (form_post, url) {
  text_val = form_post.querySelector(".smile_supported");
  _val = format_text(text_val);
  _text = _val.innerHTML;

  if (_text.replace(/<(?!br)(?!img)\/?[a-z][^>]*(>|$)/gi, "").trim() == "" && form_post.querySelector(".files_0") && !form_post.querySelector(".transfer")){
    toast_error("Напишите или прикрепите что-нибудь");
    form_post.querySelector(".message_text").classList.add("border_red");
    form_post.querySelector(".message_dropdown").classList.add("border_red");
    return
  };

  $content_input = document.createElement("input");
  $content_input.setAttribute("name", "content");
  $content_input.setAttribute("type", "hidden");
  $content_input.classList.add("input_content");
  $content_input.value = _text;
  form_post.append($content_input);

  _attach_value = "";
  attach_list = form_post.querySelectorAll(".attach");
  for (var i = 0; i < attach_list.length; i++) {
    _attach_value += attach_list[i].value + ","
  };

  $attach_input = document.createElement("input");
  $attach_input.setAttribute("name", "attach");
  $attach_input.setAttribute("type", "hidden");
  $attach_input.classList.add("input_attach");
  $attach_input.value = _attach_value.slice(0,-1);
  form_post.append($attach_input);

  pk = document.body.querySelector(".pk_saver").getAttribute("chat-pk");
  form_data = new FormData(form_post);
  form_post.append("id", id);

  message_load = form_post.parentElement.parentElement.parentElement.querySelector(".chatlist");

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', url, true );
  link_.setRequestHeader('Content-Type', 'application/json');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    clear_message_attach_block();

    elem = link_.responseText;
    new_post = document.createElement("span");
    new_post.innerHTML = elem;
    message_load.insertAdjacentHTML('beforeend', new_post.innerHTML);
    message_load.querySelector(".items_empty") ? message_load.querySelector(".items_empty").style.display = "none" : null;
    form_post.querySelector(".message_text").classList.remove("border_red");
    form_post.querySelector(".hide_block_menu").classList.remove("show");
    form_post.querySelector(".message_text").innerHTML = ""
    form_post.querySelector(".message_dropdown").classList.remove("border_red");
    try{form_post.querySelector(".parent_message_block").remove()}catch{null};
    form_post.querySelector(".input_content").remove();
    form_post.querySelector(".type_hidden").remove();
    show_message_form_voice_btn();
    if (document.querySelector(".chat_container")) {
      window.scrollTo({
        top: message_load.scrollHeight,
        behavior: "smooth"
      })
    };
  }  else {
        new_post = document.createElement("span");
        new_post.innerHTML = link_.responseText;
        if (new_post.querySelector(".exception_value")){
          text = new_post.querySelector(".exception_value").innerHTML;
          toast_info(text)
        }
    }};
  link_.send(JSON.stringify(form_data));
};

on('#ajax', 'click', '.u_message_fixed', function() {
  message = document.body.querySelector(".target_message");
  hide_chat_console();
  pk = message.getAttribute("data-pk");

  form_data = new FormData();
  form_post.append("id", id);

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/chat/user_progs/fixed_message", true );
  link.setRequestHeader('Content-Type', 'application/json');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    hide_chat_console();
    if (message.querySelector(".attach_container")) {
      parent = "Вложения"
    } else if (message.querySelector(".text") != null) {
      parent = message.querySelector(".text").innerHTML.replace(/<br>/g,"  ")
    } else if(message.querySelector(".message_sticker")) {
        parent = "Наклейка"
    };
    creator_p = '<p><svg style="width: 17px;vertical-align: bottom;" fill="currentColor" viewBox="0 0 24 24"><g><rect fill="none" height="16" width="16"/></g><g><path d="M16,9V4l1,0c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H7C6.45,2,6,2.45,6,3v0 c0,0.55,0.45,1,1,1l1,0v5c0,1.66-1.34,3-3,3h0v2h5.97v7l1,1l1-1v-7H19v-2h0C17.34,12,16,10.66,16,9z" fill-rule="evenodd"/></g></svg>' + message.querySelector(".creator_name").innerHTML + '</p>';
    message.classList.remove("target_message", "custom_color");
    block = document.body.querySelector(".fixed_messages");
    block.innerHTML = "<div class='pointer show_chat_fixed_messages'>" + creator_p + "<div class='border-bottom' style='position:relative;padding-bottom: 5px;'><div style='overflow: hidden;text-overflow:ellipsis;padding-right:5px;'><span style='white-space: nowrap;'>" + parent + "</span></div></div></div>";

    message_load = document.body.querySelector(".chatlist");
    elem = link.responseText;
    new_post = document.createElement("span");
    new_post.innerHTML = elem;
    message_load.append(new_post);
    objDiv = document.body.querySelector("#chatcontent");
    objDiv.scrollTop = objDiv.scrollHeight;
  }};
  link.send(JSON.stringify(form_data));
});

on('#ajax', 'click', '.u_message_reply', function() {
  message = document.body.querySelector(".target_message");
  hide_chat_console();
  message.classList.remove("target_message", "custom_color");
  if (message.querySelector(".attach_container")) {
    parent = "Вложения"
  } else if (message.querySelector(".text") != null) {
    parent = message.querySelector(".text").innerHTML.replace(/<br>/g,"  ")
  } else if(message.querySelector(".message_sticker")) {
      parent = "Наклейка"
  };
  creator_p = '<p><a class="underline" target="_blank" href="' + message.querySelector(".creator_link").getAttribute("href") + '">' + message.querySelector(".creator_name").innerHTML + '</a></p>'

  block = document.body.querySelector(".parent_message_block");
  block.innerHTML = "<div>" + creator_p + "<div style='position:relative;padding-bottom:7px'><input type='hidden' name='parent' value='" + message.getAttribute("data-pk") + "'><div style='overflow: hidden;text-overflow:ellipsis;padding-right:5px;'><span style='white-space: nowrap;'>" + parent + "</span><span class='remove_parent_block message_form_parent_block pointer'>x</span></div></div></div>"
  setTimeout(function(){
    form = block.parentElement;
      send_draft_message (form, "/chat/user_progs/save_draft_message");
}, 1000)
});

on('#ajax', 'click', '.u_message_edit', function() {
  hide_chat_console();
  message = document.body.querySelector(".target_message");
  message.classList.remove("target_message", "custom_color");
  message.style.display = "none";

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'GET', "/chat/user_progs/edit_message/" + message.getAttribute("data-pk"), true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    response = document.createElement("span");
    response.innerHTML = link_.responseText;
    box = message.nextElementSibling;
    box.innerHTML = response.innerHTML;
    objDiv = document.body.querySelector(".chatlist");
    objDiv.scrollTop = objDiv.scrollHeight;
    }
  };
  link_.send();
});

function send_draft_message (form_post, url) {
  _text = form_post.querySelector(".message_text").innerHTML;
  text_val = form_post.querySelector(".smile_supported");
  _val = format_text(text_val);
  _text = _val.innerHTML;

  if (!_text == "" && _text.replace(/<(?!img)\/?[a-z][^>]*(>|$)/gi, "").trim() == "") {
    console.log("Не не!");
    return
  }

  text = form_post.querySelector(".type_hidden");
  text.value = form_post.querySelector(".message_text").innerHTML.replace("data:image", '');
  setEndOfContenteditable(text_val);
  
  pk = document.body.querySelector(".pk_saver").getAttribute("chat-pk");
  form_data = new FormData(form_post);
  form_data.append("id", pk);

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', url, true );
  link_.setRequestHeader('Content-Type', 'application/json');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
  }};
  link_.send(JSON.stringify(form_data));
};

on('#ajax', 'click', '#message_post_btn', function() {
  form_post = this.parentElement.parentElement.parentElement;

  send_message (form_post, "/chat/user_progs/send_message")
});

on('#ajax', 'keydown', '.message_text', function(e) {
  if (e.shiftKey && e.keyCode === 13) {this.append("\n");}
  else if (e.keyCode == 13) {
    e.preventDefault();
  form_post = this.parentElement.parentElement;

  send_message (form_post, "/chat/user_progs/send_message")
}});
on('#ajax', 'keydown', '.page_message_text', function(e) {
  if (e.shiftKey && e.keyCode === 13) {this.append("\n");}
  else if (e.keyCode == 13) {
    this.append("\n");
}});

on('#ajax', 'click', '.chat_ajax', function(e) {
  _this = this;
  e.preventDefault();
	var url = this.getAttribute('href');
  var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    ajax_link.open( 'GET', url + "?ajax=2", true ); 
		ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    ajax_link.onreadystatechange = function () {
      if ( this.readyState == 4 && this.status == 200 ) {
        elem_ = document.createElement('span');
        elem_.innerHTML = ajax_link.responseText;
        ajax = elem_.querySelector("#reload_block");

        rtr = document.getElementById('ajax');
        rtr.innerHTML = ajax.innerHTML;

        _meta = rtr.querySelector(".main-container");
        _title = _meta.getAttribute("data-title");
        _uri = "http://трезвый.рус" + _meta.getAttribute("data-uri");
        _description = _meta.getAttribute("data-description");
        _image = "http://трезвый.рус" + _meta.getAttribute("data-image");
        document.title = _title;
        document.querySelector('meta[name="url"]').setAttribute("content", _uri);
        document.querySelector('meta[name="title"]').setAttribute("content", _title);
        document.querySelector('meta[name="description"]').setAttribute("content", _description);
        document.querySelector('meta[name="image"]').setAttribute("content", _image);

        width = rtr.querySelector(".main_chat_block").offsetWidth - 14;
        rtr.querySelector(".fixed_header_chat").style.width = width + "px";
        window.scrollTo( 0, 3000 );
        scrolled(rtr.querySelector('.is_paginate'));
        window.history.pushState(null, "vfgffgfgf", url);

        chats = document.body.querySelector(".new_unread_chats");
        if (chats.querySelector(".tab_badge_left_menu") && _this.querySelector(".tab_badge")) {
          tab_badge = chats.querySelector(".tab_badge_left_menu");
          all_count = tab_badge.innerHTML.replace(/\s+/g, '');
          all_count = all_count * 1;
          result = all_count - 1;
          result > 0 ? tab_badge.innerHTML = result : chats.querySelector(".tab_badge_left_menu").remove();
        };
        if (document.body.querySelector(".left_panel_menu")) {
          setEndOfContenteditable(document.body.querySelector(".message_text"));
        };
      }
      }
    ajax_link.send();
});

on('#ajax', 'click', '.toggle_message', function(e) {
  if (e.target.classList.contains("t_f")) {
  message = this, is_toggle = false, btn_console = document.body.querySelector(".console_btn_other");

  if (message.classList.contains("custom_color")) {
    message.classList.remove("custom_color", "target_message");
    for (var i = 0; i < list.length; i++){
      if (list[i].classList.contains("custom_color")) {
        is_toggle = true
      }
    };
    is_toggle ? show_chat_console(message) : hide_chat_console();

  } else {
    // сообщение не выбрано
    message.classList.add("custom_color", "target_message");
    show_chat_console(message)
  };

  if (get_toggle_messages().length > 1) {
    btn_console.querySelector(".one_message").style.display = "none"
  } else {
    btn_console.querySelector(".one_message").style.display = "unset"
  }
}});

on('#ajax', 'click', '.u_message_delete', function() {
  list = get_toggle_messages();
  for (var i = 0; i < list.length; i++){
    list[i].classList.remove("custom_color", "target_message");
    remove_item_and_show_restore_block(list[i], "/chat/user_progs/delete_message", "u_message_restore", "Сообщение удалено")
  };
  hide_chat_console()
});

on('#ajax', 'click', '.remove_parent_block', function() {
  form = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  setTimeout(function(){
    send_draft_message (form, "/chat/user_progs/save_draft_message");
}, 1000)
  this.parentElement.parentElement.parentElement.remove()
});

on('#ajax', 'click', '.u_message_restore', function() {
  item = this.parentElement.nextElementSibling;
  pk = this.getAttribute("data-pk");
  form_data = new FormData();
  form_data.append("id", pk);

  block = this.parentElement;
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/chat/user_progs/restore_message", true );
  link.setRequestHeader('Content-Type', 'application/json');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    block.remove();
    item.style.display = "flex";
    item.classList.remove("custom_color")
  }};
  link.send(JSON.stringify(form_data));
});

on('#ajax', 'click', '.edit_message_form_remove', function() {
  box = this.parentElement.parentElement;
  box.innerHTML = "";
  box.previousElementSibling.style.display = "flex"
});

on('#ajax', 'change', '#u_photo_message_attach', function() {
  if (this.files.length > 10) {
      toast_error("Не больше 10 фотографий");return
  }
  form_data = new FormData(this.parentElement);
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/chat/user_progs/add_attach_photo", true );
  link_.setRequestHeader('Content-Type', 'application/json');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
      elem = link_.responseText;
      response = document.createElement("span");
      response.innerHTML = elem;
      photo_list = response.querySelectorAll(".pag");
      photo_message_upload_attach(photo_list, document.body.querySelector(".message_attach_block"));
    };
    close_work_fullscreen();
    show_message_form_send_btn();
  }
  link_.send(JSON.stringify(form_data));
});

on('#ajax', 'click', '.edit_message_post_btn', function() {
  form_post = this.parentElement.parentElement.parentElement;
  _text = form_post.querySelector(".message_text").innerHTML;
  if (_text.replace(/<(?!br)(?!img)\/?[a-z][^>]*(>|$)/gi, "").trim() == "" && !form_post.querySelector(".special_block").innerHTML){
    toast_error("Напишите или прикрепите что-нибудь");
    form_post.querySelector(".message_text").classList.add("border_red");
    form_post.querySelector(".message_dropdown").classList.add("border_red");
    return
  };

  $content_input = document.createElement("input");
  $content_input.setAttribute("name", "content");
  $content_input.setAttribute("type", "hidden");
  $content_input.classList.add("input_content");
  $content_input.value = _text;
  form_post.append($content_input);

  _attach_value = "";
  attach_list = form_post.querySelectorAll(".attach");
  for (var i = 0; i < attach_list.length; i++) {
    _attach_value += attach_list[i].value + ","
  };

  $attach_input = document.createElement("input");
  $attach_input.setAttribute("name", "attach");
  $attach_input.setAttribute("type", "hidden");
  $attach_input.classList.add("input_attach");
  $attach_input.value = _attach_value.slice(0,-1);
  form_post.append($attach_input);

  message = form_post.parentElement.previousElementSibling;
  form_data = new FormData(form_post);
  form_data.append("id", message.getAttribute("data-pk"));

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/chat/user_progs/edit_message", true );
  link_.setRequestHeader('Content-Type', 'application/json');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    elem = link_.responseText;
    new_post = document.createElement("span");
    new_post.innerHTML = elem;
    message.innerHTML = new_post.innerHTML;
    form_post.parentElement.innerHTML = "";
    message.style.display = "flex"
  }};

  link_.send(JSON.stringify(form_data));
});


on('#ajax', 'click', '.u_message_transfer', function() {
  create_fullscreen('/users/load/chats', "item_fullscreen");
  hide_chat_console();
});

on('#ajax', 'click', '.go_transfer_messages', function() {
  url = "/chat/" + this.getAttribute("data-pk");
  list = get_toggle_messages();
  get_document_opacity_1();
  saver = document.createElement("div");
  for (var i = 0; i < list.length; i++) {
    $input = document.createElement("input");
    $input.setAttribute("type", "hidden");
    $input.setAttribute("name", "transfer");
    $input.setAttribute("value", list[i].getAttribute("data-pk"));
    $input.classList.add("transfer");
    saver.append($input)
  };

  if (list.length > 1) {
    count = list.length
    a = count % 10, b = count % 100;
    if (a == 1 && b != 11){
      preview = "<span class='pointer underline'>" + count + " сообщение</span>"
    }
    else if (a >= 2 && a <= 4 && (b < 10 || b >= 20)) {
      preview = "<span class='pointer underline'>" + count + " сообщения</span>"
    }
    else {
      preview = "<span class='pointer underline'>" + count + " сообщений</span>"
    };
    creator_p = '<p>Пересланные сообщения</p>'
  } else {
    message = document.body.querySelector(".target_message");
    if (message.querySelector(".attach_container")) {
      preview = "Вложения"
    } else if (message.querySelector(".text") != null) {
      text = message.querySelector(".text").innerHTML;
      preview = text.replace(/[<]br[^>]*[>]/gi, " ");
    } else if(message.querySelector(".message_sticker")) {
        preview = "Наклейка"
    };
    creator_p = '<p><a class="underline" target="_blank" href="' + message.querySelector(".creator_link").getAttribute("href") + '">' + message.querySelector(".creator_name").innerHTML + '</a></p>'
  };
  if (url == window.location.href) {
    block = rtr.querySelector(".parent_message_block");
    block.innerHTML = "<div>" + creator_p + "<div style='position:relative;padding-bottom:7px'><div>" + preview + "<span class='remove_parent_block message_form_parent_block pointer'>x</span></div></div></div>";
    block.append(saver);
  } else {
  var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', url, true );
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  ajax_link.onreadystatechange = function () {
    if ( this.readyState == 4 && this.status == 200 ) {
      elem_ = document.createElement('span');
      elem_.innerHTML = ajax_link.responseText;
      ajax = elem_.querySelector("#reload_block");
      rtr = document.getElementById('ajax');
      rtr.innerHTML = ajax.innerHTML;
      objDiv = document.querySelector(".chat_container");
      objDiv.scrollTop = objDiv.scrollHeight;
      window.history.pushState(null, "vfgffgfgf", url);
      scrolled(rtr.querySelector('.chat_container'));
      block = rtr.querySelector(".parent_message_block");
      block.innerHTML = "<div>" + creator_p + "<div style='position:relative;padding-bottom:7px'><div style='overflow: hidden;text-overflow:ellipsis;padding-right:5px;'><span style='white-space: nowrap;'>" + preview + "</span><span class='remove_parent_block pointer message_form_parent_block'>x</span></div></div></div>";
      block.append(saver);
      show_message_form_send_btn();
    }
  }
  ajax_link.send();
};
setTimeout(function(){
  form = document.body.querySelector(".customize_form");
    send_draft_message (form, "/chat/user_progs/save_draft_message");
}, 1000)
});

on('#ajax', 'click', '.on_full_chat_notify', function() {
  document.body.querySelector(".notify_box").innerHTML= ''
  chat_send_change(this, "/chat/user_progs/beep_on", "off_full_chat_notify", "Откл. уведомления");
});
on('#ajax', 'click', '.off_full_chat_notify', function() {
  document.body.querySelector(".notify_box").innerHTML= ' <svg style="width: 14px;" enable-background="new 0 0 24 24" height="14px" viewBox="0 0 24 24" width="17px" fill="currentColor"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M4.34 2.93L2.93 4.34 7.29 8.7 7 9H3v6h4l5 5v-6.59l4.18 4.18c-.65.49-1.38.88-2.18 1.11v2.06c1.34-.3 2.57-.92 3.61-1.75l2.05 2.05 1.41-1.41L4.34 2.93zM10 15.17L7.83 13H5v-2h2.83l.88-.88L10 11.41v3.76zM19 12c0 .82-.15 1.61-.41 2.34l1.53 1.53c.56-1.17.88-2.48.88-3.87 0-4.28-2.99-7.86-7-8.77v2.06c2.89.86 5 3.54 5 6.71zm-7-8l-1.88 1.88L12 7.76zm4.5 8c0-1.77-1.02-3.29-2.5-4.03v1.79l2.48 2.48c.01-.08.02-.16.02-.24z"/></svg>'
  chat_send_change(this, "/chat/user_progs/beep_off", "on_full_chat_notify", "Вкл. уведомления");
});

on('#ajax', 'click', '.remove_user_from_chat', function() {
  item = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  user_pk = item.getAttribute("data-pk");
  form_data = new FormData();
  form_data.append("user_id", user_pk);
  form_data.append("chat_id", item.parentElement.parentElement.parentElement.getAttribute("chat-pk"));

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/chat/user_progs/remove_member", true );
  link.setRequestHeader('Content-Type', 'application/json');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    item.remove()
  }};
  link.send(JSON.stringify(form_data));
});
on('#ajax', 'click', '.user_exit_in_user_chat', function() {
  if (this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.getAttribute("chat-pk")){
    pk = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.getAttribute("chat-pk");
  } else { pk = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.getAttribute("chat-pk")}
  
  form_data = new FormData();
  form_data.append("id", pk);

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/chat/user_progs/exit_user_from_user_chat", true );
  link.setRequestHeader('Content-Type', 'application/json');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    ajax_get_reload("/chat");
  }};
  link.send(JSON.stringify(form_data));
});

on('#ajax', 'click', '.u_clean_chat_messages', function() {
  form_data = new FormData();
  form_data.append("id", this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.getAttribute("chat-pk"));

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/chat/user_progs/clean_messages", true );
  link.setRequestHeader('Content-Type', 'application/json');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    ajax_get_reload("/chat");
  }};
  link.send(JSON.stringify(form_data));
});
on('#ajax', 'click', '.close_support_chat', function() {
  form_data = new FormData();
  form_data.append("id", this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.getAttribute("chat-pk"));

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/chat/user_progs/delete_support_chat", true );
  link.setRequestHeader('Content-Type', 'application/json');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    ajax_get_reload("/chat");
  }};
  link.send(JSON.stringify(form_data));
});
on('#ajax', 'click', '.refresh_support_chat', function() {
  form_data = new FormData();
  form_data.append("id", this.getAttribute("chat-pk"));

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/chat/user_progs/refresh_support_chat", true );
  link.setRequestHeader('Content-Type', 'application/json');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    ajax_get_reload("/chat/" + this.getAttribute("chat-pk"));
  }};
  link.send(JSON.stringify(form_data));
});

on('body', 'click', '.add_perm_user_chat', function() {
  _this = this;
  item = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  user_pk = item.getAttribute("data-pk");
  form_data = new FormData();
  form_data.append("user_id", user_pk);
  form_data.append("chat_id", item.parentElement.parentElement.parentElement.getAttribute("chat-pk"));

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/chat/user_progs/add_admin", true );
  link.setRequestHeader('Content-Type', 'application/json');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    _this.classList.remove("add_perm_user_chat");
    _this.classList.add("remove_perm_user_chat");
    _this.innerHTML = "Расжаловать";
    item.querySelector('.member_role').innerHTML = "Администратор"
  }};
  link.send(JSON.stringify(form_data));
});
on('body', 'click', '.remove_perm_user_chat', function() {
  _this = this;
  item = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  user_pk = item.getAttribute("data-pk");
  form_data = new FormData();
  form_data.append("user_id", user_pk);
  form_data.append("chat_id", item.parentElement.parentElement.parentElement.getAttribute("chat-pk"));

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/chat/user_progs/remove_admin", true );
  link.setRequestHeader('Content-Type', 'application/json');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    _this.classList.remove("remove_perm_user_chat");
    _this.classList.add("add_perm_user_chat");
    _this.innerHTML = "Сделать админом";
    item.querySelector('.member_role').innerHTML = "Участник"
  }};
  link.send(JSON.stringify(form_data));
});

on('#ajax', 'click', '#u_chat_settings_btn', function() {
  form = this.parentElement.parentElement.parentElement;
  pk = form.getAttribute("data-pk");
  form_data = new FormData(form);
  form_data.append("id", pk);
  this.disabled = true;

    var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      ajax_link.open( 'POST', '/chat/user_progs/edit', true );
      ajax_link.setRequestHeader('Content-Type', 'application/json');
      ajax_link.onreadystatechange = function () {
        if ( this.readyState == 4 && this.status == 200 ) {
            elem_ = document.createElement('span');
            elem_.innerHTML = ajax_link.responseText;
            form.classList.remove("cool_private_form");
            close_work_fullscreen();
        }
      };
      ajax_link.send(JSON.stringify(form_data));
});

on('#ajax', 'click', '#add_chat_exclude_users_btn', function() {
  form = this.parentElement.parentElement;
  post_include_exclude_users(form, '/chat/user_progs/load_exclude_users')
});
on('#ajax', 'click', '#add_chat_include_users_btn', function() {
  form = this.parentElement.parentElement;
  post_include_exclude_users(form, '/chat/user_progs/load_include_users')
});


on('#ajax', 'click', '.u_add_members_in_chat', function() {
  if (this.getAttribute("chat-pk")) {
    pk = this.getAttribute("chat-pk")
  } else if (this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.getAttribute("chat-pk")){
    pk = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.getAttribute("chat-pk")
  } else {pk=null};
  create_fullscreen("/chat/user_progs/invite_members?chat_pk=" + pk, "worker_fullscreen");
});
on('#ajax', 'click', '#append_friends_to_chat_btn', function() {
  form = this.parentElement.parentElement, is_chat = false;
  this.disabled = true;
  if (form.parentElement.getAttribute("chat-pk")) {
    pk = form.parentElement.getAttribute("chat-pk");
    is_chat = true
  } else { pk=null};

  if (is_chat) {
    form_data = new FormData(form);
    form_data.append("id", pk);

    var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      ajax_link.open( 'POST', "/chat/user_progs/invite_members", true );
      ajax_link.setRequestHeader('Content-Type', 'application/json');
      ajax_link.onreadystatechange = function () {
        if ( this.readyState == 4 && this.status == 200 ) {
            elem_ = document.createElement('span');
            elem_.innerHTML = ajax_link.responseText;
            message_load = document.body.querySelector(".chatlist");
            message_load.append(elem_);
            objDiv = document.querySelector(".chatview");
            objDiv.scrollTop = objDiv.scrollHeight;
            close_work_fullscreen();
            message_load.querySelector(".items_empty") ? message_load.querySelector(".items_empty").style.display = "none" : null;
        }
      };
      ajax_link.send(JSON.stringify(form_data));
      
    } else {
      users_block = form.querySelector(".card-header");
      users_list = users_block.querySelectorAll(".custom_color");
      collector = document.body.querySelector(".collector");
      final_list = "Выбраны друзья: ";
      for (var i = 0; i < users_list.length; i++){
        a = users_list[i].querySelector("a");
        final_list += '<a href="' + a.getAttribute("href") + '" target="_blank">' + a.innerHTML + '</a>'
        final_list += '<input type="hidden" name="users" value="' + users_list[i].getAttribute("data-pk") + '" />'
      };
      collector.innerHTML = final_list;
      close_work_fullscreen();
    }
});


function create_community_input_card(name, pk, link) {
  $span = document.createElement("span");
  $span.setAttribute("data-pk", pk);
  $span.classList.add("btn","btn-sm","custom_color");
  $span.innerHTML = "<a target='_blank' href='" + link + "'>" + name + "</a><span class='remove_community_input pointer'>x<span>";
  $span.style.margin = "2px";
  $input = document.createElement("input");
  $input.classList.add("list_pk");
  $input.setAttribute("type", "hidden");
  $input.setAttribute("name", "u_c");
  $input.value = "c" + pk;
  $span.append($input);
  return $span
};
on('#ajax', 'click', '.communities_toggle', function() {
  container = this.parentElement.parentElement.parentElement;
  btn = container.querySelector(".form_btn");
  header = container.querySelector(".card-header");
  header_title = header.querySelector(".header_title");
  pk = this.getAttribute("data-pk");
  link = this.getAttribute("data-link");

  if (this.querySelector(".active_svg")) {
    input_svg = this.querySelector(".active_svg");
    input_svg.classList.remove("active_svg");
    input_svg.setAttribute("tooltip", "Выбрать сообщество")
    friend_input = header.querySelector('[data-pk=' + '"' + pk + '"' + ']');
    friend_input.remove();
    if (!header.querySelector(".remove_community_input")) {
      header.querySelector(".header_title").style.display = "block";
    }
  } else {
    input_svg = this.querySelector(".item_attach_circle");
    input_svg.classList.add("active_svg");
    input_svg.setAttribute("tooltip", "Отменить")
    header_title.style.display = "none";
    header.append(create_community_input_card(this.querySelector("h6").innerHTML, pk, link))
  };

  count = container.querySelectorAll(".active_svg").length;
  if (count > 1) {
    btn_text = "Выбрать сообщества" + " (" + count + ")";
    btn.disabled = false;
  } else if (count == 1) {
    btn_text = "Выбрать сообщество";
    btn.disabled = false;
  } else {
    btn_text = "Выберите сообщества";
    btn.disabled = true;
  };
  btn.innerHTML = btn_text;
});


on('#ajax', 'click', '.like_support_manager', function() {
  _this = this;

  form_data = new FormData();
  form_data.append("id", this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.getAttribute("chat-pk"));
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/chat/user_progs/like_manager", true );
  link.setRequestHeader('Content-Type', 'application/json');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    if (_this.classList.contains("btn_success")) {
      _this.classList.remove("btn_success");
      _this.classList.add("btn_default");
    } else {
      _this.classList.add("btn_success");
      _this.classList.remove("btn_default");
    };
    next = _this.nextElementSibling.classList;
    next.remove("btn_danger");
    next.add("btn_default");
  }};
  link.send(JSON.stringify(form_data));
});
on('#ajax', 'click', '.dislike_support_manager', function() {
  _this = this;
  form_data = new FormData();
  form_data.append("id", this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.getAttribute("chat-pk"));
  
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/chat/user_progs/dislike_manager", true );
  link.setRequestHeader('Content-Type', 'application/json');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    if (_this.classList.contains("btn_danger")) {
      _this.classList.remove("btn_danger");
      _this.classList.add("btn_default");
    } else {
      _this.classList.add("btn_danger");
      _this.classList.remove("btn_default");
    };
    next = _this.previousElementSibling.classList;
    next.remove("btn_success");
    next.add("btn_default");
  }};
  link.send(JSON.stringify(form_data));
});
