document.title = document.body.querySelector(".main-container").getAttribute("data-title");

function create_gif_loading () {
  $load_gif = document.createElement("img");
  $load_gif.setAttribute("src", "/static/images/preloader.gif");
  $load_gif.style.width = "40px";
  $load_div = document.createElement("div");
  $load_div.classList.add("centered", "m-1");
  $load_div.append($load_gif);
  return $load_div
};
function close_fullscreen() {
  container = document.body.querySelector("#fullscreens_container");
  container.querySelector(".card_fullscreen").remove();
  if (!container.innerHTML) {
    get_document_opacity_1();
    if (document.body.querySelector(".main-container")) {
      document.title = document.body.querySelector(".main-container").getAttribute("data-title");
    }
  } else {
    prev_window = container.querySelector(".card_fullscreen");
    prev_window.classList.remove("hide");
    if (container.querySelector(".data-title")) {
      document.title = container.querySelector(".data-title").getAttribute("data-title");
    }
  };
  window.history.replaceState(null, null, window.location.pathname);
  all_windows = container.querySelectorAll(".card_fullscreen");
  for (var i = 0; i < all_windows.length; i++) {
    if (!all_windows[i].querySelector("#fullscreen_loader").firstChild) {
      all_windows[i].remove();
    }
  }
  if (!container.firstChild) {
    get_document_opacity_1();
  }
};

function create_fullscreen(url, type_class, need_drag_items, need_replace_history) {
  container = document.body.querySelector("#fullscreens_container");

  try {
    if (document.body.querySelector(".video_fullscreen").style.display =="none") {
      count_items = container.querySelectorAll(".card_fullscreen").length
    } else {
      count_items = container.querySelectorAll(".card_fullscreen").length + 1
    }
  }
  catch {count_items = 0};

  $parent_div = document.createElement("div");
  $parent_div.classList.add("card_fullscreen", "mb-3", "border", type_class);
  $parent_div.style.zIndex = 100 + count_items;
  $parent_div.style.opacity = "0";

  if (document.body.querySelector(".desctop_nav")) {
    hide_svg = '<svg class="svg_default" style="position:fixed;" width="30" height="30" fill="currentColor" viewBox="0 0 24 24"><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/><path d="M0 0h24v24H0z" fill="none"/></svg>'
  } else { hide_svg = "" };
  $hide_span = document.createElement("span");
  $hide_span.classList.add("this_fullscreen_hide");

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  link.open('GET', url, true);
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
        if (container.innerHTML) {
          prev_window = container.querySelector(".card_fullscreen");
          prev_window.classList.add("hide");
        };

        $loader = document.createElement("div");

        $loader.setAttribute("id", "fullscreen_loader");
        $hide_span.innerHTML = hide_svg;
        $parent_div.append($hide_span);
        $parent_div.append($loader);
        $parent_div.append(create_gif_loading ());
        container.prepend($parent_div);

          $load_div.remove();
          elem = link.responseText;

          $loader.innerHTML = elem;
          height = $loader.scrollHeight*1 + 30;
          if (height < 500 && !$loader.querySelector(".data_display")) {
            $parent_div.style.height = height + "px";
            $loader.style.overflowY = "unset";

            _height = (window.innerHeight - height - 50) / 2;
            $parent_div.style.top = _height + "px";
            prev_next_height = _height*1 + 50 + "px";
            try {$loader.querySelector(".prev_item").style.top = "-" + prev_next_height}catch {null};
            try {$loader.querySelector(".next_item").style.top = "-" + prev_next_height}catch {null}
          } else {
            $parent_div.style.height = "100%";
            $parent_div.style.top = "15px";
            $loader.style.overflowY = "auto";
          };
          $parent_div.style.opacity = "1";
          $parent_div.style.opacity = "1";
          if ($loader.querySelector(".data_display")) {
            $loader.style.overflowY = "unset";
          }

          get_document_opacity_0();
          if ($loader.querySelector(".next_page_list")) {
            $loader.onscroll = function() {
              box = $loader.querySelector('.next_page_list');
              if (box && box.classList.contains("next_page_list")) {
                  inViewport = elementInViewport(box);
                  if (inViewport) {
                      box.remove();
                      var link_3 = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
                      link_3.open('GET', location.protocol + "//" + location.host + box.getAttribute("data-link"), true);
                      link_3.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

                      link_3.onreadystatechange = function() {
                          if (this.readyState == 4 && this.status == 200) {
                              var elem = document.createElement('span');
                              elem.innerHTML = link_3.responseText;
                              $loader.querySelector(".is_block_paginate").insertAdjacentHTML('beforeend', elem.querySelector(".is_block_paginate").innerHTML);
                            }
                      }
                      link_3.send();
                  }
              };
            }
          }
          get_music_player_support($loader);
          if ($loader.querySelector(".data-title")) {
            document.title = $loader.querySelector(".data-title").getAttribute("data-title");
          }
          if (need_replace_history) {
            window.history.pushState(null, "vfgffgfgf", window.location.href + "?url=" + url + "&screen=" + type_class);
          }
      }
  };
  link.send();
};


function change_this_fullscreen(_this, type_class) {
  _this.parentElement.classList.contains("col") ? $loader = _this.parentElement.parentElement.parentElement.parentElement : $loader = _this.parentElement.parentElement.parentElement;
  $loader.innerHTML = "";
  $parent_div.style.opacity = "0";
  $parent_div.style.height = "35px";

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  link.open('GET', _this.getAttribute("href"), true);
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
          elem = link.responseText;
          $loader.innerHTML = elem;
          height = $loader.scrollHeight*1 + 30;
          $parent_div = $loader.parentElement
          if (height < 500 && !$loader.querySelector(".data_display")){
            $parent_div.style.height = height + "px";
            _height = (window.innerHeight - height - 50) / 2;
            $parent_div.style.top = _height + "px";
            prev_next_height = _height*1 + 50 + "px";
            $loader.style.overflowY = "unset";
            try {$loader.querySelector(".prev_item").style.top = "-" + prev_next_height}catch {null};
            try {$loader.querySelector(".next_item").style.top = "-" + prev_next_height}catch {null}
          } else {
            $parent_div.style.height = "100%";
            $parent_div.style.top = "15px";
            $loader.style.overflowY = "auto";
          };
          $parent_div.style.opacity = "1";
          $parent_div.style.opacity = "1";
          if ($loader.querySelector(".data_display")) {
            $loader.style.overflowY = "unset";
          }
          get_music_player_support($loader);
          if ($loader.querySelector(".data-title")) {
            document.title = $loader.querySelector(".data-title").getAttribute("data-title");
          }
          window.history.pushState(null, "vfgffgfgf", window.location.href + "?url=" + _this.getAttribute("href") + "&screen=" + type_class);
      }
  };
  link.send();
};

on('body', 'click', '.this_fullscreen_hide', function() {
  close_fullscreen()
});
on('body', 'click', '.this_mob_fullscreen_hide', function() {
  close_fullscreen()
});

on('body', 'click', '.body_overlay', function() {
  close_fullscreen()
});

function get_document_opacity_0() {
  document.body.style.overflow = "hidden";
  document.body.style.marginRight = "4px";
  overlay = document.body.querySelector(".body_overlay");
  overlay.style.visibility = "unset";
  overlay.style.opacity = "1";
};
function get_document_opacity_1() {
  document.body.style.overflow = "scroll";
  document.body.style.marginRight = "0";
  overlay = document.body.querySelector(".body_overlay");
  overlay.style.visibility = "hidden";
  overlay.style.opacity = "0";
};

function addStyleSheets (href) {
  $head = document.head,
  $link = document.createElement('link');
  $link.rel = 'stylesheet';
  $link.classList.add("my_color_settings");
  $link.href = href;
  $head.appendChild($link);
};

function get_music_player_support () {
  return
}
function get_video_dop() {
  styles = document.querySelectorAll(".my_color_settings");
  style = styles[styles.length- 1];
  settings = [];
  if (style.href.indexOf("a.css") != -1){
    settings += ["images/video_white",'#eeeeee','#FFFFFF']
  } else {
    settings += ["images/video_dark",'#000000','#000000']
  }
  return settings.split(',')
};
function get_resize_screen(){
  video_player.maxWidth = 360;
  video_player.maxHeight = 270;
  video_player.showPlaylist();
};
function get_normal_screen(){
  video_player.maxWidth = 1170;
  video_player.maxHeight = 659;
  video_player.hidePlaylist();
};
function load_video_playlist(video_saver_id) {
  video_saver = document.body.querySelector("#video_id_saver");
  styles = document.querySelectorAll(".my_color_settings");
  style = styles[styles.length- 1];

video_player = new FWDUVPlayer({
    instanceName:video_saver_id,
    parentId: "video_player",
    playlistsId:"video_playlists",
    skinPath: get_video_dop()[0],
    mainFolderPath:"/static",
    displayType:"responsive",                 // тип дисплея (выбран отзывчивый к размерам экрана)
    useVectorIcons:"no",                      // использование векторной графики
    fillEntireVideoScreen:"no",               // заполнение всего экрана видео-роликом
    fillEntireposterScreen:"yes",             // заполнение всего экрана постером
    goFullScreenOnButtonPlay:"no",            // показывать кнопку включения полноэкранного режима
    playsinline:"yes",                        // играет в ряд
    initializeOnlyWhenVisible:"no",           // инициализировать плеер только тогда, когда он виден
    youtubeAPIKey:'AIzaSyCgbixU3aIWCkiZ76h_E-XpEGig5mFhnVY', // ключ разработчика для ютуба
    useHEXColorsForSkin:"no",                 // использование hex кодировки для скина
    normalHEXButtonsColor:"#FF0000",          // цвет кнопки
    selectedHEXButtonsColor:"#000000",        // цвет нажатой кнопки
    useResumeOnPlay:"no",                     // использование резюме при проигрывании
    useDeepLinking:"no",                      // использование глубоких ссылок для ограничения перехвата ссылки на видео
    showPreloader:"yes",                      // gjrfpsdfnm ghtkjflth ghb pfuheprt gktthf
    preloaderBackgroundColor:"#000000",       // цвет фона прелоадера
    preloaderFillColor:"#FFFFFF",             // цвет прелоадера
    addKeyboardSupport:"no",                 // использовать поддержку клавиатуры
    autoScale:"yes",                          // автоматическое масштабирование
    stopVideoWhenPlayComplete:"no",           // остановить плеер после проигрывания последнего ролика
    playAfterVideoStop:"yes",                 // воспроизведение после остановки видео
    autoPlay:"no",                            // автоматический старт проигрывания
    loop:"no",                                // повтор видео сразу
    shuffle:"no",                             // перемешивание видео сразу
    showErrorInfo:"no",                       // показывать информацию об ошибках
    maxWidth:1170,                            // максимальная ширина
    maxHeight:659,                            // максимальная высота
    volume:.8,                                // начальная громкость плеера (1 - 100%)
    backgroundColor:get_video_dop()[1],                // цвет фона
    videoBackgroundColor:"#000000",           // цвет фона видео-секции
    posterBackgroundColor:"#000000",          // цвет фона постера

    //logo settings
    showLogo:"no",                            // показывать логотип над секцией видео справа

    //playlists/categories settings
    showPlaylistsSearchInput:"no",            // показывать поле поиска плейлиста
    usePlaylistsSelectBox:"no",              // использовать выбор плейлистов в окне сверху
    showPlaylistsByDefault:"no",              // показать плейлист по умолчанию
    thumbnailSelectedType:"opacity",          // анимация выбранного плейлиста в окне сверху
    startAtPlaylist:0,                        // проигрывать плейлист номер ...
    buttonsMargins:15,                        // расстояние между кнопками
    thumbnailMaxWidth:350,                    // максимальная ширина миниатюры
    thumbnailMaxHeight:350,                   // максимальная высота миниатюры
    horizontalSpaceBetweenThumbnails:40,      // расстояние между миниатюрами по горизонтали
    verticalSpaceBetweenThumbnails:40,        // расстояние между миниатюрами по вертикали
    inputBackgroundColor:"#333333",           // цвет фона поля ввода
    inputColor:"#000000",                     // цвет текста поля ввода

    //playlist settings
    showPlaylistButtonAndPlaylist:"yes",      // показывать кнопку выбора плейлистов и сами плейлисты сверху
    playlistPosition:"right",                 // расположение плейлиста
    showPlaylistByDefault:"yes",              // показать плейлист по умолчанию
    showPlaylistName:"no",                   // показывать название плейлиста
    showSearchInput:"no",                    // показывать поле поиска
    showLoopButton:"yes",                     // показывать кнопку повтора
    showShuffleButton:"yes",                  // показывать кнопку перемешивания
    showPlaylistOnFullScreen:"no",            // показывать плейлист в режиме полного экрана
    showNextAndPrevButtons:"yes",             // показывать кнопки пред/след видео
    showThumbnail:"yes",                      // показывать миниатюры
    addMouseWheelSupport:"yes",               // поддержка управления мыши
    startAtRandomVideo:"no",                  // начинать воспроиведение со случайного видео ролика
    stopAfterLastVideoHasPlayed:"no",         // останавливать воспроизведение после последнего ролика
    addScrollOnMouseMove:"no",                // перемотка движениями мыши
    randomizePlaylist:'no',                   // случайные плейлисты
    folderVideoLabel:"VIDEO ",                // название папки видео
    playlistRightWidth:320,                   // ширина плейлиста справа
    playlistBottomHeight:380,                 // высота плейлиста снизу
    startAtVideo:0,                           // начинать с ролика номер ...
    maxPlaylistItems:50,                      // максимальное количество роликов в плейлисте
    thumbnailWidth:71,                        // ширина миниатюры
    thumbnailHeight:71,                       // высота миниатюры
    spaceBetweenControllerAndPlaylist:1,      // расстояние между контроллером и плейлистом
    spaceBetweenThumbnails:1,                 // расстояние между миниатюрами
    scrollbarOffestWidth:8,                   // отступ ширины скроллбара
    scollbarSpeedSensitivity:.5,              // скорость отклика скроллбара
    playlistBackgroundColor:get_video_dop()[1],         // цвет фона плейлиста
    playlistNameColor:get_video_dop()[1],              // цвет названия плейлиста
    thumbnailNormalBackgroundColor:get_video_dop()[2], // цвет фона миниатюры
    thumbnailHoverBackgroundColor:get_video_dop()[1],  // цвет фона активной миниатюры
    thumbnailDisabledBackgroundColor:get_video_dop()[1], // цвет фона disabled миниатюры
    youtubeAndFolderVideoTitleColor:get_video_dop()[1],// цвет плейлиста роликов с папок и ютуба
    youtubeOwnerColor:"#919191",              // цвет названия ролика я ютуба
    youtubeDescriptionColor:"#919191",        // цвет описания ролика я ютуба
    mainSelectorBackgroundSelectedColor:get_video_dop()[2], // цвет фона плейлиста при наведении
    mainSelectorTextNormalColor:get_video_dop()[1],    // цвет текста плейлиста
    mainSelectorTextSelectedColor:get_video_dop()[2],
    mainButtonBackgroundNormalColor:get_video_dop()[2],// цвет фона кнопок
    mainButtonBackgroundSelectedColor:get_video_dop()[2],// цвет фона нажатой кнопки
    mainButtonTextNormalColor:get_video_dop()[2],      // цвет текста кнопок
    mainButtonTextSelectedColor:get_video_dop()[2],    // цвет текста нажатой кнопки

    //controller settings
    showController:"yes",                     // показывать контроллер
    showControllerWhenVideoIsStopped:"yes",   // показывать контроллер при остановке проигрывания
    showNextAndPrevButtonsInController:"no",  // показывать кнопки пред / след на контроллере
    showRewindButton:"yes",                   // показать кнопку перемотки назад
    showPlaybackRateButton:"yes",             // показать кнопку выбора скорости воспроизведения
    showVolumeButton:"yes",                   // показать кнопку громкости
    showTime:"yes",                           // показать время воспроизведения
    showQualityButton:"yes",                  // показать время выбора качества видео
    showInfoButton:"no",                     // показывать кнопку информации ролика
    showShareButton:"no",                     // показывать кнопку расшаривания ролика
    showEmbedButton:"no",                    // показывать кнопку получения ссылки ролика и фрейма для вставки на другие сайты
    showChromecastButton:"no",                // показывать кнопку подкастов
    showFullScreenButton:"yes",               // показывать кнопку полноэкранного режима
    disableVideoScrubber:"no",                // выключить ползунок переключения времени видео
    showScrubberWhenControllerIsHidden:"yes", // показывать ползунок времени воспроизведенного ролика при скрытом контроллере
    showDefaultControllerForVimeo:"no",       // показывать контроллер vimeo
    repeatBackground:"yes",                   // повтор бекгроунда
    controllerHeight:42,                      // высота контроллера
    controllerHideDelay:3,                    // время, через которое скроется контроллер
    startSpaceBetweenButtons:7,               // начальное расстояние между кнопками
    spaceBetweenButtons:8,                    // расстояние между кнопками
    scrubbersOffsetWidth:2,                   // ширина отступа скруббера
    mainScrubberOffestTop:14,                 // отступ скруббера всерху
    timeOffsetLeftWidth:5,                    // ширина отступа времени воспроизведения слева
    timeOffsetRightWidth:3,                   // ширина отступа времени воспроизведения справа
    timeOffsetTop:0,                          // отступ времени воспроизведения сверху
    volumeScrubberHeight:80,                  // высота скруббера громкости
    volumeScrubberOfsetHeight:12,             // отступскруббера громкости по высоте
    timeColor:"#919191",                      // цвет времени воспроизведения
    youtubeQualityButtonNormalColor:"#919191",// кнопка выбора качества плейлитса ютуба
    youtubeQualityButtonSelectedColor:"#000000",// нажатая кнопка выбора качества плейлитса ютуба

    //advertisement on pause window
    aopwTitle:"Advertisement",                // название рекламной вставки
    aopwWidth:400,                            // ширина вставки
    aopwHeight:240,                           // высота вставки
    aopwBorderSize:6,                         // размер рамки вставки
    aopwTitleColor:"#000000",                 // цветназвания вставки

    //subtitle
    subtitlesOffLabel:"Субтитры откл.",       // надпись, когда субтитры отключены

    //popup add windows
    showPopupAdsCloseButton:"no",            // показать кнопку закрытия окна подставки

    //окно размещения и информации
    embedAndInfoWindowCloseButtonMargins:15,  // отступ кнопки закрытия
    borderColor:"#CDCDCD",                    // цвет рамки
    mainLabelsColor:"#000000",                // цвет названия
    secondaryLabelsColor:"#444444",           // вторичный цвет названия
    shareAndEmbedTextColor:"#777777",         // цвет тектса овна расшаривания и вставки
    inputBackgroundColor:"#c0c0c0",           // цвет фона поля ввода
    inputColor:"#333333",                     // цвет фона текста ввода

    //audio visualizer
    audioVisualizerLinesColor:"#ff9f00",      // цвет линий аудио визуализатора
    audioVisualizerCircleColor:"#FFFFFF",     // цвет кругов аудио визуализатора

    //lightbox settings
    lightBoxBackgroundOpacity:.6,             // прозрачность
    lightBoxBackgroundColor:"#000000",        // цвет фона

    //sticky on scroll
    stickyOnScroll:"no",                      // липкое листание
    stickyOnScrollShowOpener:"yes",           // показывать эффект
    stickyOnScrollWidth:"700",                // ширина
    stickyOnScrollHeight:"394",               // высота

    //настройки липкого дисплея
    showOpener:"yes",                         // показывать вставки
    showOpenerPlayPauseButton:"yes",          // показывать кнопку плей при паузе
    verticalPosition:"bottom",                // позиция по вертикали
    horizontalPosition:"center",              // позиция по горизонтали
    showPlayerByDefault:"yes",                // показывать плеер по умолчанию
    animatePlayer:"yes",                      // анимировать плеер
    openerAlignment:"right",                  // выравнивание вставки
    mainBackgroundImagePath:"main-background.png", // путь до изображения фона
    openerEqulizerOffsetTop:-1,               // отступ эквалайзера сверху
    openerEqulizerOffsetLeft:3,               // отступ эквалайзера слева
    offsetX:0,                                // отступ по оси X
    offsetY:0,																// отступ по оси Y

    //скорость воспроизведения
    defaultPlaybackRate:1,                   //0.25, 0.5, 1, 1.25, 1.2, 2
    //cuepoints
    executeCuepointsOnlyOnce:"no",           // выполнение ключевых точек только один раз
    //annotations
    showAnnotationsPositionTool:"no",        // показывать координаты аннотаций на экране

    //ads
    openNewPageAtTheEndOfTheAds:"no",        // открыть новую страницу в конце объявления
    adsButtonsPosition:"left",               // позиция окна рекламы
    skipToVideoText:"Закрыть через: ",       // текст окна рекламы
    skipToVideoButtonText:"Закрыть",         // текст кнопки закрытия рекламного окна
    adsTextNormalColor:"#888888",            // цвет рекламного текста
    adsTextSelectedColor:"#000000",          // цвет выбранного текста
    adsBorderNormalColor:"#AAAAAA",          // цвет рамки рекламного окна
    adsBorderSelectedColor:"#000000",        // цвет выбраной рамки рекламного окна

    //a to b loop
    useAToB:"no",                            // использование повтора от...до
    atbTimeBackgroundColor:"transparent",    // время фона от...до
    atbTimeTextColorNormal:"#888888",        // время текста от...до
    atbTimeTextColorSelected:"#FFFFFF",
    atbButtonTextNormalColor:"#888888",
    atbButtonTextSelectedColor:"#FFFFFF",
    atbButtonBackgroundNormalColor:"#FFFFFF",
    atbButtonBackgroundSelectedColor:"#000000",

    //thumbnails preview
    thumbnailsPreviewWidth:196,              // ширина предпросмотра миниатюры
    thumbnailsPreviewHeight:110,             // высота предпросмотра миниатюры
    thumbnailsPreviewBackgroundColor:get_video_dop()[1],// цвет фона  миниатюры
    thumbnailsPreviewBorderColor:"#666",     // цвет названия миниатюры
    thumbnailsPreviewLabelBackgroundColor:"#666", // цвет фона названия минатюры
    thumbnailsPreviewLabelFontColor:"#FFF",
    // context menu
    showContextmenu:'no',
    showScriptDeveloper:"no",
    contextMenuBackgroundColor:"#ebebeb",
    contextMenuBorderColor:"#ebebeb",
    contextMenuSpacerColor:"#CCC",
    contextMenuItemNormalColor:"#888888",
    contextMenuItemSelectedColor:"#000",
    contextMenuItemDisabledColor:"#BBB"
});
};

class ToastManager{constructor(){this.id=0;this.toasts=[];this.icons={'SUCCESS':"",'ERROR':'','INFO':'','WARNING':'',};var body=document.querySelector('#ajax');this.toastsContainer=document.createElement('div');this.toastsContainer.classList.add('toasts','border-0');body.appendChild(this.toastsContainer)}showSuccess(message){return this._showToast(message,'SUCCESS')}showError(message){return this._showToast(message,'ERROR')}showInfo(message){return this._showToast(message,'INFO')}showWarning(message){return this._showToast(message,'WARNING')}_showToast(message,toastType){var newId=this.id+1;var newToast=document.createElement('div');newToast.style.display='inline-block';newToast.classList.add(toastType.toLowerCase());newToast.classList.add('toast');newToast.innerHTML=`<progress max="100"value="0"></progress><h3>${message}</h3>`;var newToastObject={id:newId,message,type:toastType,timeout:4000,progressElement:newToast.querySelector('progress'),counter:0,timer:setInterval(()=>{newToastObject.counter+=1000/newToastObject.timeout;newToastObject.progressElement.value=newToastObject.counter.toString();if(newToastObject.counter>=100){newToast.style.display='none';clearInterval(newToastObject.timer);this.toasts=this.toasts.filter((toast)=>{return toast.id===newToastObject.id})}},10)};newToast.addEventListener('click',()=>{newToast.style.display='none';clearInterval(newToastObject.timer);this.toasts=this.toasts.filter((toast)=>{return toast.id===newToastObject.id})});this.toasts.push(newToastObject);this.toastsContainer.appendChild(newToast);return this.id++}}function toast_success(text){var toasts=new ToastManager();toasts.showSuccess(text)}function toast_error(text){var toasts=new ToastManager();toasts.showError(text)}function toast_info(text){var toasts=new ToastManager();toasts.showInfo(text)}function toast_warning(text){var toasts=new ToastManager();toasts.showWarning(text)}

function on(elSelector,eventName,selector,fn) {var element = document.querySelector(elSelector);element.addEventListener(eventName, function(event) {var possibleTargets = element.querySelectorAll(selector);var target = event.target;for (var i = 0, l = possibleTargets.length; i < l; i++) {var el = target;var p = possibleTargets[i];while(el && el !== element) {if (el === p) {return fn.call(p, event);}el = el.parentNode;}}});};
function on(e,t,i,c){var l=document.querySelector(e);l.addEventListener(t,function(e){for(var t=l.querySelectorAll(i),n=e.target,r=0,o=t.length;r<o;r++)for(var a=n,d=t[r];a&&a!==l;){if(a===d)return c.call(d,e);a=a.parentNode}})}function loadScripts(r){var e=document.createElement("SCRIPT"),t=document.getElementsByTagName("head")[0],o=!1;function n(e,t,n){return t==r&&(o=!0,a()),!1}function a(){e.onreadystatechange=e.onload=e.onerror=null,window.removeEventListener?window.removeEventListener("error",n,!1):window.detachEvent("onerror",n)}e.type="text/javascript",e.onload=e.onreadystatechange=function(e){this.readyState&&"loaded"!=this.readyState&&"complete"!=this.readyState||o||a()},e.onerror=function(){o=!0,a()},window.addEventListener?window.addEventListener("error",n,!1):window.attachEvent("onerror",n),e.src=r,t.appendChild(e)}
function good_gallery(loader){thumb_list = loader.querySelectorAll(".thumb_list li");thumb = loader.querySelector(".big_img");thumb_list.forEach((item) => {item.addEventListener("mouseover", function () {image = item.children[0].src;thumb.src = image;}); }); };
function clear_comment_dropdown(){
  try{
  dropdowns = document.body.querySelectorAll(".current_file_dropdown");
  for (var i = 0; i < dropdowns.length; i++) {
    btn = dropdowns[i].parentElement.parentElement;
    btn.classList.remove("files_two");
    btn.classList.remove("files_one");
    btn.classList.add("files_null");
    btn.style.display = "block";
    dropdowns[i].classList.remove("current_file_dropdown");
  }} catch { null }
  try{
  img_blocks = document.body.querySelectorAll(".img_block");
  for (var i = 0; i < img_blocks.length; i++) {
    img_blocks[i].innerHTML = "";
  }} catch { null }
};
var ready = (callback) => {
  if (document.readyState != "loading") callback();
  else document.addEventListener("DOMContentLoaded", callback);
};
function send_user_form (url) {
  form_data = new FormData();
  form_data.append("first_name", document.body.querySelector(".first_name").value);
  form_data.append("last_name", document.body.querySelector(".last_name").value);
  form_data.append("gender", document.body.querySelector(".gender").value);
  form_data.append("password", document.body.querySelector(".password1").value);
  form_data.append("birthday", document.body.querySelector(".birthday").value);

  request_2 = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  request_2.open( 'POST', url, true );
  request_2.onreadystatechange = function() {
      if (request_2.readyState == 4 && request_2.status == 200) {
        window.location.href = "/"
      }
  }
  request_2.send(form_data);
};
on('#ajax', 'click', '#code_send', function() {
  _form = document.body.querySelector(".final_process_form");
  _user_phone = document.getElementById('phone').value;
  _user_phone = _user_phone.replace(/[^0-9]/g, '');

  if (_user_phone[0] == "+") {
    _user_phone = _user_phone.slice(2)
  }
  else if (_user_phone[0] == "8" || _user_phone[0] == "7") {
    _user_phone = _user_phone.slice(1)
  };

  var code = document.getElementById('code').value;
  form_data = new FormData();
  form_data.append("phone", 7 + _user_phone);
  form_data.append("code", code);
  var request = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    request.open('POST', "/phone_verify", true);
    request.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    request.onreadystatechange = function() { 
        if (request.readyState == 4 && request.status == 200) {
          var div = document.getElementById('jsondata');

          div.innerHTML = request.responseText;
          console.log(request.responseText);
            if (request.responseText == "ok") {
              send_user_form("/signup");
            }
        }
    };
    request.send(form_data)
});

function phone_check() {
    if (document.getElementById('phone').value.length > 9)
        document.getElementById("phone_send").removeAttribute('disabled');
    else
        document.getElementById("phone_send").setAttribute("disabled", "true");
};

function code_check() {
    if (document.getElementById('code').value.length === 4)
        document.getElementById("code_send").removeAttribute('disabled');
    else
        document.getElementById("code_send").setAttribute("disabled", "true");
};

on('#ajax', 'click', '#phone_send', function() {
  _user_phone = document.getElementById('phone').value;
  _user_phone = _user_phone.replace(/[^0-9]/g, '');

  if (_user_phone[0] == "+") {
    _user_phone = _user_phone.slice(2)
  }
  else if (_user_phone[0] == "8" || _user_phone[0] == "7") {
    _user_phone = _user_phone.slice(1)
  };


  form_data = new FormData();
  form_data.append("phone", 7 + _user_phone);

  var request = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  request.open('POST', "/phone_send", true);
  request.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  request.onreadystatechange = function() {
      if (request.readyState == 4 && request.status == 200) {
          var div = document.getElementById('jsondata');
          div.innerHTML = request.responseText;
          if (div.querySelector("#code_send")) {
            document.querySelector('#phone_send').setAttribute("disabled", "true")
          }
      }
  };
  request.send(form_data);
});

function create_hide_input (name, value, _class) {
  $input = document.createElement("input");
  $input.setAttribute("name", name);
  $input.setAttribute("type", "hidden");
  $input.setAttribute("disabled", "true");
  $input.classList.add(_class);
  $input.value = value;
  return $input
}

on('body', 'click', '#register_ajax', function() {
  form = document.querySelector("#signup");
  first_name = form.querySelector("#id_first_name");
  last_name = form.querySelector("#id_last_name");

  if (!first_name.value){
    first_name.style.border = "1px #FF0000 solid";
    toast_error("Имя - обязательное поле!");
    return
  } else if (!last_name.value){
    last_name.style.border = "1px #FF0000 solid";
    toast_error("Фамилия - обязательное поле!");
    return
  } else if (!form.querySelector("#password1").value){
    form.querySelector("#password1").style.border = "1px #FF0000 solid";
    toast_error("Пароль - обязательное поле!");
    return
  } else if (!form.querySelector("#password2").value){
    form.querySelector("#password2").style.border = "1px #FF0000 solid";
    toast_error("Введите пароль еще раз!");
    return
  }

  else if (!form.querySelector("#date_day").value){
      form.querySelector("#date_day").style.border = "1px #FF0000 solid";
      toast_error("День рождения - обязательное поле!");
      return
  } else if (!form.querySelector("#date_month").value){
      form.querySelector("#date_month").style.border = "1px #FF0000 solid";
      toast_error("Месяц рождения - обязательное поле!");
      return
  } else if (!form.querySelector("#date_year").value){
      form.querySelector("#date_year").style.border = "1px #FF0000 solid";
      toast_error("Год рождения - обязательное поле!");
      return
  } else {
    this.disabled = true
  }

    //_span.querySelector(".user_name").innerHTML = first_name.value + " " + last_name.value;
    container = document.body.querySelector(".main-container");
    container.innerHTML = "<form class='h-sm-auto final_process_form'><h1 class='font-weight-light mb-3 mt-4 text-center user_name'></h1><p class='text-center'>Вам нужно подтвердить с помощью телефона свой профиль, и в будущем использовать номер телефона в качестве логина к профилю соцсети трезвый.рус.</p><div class='row'><div class='col-md-2'></div><div class='col-md-4' style='display: inline-flex;margin-top: 12px;margin-bottom: 12px;'><input type='number' id='phone' min='0' onkeyup='phone_check();' class='form-control border-0' placeholder='Телефон'><hr class='my-0'></div><div class='col-md-4' style='margin-top: 12px;'><button type='button' disabled='disabled' id='phone_send' class='btn btn-primary pink-gradient'>Получить код</button></div><div class='col-md-2'></div></div><div id='jsondata' style='margin-top:50px'></div><div id='jsondata2' style='margin-top:50px'></div></form>";

    final_form = container.querySelector(".final_process_form");
    final_form.append(create_hide_input ("first_name", first_name.value, "first_name"));
    final_form.append(create_hide_input ("last_name", last_name.value, "last_name"));
    final_form.append(create_hide_input ("password", form.querySelector("#password1").value, "password1"));
    final_form.append(
      create_hide_input ("birthday",
        form.querySelector("#date_year").value
        + "-" + form.querySelector("#date_month").value +
        "-" + form.querySelector("#date_day").value, "birthday"));
    final_form.append(create_hide_input ("gender", form.querySelector("#customradio1").value,"gender"));
});

on('body', 'click', '#logg', function() {
  _this = this;
  form = document.querySelector("#login_form");
  response = form.querySelector(".api_response")
  user_phone = form.querySelector("#id_username").value;
  _user_phone = user_phone.replace(/[^0-9]/g, '');
  if (!_user_phone){
    form.querySelector("#id_username").style.border = "1px #FF0000 solid";
    response.innerHTML = "Введите телефон!";
    response.classList.add("error");
    return
  }
  else if (!form.querySelector("#id_password").value){
    form.querySelector("#id_password").style.border = "1px #FF0000 solid";
    response.innerHTML = "Введите пароль!";
    response.classList.add("error")
    return
  }
  else {
    _this.disabled = true;
    form.querySelector("#id_username").style.display = "none";
    if (_user_phone[0] == "+") {
      _user_phone = _user_phone.slice(2)
    }
    else if (_user_phone[0] == "8" || _user_phone[0] == "7") {
      _user_phone = _user_phone.slice(1)
    };
    form.querySelector("#id_username").value = 7 + _user_phone;
  }
  if (form.querySelector("#id_username").value){form.querySelector("#id_username").style.border = "rgba(0, 0, 0, 0.2)";}
  if (form.querySelector("#id_password").value){form.querySelector("#id_password").style.border = "rgba(0, 0, 0, 0.2)";}

  //form_data = new FormData(form);
  form_data = {
    "phone": 7 + _user_phone,
    "password": form.querySelector("#id_password").value
  };

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/login", true ); 
  link.setRequestHeader('Content-Type', 'application/json');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    response.classList.replace("error", "success");
    response.innerHTML = "Успешный вход";
    _this.disabled = true;
    window.location.href = "/"
  } 
  else {
    _this.disabled = false;
    response.style.display = "block";
    response.innerHTML = "Телефон или пароль - неверный!";
    response.classList.add("error");
    form.querySelector("#id_username").style.display = "block";
    form.querySelector("#id_username").value = '';
    form.querySelector("#id_password").value = '';
  }};
  link.send(JSON.stringify(form_data);
}); 

function play_video_list(url, counter, video_pk) {
  loader = document.getElementById("video_loader");
  open_video_fullscreen(url);

  video_player_id = document.body.getAttribute('data-video');
  document.body.setAttribute('data-video', document.body.getAttribute('data-video') + "a");

  setTimeout(function() {
    load_video_playlist(video_player_id + "a");
    info_video = document.body.querySelector('#info_video');
    _playlist = info_video.parentElement.querySelector('#my_video_playlist');
    items = _playlist.querySelectorAll('.video_playlist_li');
    ids = []
    for (var i = 0; i < items.length; i++) {
      id = items[i].getAttribute("data-video-uuid");
      ids.push(id);
    };

    video_player.addListener(FWDUVPlayer.READY, videoReady);
    video_player.addListener(FWDUVPlayer.PLAY, videoStart);
    video_player.addListener(FWDUVPlayer.PAUSE, videoPause);

    function videoReady() {
      console.log("video player ready");
      info_video.classList.remove("hide");
      setTimeout(function() {
        video_player.playVideo(counter);
      }, 1000);

      info_video = document.body.querySelector("#info_video");
      if (info_video.innerHTML == "" || info_video.getAttribute("data-pk") != video_pk){
        info_video.setAttribute("data-pk", video_pk);
        console.log("Воспроизводится ролик № : " + video_pk)
      }
    };
    function videoStart() {
      new_counter = video_player.getVideoId();
      console.log("current id ", ids[new_counter]);
      if (new_counter != counter) {
        counter = new_counter;
        list_block_load(info_video, "#info_video", "/video/load_video/" + ids[new_counter] + "/");
      }
    };
    function videoPause() {
      console.log("video player pause!");
    }
  }, 500);
  video = document.createElement("div");
  video.classList.add("video_init");
  document.body.querySelector("#video_loader").append(video)
};

function dragElement(elmnt) {
  var pos1 = 0, pos2 = 0, pos3 = 0, pos4 = 0;
  document.querySelector("#draggable-header").onmousedown = dragMouseDown;
	document.querySelector("#draggable-resize").onmousedown = resizeMouseDown;

  function dragMouseDown(e) {
    e = e || window.event;
    e.preventDefault();
    pos3 = e.clientX;
    pos4 = e.clientY;
    document.onmouseup = closeDragElement;
    document.onmousemove = elementDrag;
  }

	function resizeMouseDown(e) {
    e = e || window.event;
    e.preventDefault();
    pos3 = 0;
    pos4 = 0;
    document.onmouseup = closeDragElement;
    document.onmousemove = elementResize;
  }

	function elementResize(e) {
		e = e || window.event;
    e.preventDefault();
		var content = document.querySelector(".draggable");
		var width = content.offsetWidth;
		var height = content.offsetHeight;

		pos1 = (e.clientX - width) - content.offsetLeft;
    pos2 = (e.clientY - height) - content.offsetTop;

		content.style.width = width + pos1 + 'px';
		content.style.height = height + pos2 + 'px';
	}

  function elementDrag(e) {
    e = e || window.event;
    e.preventDefault();
    pos1 = pos3 - e.clientX;
    pos2 = pos4 - e.clientY;
    pos3 = e.clientX;
    pos4 = e.clientY;
    elmnt.style.top = (elmnt.offsetTop - pos2) + "px";
    elmnt.style.left = (elmnt.offsetLeft - pos1) + "px";
  }

  function closeDragElement() {
    document.onmouseup = null;
    document.onmousemove = null;
  }
};

function open_video_fullscreen(url) {
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    link.open('GET', url, true);
    link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

    link.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
          elem = link.responseText;
          block = document.body.querySelector("#video_loader");
          try {
            count_items = container.querySelectorAll(".card_fullscreen").length
          } catch {count_items = 0};
          block.parentElement.style.zIndex = 100 + count_items;
          block.parentElement.style.display = "block";
          block.innerHTML = elem;
        }
    };
    link.send();
};

function if_list(block) {
  if (block.querySelector('.load_block') && block.querySelector('.load_block').getAttribute) {
      _block = block.querySelector('.load_block');
      list_block_load(_block, ".load_block", _block.getAttribute("data-link"));
      scrolled(_block.querySelector('.is_paginate'));
  } else if (block.querySelector('.is_block_paginate')) {
      lenta = block.querySelector('.is_block_paginate');
      link = lenta.getAttribute("data-link");
      list_load(block.querySelector(".is_block_paginate"), link);
      scrolled(lenta.querySelector('.list_pk'));
  };

  params = window.location.search.replace( '?', '').split('&');
  if (params) {
    if (params[0].split("=")[0] == "url") {
      try {
        url = params[0].split("=")[1];
        fullscreen = params[1].split("=")[1];
        setTimeout(create_fullscreen(url, fullscreen, false), 3000)
      } catch { null };
    }
  }
};

function list_load(block, link) {
  // грузим что-то по ссылке link в блок block
  var request = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  request.open( 'GET', link, true );
  request.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  request.onreadystatechange = function () {if ( request.readyState == 4 && request.status == 200 ) {block.innerHTML = request.responseText;}};request.send( null );
  try{ fullscreen_resize() } catch { null };
};
function list_block_load(target_block, response_block, link) {
  // грузим блок response_block по ссылке link в блок target_block
  var request = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  request.open( 'GET', link, true );
  request.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  request.onreadystatechange = function () {
    if ( request.readyState == 4 && request.status == 200 ){
        elem_ = document.createElement('span');
        elem_.innerHTML = request.responseText;
       target_block.innerHTML = elem_.querySelector(response_block).innerHTML;
       create_pagination(target_block);
       get_music_player_support(target_block);
    }};
    request.send( null );
};

function ajax_get_reload(url) { 
  var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  ajax_link.open('GET', url + "?ajax=2", true);
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  ajax_link.onreadystatechange = function() {
    if (this.readyState == 4 && this.status == 200) {
      $link = document.location.pathname;
      rtr = document.getElementById('ajax');
      elem_ = document.createElement('span');
      elem_.innerHTML = ajax_link.responseText;  
      rtr.innerHTML = elem_.innerHTML;
  
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
      //document.querySelector('link[rel="canonical"]').setAttribute("href", _uri);
      window.scrollTo(0, 0);
      window.history.pushState(null, "vfgffgfgf", url);
      if_list(rtr);
      create_pagination(rtr);
      get_document_opacity_1();
    }
  }
  ajax_link.send()
};

function search_ajax_get_reload(url) { 
  $serf_history.push(document.location.href);

    var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    ajax_link.open('GET', url + "?ajax=2", true);
    ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    ajax_link.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            elem_ = document.createElement('span');
            elem_.innerHTML = ajax_link.responseText;
            rtr = document.body.querySelector(".load_search_container");
            rtr.innerHTML = elem_.innerHTML;
            window.scrollTo(0, 0);
            
            document.title = rtr.querySelector('.main-container').getAttribute("data-title");
            if_list(rtr);
            create_pagination(rtr);
            get_document_opacity_1();
        }
    }
    ajax_link.send()
};


on('body', 'click', '.ajax', function(event) {
  event.preventDefault();
  var url = this.getAttribute('href');
  if (url != window.location.pathname){
    ajax_get_reload(url);
  } else {toast_info("Вы уже на этой странице")}
});
on('body', 'click', '.search_ajax', function(event) {
  event.preventDefault();
  var url = this.getAttribute('href');
  if (url != window.location.href){
    search_ajax_get_reload(url);
    search_panel = document.body.querySelector(".search_panel");
    items = search_panel.querySelectorAll(".search_ajax");
    for (var i = 0; i < items.length; i++){
      items[i].classList.remove("active")
    };
    this.classList.add("active");
  }
  else {toast_info("Список уже получен...")}
});

if_list(document.getElementById('ajax'));

function elementInViewport(el){var bounds = el.getBoundingClientRect();return ((bounds.top + bounds.height > 0) && (window.innerHeight - bounds.top > 0));}

function scrolled(_block) {
    // работа с прокруткой:
    // 1. Ссылка на страницу с пагинацией
    // 2. id блока, куда нужно грузить следующие страницы
    onscroll = function() {
        try {
            box = _block.querySelector('.next_page_list');
            if (box && box.classList.contains("next_page_list")) {
                inViewport = elementInViewport(box);
                if (inViewport) {
                    box.classList.remove("next_page_list");
                    paginate(box, target);
                }
            };
        } catch {return}
    }
};

function paginate(block, target) {
        var link_3 = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
        link_3.open('GET', location.protocol + "//" + location.host + block.getAttribute("data-link"), true);
        link_3.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

        link_3.onreadystatechange = function() {
            if (this.readyState == 4 && this.status == 200) {
                var elem = document.createElement('span');
                elem.innerHTML = link_3.responseText;
                if (elem.querySelector(".is_post_paginate")) {
                  block.parentElement.insertAdjacentHTML('beforeend', elem.querySelector(".is_post_paginate").innerHTML)
                } else if (elem.querySelector(".is_paginate")){
                  block.parentElement.insertAdjacentHTML('beforeend', elem.querySelector(".is_paginate").innerHTML)
                } else if (document.body.querySelector(".is_block_paginate")){
                  block_paginate = document.body.querySelector(".is_block_paginate");
                  if (elem.querySelector(".load_block")){
                      block.parentElement.insertAdjacentHTML('beforeend', elem.querySelector(".is_block_paginate").innerHTML)
                  } else {
                    block.parentElement.insertAdjacentHTML('beforeend', elem.innerHTML)
                  }};
                block.remove()
            }
        }
        link_3.send();
};

function create_pagination(block) {
  if (block.querySelector('.chat_container')) {
    scrolled(block.querySelector('.chat_container'))
  }
  else if (block.querySelector('.is_paginate')) {
    scrolled(block.querySelector('.is_paginate'));
    console.log("Работает пагинация для списка не постов")
  }
  else if (block.querySelector('.is_post_paginate')) {
    scrolled(block.querySelector('.is_post_paginate'));
    console.log("Работает пагинация для списка постов")
  }
  else if (block.querySelector('.is_block_paginate')) {
    scrolled(block.querySelector('.is_block_paginate'));
    console.log("Работает пагинация для списка в блоке")
    console.log(block);
  }
};

on('body', 'click', '.next_item', function(event) {
  event.preventDefault();
  this.style.display = "none";
  change_this_fullscreen(this, "photo_fullscreen")
});
on('body', 'click', '.prev_item', function(event) {
  event.preventDefault();
  this.style.display = "none";
  change_this_fullscreen(this, "photo_fullscreen")
});

on('#ajax', 'click', '.item_fullscreen_hide', function() {
  close_work_fullscreen();
});
on('body', 'click', '.video_fullscreen_hide', function() {get_document_opacity_1(), document.querySelector(".video_fullscreen").style.display = "none";document.getElementById("video_loader").innerHTML=""});
on('body', 'click', '.small_video_fullscreen_hide', function() {
  get_document_opacity_1();
  document.querySelector(".video_fullscreen").style.display = "none";
  video_window = document.querySelector(".video_fullscreen");
  video_window.classList.remove("video_fullscreen_resized", "draggable");
  document.body.querySelector(".video_btn_small").style.display = "none";
  document.body.querySelector(".video_btn_big").style.display = "block";
  document.getElementById("video_loader").innerHTML=""
});

on('#ajax', 'click', '.show_replies', function() {
  this.nextElementSibling.classList.toggle('replies_open');
});

loadScripts('/static/scripts/lib/video_player.js');
loadScripts('/static/scripts/lib/video_init.js');
loadScripts('/static/scripts/lib/progressive-image.js');
loadScripts('/static/scripts/docs/get.js');
loadScripts('/static/scripts/survey/get.js');
loadScripts('/static/scripts/posts/community_get.js');
loadScripts('/static/scripts/posts/user_get.js?ver2');
loadScripts('/static/scripts/gallery/user_get.js');
loadScripts('/static/scripts/goods/user_get.js');
loadScripts('/static/scripts/video/user_get.js');
loadScripts('/static/scripts/music/get.js');

function get_custom_design() {
  background = getCookie("background");
  if (background != "") {
    addStyleSheets("/static/styles/color/" + background + ".css")
  }
  else {
    addStyleSheets("/static/styles/color/white.css")
  }
};

on('body', 'click', '.background_change', function() { 
  color = this.getAttribute('data-color');
  input = this.querySelector(".custom-control-input");
  uncheck = document.getElementsByTagName('input');
  for(var i=0;i<uncheck.length;i++) { 
    uncheck[i].checked=false;
  }
  input.checked = true;
  addStyleSheets("/static/styles/color/" + color + ".css");
  setCookie("background", color, 90);
}); 

on('body', 'click', '.menu_drop', function() {
  block = this.nextElementSibling;
  if (block.classList.contains("show")) { block.classList.remove("show") }
  else {
  all_drop = document.body.querySelectorAll(".dropdown-menu");
  for(i=0; i<all_drop.length; i++) {
    all_drop[i].classList.remove("show")
  } block.classList.add("show")}
});

on('#ajax', 'click', '.select_tab_content_universal', function() {
  _this = this;
  if (_this.classList.contains("active")) {
    return
  }
  parent = _this.parentElement.parentElement;
  nav_items = parent.querySelectorAll(".nav-link");
  for (var i = 0; i < nav_items.length; i++) {
    nav_items[i].classList.remove("active");
  }

  var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  ajax_link.open('GET', _this.getAttribute("data-link"), true);
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  ajax_link.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;

          parent.nextElementSibling.innerHTML = elem_.querySelector(".load_block").innerHTML;
          _this.classList.add("active");
      }
  }
  ajax_link.send();
});

on('#ajax', 'click', '.toggle_visible_owner_lists', function() {
  this.parentElement.nextElementSibling.querySelector(".owner_lists").classList.toggle("hide");
})

get_custom_design();