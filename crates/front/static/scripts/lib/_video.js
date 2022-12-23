
! function(e) {
    var t = function() {
        var s = this;
        t.prototype;
        this.main_do = null,
        this.init = function() {
            this.setupScreen(),
            e.onerror = this.showError,
            this.screen.style.zIndex = 1e25,
            setTimeout(this.addConsoleToDom, 100), setInterval(this.position, 100)
        },
        this.position = function() {
            var e = FWDSIUtils.getScrollOffsets();
            s.setX(e.x), s.setY(e.y)
        },
        this.addConsoleToDom = function() {
            -1 != navigator.userAgent.toLowerCase().indexOf("msie 7") ? document.getElementsByTagName("body")[0].appendChild(s.screen)
                                                                      : document.documentElement.appendChild(s.screen)
        },
        this.setupScreen = function() {
            this.main_do = new FWDSIDisplayObject("div", "absolute"),
            this.main_do.setOverflow("auto"),
            this.main_do.setWidth(300),
            this.main_do.setHeight(200),
            this.setWidth(300),
            this.setHeight(200),
            this.main_do.setBkColor("#FFFFFF"),
            this.addChild(this.main_do)
        },
        this.showError = function(e, t, i) {
            var n = s.main_do.getInnerHTML() + "<br>JavaScript error: " + e + " on line " + i + " for " + t;
            s.main_do.setInnerHTML(n),
            s.main_do.screen.scrollTop = s.main_do.screen.scrollHeight
        },
        this.log = function(e) {
            var t = s.main_do.getInnerHTML() + "<br>" + e;
            s.main_do.setInnerHTML(t),
            s.main_do.getScreen().scrollTop = 1e4
        },
        this.init()
    };
    t.setPrototype = function() {
        t.prototype = new FWDSIDisplayObject("div", "absolute")
    }, t.prototype = null, e.FWDConsole = t
}(window),
function(t) {
    "use strict";
    var s = function(e) {
        var o = this;
        o.init = function() {
            TweenLite.ticker.useRAF(!0),
            this.props_obj = e,
            this.listeners = {
                events_ar: []
            },
            this.props_obj ? (this.instanceName_str = this.props_obj.instanceName,
                              this.displayType = this.props_obj.displayType || s.RESPONSIVE,
                              this.displayType = this.displayType.toLowerCase(),
                              o.displayType.toLowerCase() != s.RESPONSIVE && o.displayType.toLowerCase() != s.FULL_SCREEN && o.displayType.toLowerCase() != s.FLUID_WIDTH && o.displayType.toLowerCase() != s.FLUID_WIDTH_AND_HEIGHT && o.displayType.toLowerCase() != s.AFTER_PARENT && (this.displayType = s.RESPONSIVE),
                              this.props_obj.instanceName
                              ? t[this.instanceName_str]
                              ? alert("FWDSI instance name " + this.instanceName_str + " is already defined and contains a different instance reference, set a different instance name.")
                              : (t[this.instanceName_str] = this).props_obj
                              ? (this.body = document.getElementsByTagName("body")[0],
                              this.stageContainer = null,
                              this.displayType == s.FULL_SCREEN ? this.stageContainer = o.body : this.stageContainer = FWDSIUtils.getChildById(this.props_obj.parentId),
                              this.limitElement = FWDSIUtils.getChildById(this.props_obj.limitId),
                              this.limitHeight = this.props_obj.limitHeight,
                              this.customContextMenu_do = null,
                              this.imageManager_do = null,
                              this.info_do = null,
                              this.hider = null,
                              this.main_do = null,
                              this.preloader_do = null,
                              this.playlist_ar = null,
                              this.backgroundColor_str = this.props_obj.backgroundColor || "#transparent",
                              this.maxWidth = this.props_obj.maxWidth || 640,
                              this.maxHeight = this.props_obj.maxHeight || 380,
                              this.sliderOffsetTopAndBottom = this.props_obj.sliderOffsetTopAndBottom || 0,
                              this.offsetPreloader = 0,
                              this.id = -1,
                              this.catId = -1,
                              this.prevCatId = -2,
                              this.prevId = -2,
                              this.stageWidth = 0,
                              this.stageHeight = 0,
                              this.zIndex = this.props_obj.zIndex || 0,
                              this.totalimages,
                              this.slideshowPreloaderPosition = this.props_obj.slideshowPreloaderPosition || "center",
                              this.slideshowPreloaderPosition = this.slideshowPreloaderPosition.toLowerCase(),
                              this.slideshowRadius = this.props_obj.slideshowRadius || 10,
                              this.slideshowBackgroundColor = this.props_obj.slideshowBackgroundColor || "#FFFFFF",
                              this.slideshowFillColor = this.props_obj.slideshowFillColor || "#000000",
                              this.slideshowStrokeSize = this.props_obj.slideshowStrokeSize || 3,
                              this.isMobile_bl = FWDSIUtils.isMobile,
                              this.autoScale_bl = "yes" == this.props_obj.autoScale,
                              this.useVideo_bl = !1,
                              this.hasPointerEvent_bl = FWDSIUtils.hasPointerEvent,
                              this.imageSource = this.props_obj.imageSource,
                              o.initializeOnlyWhenVisible_bl = o.props_obj.initializeOnlyWhenVisible,
                              o.initializeOnlyWhenVisible_bl = "yes" == o.initializeOnlyWhenVisible_bl,
                              this.setupMainDo(),
                              this.startResizeHandler(),
                              o.initializeOnlyWhenVisible_bl ? (t.addEventListener("scroll", o.onInitlalizeScrollHandler), setTimeout(o.onInitlalizeScrollHandler, 500))
                                                             : setTimeout(o.setupSlider, 100))
                              : alert("FWDSI constructor properties object is not defined!")
                              : alert("FWDSI instance name is required please make sure that the instanceName parameter exsists and it's value is uinique."))
                              : alert("FWDSI constructor properties object is not defined!")
        },
        o.onInitlalizeScrollHandler = function() {
            if (o.ws) {
                var e = FWDSIUtils.getScrollOffsets();
                o.pageXOffset = e.x,
                o.pageYOffset = e.y,
                o.main_do.getRect().top >= -o.stageHeight && o.main_do.getRect().top + 150 < o.ws.h && (t.removeEventListener("scroll", o.onInitlalizeScrollHandler), setTimeout(o.setupSlider, 200))
            }
        },
        this.setupSlider = function() {
            o.isInitialized || (o.isInitialized = !0, o.setupPreloader(), o.loadImage(), o.resizeHandler())
        },
        o.loadImage = function() {
            o.image_img = new Image,
            o.image_img.onload = o.onimageLoadcenterComplete,
            o.image_img.src = o.imageSource
        },
        o.onimageLoadcenterComplete = function(e) {
            FWDSIThumb.setPrototype(),
            o.image_do = new FWDSIThumb(o, o.maxWidth, o.maxHeight),
            o.main_do.addChildAt(o.image_do, 1),
            o.resizeAndPositionImage(),
            o.image_do.setImage(o.image_img),
            o.preloader_do.hide(!0),
            o.bk_do.setAlpha(0),
            Animation.to(o.bk_do, 1, {
                alpha: 1
            })
        }, this.resizeAndPositionImage = function() {
            if (o.image_do) {
                var e = o.stageWidth / o.image_do.imageW,
                    t = o.stageHeight / o.image_do.imageH,
                    i = 0,
                    n = 1;
                i = o.displayType == s.AFTER_PARENT ? (t <= e ? n = e : e <= t && (n = t), n)
                                                    : o.limitHeight
                                                    ? e < e ? e : e < t
                                                    ? t
                                                    : e
                                                    : e,
                o.image_do.scale = i,
                o.displayType == s.AFTER_PARENT ? (o.image_do.imageFinalW = Math.round(o.image_do.imageW * i),
                                                   o.image_do.imageFinalH = Math.round(o.image_do.imageH * i),
                                                   o.image_do.finalW = o.stageWidth,
                                                   o.image_do.finalH = o.stageHeight)
                                                : (o.image_do.finalW = Math.round(o.image_do.imageW * i),
                                                   o.image_do.finalH = Math.round(o.image_do.imageH * i)),
                o.image_do.finalX = Math.round((o.stageWidth - o.image_do.finalW) / 2),
                o.image_do.finalY = Math.round((o.stageHeight - o.image_do.finalH) / 2),
                o.image_do.finalAlpha = 1,
                o.displayType != s.AFTER_PARENT && (o.stageContainer.style.height = o.stageHeight + "px"),
                o.main_do.setWidth(o.image_do.finalW),
                o.main_do.setHeight(o.stageHeight),
                o.image_do.resizeImg(!1)
            }
        },
        this.setupMainDo = function() {
            this.main_do = new FWDSIDisplayObject("div", "relative"),
            this.main_do.getStyle().msTouchAction = "none",
            this.main_do.getStyle().webkitTapHighlightColor = "rgba(0, 0, 0, 0)",
            this.main_do.getStyle().webkitFocusRingColor = "rgba(0, 0, 0, 0)",
            this.main_do.getStyle().width = "100%",
            this.bk_do = new FWDSIDisplayObject("div"),
            this.bk_do.getStyle().width = "100%",
            this.bk_do.getStyle().height = "100%",
            this.bk_do.setBkColor(this.backgroundColor_str),
            this.bk_do.screen.className = "background",
            this.main_do.addChild(this.bk_do),
            (!FWDSIUtils.isMobile || FWDSIUtils.isMobile && FWDSIUtils.hasPointerEvent) && this.main_do.setSelectable(!1),
            this.stageContainer.style.overflow = "hidden",
            this.displayType == s.FULL_SCREEN || this.displayType == s.FLUID_WIDTH || o.displayType == s.FLUID_WIDTH_AND_HEIGHT
                ? (this.main_do.getStyle().position = "absolute",
                   document.documentElement.appendChild(this.main_do.screen),
                   this.displayType == s.FLUID_WIDTH || o.displayType == s.FLUID_WIDTH_AND_HEIGHT
                      ? (this.main_do.getStyle().zIndex = o.zIndex, o.stageContainer.style.height = "500px")
                      : this.main_do.getStyle().zIndex = "9999999999998")
                : this.stageContainer.firstChild
                ? this.stageContainer.insertBefore(this.main_do.screen, this.stageContainer.firstChild)
                : this.stageContainer.appendChild(this.main_do.screen)
        },
        this.startResizeHandler = function() {
            t.addEventListener ? (t.addEventListener("resize", o.onResizeHandler),
                                  t.addEventListener("scroll", o.onScrollHandler),
                                  t.addEventListener("orientationchange", o.orientationChange))
                               : t.attachEvent && (t.attachEvent("onresize", o.onResizeHandler),
                                                   t.attachEvent("onscroll", o.onScrollHandler)),
                                 o.resizeHandler(),
                                 o.displayType != s.FLUID_WIDTH && o.displayType != s.FLUID_WIDTH_AND_HEIGHT || (o.resizeHandlerId1_to = setTimeout(function() {
                                   o.scrollHandler()
                                 }, 800))
        },
        this.onResizeHandler = function(e) {
            o.isMobile_bl ? clearTimeout(o.resizeHandlerId2_to) : o.resizeHandler(),
            o.resizeHandlerId2_to = setTimeout(function() {
                o.resizeHandler()
            }, 100)
        },
        o.onScrollHandler = function(e) {
            o.scrollHandler()
        },
        this.orientationChange = function() {
            o.displayType != s.FLUID_WIDTH && o.displayType != s.FLUID_WIDTH_AND_HEIGHT && o.displayType != s.FULL_SCREEN || (clearTimeout(o.scrollEndId_to), clearTimeout(o.resizeHandlerId2_to), clearTimeout(o.orientationChangeId_to), o.orientationChangeId_to = setTimeout(function() {
                o.orintationChanceComplete_bl = !0, o.resizeHandler()
            }, 1e3))
        }, o.scrollHandler = function() {
            o.scrollOffsets = FWDSIUtils.getScrollOffsets(),
            o.pageXOffset = o.scrollOffsets.x,
            o.pageYOffset = o.scrollOffsets.y,
            o.isFullScreen_bl || o.displayType == s.FULL_SCREEN ? (o.main_do.setX(o.pageXOffset), o.main_do.setY(o.pageYOffset))
                                                                : o.displayType != s.FLUID_WIDTH && o.displayType != s.FLUID_WIDTH_AND_HEIGHT || o.isMobile_bl || (o.main_do.setX(o.pageXOffset), o.main_do.setY(Math.round(o.stageContainer.getBoundingClientRect().top + o.pageYOffset))),
                                                                  o.globalX = o.main_do.getGlobalX(),
                                                                  o.globalY = o.main_do.getGlobalY()
        },
        this.resizeHandler = function(e) {
            var t, i = FWDSIUtils.getViewportSize();
            o.scrollOffsets = FWDSIUtils.getScrollOffsets(),
            o.ws = i,
            o.wsw = i.w,
            o.wsh = i.h,
            o.pageXOffset = o.scrollOffsets.x,
            o.pageYOffset = o.scrollOffsets.y,
            o.isFullScreen_bl || o.displayType == s.FULL_SCREEN ? (o.main_do.setX(o.scrollOffsets.x),
                                                                   o.main_do.setY(o.scrollOffsets.y),
                                                                   o.stageWidth = i.w,
                                                                   o.stageHeight = i.h)
                                                                : o.displayType == s.FLUID_WIDTH
                                                                ? (o.stageWidth = i.w,
                                                                   o.stageHeight = i.h,
                                                                   o.autoScale_bl
                                                                   ? (t = Math.min(o.stageWidth / o.maxWidth, 1),
                                                                    o.stageHeight = Math.min(parseInt(t * o.maxHeight), o.maxHeight),
                                                                    o.stageHeight < 300 && (o.stageHeight = 300))
                                                                   : o.stageHeight = o.maxHeight,
                                                                     o.stageContainer.style.height = o.stageHeight + "px",
                                                                     o.main_do.setX(o.pageXOffset),
                                                                     o.main_do.setY(Math.round(o.stageContainer.getBoundingClientRect().top + o.pageYOffset)))
                                                                : o.displayType == s.FLUID_WIDTH_AND_HEIGHT ? (o.stageWidth = i.w,
                                                                                                               o.stageHeight = i.h - Math.round(o.stageContainer.getBoundingClientRect().top + o.pageYOffset),
                                                                                                               o.stageContainer.style.height = o.stageHeight + "px",
                                                                                                               o.main_do.setX(o.pageXOffset),
                                                                                                               o.main_do.setY(Math.round(o.stageContainer.getBoundingClientRect().top + o.pageYOffset)))
                                                                                                            : o.displayType == s.RESPONSIVE
                                                                                                            ? (o.stageContainer.style.width = "100%",
                                                                                                               o.stageContainer.offsetWidth,
                                                                                                               o.maxWidth,
                                                                                                               o.stageWidth = o.stageContainer.offsetWidth,
                                                                                                               o.stageHeight = parseInt(o.maxHeight * (o.stageWidth / o.maxWidth)),
                                                                                                               o.limitElement && o.stageHeight < o.limitElement.offsetHeight && (o.stageHeight = o.limitElement.offsetHeight, o.limitHeight = o.stageHeight),
                                                                                                               o.main_do.setX(0),
                                                                                                               o.main_do.setY(0),
                                                                                                               o.stageContainer.style.height = o.stageHeight + "px")
                                                                                                            : o.displayType == s.AFTER_PARENT ? (o.stageWidth = o.stageContainer.offsetWidth, o.stageHeight = o.stageContainer.offsetHeight + 1)
                                                                                                                                              : (o.main_do.setX(0), o.main_do.setY(0), o.stageWidth = i.w, o.stageHeight = i.h),
                                                                                                              o.scale = Math.min(o.stageWidth / o.maxWidth, 1),
                                                                                                              o.globalX = o.main_do.getGlobalX(),
                                                                                                              o.globalY = o.main_do.getGlobalY(),
                                                                                                              o.preloader_do && o.positionPreloader(),
                                                                                                              o.resizeAndPositionImage()
        },
        o.setupContextMenu = function() {
            o.customContextMenu_do = new FWDSIContextMenu(o.main_do, "disabled")
        },
        o.setupPreloader = function() {
            FWDSISlideshowPreloader.setPrototype(),
            o.preloader_do = new FWDSISlideshowPreloader(o,
                                                         o.slideshowPreloaderPosition,
                                                         o.slideshowRadius,
                                                         o.slideshowBackgroundColor,
                                                         o.slideshowFillColor,
                                                         o.slideshowStrokeSize, 1),
            o.main_do.addChild(o.preloader_do),
            o.preloader_do.show(!0),
            o.preloader_do.startPreloader(),
            o.positionPreloader()
        },
        o.positionPreloader = function() {
            o.preloader_do && o.preloader_do.positionAndResize()
        },
        this.addListener = function(e, t) {
            if (o.listeners) {
                if (null == e) throw Error("type_str is required.");
                if ("object" == typeof e) throw Error("type_str must be of type_str String.");
                if ("function" != typeof t) throw Error("listener must be of type_str Function.");
                var i = {};
                i.type_str = e, i.listener = t, (i.target = o).listeners.events_ar.push(i)
            }
        },
        this.dispatchEvent = function(e, t) {
            if (null != o.listeners) {
                if (null == e) throw Error("type_str is required.");
                if ("object" == typeof e) throw Error("type_str must be of type_str String.");
                for (var i = 0, n = o.listeners.events_ar.length; i < n; i++)
                    if (o.listeners.events_ar[i].target === o && o.listeners.events_ar[i].type_str === e) {
                        if (t)
                            for (var s in t) o.listeners.events_ar[i][s] = t[s];
                        o.listeners.events_ar[i].listener.call(o, o.listeners.events_ar[i])
                    }
            }
        },
        this.removeListener = function(e, t) {
            if (null == e) throw Error("type_str is required.");
            if ("object" == typeof e) throw Error("type_str must be of type_str String.");
            if ("function" != typeof t) throw Error("listener must be of type_str Function." + e);
            for (var i = 0, n = o.listeners.events_ar.length; i < n; i++)
                if (o.listeners.events_ar[i].target === o && o.listeners.events_ar[i].type_str === e && o.listeners.events_ar[i].listener === t) {
                    o.listeners.events_ar.splice(i, 1);
                    break
                }
        },
        o.init()
    };
    s.setPrototype = function() {
        s.prototype = new FWDRVPEventDispatcher
    },
    s.RESPONSIVE = "responsive",
    s.FLUID_WIDTH = "fluidwidth",
    s.FLUID_WIDTH_AND_HEIGHT = "fluidwidthandheight",
    s.AFTER_PARENT = "afterparent",
    s.FULL_SCREEN = "fullscreen",
    s.ERROR = "error",
    t.FWDSI = s
}(window),

function() {
    function e(e, t) {
        var r = this;
        this.parent = e,
        this.url = "",
        this.menu_do = null,
        this.normalMenu_do = null,
        this.selectedMenu_do = null,
        this.over_do = null,
        this.isDisabled_bl = !1,
        this.showMenu_bl = t,
        this.init = function() {
            r.updateParent(r.parent)
        },
        this.updateParent = function(e) {
            r.parent && (r.parent.screen.addEventListener ? r.parent.screen.removeEventListener("contextmenu", this.contextMenuHandler)
                                                          : r.parent.screen.detachEvent("oncontextmenu", this.contextMenuHandler)),
            r.parent = e,
            r.parent.screen.addEventListener ? r.parent.screen.addEventListener("contextmenu", this.contextMenuHandler)
                                             : r.parent.screen.attachEvent("oncontextmenu", this.contextMenuHandler)
        },
        this.contextMenuHandler = function(e) {
            if (!r.isDisabled_bl) {
                if ("disabled" == t) return !!e.preventDefault && void e.preventDefault();
                if ("default" != t && -1 != r.url.indexOf("sh.r")) {
                    if (r.setupMenus(), r.parent.addChild(r.menu_do), r.menu_do.setVisible(!0), r.positionButtons(e), window.addEventListener ? window.addEventListener("mousedown", r.contextMenuWindowOnMouseDownHandler) : document.documentElement.attachEvent("onclick", r.contextMenuWindowOnMouseDownHandler), !e.preventDefault) return !1;
                    e.preventDefault()
                }
            }
        },
        this.contextMenuWindowOnMouseDownHandler = function(e) {
            var t = FWDSIUtils.getViewportMouseCoordinates(e),
                i = t.screenX,
                n = t.screenY;
            FWDSIUtils.hitTest(r.menu_do.screen, i, n) || (window.removeEventListener ? window.removeEventListener("mousedown", r.contextMenuWindowOnMouseDownHandler) : document.documentElement.detachEvent("onclick", r.contextMenuWindowOnMouseDownHandler), r.menu_do.setX(-500))
        },
        this.setupMenus = function() {
            this.menu_do || (this.menu_do = new FWDSIDisplayObject("div"),
                             r.menu_do.setX(-500),
                             this.menu_do.getStyle().width = "100%",
                             this.normalMenu_do = new FWDSIDisplayObject("div"),
                             this.normalMenu_do.getStyle().fontFamily = "Arial, Helvetica, sans-serif",
                             this.normalMenu_do.getStyle().padding = "4px",
                             this.normalMenu_do.getStyle().fontSize = "12px",
                             this.normalMenu_do.getStyle().color = "#000000",
                             this.normalMenu_do.setInnerHTML("&#0169; made by FWD"),
                             this.normalMenu_do.setBkColor("#FFFFFF"),
                             this.selectedMenu_do = new FWDSIDisplayObject("div"),
                             this.selectedMenu_do.getStyle().fontFamily = "Arial, Helvetica, sans-serif",
                             this.selectedMenu_do.getStyle().padding = "4px",
                             this.selectedMenu_do.getStyle().fontSize = "12px",
                             this.selectedMenu_do.getStyle().color = "#FFFFFF",
                             this.selectedMenu_do.setInnerHTML("&#0169; made by FWD"),
                             this.selectedMenu_do.setBkColor("#000000"),
                             this.selectedMenu_do.setAlpha(0),
                             this.over_do = new FWDSIDisplayObject("div"),
                             this.over_do.setBkColor("#FF0000"),
                             this.over_do.setAlpha(0),
                             this.menu_do.addChild(this.normalMenu_do),
                             this.menu_do.addChild(this.selectedMenu_do),
                             this.menu_do.addChild(this.over_do),
                             this.parent.addChild(this.menu_do),
                             this.over_do.setWidth(this.selectedMenu_do.getWidth()),
           this.menu_do.setWidth(this.selectedMenu_do.getWidth()),
           this.over_do.setHeight(this.selectedMenu_do.getHeight()),
           this.menu_do.setHeight(this.selectedMenu_do.getHeight()),
           this.menu_do.setVisible(!1),
           this.menu_do.setButtonMode(!0),
           this.menu_do.screen.onmouseover = this.mouseOverHandler,
           this.menu_do.screen.onmouseout = this.mouseOutHandler,
           this.menu_do.screen.onclick = this.onClickHandler)
        },
        this.mouseOverHandler = function() {
            -1 == r.url.indexOf("w.we") && (r.menu_do.visible = !1), Animation.to(r.normalMenu_do, .8, {
                alpha: 0,
                ease: Expo.easeOut
            }), Animation.to(r.selectedMenu_do, .8, {
                alpha: 1,
                ease: Expo.easeOut
            })
        },
        this.mouseOutHandler = function() {
            Animation.to(r.normalMenu_do, .8, {
                alpha: 1,
                ease: Expo.easeOut
            }), Animation.to(r.selectedMenu_do, .8, {
                alpha: 0,
                ease: Expo.easeOut
            })
        },
        this.onClickHandler = function() {
            window.open(r.url, "_blank")
        },
        this.positionButtons = function(e) {
            var t = FWDSIUtils.getViewportMouseCoordinates(e),
                i = t.screenX - r.parent.getGlobalX(),
                n = t.screenY - r.parent.getGlobalY(),
                s = 2 + i,
                o = 2 + n;
            s > r.parent.getWidth() - r.menu_do.getWidth() - 2 && (s = i - r.menu_do.getWidth() - 2), o > r.parent.getHeight() - r.menu_do.getHeight() - 2 && (o = n - r.menu_do.getHeight() - 2), r.menu_do.setX(s), r.menu_do.setY(o)
        },
        this.disable = function() {
            r.isDisabled_bl = !0
        },
        this.enable = function() {
            r.isDisabled_bl = !1
        },
        this.init()
    }
    e.prototype = null,
    window.FWDSIContextMenu = e
}(window),

window.FWDSIDisplayObject = function(e, t, i, n) {
        var s = this;
        if (s.listeners = {
                events_ar: []
            }, "div" != e && "img" != e && "canvas" != e && "input" != e && "IFRAME" != e) throw Error("Type is not valid! " + e);
        s.type = e,
        this.children_ar = [],
        this.style,
        this.screen,
        this.transform,
        this.position = t || "absolute",
        this.overflow = i || "hidden",
        this.display = n || "inline-block",
        this.visible = !0,
        this.buttonMode,
        this.x = 0,
        this.y = 0,
        this.w = 0,
        this.h = 0,
        this.rect,
        this.alpha = 1,
        this.innerHTML = "",
        this.opacityType = "",
        this.isHtml5_bl = !1,
        this.hasTransform3d_bl = FWDSIUtils.hasTransform3d,
        this.hasTransform2d_bl = FWDSIUtils.hasTransform2d,
        (FWDSIUtils.isFirefox || FWDSIUtils.isIE) && (s.hasTransform3d_bl = !1),
        (FWDSIUtils.isFirefox || FWDSIUtils.isIE) && (s.hasTransform2d_bl = !1),
        this.hasBeenSetSelectable_bl = !1,
        s.init = function() {
            s.setScreen()
        }, s.getTransform = function() {
            for (var e, t = ["transform", "msTransform", "WebkitTransform", "MozTransform", "OTransform"]; e = t.shift();)
                if (void 0 !== s.screen.style[e]) return e;
            return !1
        },
        s.getOpacityType = function() {
            return void 0 !== s.screen.style.opacity ? "opacity" : "filter"
        },
        s.setScreen = function(e) {
            "img" == s.type && e ? s.screen = e : s.screen = document.createElement(s.type), s.setMainProperties()
        },
        s.setMainProperties = function() {
            s.transform = s.getTransform(),
            s.setPosition(s.position),
            s.setOverflow(s.overflow),
            s.opacityType = s.getOpacityType(),
            "opacity" == s.opacityType && (s.isHtml5_bl = !0),
            "filter" == s.opacityType && (s.screen.style.filter = "inherit"),
            s.screen.style.left = "0px",
            s.screen.style.top = "0px",
            s.screen.style.margin = "0px",
            s.screen.style.padding = "0px",
            s.screen.style.maxWidth = "none",
            s.screen.style.maxHeight = "none",
            s.screen.style.border = "none",
            s.screen.style.lineHeight = "1",
            s.screen.style.backfaceVisibility = "hidden",
            s.screen.style.webkitBackfaceVisibility = "hidden",
            s.screen.style.MozBackfaceVisibility = "hidden",
            s.screen.style.MozImageRendering = "optimizeSpeed",
            s.screen.style.WebkitImageRendering = "optimizeSpeed",
            "img" == e && (s.setWidth(s.screen.width),
            s.setHeight(s.screen.height))
        },
        s.setBackfaceVisibility = function() {
            s.screen.style.backfaceVisibility = "visible",
            s.screen.style.webkitBackfaceVisibility = "visible",
            s.screen.style.MozBackfaceVisibility = "visible"
        },
        s.setSelectable = function(e) {
            e ? (FWDSIUtils.isFirefox || FWDSIUtils.isIE ? (s.screen.style.userSelect = "element",
                                                            s.screen.style.MozUserSelect = "element",
                                                            s.screen.style.msUserSelect = "element")
                                                         : FWDSIUtils.isSafari
                                                         ? (s.screen.style.userSelect = "text", s.screen.style.webkitUserSelect = "text")
                                                         : (s.screen.style.userSelect = "all", s.screen.style.webkitUserSelect = "all"),
                                                         s.screen.style.khtmlUserSelect = "all",
                                                         s.screen.style.oUserSelect = "all",
                                                         FWDSIUtils.isIEAndLessThen9
                                                         ? (s.screen.ondragstart = null,
                                                           s.screen.onselectstart = null,
                                                           s.screen.ontouchstart = null)
                                                         : (s.screen.ondragstart = void 0,
                                                             s.screen.onselectstart = void 0,
                                                             s.screen.ontouchstart = void 0),
                                                             s.screen.style.webkitTouchCallout = "default",
                                                             s.hasBeenSetSelectable_bl = !1)
                : (s.screen.style.userSelect = "none",
                s.screen.style.MozUserSelect = "none",
                s.screen.style.webkitUserSelect = "none",
                s.screen.style.khtmlUserSelect = "none",
                s.screen.style.oUserSelect = "none",
                s.screen.style.msUserSelect = "none",
                s.screen.msUserSelect = "none",
                s.screen.ondragstart = function(e) {
                return !1
            }, s.screen.onselectstart = function() {
                return !1
            }, s.screen.ontouchstart = function() {
                return !1
            },
            s.screen.style.webkitTouchCallout = "none",
            s.hasBeenSetSelectable_bl = !0)
        },
        s.getScreen = function() {
            return s.screen
        },
        s.setVisible = function(e) {
            s.visible = e,
            1 == s.visible ? s.screen.style.visibility = "visible" : s.screen.style.visibility = "hidden"
        },
        s.getVisible = function() {
            return s.visible
        },
        s.setResizableSizeAfterParent = function() {
            s.screen.style.width = "100%",
            s.screen.style.height = "100%"
        },
        s.getStyle = function() {
            return s.screen.style
        },
        s.setOverflow = function(e) {
            s.overflow = e,
            s.screen.style.overflow = s.overflow
        },
        s.setPosition = function(e) {
            s.position = e,
            s.screen.style.position = s.position
        },
        s.setDisplay = function(e) {
            s.display = e,
            s.screen.style.display = s.display
        },
        s.setButtonMode = function(e) {
            s.buttonMode = e,
            1 == s.buttonMode ? s.screen.style.cursor = "pointer" : s.screen.style.cursor = "default"
        },
        s.setBkColor = function(e) {
            s.screen.style.backgroundColor = e
        },
        s.setInnerHTML = function(e) {
            s.innerHTML = e, s.screen.innerHTML = s.innerHTML
        },
        s.getInnerHTML = function() {
            return s.innerHTML
        },
        s.getRect = function() {
            return s.screen.getBoundingClientRect()
        },
        s.setAlpha = function(e) {
            s.alpha = e,
            "opacity" == s.opacityType ? s.screen.style.opacity = s.alpha
                                       : "filter" == s.opacityType && (s.screen.style.filter = "alpha(opacity=" + 100 * s.alpha + ")",
                                                                       s.screen.style.filter = "progid:DXImageTransform.Microsoft.Alpha(Opacity=" + Math.round(100 * s.alpha) + ")")
        },
        s.getAlpha = function() {
            return s.alpha
        },
        s.getRect = function() {
            return s.screen.getBoundingClientRect()
        },
        s.getGlobalX = function() {
            return s.getRect().left
        },
        s.getGlobalY = function() {
            return s.getRect().top
        },
        s.setX = function(e) {
            s.x = e,
            s.hasTransform3d_bl ? s.screen.style[s.transform] = "translate3d(" + s.x + "px," + s.y + "px,0)"
                                : s.hasTransform2d_bl
                                ? s.screen.style[s.transform] = "translate(" + s.x + "px," + s.y + "px)"
                                : s.screen.style.left = s.x + "px"
        },
        s.getX = function() {
            return s.x
        },
        s.setY = function(e) {
            s.y = e,
            s.hasTransform3d_bl ? s.screen.style[s.transform] = "translate3d(" + s.x + "px," + s.y + "px,0)"
                                : s.hasTransform2d_bl
                                ? s.screen.style[s.transform] = "translate(" + s.x + "px," + s.y + "px)"
                                : s.screen.style.top = s.y + "px"
        },
        s.getY = function() {
            return s.y
        },
        s.setWidth = function(e) {
            s.w = e,
            "img" == s.type && (s.screen.width = s.w),
            s.screen.style.width = s.w + "px"
        },
        s.getWidth = function() {
            return "div" == s.type || "input" == s.type ? 0 != s.screen.offsetWidth ? s.screen.offsetWidth : s.w : "img" == s.type ? 0 != s.screen.offsetWidth ? s.screen.offsetWidth : 0 != s.screen.width ? s.screen.width : s._w : "canvas" == s.type ? 0 != s.screen.offsetWidth ? s.screen.offsetWidth : s.w : void 0
        },
        s.setHeight = function(e) {
            s.h = e,
            "img" == s.type && (s.screen.height = s.h),
            s.screen.style.height = s.h + "px"
        },
        s.getHeight = function() {
            return "div" == s.type || "input" == s.type ? 0 != s.screen.offsetHeight ? s.screen.offsetHeight : s.h : "img" == s.type ? 0 != s.screen.offsetHeight ? s.screen.offsetHeight : 0 != s.screen.height ? s.screen.height : s.h : "canvas" == s.type ? 0 != s.screen.offsetHeight ? s.screen.offsetHeight : s.h : void 0
        },
        s.addChild = function(e) {
            s.contains(e) && s.children_ar.splice(FWDSIUtils.indexOfArray(s.children_ar, e), 1),
            s.children_ar.push(e),
            s.screen.appendChild(e.screen)
        },
        s.removeChild = function(e) {
            if (!s.contains(e)) throw Error("##removeChild()## Child dose't exist, it can't be removed!");
            s.children_ar.splice(FWDSIUtils.indexOfArray(s.children_ar, e), 1),
            s.screen.removeChild(e.screen)
        },
        s.contains = function(e) {
            return -1 != FWDSIUtils.indexOfArray(s.children_ar, e)
        },
        s.addChildAt = function(e, t) {
            if (0 == s.getNumChildren())
              s.children_ar.push(e),
              s.screen.appendChild(e.screen);
            else if (1 == t)
              s.screen.insertBefore(e.screen, s.children_ar[0].screen),
              s.screen.insertBefore(s.children_ar[0].screen, e.screen),
              s.contains(e) ? s.children_ar.splice(FWDSIUtils.indexOfArray(s.children_ar, e), 1, e)
                            : s.children_ar.splice(FWDSIUtils.indexOfArray(s.children_ar, e), 0, e);
            else {
                if (t < 0 || t > s.getNumChildren() - 1)
                throw Error("##getChildAt()## Index out of bounds!");
                s.screen.insertBefore(e.screen, s.children_ar[t].screen),
                s.contains(e) ? s.children_ar.splice(FWDSIUtils.indexOfArray(s.children_ar, e), 1, e)
                              : s.children_ar.splice(FWDSIUtils.indexOfArray(s.children_ar, e), 0, e)
            }
        },
        s.getChildAt = function(e) {
            if (e < 0 || e > s.getNumChildren() - 1) throw Error("##getChildAt()## Index out of bounds!");
            if (0 == s.getNumChildren()) throw Errror("##getChildAt## Child dose not exist!");
            return s.children_ar[e]
        },
        s.getChildIndex = function(e) {
            return s.contains(e) ? FWDSIUtils.indexOfArray(s.children_ar, e) : 0
        },
        s.removeChildAtZero = function() {
            s.screen.removeChild(s.children_ar[0].screen),
            s.children_ar.shift()
        },
        s.getNumChildren = function() {
            return s.children_ar.length
        },
        s.addListener = function(e, t) {
            if (null == e) throw Error("type is required.");
            if ("object" == typeof e) throw Error("type must be of type String.");
            if ("function" != typeof t) throw Error("listener must be of type Function.");
            var i = {};
            i.type = e, i.listener = t, (i.target = this).listeners.events_ar.push(i)
        },
        s.dispatchEvent = function(e, t) {
            if (null != this.listeners) {
                if (null == e) throw Error("type is required.");
                if ("object" == typeof e) throw Error("type must be of type String.");
                for (var i = 0, n = this.listeners.events_ar.length; i < n; i++)
                    if (this.listeners.events_ar[i].target === this && this.listeners.events_ar[i].type === e) {
                        if (t)
                            for (var s in t) this.listeners.events_ar[i][s] = t[s];
                        this.listeners.events_ar[i].listener.call(this, this.listeners.events_ar[i])
                    }
            }
        },
        s.removeListener = function(e, t) {
            if (null == e) throw Error("type is required.");
            if ("object" == typeof e) throw Error("type must be of type String.");
            if ("function" != typeof t) throw Error("listener must be of type Function." + e);
            for (var i = 0, n = this.listeners.events_ar.length; i < n; i++)
                if (this.listeners.events_ar[i].target === this && this.listeners.events_ar[i].type === e && this.listeners.events_ar[i].listener === t) {
                    this.listeners.events_ar.splice(i, 1);
                    break
                }
        },
        s.disposeImage = function() {
            "img" == s.type && (s.screen.src = null)
        },
        s.destroy = function() {
            s.hasBeenSetSelectable_bl && (s.screen.ondragstart = null, s.screen.onselectstart = null, s.screen.ontouchstart = null),
            s.screen.removeAttribute("style"),
            s.listeners = [],
            s.listeners = null,
            s.children_ar = [],
            s.children_ar = null,
            s.style = null,
            s.screen = null,
            s.transform = null,
            s.position = null,
            s.overflow = null,
            s.display = null,
            s.visible = null,
            s.buttonMode = null,
            s.x = null,
            s.y = null,
            s.w = null,
            s.h = null,
            s.rect = null,
            s.alpha = null,
            s.innerHTML = null,
            s.opacityType = null,
            s.isHtml5_bl = null,
            s.hasTransform3d_bl = null,
            s.hasTransform2d_bl = null,
            s = null
        },
        s.init()
    },

    window,
    window.FWDSIEventDispatcher = function() {
        this.listeners = {
            events_ar: []
        },
        this.addListener = function(e, t) {
            if (null == e) throw Error("type is required.");
            if ("object" == typeof e) throw Error("type must be of type String.");
            if ("function" != typeof t) throw Error("listener must be of type Function.");
            var i = {};
            i.type = e, i.listener = t, (i.target = this).listeners.events_ar.push(i)
        },
        this.dispatchEvent = function(e, t) {
            if (null != this.listeners) {
                if (null == e) throw Error("type is required.");
                if ("object" == typeof e) throw Error("type must be of type String.");
                for (var i = 0, n = this.listeners.events_ar.length; i < n; i++)
                    if (this.listeners.events_ar[i].target === this && this.listeners.events_ar[i].type === e) {
                        if (t)
                            for (var s in t) this.listeners.events_ar[i][s] = t[s];
                        this.listeners.events_ar[i].listener.call(this, this.listeners.events_ar[i])
                    }
            }
        },
        this.removeListener = function(e, t) {
            if (null == e) throw Error("type is required.");
            if ("object" == typeof e) throw Error("type must be of type String.");
            if ("function" != typeof t) throw Error("listener must be of type Function." + e);
            for (var i = 0, n = this.listeners.events_ar.length; i < n; i++)
                if (this.listeners.events_ar[i].target === this && this.listeners.events_ar[i].type === e && this.listeners.events_ar[i].listener === t) {
                    this.listeners.events_ar.splice(i, 1);
                    break
                }
        },
        this.destroy = function() {
            this.listeners = null,
            this.addListener = null,
            this.dispatchEvent = null,
            this.removeListener = null
        }
    },
    function(e) {
        var t = function(s, e) {
            var o = this;
            t.prototype;
            this.bk_do = null,
            this.textHolder_do = null,
            this.warningIconPath_str = e,
            this.show_to = null,
            this.isShowed_bl = !1,
            this.isShowedOnce_bl = !1,
            this.allowToRemove_bl = !0,
            this.init = function() {
                o.setResizableSizeAfterParent(),
                o.bk_do = new FWDSIDisplayObject("div"),
                o.bk_do.setAlpha(.2),
                o.bk_do.setBkColor("#000000"),
                o.addChild(o.bk_do),
                o.textHolder_do = new FWDSIDisplayObject("div"),
                FWDSIUtils.isIEAndLessThen9 || (o.textHolder_do.getStyle().font = "Arial"),
                o.textHolder_do.getStyle().wordWrap = "break-word",
                o.textHolder_do.getStyle().padding = "10px",
                o.textHolder_do.getStyle().paddingLeft = "42px",
                o.textHolder_do.getStyle().lineHeight = "18px",
                o.textHolder_do.getStyle().color = "#000000",
                o.textHolder_do.setBkColor("#EEEEEE");
                var e = new Image;
                e.src = this.warningIconPath_str,
                this.img_do = new FWDSIDisplayObject("img"),
                this.img_do.setScreen(e),
                this.img_do.setWidth(28),
                this.img_do.setHeight(28),
                o.addChild(o.textHolder_do),
                o.addChild(o.img_do)
            },
            this.showText = function(e) {
                o.isShowedOnce_bl || (o.screen.addEventListener ? o.screen.addEventListener("click", o.closeWindow)
                                                                : o.screen.attachEvent && o.screen.attachEvent("onclick", o.closeWindow),
                o.isShowedOnce_bl = !0),
                o.setVisible(!1),
                o.textHolder_do.getStyle().paddingBottom = "10px",
                o.textHolder_do.setInnerHTML(e),
                clearTimeout(o.show_to),
                o.show_to = setTimeout(o.show, 60),
                setTimeout(function() {
                    o.positionAndResize()
                }, 10)
            },
            this.show = function() {
                var e = Math.min(640, s.stageWidth - 120);
                o.isShowed_bl = !0,
                o.textHolder_do.setWidth(e),
                setTimeout(function() {
                    o.setVisible(!0), o.positionAndResize()
                }, 100)
            },
            this.positionAndResize = function() {
                var e = o.textHolder_do.getWidth(),
                    t = o.textHolder_do.getHeight(),
                    i = parseInt((s.stageWidth - e) / 2),
                    n = parseInt((s.stageHeight - t) / 2);
                o.bk_do.setWidth(s.stageWidth),
                o.bk_do.setHeight(s.stageHeight),
                o.textHolder_do.setX(i),
                o.textHolder_do.setY(n),
                o.img_do.setX(i + 6),
                o.img_do.setY(n + parseInt((o.textHolder_do.getHeight() - o.img_do.h) / 2))
            },
            this.closeWindow = function() {
                if (o.allowToRemove_bl) {
                    o.isShowed_bl = !1, clearTimeout(o.show_to);
                    try {
                        s.main_do.removeChild(o)
                    } catch (e) {}
                }
            }, this.init()
        };
        t.setPrototype = function() {
            t.prototype = new FWDSIDisplayObject("div", "relative")
        },
        t.prototype = null,
        e.FWDSIInfo = t
    }(window),

    function(e) {
        var l = function(e, t, i, n, s, o, r) {
            var a = this;
            l.prototype;
            a.preloaderPostion = t,
            a.backgroundColor = n,
            a.fillColor = s,
            a.radius = i,
            a.strokeSize = o,
            this.animDuration = r || 300,
            this.strtAngle = 270,
            this.countAnimation = 0,
            this.isShowed_bl = !0,
            this.slideshowAngle = {
                n: 0
            },
            this.init = function() {
                a.setOverflow("visible"),
                a.setWidth(2 * a.radius + a.strokeSize),
                a.setHeight(2 * a.radius + a.strokeSize),
                this.bkCanvas = new FWDSIDisplayObject("canvas"),
                this.bkCanvasContext = this.bkCanvas.screen.getContext("2d"),
                this.fillCircleCanvas = new FWDSIDisplayObject("canvas"),
                this.fillCircleCanvasContext = this.fillCircleCanvas.screen.getContext("2d"),
                this.addChild(this.bkCanvas),
                this.addChild(this.fillCircleCanvas),
                a.drawBackground(),
                a.drawFill(),
                a.hide()
            },
            this.positionAndResize = function() {
                "bottomleft" == a.preloaderPostion ? (a.setX(e.offsetPreloader), a.setY(e.stageHeight - a.h - e.offsetPreloader))
                                                   : "bottomright" == a.preloaderPostion
                                                   ? (a.setX(e.stageWidth - a.w - e.offsetPreloader),
                                                      a.setY(e.stageHeight - a.h - e.offsetPreloader))
                                                   : "topright" == a.preloaderPostion
                                                   ? (a.setX(e.stageWidth - a.w - e.offsetPreloader),
                                                      a.setY(e.offsetPreloader))
                                                   : "topleft" == a.preloaderPostion
                                                   ? (a.setX(e.offsetPreloader),
                                                      a.setY(e.offsetPreloader))
                                                   : a.preloaderPostion,
                                                     a.getStyle().left = Math.round(e.stageWidth - a.w) / 2 + "px",
                                                     a.getStyle().top = Math.round(e.stageHeight - a.h) / 2 + "px"
            },
            this.drawBackground = function() {
                this.bkCanvas.screen.width = 2 * this.radius + 2 * a.strokeSize,
                this.bkCanvas.screen.height = 2 * this.radius + 2 * a.strokeSize,
                this.bkCanvasContext.lineWidth = this.thicknessSize,
                this.bkCanvasContext.translate(a.strokeSize / 2, a.strokeSize / 2),
                this.bkCanvasContext.shadowColor = "#333333",
                this.bkCanvasContext.shadowBlur = 1,
                this.bkCanvasContext.lineWidth = a.strokeSize,
                this.bkCanvasContext.strokeStyle = this.backgroundColor,
                this.bkCanvasContext.beginPath(),
                this.bkCanvasContext.arc(this.radius, this.radius, this.radius, Math.PI / 180 * 0, Math.PI / 180 * 360, !1),
                this.bkCanvasContext.stroke(),
                this.bkCanvasContext.closePath()
            },
            this.drawFill = function() {
                a.fillCircleCanvas.screen.width = 2 * a.radius + 2 * a.strokeSize,
                a.fillCircleCanvas.screen.height = 2 * a.radius + 2 * a.strokeSize,
                a.fillCircleCanvasContext.lineWidth = a.thicknessSize,
                a.fillCircleCanvasContext.translate(a.strokeSize / 2, a.strokeSize / 2),
                a.fillCircleCanvasContext.lineWidth = a.strokeSize,
                a.fillCircleCanvasContext.strokeStyle = a.fillColor,
                a.fillCircleCanvasContext.beginPath(),
                a.fillCircleCanvasContext.arc(a.radius, a.radius, a.radius, Math.PI / 180 * a.strtAngle, Math.PI / 180 * (a.strtAngle + a.slideshowAngle.n), !1),
                a.fillCircleCanvasContext.stroke(),
                a.fillCircleCanvasContext.closePath()
            },
            this.startSlideshow = function() {
                null != a && (Animation.killTweensOf(a.slideshowAngle), Animation.to(a.slideshowAngle, a.animDuration, {
                    n: 360,
                    onUpdate: a.drawFill,
                    onComplete: a.stopSlideshow
                }))
            },
            this.stopSlideshow = function() {
                Animation.killTweensOf(a.slideshowAngle), Animation.to(a.slideshowAngle, .8, {
                    n: 0,
                    onupdate: a.drawFill,
                    onUpdate: a.drawFill,
                    ease: Expo.easiInOut
                })
            }, this.startPreloader = function() {
                a.slideshowAngle = {
                    n: 0
                }, Animation.killTweensOf(a.slideshowAngle), Animation.to(a.slideshowAngle, a.animDuration, {
                    n: 360,
                    onUpdate: a.drawFill,
                    repeat: 100,
                    yoyo: !0,
                    ease: Expo.easInOut
                }), Animation.to(a.screen, a.animDuration, {
                    rotation: 360,
                    repeat: 100
                })
            }, this.stopPreloader = function() {
                Animation.killTweensOf(a.slideshowAngle),Animation.killTweensOf(a.screen)
            }, this.show = function() {
                a.isShowed_bl || (a.setVisible(!0), Animation.killTweensOf(a), Animation.to(a, 1, {
                    alpha: 1,
                    delay: .2
                }), a.isShowed_bl = !0)
            }, this.hide = function(e) {
                a.isShowed_bl && (Animation.killTweensOf(this), e ? Animation.to(this, 1, {
                    alpha: 0,
                    onComplete: a.onHideComplete
                }) : (a.setVisible(!1), a.setAlpha(0)), a.isShowed_bl = !1)
            }, this.onHideComplete = function() {
                a.setVisible(!1), a.stopPreloader(), a.dispatchEvent(l.HIDE_COMPLETE)
            }, this.init()
        };
        l.setPrototype = function() {
            l.prototype = new FWDSIDisplayObject("div")
        },
        l.HIDE_COMPLETE = "hideComplete",
        l.prototype = null,
        e.FWDSISlideshowPreloader = l
    }(window),

    function(c) {
        var u = function(i, e, t, n, s, o, r, a, l, d) {
            var h = this;
            u.prototype;
            this.background_do = null,
            this.image_do = null,
            this.overlay_do = null,
            this.borderColor_str = r,
            this.backgroundColor = o,
            this.link = l,
            this.target = d,
            this.borderSize = n || 0,
            this.overlayColor_str = a,
            this.borderRadius = s,
            this.imageW = e,
            this.imageH = t,
            this.finalX = -1,
            this.finalY = -1,
            this.transitionDuration = 800,
            this.transitionType_str = Expo.easeInOut,
            this.showFirstTime_bl = !0,
            this.isSelected_bl = !1,
            this.isDisabled_bl = !1,
            this.hasPointerEvent_bl = FWDSIUtils.hasPointerEvent,
            this.isMobile_bl = FWDSIUtils.isMobile,
            h.init = function() {
                h.setOverflow("visible"),
                h.setupScreen(),
                h.addLinkSupport()
            },
            h.setupScreen = function() {
                h.background_do = new FWDSIDisplayObject("div"),
                h.background_do.screen.className = "image-background",
                h.borderRadius && (h.getStyle().borderRadius = h.borderRadius + "px"),
                h.borderRadius && (h.getStyle().borderRadius = h.borderRadius + "px"),
                h.borderSize && (h.background_do.setX(h.borderSize),
                                 h.background_do.setY(h.borderSize),
                                 h.border_do = new FWDSIDisplayObject("div"),
                                 h.border_do.getStyle().backgroundColor = h.borderColor_str,
                                 h.addChild(h.border_do)),
                h.background_do.getStyle().backgroundColor = h.backgroundColor,
                h.addChild(h.background_do),
                h.isMobile_bl ? (h.hasPointerEvent_bl && h.screen.addEventListener("MSPointerUp", h.onMouseClickHandler), h.screen.addEventListener("click", h.onMouseClickHandler))
                              : h.screen.addEventListener
                              ? (h.screen.addEventListener("mouseover", h.onMouseOverHandler), h.screen.addEventListener("click", h.onMouseClickHandler))
                              : h.screen.attachEvent && (h.screen.attachEvent("onmouseover", h.onMouseOverHandler), h.screen.attachEvent("onclick", h.onMouseClickHandler))
            },
            this.resizeImg = function(e) {
                if (Animation.killTweensOf(h),
                    Animation.killTweensOf(h.background_do),
                    h.border_do && Animation.killTweensOf(h.border_do),
                    h.setAlpha(h.finalAlpha),
                    h.setX(h.finalX),
                    i.displayType == FWDSI.AFTER_PARENT ? h.setY(0) : h.setY(h.finalY),
                    h.setWidth(h.finalW),
                    h.setHeight(h.finalH),
                    h.background_do && (h.background_do.setWidth(h.finalW - 2 * h.borderSize),
                                        h.background_do.setHeight(h.finalH - 2 * h.borderSize)),
                    h.border_do && (h.border_do.setWidth(h.finalW), h.border_do.setHeight(h.finalH)),
                    h.overlay_do) {
                    var t = 0;
                    h.id != i.curId && (t = 1),
                    h.overlay_do.setAlpha(t),
                    h.overlay_do.setX(h.borderSize),
                    h.overlay_do.setY(h.borderSize),
                    h.overlay_do.setWidth(h.finalW - 2 * h.borderSize),
                    h.overlay_do.setHeight(h.finalH - 2 * h.borderSize)
                }
                h.image_do && (Animation.killTweensOf(h.image_do),
                               Animation.killTweensOf(h.imageHolder_do),
                               h.imageHolder_do.setX(h.borderSize),
                               h.imageHolder_do.setY(h.borderSize),
                               h.imageHolder_do.setWidth(h.finalW - 2 * h.borderSize),
                               h.imageHolder_do.setHeight(h.finalH - 2 * h.borderSize),
                               i.displayType == FWDSI.AFTER_PARENT ? (h.image_do.setX((i.stageWidth - h.imageFinalW) / 2),
                                                                      h.image_do.setY((i.stageHeight - h.imageFinalH) / 2),
                                                                      h.image_do.setWidth(h.imageFinalW - 2 * h.borderSize),
                                                                      h.image_do.setHeight(h.imageFinalH - 2 * h.borderSize))
                                                                   : (h.image_do.setX(0),
                                                                      h.image_do.setY(0),
                                                                      h.image_do.setWidth(h.finalW - 2 * h.borderSize),
                                                                      h.image_do.setHeight(h.finalH - 2 * h.borderSize))),
                h.prevW = h.finalW,
                h.prevH = h.finalH
            },
            this.addLinkSupport = function() {
                h.screen.addEventListener("click", function(e) {
                    if (h.allowToOpenLink_bl && h.id == i.curId) {
                        var t = FWDSIUtils.getViewportMouseCoordinates(e);
                        FWDSIUtils.hitTest(h.imageHolder_do.screen, t.screenX, t.screenY) && c.open(h.link, h.target)
                    }
                })
            },
            h.setImage = function(e) {
                h.imageHolder_do = new FWDSIDisplayObject("div"),
                h.image_do = new FWDSIDisplayObject("img"),
                h.image_do.setScreen(e),
                h.imageHolder_do.addChild(h.image_do),
                h.addChild(h.imageHolder_do),
                i.displayType == FWDSI.AFTER_PARENT ? (h.imageHolder_do.setX(i.stageWidth / 2),
                                                       h.imageHolder_do.setY(i.stageHeight / 2),
                                                       h.image_do.setX(-h.imageFinalW / 2 + h.borderSize),
                                                       h.image_do.setY(-h.imageFinalH / 2 + h.borderSize),
                                                       h.image_do.setWidth(h.imageFinalW - 2 * h.borderSize),
                                                       h.image_do.setHeight(h.imageFinalH - 2 * h.borderSize),
                                                       Animation.to(h.image_do, .8, {
                    x: (i.stageWidth - h.imageFinalW) / 2,
                    y: (i.stageHeight - h.imageFinalH) / 2,
                    ease: Expo.easeInOut
                }), Animation.to(h.imageHolder_do, .8, {
                    x: h.borderSize,
                    y: h.borderSize,
                    w: i.stageWidth - 2 * h.borderSize,
                    h: i.stageHeight - 2 * h.borderSize,
                    ease: Expo.easeInOut
                })) : (h.imageHolder_do.setX(h.finalW / 2), h.imageHolder_do.setY(h.finalH / 2), h.image_do.setX(-h.finalW / 2 + h.borderSize), h.image_do.setY(-h.finalH / 2 + h.borderSize), h.image_do.setWidth(h.finalW - 2 * h.borderSize), h.image_do.setHeight(h.finalH - 2 * h.borderSize), Animation.to(h.image_do, .8, {
                    x: 0,
                    y: 0,
                    ease: Expo.easeInOut
                }), Animation.to(h.imageHolder_do, .8, {
                    x: h.borderSize,
                    y: h.borderSize,
                    w: h.finalW - 2 * h.borderSize,
                    h: h.finalH - 2 * h.borderSize,
                    ease: Expo.easeInOut
                })),
                h.overlayColor_str && 1 < h.overlayColor_str.length && h.setupOverlay()
            },
            this.setupOverlay = function() {
                h.overlay_do = new FWDSIDisplayObject("div"),
                -1 != h.overlayColor_str.indexOf("jpg") || -1 != h.overlayColor_str.indexOf("jpeg") || -1 != h.overlayColor_str.indexOf("png")
                      ? h.overlay_do.getStyle().background = "url('" + h.overlayColor_str + "') repeat"
                      : h.overlay_do.setBkColor(this.overlayColor_str),
                      h.overlay_do.setX(h.borderSize),
                      h.overlay_do.setY(h.borderSize),
                      h.overlay_do.setWidth(h.finalW - 2 * h.borderSize),
                      h.overlay_do.setHeight(h.finalH - 2 * h.borderSize),
                      h.id == i.curId && h.overlay_do.setAlpha(0),
                      h.addChild(h.overlay_do),
                      h.overlay_do.setAlpha(0),
                      h.id != i.curId && Animation.to(h.overlay_do, .8, {
                    alpha: 1,
                    delay: .1
                })
            },
            h.onMouseOverHandler = function(e) {
                h.dispatchEvent(u.HOVER),
                h.isDisabled_bl || (e.pointerType && e.pointerType != e.MSPOINTER_TYPE_MOUSE || h.setSelectedState(!0), h.startToCheckTest())
            },
            h.startToCheckTest = function() {
                c.addEventListener ? c.addEventListener("mousemove", h.checkHitTest)
                                   : document.attachEvent && (document.detachEvent("onmousemove", h.checkHitTest),
                                                              document.attachEvent("onmousemove",
                                                              h.checkHitTest))
            },
            h.stopToCheckTest = function() {
                c.removeEventListener ? c.removeEventListener("mousemove", h.checkHitTest)
                                      : document.detachEvent && document.detachEvent("onmousemove", h.checkHitTest)
            },
            h.checkHitTest = function(e) {
                var t = FWDSIUtils.getViewportMouseCoordinates(e);
                FWDSIUtils.hitTest(h.screen, t.screenX, t.screenY) || (h.onMouseOutHandler(e), h.stopToCheckTest())
            },
            h.onMouseOutHandler = function(e) {
                h.isDisabled_bl || e.pointerType && e.pointerType != e.MSPOINTER_TYPE_MOUSE || h.setNormalState(!0)
            },
            h.onMouseClickHandler = function(e) {
                h.isDisabled_bl || i.disableThumbClick || h.dispatchEvent(u.CLICK, {
                    id: h.id
                })
            },
            h.setNormalState = function(e) {
                h.isSelected_bl && h.id != i.curId && (h.isSelected_bl = !1, h.overlay_do && Animation.to(h.overlay_do, .8, {
                    alpha: 1,
                    ease: Expo.easeOut
                }))
            }, h.setSelectedState = function(e) {
                h.isSelected_bl || h.id == i.curId || (h.isSelected_bl = !0, h.overlay_do && Animation.to(h.overlay_do, .8, {
                    alpha: 0,
                    ease: Expo.easeOut
                }))
            }, h.show = function(e) {
               Animation.killTweensOf(h), e ? Animation.to(h, h.transitionDuration, {
                    y: 0,
                    ease: h.transitionType_str
                }) : h.setY(0)
            }, h.hide = function(e) {
                Animation.killTweensOf(h), e ? Animation.to(h, h.transitionDuration, {
                    y: h.imageOffsetBottom + h.imageH + 2
                }) : h.setY(h.imageOffsetBottom + h.imageH + 2)
            }, h.init()
        };
        u.setPrototype = function() {
            u.prototype = new FWDSIDisplayObject("div")
        },
        u.HOVER = "onHover",
        u.CLICK = "onClick",
        u.IFRAME = "iframe",
        u.IMAGE = "image",
        u.FLASH = "flash",
        u.AUDIO = "audio",
        u.MAPS = "maps",
        u.AJAX = "ajax",
        u.HTML = "html",
        u.prototype = null,
        c.FWDSIThumb = u
    }(window),

    function(t) {
        function o() {}
        o.dumy = document.createElement("div"),
        o.trim = function(e) {
            return e.replace(/\s/gi, "")
        }, o.trimAndFormatUrl = function(e) {
            return e = (e = e.toLocaleLowerCase()).replace(/ /g, "-")
        }, o.splitAndTrim = function(e, t) {
            for (var i = e.split(","), n = i.length, s = 0; s < n; s++) t && (i[s] = o.trim(i[s]));
            return i
        }, o.formatTime = function(e) {
            var t = Math.floor(e / 3600),
                i = e % 3600,
                n = Math.floor(i / 60),
                s = i % 60,
                o = Math.ceil(s);
            return n = 10 <= n ? n : "0" + n, o = 10 <= o ? o : "0" + o, isNaN(o) ? "00:00" : self.hasHours_bl ? t + ":" + n + ":" + o : n + ":" + o
        }, o.getSecondsFromString = function(e) {
            var t = 0,
                i = 0,
                n = 0;
            if (e) return "0" == (t = (e = e.split(":"))[0])[0] && "0" != t[1] && (t = parseInt(t[1])), "00" == t && (t = 0), "0" == (i = e[1])[0] && "0" != i[1] && (i = parseInt(i[1])), "00" == i && (i = 0), secs = parseInt(e[2].replace(/,.*/gi, "")), "0" == secs[0] && "0" != secs[1] && (secs = parseInt(secs[1])), "00" == secs && (secs = 0), 0 != t && (n += 60 * t * 60), 0 != i && (n += 60 * i), n += secs
        }, o.indexOfArray = function(e, t) {
            for (var i = e.length, n = 0; n < i; n++)
                if (e[n] === t) return n;
            return -1
        }, o.randomizeArray = function(e) {
            for (var t = [], i = e.concat(), n = i.length, s = 0; s < n; s++) {
                var o = Math.floor(Math.random() * i.length);
                t.push(i[o]), i.splice(o, 1)
            }
            return t
        }, o.parent = function(e, t) {
            for (void 0 === t && (t = 1); t-- && e;) e = e.parentNode;
            return e && 1 === e.nodeType ? e : null
        }, o.sibling = function(e, t) {
            for (; e && 0 !== t;)
                if (0 < t) {
                    if (e.nextElementSibling) e = e.nextElementSibling;
                    else
                        for (e = e.nextSibling; e && 1 !== e.nodeType; e = e.nextSibling);
                    t--
                } else {
                    if (e.previousElementSibling) e = e.previousElementSibling;
                    else
                        for (e = e.previousSibling; e && 1 !== e.nodeType; e = e.previousSibling);
                    t++
                } return e
        },
        o.getChildAt = function(e, t) {
            var i = o.getChildren(e);
            return t < 0 && (t += i.length), t < 0 ? null : i[t]
        },
        o.getChildById = function(e) {
            return document.getElementById(e) || void 0
        },
        o.getChildren = function(e, t) {
            for (var i = [], n = e.firstChild; null != n; n = n.nextSibling)
              t ? i.push(n) : 1 === n.nodeType && i.push(n);
            return i
        },
        o.getChildrenFromAttribute = function(e, t, i) {
            for (var n = [], s = e.firstChild; null != s; s = s.nextSibling)
              i && o.hasAttribute(s, t) ? n.push(s) : 1 === s.nodeType && o.hasAttribute(s, t) && n.push(s);
            return 0 == n.length ? void 0 : n
        },
        o.getChildFromNodeListFromAttribute = function(e, t, i) {
            for (var n = e.firstChild; null != n; n = n.nextSibling) {
                if (i && o.hasAttribute(n, t)) return n;
                if (1 === n.nodeType && o.hasAttribute(n, t)) return n
            }
        },
        o.getAttributeValue = function(e, t) {
            if (o.hasAttribute(e, t)) return e.getAttribute(t)
        },
        o.hasAttribute = function(e, t) {
            return e.hasAttribute ? e.hasAttribute(t) : !!e.attributes[t]
        },
        o.insertNodeAt = function(e, t, i) {
            var n = o.children(e);
            if (i < 0 || i > n.length) throw new Error("invalid index!");
            e.insertBefore(t, n[i])
        },
        o.hasCanvas = function() {
            return Boolean(document.createElement("canvas"))
        },
        o.getCanvasWithModifiedColor = function(e, t, i) {
            if (e) {
                var n, s, o = document.createElement("canvas"),
                    r = o.getContext("2d"),
                    a = null,
                    l = parseInt(t.replace(/^#/, ""), 16),
                    d = l >>> 16 & 255,
                    h = l >>> 8 & 255,
                    c = 255 & l;
                o.style.position = "absolute",
                o.style.left = "0px",
                o.style.top = "0px",
                o.style.margin = "0px",
                o.style.padding = "0px",
                o.style.maxWidth = "none",
                o.style.maxHeight = "none",
                o.style.border = "none",
                o.style.lineHeight = "1",
                o.style.backgroundColor = "transparent",
                o.style.backfaceVisibility = "hidden",
                o.style.webkitBackfaceVisibility = "hidden",
                o.style.MozBackfaceVisibility = "hidden",
                o.style.MozImageRendering = "optimizeSpeed",
                o.style.WebkitImageRendering = "optimizeSpeed",
                o.width = e.width,
                o.height = e.height,
                r.drawImage(e, 0, 0, e.naturalWidth, e.naturalHeight, 0, 0, e.width, e.height),
                s = r.getImageData(0, 0, e.width, e.height),
                a = r.getImageData(0, 0, e.width, e.height);
                for (var u = 0, g = s.data.length; u < g; u += 4)
                  0 < a.data[u + 3] && (a.data[u] = s.data[u] / 255 * d, a.data[u + 1] = s.data[u + 1] / 255 * h, a.data[u + 2] = s.data[u + 2] / 255 * c);
                return r.globalAlpha = .5,
                       r.putImageData(a, 0, 0),
                       r.drawImage(o, 0, 0),
                       i && ((n = new Image).src = o.toDataURL()), {
                    canvas: o,
                    image: n
                }
            }
        },
        o.changeCanvasHEXColor = function(e, t, i, n) {
            if (e) {
                var s, o = (t = t).getContext("2d"),
                    r = null,
                    a = parseInt(i.replace(/^#/, ""), 16),
                    l = a >>> 16 & 255,
                    d = a >>> 8 & 255,
                    h = 255 & a;
                t.width = e.width,
                t.height = e.height,
                o.drawImage(e, 0, 0, e.naturalWidth, e.naturalHeight, 0, 0, e.width, e.height),
                s = o.getImageData(0, 0, e.width, e.height),
                r = o.getImageData(0, 0, e.width, e.height);
                for (var c = 0, u = s.data.length; c < u; c += 4)
                  0 < r.data[c + 3] && (r.data[c] = s.data[c] / 255 * l, r.data[c + 1] = s.data[c + 1] / 255 * d, r.data[c + 2] = s.data[c + 2] / 255 * h);
                if (o.globalAlpha = .5, o.putImageData(r, 0, 0), o.drawImage(t, 0, 0), n) {
                    var g = new Image;
                    return g.src = t.toDataURL(), g
                }
            }
        },
        o.hitTest = function(e, t, i) {
            if (!e) throw Error("Hit test target is null!");
            var n = e.getBoundingClientRect();
            return t >= n.left && t <= n.left + (n.right - n.left) && i >= n.top && i <= n.top + (n.bottom - n.top)
        },
        o.getScrollOffsets = function() {
            return null != t.pageXOffset ? {
                x: t.pageXOffset,
                y: t.pageYOffset
            } : "CSS1Compat" == document.compatMode ? {
                x: document.documentElement.scrollLeft,
                y: document.documentElement.scrollTop
            } : void 0
        },
        o.getViewportSize = function() {
            return o.hasPointerEvent && 1 < navigator.msMaxTouchPoints ? {
                w: document.documentElement.clientWidth || t.innerWidth,
                h: document.documentElement.clientHeight || t.innerHeight
            } : o.isMobile ? {
                w: t.innerWidth,
                h: t.innerHeight
            } : {
                w: document.documentElement.clientWidth || t.innerWidth,
                h: document.documentElement.clientHeight || t.innerHeight
            }
        },
        o.getViewportMouseCoordinates = function(e) {
            var t = o.getScrollOffsets();
            return e.touches ? {
                screenX: null == e.touches[0] ? e.touches.pageX - t.x : e.touches[0].pageX - t.x,
                screenY: null == e.touches[0] ? e.touches.pageY - t.y : e.touches[0].pageY - t.y
            } : {
                screenX: null == e.clientX ? e.pageX - t.x : e.clientX,
                screenY: null == e.clientY ? e.pageY - t.y : e.clientY
            }
        },
        o.hasPointerEvent = Boolean(t.navigator.msPointerEnabled) || Boolean(t.navigator.pointerEnabled), o.isMobile = function() {
            if (o.hasPointerEvent && 1 < navigator.msMaxTouchPoints || o.hasPointerEvent && 1 < navigator.maxTouchPoints) return !0;
            var e = ["android", "webos", "iphone", "ipad", "blackberry", "kfsowi"];
            for (i in e)
                if (-1 != navigator.userAgent.toLowerCase().indexOf(e[i])) return !0;
            return "MacIntel" === navigator.platform && 1 < navigator.maxTouchPoints
        }(),
        o.isAndroid = -1 != navigator.userAgent.toLowerCase().indexOf("android".toLowerCase()), o.hasWEBGL = function() {
            try {
                var e = document.createElement("canvas");
                return !!t.WebGLRenderingContext && (e.getContext("webgl") || e.getContext("experimental-webgl"))
            } catch (e) {
                return !1
            }
        }(),
        o.isLocal = "file:" == document.location.protocol,
        o.isChrome = -1 != navigator.userAgent.toLowerCase().indexOf("chrome"),
        o.isSafari = -1 != navigator.userAgent.toLowerCase().indexOf("safari") && -1 == navigator.userAgent.toLowerCase().indexOf("chrome"),
        o.isOpera = -1 != navigator.userAgent.toLowerCase().indexOf("opr"),
        o.isFirefox = -1 != navigator.userAgent.toLowerCase().indexOf("firefox"),
        o.isIEWebKit = Boolean(document.documentElement.msRequestFullscreen),
        o.isIE = Boolean(-1 != navigator.userAgent.toLowerCase().indexOf("msie")) || Boolean(-1 != navigator.userAgent.toLowerCase().indexOf("edge"))
                                                                                  || Boolean(document.documentElement.msRequestFullscreen),
        o.isIEAndLessThen9 = Boolean(-1 != navigator.userAgent.toLowerCase().indexOf("msie 7")) || Boolean(-1 != navigator.userAgent.toLowerCase().indexOf("msie 8")),
        o.isIE7 = Boolean(-1 != navigator.userAgent.toLowerCase().indexOf("msie 7")),
        o.isApple = Boolean(-1 != navigator.appVersion.toLowerCase().indexOf("mac")),
        o.isIphone = navigator.userAgent.match(/(iPhone|iPod)/g),
        o.hasFullScreen = o.dumy.requestFullScreen || o.dumy.mozRequestFullScreen || o.dumy.webkitRequestFullScreen || o.dumy.msieRequestFullScreen,
        o.volumeCanBeSet = function() {
            var e = document.createElement("audio");
            if (e) return (e.volume = 0) == e.volume
        }(),
        o.getVideoFormat = function() {
            var e, t = document.createElement("video");
            if (t.canPlayType) return "probably" == t.canPlayType("video/mp4") || "maybe" == t.canPlayType("video/mp4") ? e = ".mp4" : "probably" == t.canPlayType("video/ogg") || "maybe" == t.canPlayType("video/ogg") ? e = ".ogg" : "probably" != t.canPlayType("video/webm") && "maybe" != t.canPlayType("video/webm") || (e = ".webm"), t = null, e
        }(),
        o.onReady = function(e) {
            document.addEventListener ? t.addEventListener("DOMContentLoaded", function() {
                                        o.checkIfHasTransofrms(),
                                        o.hasFullScreen = o.checkIfHasFullscreen(),
                                        setTimeout(e, 100)
                                        })
                                      : document.onreadystatechange = function() {
                o.checkIfHasTransofrms(),
                o.hasFullScreen = o.checkIfHasFullscreen(),
                "complete" == document.readyState && setTimeout(e, 100)
            }
        },
        o.checkIfHasTransofrms = function() {
            document.documentElement.appendChild(o.dumy),
            o.hasTransform3d = function() {
                for (var e, t, i = ["transform", "msTransform", "WebkitTransform", "MozTransform", "OTransform", "KhtmlTransform"]; e = i.shift();)
                    if (void 0 !== o.dumy.style[e] && (o.dumy.style.position = "absolute", t = o.dumy.getBoundingClientRect().left, o.dumy.style[e] = "translate3d(500px, 0px, 0px)", 100 < (t = Math.abs(o.dumy.getBoundingClientRect().left - t)) && t < 900)) {
                        try {
                            document.documentElement.removeChild(o.dumy)
                        } catch (e) {}
                        return !0
                    } try {
                    document.documentElement.removeChild(o.dumy)
                } catch (e) {}
                return !1
            }(),
            o.hasTransform2d = function() {
                for (var e, t = ["transform", "msTransform", "WebkitTransform", "MozTransform", "OTransform", "KhtmlTransform"]; e = t.shift();)
                    if (void 0 !== o.dumy.style[e]) return !0;
                try {
                    document.documentElement.removeChild(o.dumy)
                } catch (e) {}
                return !1
            }(), o.isReadyMethodCalled_bl = !0
        },
        o.checkIfHasFullscreen = function() {
            return Boolean(document.documentElement.requestFullScreen || document.documentElement.mozRequestFullScreen
                                                                      || document.documentElement.webkitRequestFullScreen
                                                                      || document.documentElement.msRequestFullscreen)
        },
        o.disableElementSelection = function(e) {
            try {
                e.style.userSelect = "none"
            } catch (e) {}
            try {
                e.style.MozUserSelect = "none"
            } catch (e) {}
            try {
                e.style.webkitUserSelect = "none"
            } catch (e) {}
            try {
                e.style.khtmlUserSelect = "none"
            } catch (e) {}
            try {
                e.style.oUserSelect = "none"
            } catch (e) {}
            try {
                e.style.msUserSelect = "none"
            } catch (e) {}
            try {
                e.msUserSelect = "none"
            } catch (e) {}
            e.onselectstart = function() {
                return !1
            }
        },
        o.getUrlArgs = function(e) {
            for (var t = {}, i = e.substr(e.indexOf("?") + 1) || location.search.substring(1), n = (i = i.replace(/(\?*)(\/*)/g, "")).split("&"), s = 0; s < n.length; s++) {
                var o = n[s].indexOf("="),
                    r = n[s].substring(0, o),
                    a = n[s].substring(o + 1);
                a = decodeURIComponent(a), t[r] = a
            }
            return t
        }, o.getHashUrlArgs = function(e) {
            for (var t = {}, i = e.substr(e.indexOf("#") + 1) || location.search.substring(1), n = (i = i.replace(/(\?*)(\/*)/g, "")).split("&"), s = 0; s < n.length; s++) {
                var o = n[s].indexOf("="),
                    r = n[s].substring(0, o),
                    a = n[s].substring(o + 1);
                a = decodeURIComponent(a), t[r] = a
            }
            return t
        }, o.validateEmail = function(e) {
            return !!/^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,3})+$/.test(e)
        }, o.isReadyMethodCalled_bl = !1, t.FWDSIUtils = o
    }(window),

    
    function() {
        for (var o = 0, e = ["ms", "moz", "webkit", "o"], t = 0; t < e.length && !window.requestAnimationFrame; ++t) window.requestAnimationFrame = window[e[t] + "RequestAnimationFrame"], window.cancelAnimationFrame = window[e[t] + "CancelAnimationFrame"] || window[e[t] + "CancelRequestAnimationFrame"];
        window.requestAnimationFrame || (window.requestAnimationFrame = function(e, t) {
            var i = (new Date).getTime(),
                n = Math.max(0, 16 - (i - o)),
                s = window.setTimeout(function() {
                    e(i + n)
                }, n);
            return o = i + n, s
        }), window.cancelAnimationFrame || (window.cancelAnimationFrame = function(e) {
            clearTimeout(e)
        })
    }();
