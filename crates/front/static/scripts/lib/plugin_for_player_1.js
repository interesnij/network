if (! function(e) {
        function n() {}
        n.dumy = document.createElement("div"), n.trim = function(e) {
            return e.replace(/\s/gi, "")
        }, n.splitAndTrim = function(e, t) {
            for (var o = e.split(","), s = o.length, i = 0; i < s; i++) t && (o[i] = n.trim(o[i]));
            return o
        }, n.MD5 = function(e) {
            function a(e, t) {
                return e << t | e >>> 32 - t
            }

            function d(e, t) {
                var o, s, i, n, l;
                return i = 2147483648 & e, n = 2147483648 & t, l = (1073741823 & e) + (1073741823 & t), (o = 1073741824 & e) & (s = 1073741824 & t) ? 2147483648 ^ l ^ i ^ n : o | s ? 1073741824 & l ? 3221225472 ^ l ^ i ^ n : 1073741824 ^ l ^ i ^ n : l ^ i ^ n
            }

            function t(e, t, o, s, i, n, l) {
                var r;
                return e = d(e, d(d((r = t) & o | ~r & s, i), l)), d(a(e, n), t)
            }

            function o(e, t, o, s, i, n, l) {
                var r;
                return e = d(e, d(d(t & (r = s) | o & ~r, i), l)), d(a(e, n), t)
            }

            function s(e, t, o, s, i, n, l) {
                return e = d(e, d(d(t ^ o ^ s, i), l)), d(a(e, n), t)
            }

            function i(e, t, o, s, i, n, l) {
                return e = d(e, d(d(o ^ (t | ~s), i), l)), d(a(e, n), t)
            }

            function n(e) {
                var t, o = "",
                    s = "";
                for (t = 0; t <= 3; t++) o += (s = "0" + (e >>> 8 * t & 255).toString(16)).substr(s.length - 2, 2);
                return o
            }
            var l, r, u, c, h, _, f, p, m, b = Array();
            for (b = function(e) {
                    for (var t, o = e.length, s = o + 8, i = 16 * (1 + (s - s % 64) / 64), n = Array(i - 1), l = 0, r = 0; r < o;) l = r % 4 * 8, n[t = (r - r % 4) / 4] = n[t] | e.charCodeAt(r) << l, r++;
                    return l = r % 4 * 8, n[t = (r - r % 4) / 4] = n[t] | 128 << l, n[i - 2] = o << 3, n[i - 1] = o >>> 29, n
                }(e = function(e) {
                    e = e.replace(/\r\n/g, "\n");
                    for (var t = "", o = 0; o < e.length; o++) {
                        var s = e.charCodeAt(o);
                        s < 128 ? t += String.fromCharCode(s) : (127 < s && s < 2048 ? t += String.fromCharCode(s >> 6 | 192) : (t += String.fromCharCode(s >> 12 | 224), t += String.fromCharCode(s >> 6 & 63 | 128)), t += String.fromCharCode(63 & s | 128))
                    }
                    return t
                }(e)), _ = 1732584193, f = 4023233417, p = 2562383102, m = 271733878, l = 0; l < b.length; l += 16) _ = t(r = _, u = f, c = p, h = m, b[l + 0], 7, 3614090360), m = t(m, _, f, p, b[l + 1], 12, 3905402710), p = t(p, m, _, f, b[l + 2], 17, 606105819), f = t(f, p, m, _, b[l + 3], 22, 3250441966), _ = t(_, f, p, m, b[l + 4], 7, 4118548399), m = t(m, _, f, p, b[l + 5], 12, 1200080426), p = t(p, m, _, f, b[l + 6], 17, 2821735955), f = t(f, p, m, _, b[l + 7], 22, 4249261313), _ = t(_, f, p, m, b[l + 8], 7, 1770035416), m = t(m, _, f, p, b[l + 9], 12, 2336552879), p = t(p, m, _, f, b[l + 10], 17, 4294925233), f = t(f, p, m, _, b[l + 11], 22, 2304563134), _ = t(_, f, p, m, b[l + 12], 7, 1804603682), m = t(m, _, f, p, b[l + 13], 12, 4254626195), p = t(p, m, _, f, b[l + 14], 17, 2792965006), _ = o(_, f = t(f, p, m, _, b[l + 15], 22, 1236535329), p, m, b[l + 1], 5, 4129170786), m = o(m, _, f, p, b[l + 6], 9, 3225465664), p = o(p, m, _, f, b[l + 11], 14, 643717713), f = o(f, p, m, _, b[l + 0], 20, 3921069994), _ = o(_, f, p, m, b[l + 5], 5, 3593408605), m = o(m, _, f, p, b[l + 10], 9, 38016083), p = o(p, m, _, f, b[l + 15], 14, 3634488961), f = o(f, p, m, _, b[l + 4], 20, 3889429448), _ = o(_, f, p, m, b[l + 9], 5, 568446438), m = o(m, _, f, p, b[l + 14], 9, 3275163606), p = o(p, m, _, f, b[l + 3], 14, 4107603335), f = o(f, p, m, _, b[l + 8], 20, 1163531501), _ = o(_, f, p, m, b[l + 13], 5, 2850285829), m = o(m, _, f, p, b[l + 2], 9, 4243563512), p = o(p, m, _, f, b[l + 7], 14, 1735328473), _ = s(_, f = o(f, p, m, _, b[l + 12], 20, 2368359562), p, m, b[l + 5], 4, 4294588738), m = s(m, _, f, p, b[l + 8], 11, 2272392833), p = s(p, m, _, f, b[l + 11], 16, 1839030562), f = s(f, p, m, _, b[l + 14], 23, 4259657740), _ = s(_, f, p, m, b[l + 1], 4, 2763975236), m = s(m, _, f, p, b[l + 4], 11, 1272893353), p = s(p, m, _, f, b[l + 7], 16, 4139469664), f = s(f, p, m, _, b[l + 10], 23, 3200236656), _ = s(_, f, p, m, b[l + 13], 4, 681279174), m = s(m, _, f, p, b[l + 0], 11, 3936430074), p = s(p, m, _, f, b[l + 3], 16, 3572445317), f = s(f, p, m, _, b[l + 6], 23, 76029189), _ = s(_, f, p, m, b[l + 9], 4, 3654602809), m = s(m, _, f, p, b[l + 12], 11, 3873151461), p = s(p, m, _, f, b[l + 15], 16, 530742520), _ = i(_, f = s(f, p, m, _, b[l + 2], 23, 3299628645), p, m, b[l + 0], 6, 4096336452), m = i(m, _, f, p, b[l + 7], 10, 1126891415), p = i(p, m, _, f, b[l + 14], 15, 2878612391), f = i(f, p, m, _, b[l + 5], 21, 4237533241), _ = i(_, f, p, m, b[l + 12], 6, 1700485571), m = i(m, _, f, p, b[l + 3], 10, 2399980690), p = i(p, m, _, f, b[l + 10], 15, 4293915773), f = i(f, p, m, _, b[l + 1], 21, 2240044497), _ = i(_, f, p, m, b[l + 8], 6, 1873313359), m = i(m, _, f, p, b[l + 15], 10, 4264355552), p = i(p, m, _, f, b[l + 6], 15, 2734768916), f = i(f, p, m, _, b[l + 13], 21, 1309151649), _ = i(_, f, p, m, b[l + 4], 6, 4149444226), m = i(m, _, f, p, b[l + 11], 10, 3174756917), p = i(p, m, _, f, b[l + 2], 15, 718787259), f = i(f, p, m, _, b[l + 9], 21, 3951481745), _ = d(_, r), f = d(f, u), p = d(p, c), m = d(m, h);
            return (n(_) + n(f) + n(p) + n(m)).toLowerCase()
        }, n.getCanvasWithModifiedColor = function(e, t, o) {
            if (e) {
                var s, i, n = document.createElement("canvas"),
                    l = n.getContext("2d"),
                    r = null,
                    a = parseInt(t.replace(/^#/, ""), 16),
                    d = a >>> 16 & 255,
                    u = a >>> 8 & 255,
                    c = 255 & a;
                n.style.position = "absolute", n.style.left = "0px", n.style.top = "0px", n.style.margin = "0px", n.style.padding = "0px", n.style.maxWidth = "none", n.style.maxHeight = "none", n.style.border = "none", n.style.lineHeight = "1", n.style.backgroundColor = "transparent", n.style.backfaceVisibility = "hidden", n.style.webkitBackfaceVisibility = "hidden", n.style.MozBackfaceVisibility = "hidden", n.style.MozImageRendering = "optimizeSpeed", n.style.WebkitImageRendering = "optimizeSpeed", n.width = e.width, n.height = e.height, l.drawImage(e, 0, 0, e.naturalWidth, e.naturalHeight, 0, 0, e.width, e.height), i = l.getImageData(0, 0, e.width, e.height), r = l.getImageData(0, 0, e.width, e.height);
                for (var h = 0, _ = i.data.length; h < _; h += 4) 0 < r.data[h + 3] && (r.data[h] = i.data[h] / 255 * d, r.data[h + 1] = i.data[h + 1] / 255 * u, r.data[h + 2] = i.data[h + 2] / 255 * c);
                return l.globalAlpha = .5, l.putImageData(r, 0, 0), l.drawImage(n, 0, 0), o && ((s = new Image).src = n.toDataURL()), {
                    canvas: n,
                    image: s
                }
            }
        }, n.formatTime = function(e, t) {
            var o = Math.floor(e / 3600),
                s = e % 3600,
                i = Math.floor(s / 60),
                n = s % 60,
                l = Math.ceil(n);
            return i = 10 <= i ? i : "0" + i, l = 10 <= l ? l : "0" + l, isNaN(l) ? "00:00" : o || t ? "0" + o + ":" + i + ":" + l : i + ":" + l
        }, n.checkTime = function(e) {
            return !!/^(?:2[0-3]|[01][0-9]):[0-5][0-9]:[0-5][0-9]$/.test(e)
        }, n.getSecondsFromString = function(e) {
            var t = 0,
                o = 0,
                s = 0;
            if (e) return "0" == (t = (e = e.split(":"))[0])[0] && "0" != t[1] && (t = parseInt(t[1])), "00" == t && (t = 0), "0" == (o = e[1])[0] && "0" != o[1] && (o = parseInt(o[1])), "00" == o && (o = 0), secs = parseInt(e[2].replace(/,.*/gi, "")), "0" == secs[0] && "0" != secs[1] && (secs = parseInt(secs[1])), "00" == secs && (secs = 0), 0 != t && (s += 60 * t * 60), 0 != o && (s += 60 * o), s += secs
        }, n.changeCanvasHEXColor = function(e, t, o, s) {
            if (e) {
                var i, n = (t = t).getContext("2d"),
                    l = null,
                    r = parseInt(o.replace(/^#/, ""), 16),
                    a = r >>> 16 & 255,
                    d = r >>> 8 & 255,
                    u = 255 & r;
                t.width = e.width, t.height = e.height, n.drawImage(e, 0, 0, e.naturalWidth, e.naturalHeight, 0, 0, e.width, e.height), i = n.getImageData(0, 0, e.width, e.height), l = n.getImageData(0, 0, e.width, e.height);
                for (var c = 0, h = i.data.length; c < h; c += 4) 0 < l.data[c + 3] && (l.data[c] = i.data[c] / 255 * a, l.data[c + 1] = i.data[c + 1] / 255 * d, l.data[c + 2] = i.data[c + 2] / 255 * u);
                if (n.globalAlpha = .5, n.putImageData(l, 0, 0), n.drawImage(t, 0, 0), s) {
                    var _ = new Image;
                    return _.src = t.toDataURL(), _
                }
            }
        }, n.isURLEncoded = function(e) {
            try {
                if (decodeURIComponent(e) != e && -1 != e.indexOf("%")) return !0
            } catch (e) {}
            return !1
        }, n.indexOfArray = function(e, t) {
            for (var o = e.length, s = 0; s < o; s++)
                if (e[s] === t) return s;
            return -1
        }, n.randomizeArray = function(e) {
            for (var t = [], o = e.concat(), s = o.length, i = 0; i < s; i++) {
                var n = Math.floor(Math.random() * o.length);
                t.push(o[n]), o.splice(n, 1)
            }
            return t
        }, n.getCookie = function(e) {
            for (var t = e + "=", o = document.cookie.split(";"), s = 0; s < o.length; s++) {
                for (var i = o[s];
                    " " == i.charAt(0);) i = i.substring(1, i.length);
                if (0 == i.indexOf(t)) return i.substring(t.length, i.length)
            }
            return null
        }, n.parent = function(e, t) {
            for (void 0 === t && (t = 1); t-- && e;) e = e.parentNode;
            return e && 1 === e.nodeType ? e : null
        }, n.sibling = function(e, t) {
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
        }, n.getChildAt = function(e, t) {
            var o = n.getChildren(e);
            return t < 0 && (t += o.length), t < 0 ? null : o[t]
        }, n.getChildById = function(e) {
            return document.getElementById(e) || void 0
        }, n.getChildren = function(e, t) {
            for (var o = [], s = e.firstChild; null != s; s = s.nextSibling) t ? o.push(s) : 1 === s.nodeType && o.push(s);
            return o
        }, n.getChildrenFromAttribute = function(e, t, o) {
            for (var s = [], i = e.firstChild; null != i; i = i.nextSibling) o && n.hasAttribute(i, t) ? s.push(i) : 1 === i.nodeType && n.hasAttribute(i, t) && s.push(i);
            return 0 == s.length ? void 0 : s
        }, n.getChildFromNodeListFromAttribute = function(e, t, o) {
            for (var s = e.firstChild; null != s; s = s.nextSibling) {
                if (o && n.hasAttribute(s, t)) return s;
                if (1 === s.nodeType && n.hasAttribute(s, t)) return s
            }
        }, n.getAttributeValue = function(e, t) {
            if (n.hasAttribute(e, t)) return e.getAttribute(t)
        }, n.hasAttribute = function(e, t) {
            return e.hasAttribute ? e.hasAttribute(t) : !!e.getAttribute(t)
        }, n.insertNodeAt = function(e, t, o) {
            var s = n.children(e);
            if (o < 0 || o > s.length) throw new Error("invalid index!");
            e.insertBefore(t, s[o])
        }, n.hasCanvas = function() {
            return Boolean(document.createElement("canvas"))
        }, n.hitTest = function(e, t, o) {
            if (!e) throw Error("Hit test target is null!");
            var s = e.getBoundingClientRect();
            return t >= s.left && t <= s.left + (s.right - s.left) && o >= s.top && o <= s.top + (s.bottom - s.top)
        }, n.getScrollOffsets = function() {
            return null != e.pageXOffset ? {
                x: e.pageXOffset,
                y: e.pageYOffset
            } : "CSS1Compat" == document.compatMode ? {
                x: document.documentElement.scrollLeft,
                y: document.documentElement.scrollTop
            } : void 0
        }, n.getViewportSize = function() {
            return n.hasPointerEvent && 1 < navigator.msMaxTouchPoints ? {
                w: document.documentElement.clientWidth || e.innerWidth,
                h: document.documentElement.clientHeight || e.innerHeight
            } : n.isMobile ? {
                w: e.innerWidth,
                h: e.innerHeight
            } : {
                w: document.documentElement.clientWidth || e.innerWidth,
                h: document.documentElement.clientHeight || e.innerHeight
            }
        }, n.getViewportMouseCoordinates = function(e) {
            var t = n.getScrollOffsets();
            return e.touches ? {
                screenX: null == e.touches[0] ? e.touches.pageX - t.x : e.touches[0].pageX - t.x,
                screenY: null == e.touches[0] ? e.touches.pageY - t.y : e.touches[0].pageY - t.y
            } : {
                screenX: null == e.clientX ? e.pageX - t.x : e.clientX,
                screenY: null == e.clientY ? e.pageY - t.y : e.clientY
            }
        }, n.hasPointerEvent = Boolean(e.navigator.msPointerEnabled) || Boolean(e.navigator.pointerEnabled), n.isMobile = function() {
            if (n.hasPointerEvent && 1 < navigator.msMaxTouchPoints || n.hasPointerEvent && 1 < navigator.maxTouchPoints) return !0;
            var e = ["android", "webos", "iphone", "ipad", "blackberry"];
            for (i in e)
                if (-1 != navigator.userAgent.toLowerCase().indexOf(String(e[i]).toLowerCase())) return !0;
            return !1
        }(), n.isAndroid = -1 != navigator.userAgent.toLowerCase().indexOf("android".toLowerCase()), n.isChrome = -1 != navigator.userAgent.toLowerCase().indexOf("chrome"), n.isSafari = -1 != navigator.userAgent.toLowerCase().indexOf("safari") && -1 == navigator.userAgent.toLowerCase().indexOf("chrome"), n.isOpera = -1 != navigator.userAgent.toLowerCase().indexOf("opera") && -1 == navigator.userAgent.toLowerCase().indexOf("chrome"), n.isFirefox = -1 != navigator.userAgent.toLowerCase().indexOf("firefox"), n.isIE = Boolean(-1 != navigator.userAgent.toLowerCase().indexOf("msie")) || Boolean(-1 != navigator.userAgent.toLowerCase().indexOf("edge")) || Boolean(!n.isIE && document.documentElement.msRequestFullscreen), n.isIE11 = Boolean(!n.isIE && document.documentElement.msRequestFullscreen), n.isIEAndLessThen9 = -1 != navigator.userAgent.toLowerCase().indexOf("msie 7") || -1 != navigator.userAgent.toLowerCase().indexOf("msie 8"), n.isIEAndLessThen10 = -1 != navigator.userAgent.toLowerCase().indexOf("msie 7") || -1 != navigator.userAgent.toLowerCase().indexOf("msie 8") || -1 != navigator.userAgent.toLowerCase().indexOf("msie 9"), n.isIE7 = -1 != navigator.userAgent.toLowerCase().indexOf("msie 7"), n.isApple = -1 != navigator.appVersion.toLowerCase().indexOf("mac"), n.hasFullScreen = n.dumy.requestFullScreen || n.dumy.mozRequestFullScreen || n.dumy.webkitRequestFullScreen || n.dumy.msieRequestFullScreen, n.onReady = function(e) {
            document.addEventListener ? document.addEventListener("DOMContentLoaded", function() {
                n.checkIfHasTransofrms(), e()
            }) : document.onreadystatechange = function() {
                n.checkIfHasTransofrms(), "complete" == document.readyState && e()
            }
        }, n.checkIfHasTransofrms = function() {
            document.documentElement.appendChild(n.dumy), n.hasTransform3d = function() {
                for (var e, t, o = ["transform", "msTransform", "WebkitTransform", "MozTransform", "OTransform", "KhtmlTransform"]; e = o.shift();)
                    if (void 0 !== n.dumy.style[e] && (n.dumy.style.position = "absolute", t = n.dumy.getBoundingClientRect().left, n.dumy.style[e] = "translate3d(500px, 0px, 0px)", 100 < (t = Math.abs(n.dumy.getBoundingClientRect().left - t)) && t < 900)) {
                        try {
                            document.documentElement.removeChild(n.dumy)
                        } catch (e) {}
                        return !0
                    } try {
                    document.documentElement.removeChild(n.dumy)
                } catch (e) {}
                return !1
            }(), n.hasTransform2d = function() {
                for (var e, t = ["transform", "msTransform", "WebkitTransform", "MozTransform", "OTransform", "KhtmlTransform"]; e = t.shift();)
                    if (void 0 !== n.dumy.style[e]) return !0;
                try {
                    document.documentElement.removeChild(n.dumy)
                } catch (e) {}
                return !1
            }(), n.isReadyMethodCalled_bl = !0
        }, n.disableElementSelection = function(e) {
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
        }, n.getUrlArgs = function(e) {
            for (var t = {}, o = (e.substr(e.indexOf("?") + 1) || location.search.substring(1)).split("&"), s = 0; s < o.length; s++) {
                var i = o[s].indexOf("="),
                    n = o[s].substring(0, i),
                    l = o[s].substring(i + 1);
                l = decodeURIComponent(l), t[n] = l
            }
            return t
        }, n.isReadyMethodCalled_bl = !1, e.FWDMSPUtils = n
    }(window), !window.FWDAnimation) {
    var _fwd_gsScope = "undefined" != typeof fwd_module && fwd_module.exports && "undefined" != typeof fwd_global ? fwd_global : this || window;
    (_fwd_gsScope._fwd_gsQueue || (_fwd_gsScope._fwd_gsQueue = [])).push(function() {
            "use strict";

            function g(e, t, o, s) {
                o === s && (o = s - (s - t) / 1e6), e === t && (t = e + (o - e) / 1e6), this.a = e, this.b = t, this.c = o, this.d = s, this.da = s - e, this.ca = o - e, this.ba = t - e
            }

            function v(e, t, o, s) {
                var i = {
                        a: e
                    },
                    n = {},
                    l = {},
                    r = {
                        c: s
                    },
                    a = (e + t) / 2,
                    d = (t + o) / 2,
                    u = (o + s) / 2,
                    c = (a + d) / 2,
                    h = (d + u) / 2,
                    _ = (h - c) / 8;
                return i.b = a + (e - a) / 4, n.b = c + _, i.c = n.a = (i.b + n.b) / 2, n.c = l.a = (c + h) / 2, l.b = h - _, r.b = u + (s - u) / 4, l.c = r.a = (l.b + r.b) / 2, [i, n, l, r]
            }

            function b(e, t, o, s, i) {
                var n, l, r, a, d, u, c, h, _, f, p, m, b, g = e.length - 1,
                    S = 0,
                    y = e[0].a;
                for (n = 0; n < g; n++) l = (d = e[S]).a, r = d.d, a = e[S + 1].d, h = i ? (p = P[n], b = ((m = w[n]) + p) * t * .25 / (s ? .5 : D[n] || .5), r - ((u = r - (r - l) * (s ? .5 * t : 0 !== p ? b / p : 0)) + (((c = r + (a - r) * (s ? .5 * t : 0 !== m ? b / m : 0)) - u) * (3 * p / (p + m) + .5) / 4 || 0))) : r - ((u = r - (r - l) * t * .5) + (c = r + (a - r) * t * .5)) / 2, u += h, c += h, d.c = _ = u, d.b = 0 !== n ? y : y = d.a + .6 * (d.c - d.a), d.da = r - l, d.ca = _ - l, d.ba = y - l, o ? (f = v(l, y, _, r), e.splice(S, 1, f[0], f[1], f[2], f[3]), S += 4) : S++, y = c;
                (d = e[S]).b = y, d.c = y + .4 * (d.d - y), d.da = d.d - d.a, d.ca = d.c - d.a, d.ba = y - d.a, o && (f = v(d.a, y, d.c, d.d), e.splice(S, 1, f[0], f[1], f[2], f[3]))
            }

            function S(e, t, o, s) {
                var i, n, l, r, a, d, u = [];
                if (s)
                    for (n = (e = [s].concat(e)).length; - 1 < --n;) "string" == typeof(d = e[n][t]) && "=" === d.charAt(1) && (e[n][t] = s[t] + Number(d.charAt(0) + d.substr(2)));
                if ((i = e.length - 2) < 0) return u[0] = new g(e[0][t], 0, 0, e[i < -1 ? 0 : 1][t]), u;
                for (n = 0; n < i; n++) l = e[n][t], r = e[n + 1][t], u[n] = new g(l, 0, 0, r), o && (a = e[n + 2][t], P[n] = (P[n] || 0) + (r - l) * (r - l), w[n] = (w[n] || 0) + (a - r) * (a - r));
                return u[n] = new g(e[n][t], 0, 0, e[n + 1][t]), u
            }

            function _(e, t, o, s, i, n) {
                var l, r, a, d, u, c, h, _, f = {},
                    p = [],
                    m = n || e[0];
                for (r in i = "string" == typeof i ? "," + i + "," : ",x,y,z,left,top,right,bottom,marginTop,marginLeft,marginRight,marginBottom,paddingLeft,paddingTop,paddingRight,paddingBottom,backgroundPosition,backgroundPosition_y,", null == t && (t = 1), e[0]) p.push(r);
                if (1 < e.length) {
                    for (_ = e[e.length - 1], h = !0, l = p.length; - 1 < --l;)
                        if (r = p[l], .05 < Math.abs(m[r] - _[r])) {
                            h = !1;
                            break
                        } h && (e = e.concat(), n && e.unshift(n), e.push(e[1]), n = e[e.length - 3])
                }
                for (P.length = w.length = D.length = 0, l = p.length; - 1 < --l;) r = p[l], y[r] = -1 !== i.indexOf("," + r + ","), f[r] = S(e, r, y[r], n);
                for (l = P.length; - 1 < --l;) P[l] = Math.sqrt(P[l]), w[l] = Math.sqrt(w[l]);
                if (!s) {
                    for (l = p.length; - 1 < --l;)
                        if (y[r])
                            for (c = (a = f[p[l]]).length - 1, d = 0; d < c; d++) u = a[d + 1].da / w[d] + a[d].da / P[d] || 0, D[d] = (D[d] || 0) + u * u;
                    for (l = D.length; - 1 < --l;) D[l] = Math.sqrt(D[l])
                }
                for (l = p.length, d = o ? 4 : 1; - 1 < --l;) a = f[r = p[l]], b(a, t, o, s, y[r]), h && (a.splice(0, d), a.splice(a.length - d, d));
                return f
            }

            function f(e, t, o) {
                for (var s, i, n, l, r, a, d, u, c, h, _, f = 1 / o, p = e.length; - 1 < --p;)
                    for (n = (h = e[p]).a, l = h.d - n, r = h.c - n, a = h.b - n, s = i = 0, u = 1; u <= o; u++) s = i - (i = ((d = f * u) * d * l + 3 * (c = 1 - d) * (d * r + c * a)) * d), t[_ = p * o + u - 1] = (t[_] || 0) + s * s
            }
            var T, P, w, D, y, o, m, e, t, s;

            function a(e) {
                for (; e;) e.f || e.blob || (e.m = Math.round), e = e._next
            }
            _fwd_gsScope._gsDefine("FWDAnimation", ["core.Animation", "core.SimpleTimeline", "FWDTweenLite"], function(s, u, m) {
                function b(e) {
                    var t, o = [],
                        s = e.length;
                    for (t = 0; t !== s; o.push(e[t++]));
                    return o
                }

                function g(e, t, o) {
                    var s, i, n = e.cycle;
                    for (s in n) i = n[s], e[s] = "function" == typeof i ? i(o, t[o]) : i[o % i.length];
                    delete e.cycle
                }
                var S = function(e, t, o) {
                        m.call(this, e, t, o), this._cycle = 0, this._yoyo = !0 === this.vars.yoyo, this._repeat = this.vars.repeat || 0, this._repeatDelay = this.vars.repeatDelay || 0, this._dirty = !0, this.render = S.prototype.render
                    },
                    y = 1e-10,
                    v = m._internals,
                    P = v.isSelector,
                    T = v.isArray,
                    e = S.prototype = m.to({}, .1, {}),
                    w = [];
                S.version = "1.19.0", e.constructor = S, e.kill()._gc = !1, S.killTweensOf = S.killDelayedCallsTo = m.killTweensOf, S.getTweensOf = m.getTweensOf, S.lagSmoothing = m.lagSmoothing, S.ticker = m.ticker, S.render = m.render, e.invalidate = function() {
                    return this._yoyo = !0 === this.vars.yoyo, this._repeat = this.vars.repeat || 0, this._repeatDelay = this.vars.repeatDelay || 0, this._uncache(!0), m.prototype.invalidate.call(this)
                }, e.updateTo = function(e, t) {
                    var o, s = this.ratio,
                        i = this.vars.immediateRender || e.immediateRender;
                    for (o in t && this._startTime < this._timeline._time && (this._startTime = this._timeline._time, this._uncache(!1), this._gc ? this._enabled(!0, !1) : this._timeline.insert(this, this._startTime - this._delay)), e) this.vars[o] = e[o];
                    if (this._initted || i)
                        if (t) this._initted = !1, i && this.render(0, !0, !0);
                        else if (this._gc && this._enabled(!0, !1), this._notifyPluginsOfEnabled && this._firstPT && m._onPluginEvent("_onDisable", this), .998 < this._time / this._duration) {
                        var n = this._totalTime;
                        this.render(0, !0, !1), this._initted = !1, this.render(n, !0, !1)
                    } else if (this._initted = !1, this._init(), 0 < this._time || i)
                        for (var l, r = 1 / (1 - s), a = this._firstPT; a;) l = a.s + a.c, a.c *= r, a.s = l - a.c, a = a._next;
                    return this
                }, e.render = function(e, t, o) {
                    this._initted || 0 === this._duration && this.vars.repeat && this.invalidate();
                    var s, i, n, l, r, a, d, u, c = this._dirty ? this.totalDuration() : this._totalDuration,
                        h = this._time,
                        _ = this._totalTime,
                        f = this._cycle,
                        p = this._duration,
                        m = this._rawPrevTime;
                    if (c - 1e-7 <= e ? (this._totalTime = c, this._cycle = this._repeat, this._yoyo && 0 != (1 & this._cycle) ? (this._time = 0, this.ratio = this._ease._calcEnd ? this._ease.getRatio(0) : 0) : (this._time = p, this.ratio = this._ease._calcEnd ? this._ease.getRatio(1) : 1), this._reversed || (s = !0, i = "onComplete", o = o || this._timeline.autoRemoveChildren), 0 === p && (!this._initted && this.vars.lazy && !o || (this._startTime === this._timeline._duration && (e = 0), (m < 0 || e <= 0 && -1e-7 <= e || m === y && "isPause" !== this.data) && m !== e && (o = !0, y < m && (i = "onReverseComplete")), this._rawPrevTime = u = !t || e || m === e ? e : y))) : e < 1e-7 ? (this._totalTime = this._time = this._cycle = 0, this.ratio = this._ease._calcEnd ? this._ease.getRatio(0) : 0, (0 !== _ || 0 === p && 0 < m) && (i = "onReverseComplete", s = this._reversed), e < 0 && (this._active = !1, 0 === p && (!this._initted && this.vars.lazy && !o || (0 <= m && (o = !0), this._rawPrevTime = u = !t || e || m === e ? e : y))), this._initted || (o = !0)) : (this._totalTime = this._time = e, 0 !== this._repeat && (l = p + this._repeatDelay, this._cycle = this._totalTime / l >> 0, 0 !== this._cycle && this._cycle === this._totalTime / l && _ <= e && this._cycle--, this._time = this._totalTime - this._cycle * l, this._yoyo && 0 != (1 & this._cycle) && (this._time = p - this._time), this._time > p ? this._time = p : this._time < 0 && (this._time = 0)), this._easeType ? (r = this._time / p, (1 === (a = this._easeType) || 3 === a && .5 <= r) && (r = 1 - r), 3 === a && (r *= 2), 1 === (d = this._easePower) ? r *= r : 2 === d ? r *= r * r : 3 === d ? r *= r * r * r : 4 === d && (r *= r * r * r * r), 1 === a ? this.ratio = 1 - r : 2 === a ? this.ratio = r : this._time / p < .5 ? this.ratio = r / 2 : this.ratio = 1 - r / 2) : this.ratio = this._ease.getRatio(this._time / p)), h !== this._time || o || f !== this._cycle) {
                        if (!this._initted) {
                            if (this._init(), !this._initted || this._gc) return;
                            if (!o && this._firstPT && (!1 !== this.vars.lazy && this._duration || this.vars.lazy && !this._duration)) return this._time = h, this._totalTime = _, this._rawPrevTime = m, this._cycle = f, v.lazyTweens.push(this), void(this._lazy = [e, t]);
                            this._time && !s ? this.ratio = this._ease.getRatio(this._time / p) : s && this._ease._calcEnd && (this.ratio = this._ease.getRatio(0 === this._time ? 0 : 1))
                        }
                        for (!1 !== this._lazy && (this._lazy = !1), this._active || !this._paused && this._time !== h && 0 <= e && (this._active = !0), 0 === _ && (2 === this._initted && 0 < e && this._init(), this._startAt && (0 <= e ? this._startAt.render(e, t, o) : i = i || "_dummyGS"), this.vars.onStart && (0 === this._totalTime && 0 !== p || t || this._callback("onStart"))), n = this._firstPT; n;) {
                            if (n.f) n.t[n.p](n.c * this.ratio + n.s);
                            else {
                                var b = n.c * this.ratio + n.s;
                                "x" == n.p ? n.t.setX(b) : "y" == n.p ? n.t.setY(b) : "z" == n.p ? n.t.setZ(b) : "angleX" == n.p ? n.t.setAngleX(b) : "angleY" == n.p ? n.t.setAngleY(b) : "angleZ" == n.p ? n.t.setAngleZ(b) : "w" == n.p ? n.t.setWidth(b) : "h" == n.p ? n.t.setHeight(b) : "alpha" == n.p ? n.t.setAlpha(b) : "scale" == n.p ? n.t.setScale2(b) : n.t[n.p] = b
                            }
                            n = n._next
                        }
                        this._onUpdate && (e < 0 && this._startAt && this._startTime && this._startAt.render(e, t, o), t || this._totalTime === _ && !i || this._callback("onUpdate")), this._cycle !== f && (t || this._gc || this.vars.onRepeat && this._callback("onRepeat")), i && (this._gc && !o || (e < 0 && this._startAt && !this._onUpdate && this._startTime && this._startAt.render(e, t, o), s && (this._timeline.autoRemoveChildren && this._enabled(!1, !1), this._active = !1), !t && this.vars[i] && this._callback(i), 0 === p && this._rawPrevTime === y && u !== y && (this._rawPrevTime = 0)))
                    } else _ !== this._totalTime && this._onUpdate && (t || this._callback("onUpdate"))
                }, S.to = function(e, t, o) {
                    return new S(e, t, o)
                }, S.from = function(e, t, o) {
                    return o.runBackwards = !0, o.immediateRender = 0 != o.immediateRender, new S(e, t, o)
                }, S.fromTo = function(e, t, o, s) {
                    return s.startAt = o, s.immediateRender = 0 != s.immediateRender && 0 != o.immediateRender, new S(e, t, s)
                }, S.staggerTo = S.allTo = function(e, t, o, s, i, n, l) {
                    s = s || 0;

                    function r() {
                        o.onComplete && o.onComplete.apply(o.onCompleteScope || this, arguments), i.apply(l || o.callbackScope || this, n || w)
                    }
                    var a, d, u, c, h = 0,
                        _ = [],
                        f = o.cycle,
                        p = o.startAt && o.startAt.cycle;
                    for (T(e) || ("string" == typeof e && (e = m.selector(e) || e), P(e) && (e = b(e))), e = e || [], s < 0 && ((e = b(e)).reverse(), s *= -1), a = e.length - 1, u = 0; u <= a; u++) {
                        for (c in d = {}, o) d[c] = o[c];
                        if (f && (g(d, e, u), null != d.duration && (t = d.duration, delete d.duration)), p) {
                            for (c in p = d.startAt = {}, o.startAt) p[c] = o.startAt[c];
                            g(d.startAt, e, u)
                        }
                        d.delay = h + (d.delay || 0), u === a && i && (d.onComplete = r), _[u] = new S(e[u], t, d), h += s
                    }
                    return _
                }, S.staggerFrom = S.allFrom = function(e, t, o, s, i, n, l) {
                    return o.runBackwards = !0, o.immediateRender = 0 != o.immediateRender, S.staggerTo(e, t, o, s, i, n, l)
                }, S.staggerFromTo = S.allFromTo = function(e, t, o, s, i, n, l, r) {
                    return s.startAt = o, s.immediateRender = 0 != s.immediateRender && 0 != o.immediateRender, S.staggerTo(e, t, s, i, n, l, r)
                }, S.delayedCall = function(e, t, o, s, i) {
                    return new S(t, 0, {
                        delay: e,
                        onComplete: t,
                        onCompleteParams: o,
                        callbackScope: s,
                        onReverseComplete: t,
                        onReverseCompleteParams: o,
                        immediateRender: !1,
                        useFrames: i,
                        overwrite: 0
                    })
                }, S.set = function(e, t) {
                    return new S(e, 0, t)
                }, S.isTweening = function(e) {
                    return 0 < m.getTweensOf(e, !0).length
                };
                var n = function(e, t) {
                        for (var o = [], s = 0, i = e._first; i;) i instanceof m ? o[s++] = i : (t && (o[s++] = i), s = (o = o.concat(n(i, t))).length), i = i._next;
                        return o
                    },
                    c = S.getAllTweens = function(e) {
                        return n(s._rootTimeline, e).concat(n(s._rootFramesTimeline, e))
                    };
                S.killAll = function(e, t, o, s) {
                    null == t && (t = !0), null == o && (o = !0);
                    var i, n, l, r = c(0 != s),
                        a = r.length,
                        d = t && o && s;
                    for (l = 0; l < a; l++) n = r[l], (d || n instanceof u || (i = n.target === n.vars.onComplete) && o || t && !i) && (e ? n.totalTime(n._reversed ? 0 : n.totalDuration()) : n._enabled(!1, !1))
                }, S.killChildTweensOf = function(e, t) {
                    if (null != e) {
                        var o, s, i, n, l, r = v.tweenLookup;
                        if ("string" == typeof e && (e = m.selector(e) || e), P(e) && (e = b(e)), T(e))
                            for (n = e.length; - 1 < --n;) S.killChildTweensOf(e[n], t);
                        else {
                            for (i in o = [], r)
                                for (s = r[i].target.parentNode; s;) s === e && (o = o.concat(r[i].tweens)), s = s.parentNode;
                            for (l = o.length, n = 0; n < l; n++) t && o[n].totalTime(o[n].totalDuration()), o[n]._enabled(!1, !1)
                        }
                    }
                };

                function i(e, t, o, s) {
                    t = !1 !== t, o = !1 !== o;
                    for (var i, n, l = c(s = !1 !== s), r = t && o && s, a = l.length; - 1 < --a;) n = l[a], (r || n instanceof u || (i = n.target === n.vars.onComplete) && o || t && !i) && n.paused(e)
                }
                return S.pauseAll = function(e, t, o) {
                    i(!0, e, t, o)
                }, S.resumeAll = function(e, t, o) {
                    i(!1, e, t, o)
                }, S.globalTimeScale = function(e) {
                    var t = s._rootTimeline,
                        o = m.ticker.time;
                    return arguments.length ? (e = e || y, t._startTime = o - (o - t._startTime) * t._timeScale / e, t = s._rootFramesTimeline, o = m.ticker.frame, t._startTime = o - (o - t._startTime) * t._timeScale / e, t._timeScale = s._rootTimeline._timeScale = e, e) : t._timeScale
                }, e.progress = function(e, t) {
                    return arguments.length ? this.totalTime(this.duration() * (this._yoyo && 0 != (1 & this._cycle) ? 1 - e : e) + this._cycle * (this._duration + this._repeatDelay), t) : this._time / this.duration()
                }, e.totalProgress = function(e, t) {
                    return arguments.length ? this.totalTime(this.totalDuration() * e, t) : this._totalTime / this.totalDuration()
                }, e.time = function(e, t) {
                    return arguments.length ? (this._dirty && this.totalDuration(), e > this._duration && (e = this._duration), this._yoyo && 0 != (1 & this._cycle) ? e = this._duration - e + this._cycle * (this._duration + this._repeatDelay) : 0 !== this._repeat && (e += this._cycle * (this._duration + this._repeatDelay)), this.totalTime(e, t)) : this._time
                }, e.duration = function(e) {
                    return arguments.length ? s.prototype.duration.call(this, e) : this._duration
                }, e.totalDuration = function(e) {
                    return arguments.length ? -1 === this._repeat ? this : this.duration((e - this._repeat * this._repeatDelay) / (this._repeat + 1)) : (this._dirty && (this._totalDuration = -1 === this._repeat ? 999999999999 : this._duration * (this._repeat + 1) + this._repeatDelay * this._repeat, this._dirty = !1), this._totalDuration)
                }, e.repeat = function(e) {
                    return arguments.length ? (this._repeat = e, this._uncache(!0)) : this._repeat
                }, e.repeatDelay = function(e) {
                    return arguments.length ? (this._repeatDelay = e, this._uncache(!0)) : this._repeatDelay
                }, e.yoyo = function(e) {
                    return arguments.length ? (this._yoyo = e, this) : this._yoyo
                }, S
            }, !0), _fwd_gsScope._gsDefine("TimelineLite", ["core.Animation", "core.SimpleTimeline", "FWDTweenLite"], function(u, c, h) {
                function _(e) {
                    c.call(this, e), this._labels = {}, this.autoRemoveChildren = !0 === this.vars.autoRemoveChildren, this.smoothChildTiming = !0 === this.vars.smoothChildTiming, this._sortChildren = !0, this._onUpdate = this.vars.onUpdate;
                    var t, o, s = this.vars;
                    for (o in s) t = s[o], S(t) && -1 !== t.join("").indexOf("{self}") && (s[o] = this._swapSelfInParams(t));
                    S(s.tweens) && this.add(s.tweens, 0, s.align, s.stagger)
                }

                function f(e) {
                    var t, o = {};
                    for (t in e) o[t] = e[t];
                    return o
                }

                function p(e, t, o) {
                    var s, i, n = e.cycle;
                    for (s in n) i = n[s], e[s] = "function" == typeof i ? i.call(t[o], o) : i[o % i.length];
                    delete e.cycle
                }

                function m(e) {
                    var t, o = [],
                        s = e.length;
                    for (t = 0; t !== s; o.push(e[t++]));
                    return o
                }
                var b = 1e-10,
                    e = h._internals,
                    t = _._internals = {},
                    g = e.isSelector,
                    S = e.isArray,
                    y = e.lazyTweens,
                    v = e.lazyRender,
                    l = _fwd_gsScope._gsDefine.globals,
                    n = t.pauseCallback = function() {},
                    o = _.prototype = new c;
                return _.version = "1.19.0", o.constructor = _, o.kill()._gc = o._forcingPlayhead = o._hasPause = !1, o.to = function(e, t, o, s) {
                    var i = o.repeat && l.FWDAnimation || h;
                    return t ? this.add(new i(e, t, o), s) : this.set(e, o, s)
                }, o.from = function(e, t, o, s) {
                    return this.add((o.repeat && l.FWDAnimation || h).from(e, t, o), s)
                }, o.fromTo = function(e, t, o, s, i) {
                    var n = s.repeat && l.FWDAnimation || h;
                    return t ? this.add(n.fromTo(e, t, o, s), i) : this.set(e, s, i)
                }, o.staggerTo = function(e, t, o, s, i, n, l, r) {
                    var a, d, u = new _({
                            onComplete: n,
                            onCompleteParams: l,
                            callbackScope: r,
                            smoothChildTiming: this.smoothChildTiming
                        }),
                        c = o.cycle;
                    for ("string" == typeof e && (e = h.selector(e) || e), g(e = e || []) && (e = m(e)), (s = s || 0) < 0 && ((e = m(e)).reverse(), s *= -1), d = 0; d < e.length; d++)(a = f(o)).startAt && (a.startAt = f(a.startAt), a.startAt.cycle && p(a.startAt, e, d)), c && (p(a, e, d), null != a.duration && (t = a.duration, delete a.duration)), u.to(e[d], t, a, d * s);
                    return this.add(u, i)
                }, o.staggerFrom = function(e, t, o, s, i, n, l, r) {
                    return o.immediateRender = 0 != o.immediateRender, o.runBackwards = !0, this.staggerTo(e, t, o, s, i, n, l, r)
                }, o.staggerFromTo = function(e, t, o, s, i, n, l, r, a) {
                    return s.startAt = o, s.immediateRender = 0 != s.immediateRender && 0 != o.immediateRender, this.staggerTo(e, t, s, i, n, l, r, a)
                }, o.call = function(e, t, o, s) {
                    return this.add(h.delayedCall(0, e, t, o), s)
                }, o.set = function(e, t, o) {
                    return o = this._parseTimeOrLabel(o, 0, !0), null == t.immediateRender && (t.immediateRender = o === this._time && !this._paused), this.add(new h(e, 0, t), o)
                }, _.exportRoot = function(e, t) {
                    null == (e = e || {}).smoothChildTiming && (e.smoothChildTiming = !0);
                    var o, s, i = new _(e),
                        n = i._timeline;
                    for (null == t && (t = !0), n._remove(i, !0), i._startTime = 0, i._rawPrevTime = i._time = i._totalTime = n._time, o = n._first; o;) s = o._next, t && o instanceof h && o.target === o.vars.onComplete || i.add(o, o._startTime - o._delay), o = s;
                    return n.add(i, 0), i
                }, o.add = function(e, t, o, s) {
                    var i, n, l, r, a, d;
                    if ("number" != typeof t && (t = this._parseTimeOrLabel(t, 0, !0, e)), !(e instanceof u)) {
                        if (e instanceof Array || e && e.push && S(e)) {
                            for (o = o || "normal", s = s || 0, i = t, n = e.length, l = 0; l < n; l++) S(r = e[l]) && (r = new _({
                                tweens: r
                            })), this.add(r, i), "string" != typeof r && "function" != typeof r && ("sequence" === o ? i = r._startTime + r.totalDuration() / r._timeScale : "start" === o && (r._startTime -= r.delay())), i += s;
                            return this._uncache(!0)
                        }
                        if ("string" == typeof e) return this.addLabel(e, t);
                        if ("function" != typeof e) throw "Cannot add " + e + " into the timeline; it is not a tween, timeline, function, or string.";
                        e = h.delayedCall(0, e)
                    }
                    if (c.prototype.add.call(this, e, t), (this._gc || this._time === this._duration) && !this._paused && this._duration < this.duration())
                        for (d = (a = this).rawTime() > e._startTime; a._timeline;) d && a._timeline.smoothChildTiming ? a.totalTime(a._totalTime, !0) : a._gc && a._enabled(!0, !1), a = a._timeline;
                    return this
                }, o.remove = function(e) {
                    if (e instanceof u) {
                        this._remove(e, !1);
                        var t = e._timeline = e.vars.useFrames ? u._rootFramesTimeline : u._rootTimeline;
                        return e._startTime = (e._paused ? e._pauseTime : t._time) - (e._reversed ? e.totalDuration() - e._totalTime : e._totalTime) / e._timeScale, this
                    }
                    if (e instanceof Array || e && e.push && S(e)) {
                        for (var o = e.length; - 1 < --o;) this.remove(e[o]);
                        return this
                    }
                    return "string" == typeof e ? this.removeLabel(e) : this.kill(null, e)
                }, o._remove = function(e, t) {
                    c.prototype._remove.call(this, e, t);
                    var o = this._last;
                    return o ? this._time > o._startTime + o._totalDuration / o._timeScale && (this._time = this.duration(), this._totalTime = this._totalDuration) : this._time = this._totalTime = this._duration = this._totalDuration = 0, this
                }, o.append = function(e, t) {
                    return this.add(e, this._parseTimeOrLabel(null, t, !0, e))
                }, o.insert = o.insertMultiple = function(e, t, o, s) {
                    return this.add(e, t || 0, o, s)
                }, o.appendMultiple = function(e, t, o, s) {
                    return this.add(e, this._parseTimeOrLabel(null, t, !0, e), o, s)
                }, o.addLabel = function(e, t) {
                    return this._labels[e] = this._parseTimeOrLabel(t), this
                }, o.addPause = function(e, t, o, s) {
                    var i = h.delayedCall(0, n, o, s || this);
                    return i.vars.onComplete = i.vars.onReverseComplete = t, i.data = "isPause", this._hasPause = !0, this.add(i, e)
                }, o.removeLabel = function(e) {
                    return delete this._labels[e], this
                }, o.getLabelTime = function(e) {
                    return null != this._labels[e] ? this._labels[e] : -1
                }, o._parseTimeOrLabel = function(e, t, o, s) {
                    var i;
                    if (s instanceof u && s.timeline === this) this.remove(s);
                    else if (s && (s instanceof Array || s.push && S(s)))
                        for (i = s.length; - 1 < --i;) s[i] instanceof u && s[i].timeline === this && this.remove(s[i]);
                    if ("string" == typeof t) return this._parseTimeOrLabel(t, o && "number" == typeof e && null == this._labels[t] ? e - this.duration() : 0, o);
                    if (t = t || 0, "string" != typeof e || !isNaN(e) && null == this._labels[e]) null == e && (e = this.duration());
                    else {
                        if (-1 === (i = e.indexOf("="))) return null == this._labels[e] ? o ? this._labels[e] = this.duration() + t : t : this._labels[e] + t;
                        t = parseInt(e.charAt(i - 1) + "1", 10) * Number(e.substr(i + 1)), e = 1 < i ? this._parseTimeOrLabel(e.substr(0, i - 1), 0, o) : this.duration()
                    }
                    return Number(e) + t
                }, o.seek = function(e, t) {
                    return this.totalTime("number" == typeof e ? e : this._parseTimeOrLabel(e), !1 !== t)
                }, o.stop = function() {
                    return this.paused(!0)
                }, o.gotoAndPlay = function(e, t) {
                    return this.play(e, t)
                }, o.gotoAndStop = function(e, t) {
                    return this.pause(e, t)
                }, o.render = function(e, t, o) {
                    this._gc && this._enabled(!0, !1);
                    var s, i, n, l, r, a, d, u = this._dirty ? this.totalDuration() : this._totalDuration,
                        c = this._time,
                        h = this._startTime,
                        _ = this._timeScale,
                        f = this._paused;
                    if (u - 1e-7 <= e) this._totalTime = this._time = u, this._reversed || this._hasPausedChild() || (i = !0, l = "onComplete", r = !!this._timeline.autoRemoveChildren, 0 === this._duration && (e <= 0 && -1e-7 <= e || this._rawPrevTime < 0 || this._rawPrevTime === b) && this._rawPrevTime !== e && this._first && (r = !0, this._rawPrevTime > b && (l = "onReverseComplete"))), this._rawPrevTime = this._duration || !t || e || this._rawPrevTime === e ? e : b, e = u + 1e-4;
                    else if (e < 1e-7)
                        if (this._totalTime = this._time = 0, (0 !== c || 0 === this._duration && this._rawPrevTime !== b && (0 < this._rawPrevTime || e < 0 && 0 <= this._rawPrevTime)) && (l = "onReverseComplete", i = this._reversed), e < 0) this._active = !1, this._timeline.autoRemoveChildren && this._reversed ? (r = i = !0, l = "onReverseComplete") : 0 <= this._rawPrevTime && this._first && (r = !0), this._rawPrevTime = e;
                        else {
                            if (this._rawPrevTime = this._duration || !t || e || this._rawPrevTime === e ? e : b, 0 === e && i)
                                for (s = this._first; s && 0 === s._startTime;) s._duration || (i = !1), s = s._next;
                            e = 0, this._initted || (r = !0)
                        }
                    else {
                        if (this._hasPause && !this._forcingPlayhead && !t) {
                            if (c <= e)
                                for (s = this._first; s && s._startTime <= e && !a;) s._duration || "isPause" !== s.data || s.ratio || 0 === s._startTime && 0 === this._rawPrevTime || (a = s), s = s._next;
                            else
                                for (s = this._last; s && s._startTime >= e && !a;) s._duration || "isPause" === s.data && 0 < s._rawPrevTime && (a = s), s = s._prev;
                            a && (this._time = e = a._startTime, this._totalTime = e + this._cycle * (this._totalDuration + this._repeatDelay))
                        }
                        this._totalTime = this._time = this._rawPrevTime = e
                    }
                    if (this._time !== c && this._first || o || r || a) {
                        if (this._initted || (this._initted = !0), this._active || !this._paused && this._time !== c && 0 < e && (this._active = !0), 0 === c && this.vars.onStart && (0 === this._time && this._duration || t || this._callback("onStart")), c <= (d = this._time))
                            for (s = this._first; s && (n = s._next, d === this._time && (!this._paused || f));)(s._active || s._startTime <= d && !s._paused && !s._gc) && (a === s && this.pause(), s._reversed ? s.render((s._dirty ? s.totalDuration() : s._totalDuration) - (e - s._startTime) * s._timeScale, t, o) : s.render((e - s._startTime) * s._timeScale, t, o)), s = n;
                        else
                            for (s = this._last; s && (n = s._prev, d === this._time && (!this._paused || f));) {
                                if (s._active || s._startTime <= c && !s._paused && !s._gc) {
                                    if (a === s) {
                                        for (a = s._prev; a && a.endTime() > this._time;) a.render(a._reversed ? a.totalDuration() - (e - a._startTime) * a._timeScale : (e - a._startTime) * a._timeScale, t, o), a = a._prev;
                                        a = null, this.pause()
                                    }
                                    s._reversed ? s.render((s._dirty ? s.totalDuration() : s._totalDuration) - (e - s._startTime) * s._timeScale, t, o) : s.render((e - s._startTime) * s._timeScale, t, o)
                                }
                                s = n
                            }
                        this._onUpdate && (t || (y.length && v(), this._callback("onUpdate"))), l && (this._gc || h !== this._startTime && _ === this._timeScale || (0 === this._time || u >= this.totalDuration()) && (i && (y.length && v(), this._timeline.autoRemoveChildren && this._enabled(!1, !1), this._active = !1), !t && this.vars[l] && this._callback(l)))
                    }
                }, o._hasPausedChild = function() {
                    for (var e = this._first; e;) {
                        if (e._paused || e instanceof _ && e._hasPausedChild()) return !0;
                        e = e._next
                    }
                    return !1
                }, o.getChildren = function(e, t, o, s) {
                    s = s || -9999999999;
                    for (var i = [], n = this._first, l = 0; n;) n._startTime < s || (n instanceof h ? !1 !== t && (i[l++] = n) : (!1 !== o && (i[l++] = n), !1 !== e && (l = (i = i.concat(n.getChildren(!0, t, o))).length))), n = n._next;
                    return i
                }, o.getTweensOf = function(e, t) {
                    var o, s, i = this._gc,
                        n = [],
                        l = 0;
                    for (i && this._enabled(!0, !0), s = (o = h.getTweensOf(e)).length; - 1 < --s;)(o[s].timeline === this || t && this._contains(o[s])) && (n[l++] = o[s]);
                    return i && this._enabled(!1, !0), n
                }, o.recent = function() {
                    return this._recent
                }, o._contains = function(e) {
                    for (var t = e.timeline; t;) {
                        if (t === this) return !0;
                        t = t.timeline
                    }
                    return !1
                }, o.shiftChildren = function(e, t, o) {
                    o = o || 0;
                    for (var s, i = this._first, n = this._labels; i;) i._startTime >= o && (i._startTime += e), i = i._next;
                    if (t)
                        for (s in n) n[s] >= o && (n[s] += e);
                    return this._uncache(!0)
                }, o._kill = function(e, t) {
                    if (!e && !t) return this._enabled(!1, !1);
                    for (var o = t ? this.getTweensOf(t) : this.getChildren(!0, !0, !1), s = o.length, i = !1; - 1 < --s;) o[s]._kill(e, t) && (i = !0);
                    return i
                }, o.clear = function(e) {
                    var t = this.getChildren(!1, !0, !0),
                        o = t.length;
                    for (this._time = this._totalTime = 0; - 1 < --o;) t[o]._enabled(!1, !1);
                    return !1 !== e && (this._labels = {}), this._uncache(!0)
                }, o.invalidate = function() {
                    for (var e = this._first; e;) e.invalidate(), e = e._next;
                    return u.prototype.invalidate.call(this)
                }, o._enabled = function(e, t) {
                    if (e === this._gc)
                        for (var o = this._first; o;) o._enabled(e, !0), o = o._next;
                    return c.prototype._enabled.call(this, e, t)
                }, o.totalTime = function(e, t, o) {
                    this._forcingPlayhead = !0;
                    var s = u.prototype.totalTime.apply(this, arguments);
                    return this._forcingPlayhead = !1, s
                }, o.duration = function(e) {
                    return arguments.length ? (0 !== this.duration() && 0 !== e && this.timeScale(this._duration / e), this) : (this._dirty && this.totalDuration(), this._duration)
                }, o.totalDuration = function(e) {
                    if (arguments.length) return e && this.totalDuration() ? this.timeScale(this._totalDuration / e) : this;
                    if (this._dirty) {
                        for (var t, o, s = 0, i = this._last, n = 999999999999; i;) t = i._prev, i._dirty && i.totalDuration(), i._startTime > n && this._sortChildren && !i._paused ? this.add(i, i._startTime - i._delay) : n = i._startTime, i._startTime < 0 && !i._paused && (s -= i._startTime, this._timeline.smoothChildTiming && (this._startTime += i._startTime / this._timeScale), this.shiftChildren(-i._startTime, !1, -9999999999), n = 0), s < (o = i._startTime + i._totalDuration / i._timeScale) && (s = o), i = t;
                        this._duration = this._totalDuration = s, this._dirty = !1
                    }
                    return this._totalDuration
                }, o.paused = function(e) {
                    if (!e)
                        for (var t = this._first, o = this._time; t;) t._startTime === o && "isPause" === t.data && (t._rawPrevTime = 0), t = t._next;
                    return u.prototype.paused.apply(this, arguments)
                }, o.usesFrames = function() {
                    for (var e = this._timeline; e._timeline;) e = e._timeline;
                    return e === u._rootFramesTimeline
                }, o.rawTime = function() {
                    return this._paused ? this._totalTime : (this._timeline.rawTime() - this._startTime) * this._timeScale
                }, _
            }, !0), _fwd_gsScope._gsDefine("TimelineMax", ["TimelineLite", "FWDTweenLite", "easing.Ease"], function(t, r, e) {
                function o(e) {
                    t.call(this, e), this._repeat = this.vars.repeat || 0, this._repeatDelay = this.vars.repeatDelay || 0, this._cycle = 0, this._yoyo = !0 === this.vars.yoyo, this._dirty = !0
                }
                var B = 1e-10,
                    s = r._internals,
                    M = s.lazyTweens,
                    F = s.lazyRender,
                    a = _fwd_gsScope._gsDefine.globals,
                    d = new e(null, null, 1, 0),
                    i = o.prototype = new t;
                return i.constructor = o, i.kill()._gc = !1, o.version = "1.19.0", i.invalidate = function() {
                    return this._yoyo = !0 === this.vars.yoyo, this._repeat = this.vars.repeat || 0, this._repeatDelay = this.vars.repeatDelay || 0, this._uncache(!0), t.prototype.invalidate.call(this)
                }, i.addCallback = function(e, t, o, s) {
                    return this.add(r.delayedCall(0, e, o, s), t)
                }, i.removeCallback = function(e, t) {
                    if (e)
                        if (null == t) this._kill(null, e);
                        else
                            for (var o = this.getTweensOf(e, !1), s = o.length, i = this._parseTimeOrLabel(t); - 1 < --s;) o[s]._startTime === i && o[s]._enabled(!1, !1);
                    return this
                }, i.removePause = function(e) {
                    return this.removeCallback(t._internals.pauseCallback, e)
                }, i.tweenTo = function(e, t) {
                    t = t || {};
                    var o, s, i, n = {
                            ease: d,
                            useFrames: this.usesFrames(),
                            immediateRender: !1
                        },
                        l = t.repeat && a.FWDAnimation || r;
                    for (s in t) n[s] = t[s];
                    return n.time = this._parseTimeOrLabel(e), o = Math.abs(Number(n.time) - this._time) / this._timeScale || .001, i = new l(this, o, n), n.onStart = function() {
                        i.target.paused(!0), i.vars.time !== i.target.time() && o === i.duration() && i.duration(Math.abs(i.vars.time - i.target.time()) / i.target._timeScale), t.onStart && i._callback("onStart")
                    }, i
                }, i.tweenFromTo = function(e, t, o) {
                    o = o || {}, e = this._parseTimeOrLabel(e), o.startAt = {
                        onComplete: this.seek,
                        onCompleteParams: [e],
                        callbackScope: this
                    }, o.immediateRender = !1 !== o.immediateRender;
                    var s = this.tweenTo(t, o);
                    return s.duration(Math.abs(s.vars.time - e) / this._timeScale || .001)
                }, i.render = function(e, t, o) {
                    this._gc && this._enabled(!0, !1);
                    var s, i, n, l, r, a, d, u, c = this._dirty ? this.totalDuration() : this._totalDuration,
                        h = this._duration,
                        _ = this._time,
                        f = this._totalTime,
                        p = this._startTime,
                        m = this._timeScale,
                        b = this._rawPrevTime,
                        g = this._paused,
                        S = this._cycle;
                    if (c - 1e-7 <= e) this._locked || (this._totalTime = c, this._cycle = this._repeat), this._reversed || this._hasPausedChild() || (i = !0, l = "onComplete", r = !!this._timeline.autoRemoveChildren, 0 === this._duration && (e <= 0 && -1e-7 <= e || b < 0 || b === B) && b !== e && this._first && (r = !0, B < b && (l = "onReverseComplete"))), this._rawPrevTime = this._duration || !t || e || this._rawPrevTime === e ? e : B, this._yoyo && 0 != (1 & this._cycle) ? this._time = e = 0 : e = (this._time = h) + 1e-4;
                    else if (e < 1e-7)
                        if (this._locked || (this._totalTime = this._cycle = 0), ((this._time = 0) !== _ || 0 === h && b !== B && (0 < b || e < 0 && 0 <= b) && !this._locked) && (l = "onReverseComplete", i = this._reversed), e < 0) this._active = !1, this._timeline.autoRemoveChildren && this._reversed ? (r = i = !0, l = "onReverseComplete") : 0 <= b && this._first && (r = !0), this._rawPrevTime = e;
                        else {
                            if (this._rawPrevTime = h || !t || e || this._rawPrevTime === e ? e : B, 0 === e && i)
                                for (s = this._first; s && 0 === s._startTime;) s._duration || (i = !1), s = s._next;
                            e = 0, this._initted || (r = !0)
                        }
                    else if (0 === h && b < 0 && (r = !0), this._time = this._rawPrevTime = e, this._locked || (this._totalTime = e, 0 !== this._repeat && (a = h + this._repeatDelay, this._cycle = this._totalTime / a >> 0, 0 !== this._cycle && this._cycle === this._totalTime / a && f <= e && this._cycle--, this._time = this._totalTime - this._cycle * a, this._yoyo && 0 != (1 & this._cycle) && (this._time = h - this._time), this._time > h ? e = (this._time = h) + 1e-4 : this._time < 0 ? this._time = e = 0 : e = this._time)), this._hasPause && !this._forcingPlayhead && !t) {
                        if (_ <= (e = this._time))
                            for (s = this._first; s && s._startTime <= e && !d;) s._duration || "isPause" !== s.data || s.ratio || 0 === s._startTime && 0 === this._rawPrevTime || (d = s), s = s._next;
                        else
                            for (s = this._last; s && s._startTime >= e && !d;) s._duration || "isPause" === s.data && 0 < s._rawPrevTime && (d = s), s = s._prev;
                        d && (this._time = e = d._startTime, this._totalTime = e + this._cycle * (this._totalDuration + this._repeatDelay))
                    }
                    if (this._cycle !== S && !this._locked) {
                        var y = this._yoyo && 0 != (1 & S),
                            v = y === (this._yoyo && 0 != (1 & this._cycle)),
                            P = this._totalTime,
                            T = this._cycle,
                            w = this._rawPrevTime,
                            D = this._time;
                        if (this._totalTime = S * h, this._cycle < S ? y = !y : this._totalTime += h, this._time = _, this._rawPrevTime = 0 === h ? b - 1e-4 : b, this._cycle = S, this._locked = !0, _ = y ? 0 : h, this.render(_, t, 0 === h), t || this._gc || this.vars.onRepeat && this._callback("onRepeat"), _ !== this._time) return;
                        if (v && (_ = y ? h + 1e-4 : -1e-4, this.render(_, !0, !1)), this._locked = !1, this._paused && !g) return;
                        this._time = D, this._totalTime = P, this._cycle = T, this._rawPrevTime = w
                    }
                    if (this._time !== _ && this._first || o || r || d) {
                        if (this._initted || (this._initted = !0), this._active || !this._paused && this._totalTime !== f && 0 < e && (this._active = !0), 0 === f && this.vars.onStart && (0 === this._totalTime && this._totalDuration || t || this._callback("onStart")), _ <= (u = this._time))
                            for (s = this._first; s && (n = s._next, u === this._time && (!this._paused || g));)(s._active || s._startTime <= this._time && !s._paused && !s._gc) && (d === s && this.pause(), s._reversed ? s.render((s._dirty ? s.totalDuration() : s._totalDuration) - (e - s._startTime) * s._timeScale, t, o) : s.render((e - s._startTime) * s._timeScale, t, o)), s = n;
                        else
                            for (s = this._last; s && (n = s._prev, u === this._time && (!this._paused || g));) {
                                if (s._active || s._startTime <= _ && !s._paused && !s._gc) {
                                    if (d === s) {
                                        for (d = s._prev; d && d.endTime() > this._time;) d.render(d._reversed ? d.totalDuration() - (e - d._startTime) * d._timeScale : (e - d._startTime) * d._timeScale, t, o), d = d._prev;
                                        d = null, this.pause()
                                    }
                                    s._reversed ? s.render((s._dirty ? s.totalDuration() : s._totalDuration) - (e - s._startTime) * s._timeScale, t, o) : s.render((e - s._startTime) * s._timeScale, t, o)
                                }
                                s = n
                            }
                        this._onUpdate && (t || (M.length && F(), this._callback("onUpdate"))), l && (this._locked || this._gc || p !== this._startTime && m === this._timeScale || (0 === this._time || c >= this.totalDuration()) && (i && (M.length && F(), this._timeline.autoRemoveChildren && this._enabled(!1, !1), this._active = !1), !t && this.vars[l] && this._callback(l)))
                    } else f !== this._totalTime && this._onUpdate && (t || this._callback("onUpdate"))
                }, i.getActive = function(e, t, o) {
                    null == e && (e = !0), null == t && (t = !0), null == o && (o = !1);
                    var s, i, n = [],
                        l = this.getChildren(e, t, o),
                        r = 0,
                        a = l.length;
                    for (s = 0; s < a; s++)(i = l[s]).isActive() && (n[r++] = i);
                    return n
                }, i.getLabelAfter = function(e) {
                    e || 0 !== e && (e = this._time);
                    var t, o = this.getLabelsArray(),
                        s = o.length;
                    for (t = 0; t < s; t++)
                        if (o[t].time > e) return o[t].name;
                    return null
                }, i.getLabelBefore = function(e) {
                    null == e && (e = this._time);
                    for (var t = this.getLabelsArray(), o = t.length; - 1 < --o;)
                        if (t[o].time < e) return t[o].name;
                    return null
                }, i.getLabelsArray = function() {
                    var e, t = [],
                        o = 0;
                    for (e in this._labels) t[o++] = {
                        time: this._labels[e],
                        name: e
                    };
                    return t.sort(function(e, t) {
                        return e.time - t.time
                    }), t
                }, i.progress = function(e, t) {
                    return arguments.length ? this.totalTime(this.duration() * (this._yoyo && 0 != (1 & this._cycle) ? 1 - e : e) + this._cycle * (this._duration + this._repeatDelay), t) : this._time / this.duration()
                }, i.totalProgress = function(e, t) {
                    return arguments.length ? this.totalTime(this.totalDuration() * e, t) : this._totalTime / this.totalDuration()
                }, i.totalDuration = function(e) {
                    return arguments.length ? -1 !== this._repeat && e ? this.timeScale(this.totalDuration() / e) : this : (this._dirty && (t.prototype.totalDuration.call(this), this._totalDuration = -1 === this._repeat ? 999999999999 : this._duration * (this._repeat + 1) + this._repeatDelay * this._repeat), this._totalDuration)
                }, i.time = function(e, t) {
                    return arguments.length ? (this._dirty && this.totalDuration(), e > this._duration && (e = this._duration), this._yoyo && 0 != (1 & this._cycle) ? e = this._duration - e + this._cycle * (this._duration + this._repeatDelay) : 0 !== this._repeat && (e += this._cycle * (this._duration + this._repeatDelay)), this.totalTime(e, t)) : this._time
                }, i.repeat = function(e) {
                    return arguments.length ? (this._repeat = e, this._uncache(!0)) : this._repeat
                }, i.repeatDelay = function(e) {
                    return arguments.length ? (this._repeatDelay = e, this._uncache(!0)) : this._repeatDelay
                }, i.yoyo = function(e) {
                    return arguments.length ? (this._yoyo = e, this) : this._yoyo
                }, i.currentLabel = function(e) {
                    return arguments.length ? this.seek(e, !0) : this.getLabelBefore(this._time + 1e-8)
                }, o
            }, !0), T = 180 / Math.PI, P = [], w = [], D = [], y = {}, o = _fwd_gsScope._gsDefine.globals, m = _fwd_gsScope._gsDefine.plugin({
                propName: "bezier",
                priority: -1,
                version: "1.3.7",
                API: 2,
                fwd_global: !0,
                init: function(e, t, o) {
                    this._target = e, t instanceof Array && (t = {
                        values: t
                    }), this._func = {}, this._mod = {}, this._props = [], this._timeRes = null == t.timeResolution ? 6 : parseInt(t.timeResolution, 10);
                    var s, i, n, l, r, a = t.values || [],
                        d = {},
                        u = a[0],
                        c = t.autoRotate || o.vars.orientToBezier;
                    for (s in this._autoRotate = c ? c instanceof Array ? c : [
                            ["x", "y", "rotation", !0 === c ? 0 : Number(c) || 0]
                        ] : null, u) this._props.push(s);
                    for (n = this._props.length; - 1 < --n;) s = this._props[n], this._overwriteProps.push(s), i = this._func[s] = "function" == typeof e[s], d[s] = i ? e[s.indexOf("set") || "function" != typeof e["get" + s.substr(3)] ? s : "get" + s.substr(3)]() : parseFloat(e[s]), r || d[s] !== a[0][s] && (r = d);
                    if (this._beziers = "cubic" !== t.type && "quadratic" !== t.type && "soft" !== t.type ? _(a, isNaN(t.curviness) ? 1 : t.curviness, !1, "thruBasic" === t.type, t.correlate, r) : function(e, t, o) {
                            var s, i, n, l, r, a, d, u, c, h, _, f = {},
                                p = "cubic" === (t = t || "soft") ? 3 : 2,
                                m = "soft" === t,
                                b = [];
                            if (m && o && (e = [o].concat(e)), null == e || e.length < 1 + p) throw "invalid Bezier data";
                            for (c in e[0]) b.push(c);
                            for (a = b.length; - 1 < --a;) {
                                for (f[c = b[a]] = r = [], h = 0, u = e.length, d = 0; d < u; d++) s = null == o ? e[d][c] : "string" == typeof(_ = e[d][c]) && "=" === _.charAt(1) ? o[c] + Number(_.charAt(0) + _.substr(2)) : Number(_), m && 1 < d && d < u - 1 && (r[h++] = (s + r[h - 2]) / 2), r[h++] = s;
                                for (u = h - p + 1, d = h = 0; d < u; d += p) s = r[d], i = r[d + 1], n = r[d + 2], l = 2 == p ? 0 : r[d + 3], r[h++] = _ = 3 == p ? new g(s, i, n, l) : new g(s, (2 * i + s) / 3, (2 * i + n) / 3, n);
                                r.length = h
                            }
                            return f
                        }(a, t.type, d), this._segCount = this._beziers[s].length, this._timeRes) {
                        var h = function(e, t) {
                            var o, s, i, n, l = [],
                                r = [],
                                a = 0,
                                d = 0,
                                u = (t = t >> 0 || 6) - 1,
                                c = [],
                                h = [];
                            for (o in e) f(e[o], l, t);
                            for (i = l.length, s = 0; s < i; s++) a += Math.sqrt(l[s]), h[n = s % t] = a, n === u && (d += a, c[n = s / t >> 0] = h, r[n] = d, a = 0, h = []);
                            return {
                                length: d,
                                lengths: r,
                                segments: c
                            }
                        }(this._beziers, this._timeRes);
                        this._length = h.length, this._lengths = h.lengths, this._segments = h.segments, this._l1 = this._li = this._s1 = this._si = 0, this._l2 = this._lengths[0], this._curSeg = this._segments[0], this._s2 = this._curSeg[0], this._prec = 1 / this._curSeg.length
                    }
                    if (c = this._autoRotate)
                        for (this._initialRotations = [], c[0] instanceof Array || (this._autoRotate = c = [c]), n = c.length; - 1 < --n;) {
                            for (l = 0; l < 3; l++) s = c[n][l], this._func[s] = "function" == typeof e[s] && e[s.indexOf("set") || "function" != typeof e["get" + s.substr(3)] ? s : "get" + s.substr(3)];
                            s = c[n][2], this._initialRotations[n] = (this._func[s] ? this._func[s].call(this._target) : this._target[s]) || 0, this._overwriteProps.push(s)
                        }
                    return this._startRatio = o.vars.runBackwards ? 1 : 0, !0
                },
                set: function(e) {
                    var t, o, s, i, n, l, r, a, d, u, c = this._segCount,
                        h = this._func,
                        _ = this._target,
                        f = e !== this._startRatio;
                    if (this._timeRes) {
                        if (d = this._lengths, u = this._curSeg, e *= this._length, s = this._li, e > this._l2 && s < c - 1) {
                            for (a = c - 1; s < a && (this._l2 = d[++s]) <= e;);
                            this._l1 = d[s - 1], this._li = s, this._curSeg = u = this._segments[s], this._s2 = u[this._s1 = this._si = 0]
                        } else if (e < this._l1 && 0 < s) {
                            for (; 0 < s && (this._l1 = d[--s]) >= e;);
                            0 === s && e < this._l1 ? this._l1 = 0 : s++, this._l2 = d[s], this._li = s, this._curSeg = u = this._segments[s], this._s1 = u[(this._si = u.length - 1) - 1] || 0, this._s2 = u[this._si]
                        }
                        if (t = s, e -= this._l1, s = this._si, e > this._s2 && s < u.length - 1) {
                            for (a = u.length - 1; s < a && (this._s2 = u[++s]) <= e;);
                            this._s1 = u[s - 1], this._si = s
                        } else if (e < this._s1 && 0 < s) {
                            for (; 0 < s && (this._s1 = u[--s]) >= e;);
                            0 === s && e < this._s1 ? this._s1 = 0 : s++, this._s2 = u[s], this._si = s
                        }
                        l = (s + (e - this._s1) / (this._s2 - this._s1)) * this._prec || 0
                    } else l = (e - (t = e < 0 ? 0 : 1 <= e ? c - 1 : c * e >> 0) * (1 / c)) * c;
                    for (o = 1 - l, s = this._props.length; - 1 < --s;) i = this._props[s], r = (l * l * (n = this._beziers[i][t]).da + 3 * o * (l * n.ca + o * n.ba)) * l + n.a, this._mod[i] && (r = this._mod[i](r, _)), h[i] ? _[i](r) : "x" == i ? _.setX(r) : "y" == i ? _.setY(r) : "z" == i ? _.setZ(r) : "angleX" == i ? _.setAngleX(r) : "angleY" == i ? _.setAngleY(r) : "angleZ" == i ? _.setAngleZ(r) : "w" == i ? _.setWidth(r) : "h" == i ? _.setHeight(r) : "alpha" == i ? _.setAlpha(r) : "scale" == i ? _.setScale2(r) : _[i] = r;
                    if (this._autoRotate) {
                        var p, m, b, g, S, y, v, P = this._autoRotate;
                        for (s = P.length; - 1 < --s;) i = P[s][2], y = P[s][3] || 0, v = !0 === P[s][4] ? 1 : T, n = this._beziers[P[s][0]], p = this._beziers[P[s][1]], n && p && (n = n[t], p = p[t], m = n.a + (n.b - n.a) * l, m += ((g = n.b + (n.c - n.b) * l) - m) * l, g += (n.c + (n.d - n.c) * l - g) * l, b = p.a + (p.b - p.a) * l, b += ((S = p.b + (p.c - p.b) * l) - b) * l, S += (p.c + (p.d - p.c) * l - S) * l, r = f ? Math.atan2(S - b, g - m) * v + y : this._initialRotations[s], this._mod[i] && (r = this._mod[i](r, _)), h[i] ? _[i](r) : _[i] = r)
                    }
                }
            }), e = m.prototype, m.bezierThrough = _, m.cubicToQuadratic = v, m._autoCSS = !0, m.quadraticToCubic = function(e, t, o) {
                return new g(e, (2 * t + e) / 3, (2 * t + o) / 3, o)
            }, m._cssRegister = function() {
                var e = o.CSSPlugin;
                if (e) {
                    var t = e._internals,
                        _ = t._parseToProxy,
                        f = t._setPluginRatio,
                        p = t.CSSPropTween;
                    t._registerComplexSpecialProp("bezier", {
                        parser: function(e, t, o, s, i, n) {
                            t instanceof Array && (t = {
                                values: t
                            }), n = new m;
                            var l, r, a, d = t.values,
                                u = d.length - 1,
                                c = [],
                                h = {};
                            if (u < 0) return i;
                            for (l = 0; l <= u; l++) a = _(e, d[l], s, i, n, u !== l), c[l] = a.end;
                            for (r in t) h[r] = t[r];
                            return h.values = c, (i = new p(e, "bezier", 0, 0, a.pt, 2)).data = a, i.plugin = n, i.setRatio = f, 0 === h.autoRotate && (h.autoRotate = !0), !h.autoRotate || h.autoRotate instanceof Array || (l = !0 === h.autoRotate ? 0 : Number(h.autoRotate), h.autoRotate = null != a.end.left ? [
                                ["left", "top", "rotation", l, !1]
                            ] : null != a.end.x && [
                                ["x", "y", "rotation", l, !1]
                            ]), h.autoRotate && (s._transform || s._enableTransforms(!1), a.autoRotate = s._target._gsTransform, a.proxy.rotation = a.autoRotate.rotation || 0, s._overwriteProps.push("rotation")), n._onInitTween(a.proxy, h, s._tween), i
                        }
                    })
                }
            }, e._mod = function(e) {
                for (var t, o = this._overwriteProps, s = o.length; - 1 < --s;)(t = e[o[s]]) && "function" == typeof t && (this._mod[o[s]] = t)
            }, e._kill = function(e) {
                var t, o, s = this._props;
                for (t in this._beziers)
                    if (t in e)
                        for (delete this._beziers[t], delete this._func[t], o = s.length; - 1 < --o;) s[o] === t && s.splice(o, 1);
                if (s = this._autoRotate)
                    for (o = s.length; - 1 < --o;) e[s[o][2]] && s.splice(o, 1);
                return this._super._kill.call(this, e)
            }, _fwd_gsScope._gsDefine("plugins.CSSPlugin", ["plugins.TweenPlugin", "FWDTweenLite"], function(n, U) {
                var f, w, D, p, N = function() {
                        n.call(this, "css"), this._overwriteProps.length = 0, this.setRatio = N.prototype.setRatio
                    },
                    d = _fwd_gsScope._gsDefine.globals,
                    m = {},
                    e = N.prototype = new n("css");
                (e.constructor = N).version = "1.19.0", N.API = 2, N.defaultTransformPerspective = 0, N.defaultSkewType = "compensated", N.defaultSmoothOrigin = !0, e = "px", N.suffixMap = {
                    top: e,
                    right: e,
                    bottom: e,
                    left: e,
                    width: e,
                    height: e,
                    fontSize: e,
                    padding: e,
                    margin: e,
                    perspective: e,
                    lineHeight: ""
                };

                function l(e, t) {
                    return t.toUpperCase()
                }

                function t(e) {
                    return Z.createElementNS ? Z.createElementNS("http://www.w3.org/1999/xhtml", e) : Z.createElement(e)
                }

                function r(e) {
                    return R.test("string" == typeof e ? e : (e.currentStyle ? e.currentStyle.filter : e.style.filter) || "") ? parseFloat(RegExp.$1) / 100 : 1
                }

                function b(e) {
                    window.console && console.log(e)
                }

                function B(e, t) {
                    var o, s, i = (t = t || ee).style;
                    if (void 0 !== i[e]) return e;
                    for (e = e.charAt(0).toUpperCase() + e.substr(1), o = ["O", "Moz", "ms", "Ms", "Webkit"], s = 5; - 1 < --s && void 0 === i[o[s] + e];);
                    return 0 <= s ? (ne = "-" + (le = 3 === s ? "ms" : o[s]).toLowerCase() + "-", le + e) : null
                }

                function g(e, t) {
                    var o, s, i, n = {};
                    if (t = t || re(e, null))
                        if (o = t.length)
                            for (; - 1 < --o;) - 1 !== (i = t[o]).indexOf("-transform") && xe !== i || (n[i.replace(h, l)] = t.getPropertyValue(i));
                        else
                            for (o in t) - 1 !== o.indexOf("Transform") && Ie !== o || (n[o] = t[o]);
                    else if (t = e.currentStyle || e.style)
                        for (o in t) "string" == typeof o && void 0 === n[o] && (n[o.replace(h, l)] = t[o]);
                    return ie || (n.opacity = r(e)), s = Ge(e, t, !1), n.rotation = s.rotation, n.skewX = s.skewX, n.scaleX = s.scaleX, n.scaleY = s.scaleY, n.x = s.x, n.y = s.y, Re && (n.z = s.z, n.rotationX = s.rotationX, n.rotationY = s.rotationY, n.scaleZ = s.scaleZ), n.filters && delete n.filters, n
                }

                function S(e, t, o, s, i) {
                    var n, l, r, a = {},
                        d = e.style;
                    for (l in o) "cssText" !== l && "length" !== l && isNaN(l) && (t[l] !== (n = o[l]) || i && i[l]) && -1 === l.indexOf("Origin") && ("number" != typeof n && "string" != typeof n || (a[l] = "auto" !== n || "left" !== l && "top" !== l ? "" !== n && "auto" !== n && "none" !== n || "string" != typeof t[l] || "" === t[l].replace(u, "") ? n : 0 : ue(e, l), void 0 !== d[l] && (r = new ye(d, l, d[l], r))));
                    if (s)
                        for (l in s) "className" !== l && (a[l] = s[l]);
                    return {
                        difs: a,
                        firstMPT: r
                    }
                }

                function y(e, t, o) {
                    if ("svg" === (e.nodeName + "").toLowerCase()) return (o || re(e))[t] || 0;
                    if (e.getBBox && Ve(e)) return e.getBBox()[t] || 0;
                    var s = parseFloat("width" === t ? e.offsetWidth : e.offsetHeight),
                        i = ce[t],
                        n = i.length;
                    for (o = o || re(e, null); - 1 < --n;) s -= parseFloat(ae(e, "padding" + i[n], o, !0)) || 0, s -= parseFloat(ae(e, "border" + i[n] + "Width", o, !0)) || 0;
                    return s
                }

                function M(e, t) {
                    return "function" == typeof e && (e = e(O, E)), "string" == typeof e && "=" === e.charAt(1) ? parseInt(e.charAt(0) + "1", 10) * parseFloat(e.substr(2)) : parseFloat(e) - parseFloat(t) || 0
                }

                function F(e, t) {
                    return "function" == typeof e && (e = e(O, E)), null == e ? t : "string" == typeof e && "=" === e.charAt(1) ? parseInt(e.charAt(0) + "1", 10) * parseFloat(e.substr(2)) + t : parseFloat(e) || 0
                }

                function W(e, t, o, s) {
                    var i, n, l, r, a;
                    return "function" == typeof e && (e = e(O, E)), (r = null == e ? t : "number" == typeof e ? e : (i = 360, n = e.split("_"), l = ((a = "=" === e.charAt(1)) ? parseInt(e.charAt(0) + "1", 10) * parseFloat(n[0].substr(2)) : parseFloat(n[0])) * (-1 === e.indexOf("rad") ? 1 : Q) - (a ? 0 : t), n.length && (s && (s[o] = t + l), -1 !== e.indexOf("short") && (l %= i) !== l % 180 && (l = l < 0 ? l + i : l - i), -1 !== e.indexOf("_cw") && l < 0 ? l = (l + 3599999999640) % i - (l / i | 0) * i : -1 !== e.indexOf("ccw") && 0 < l && (l = (l - 3599999999640) % i - (l / i | 0) * i)), t + l)) < 1e-6 && -1e-6 < r && (r = 0), r
                }

                function _(e, t, o) {
                    return 255 * (6 * (e = e < 0 ? e + 1 : 1 < e ? e - 1 : e) < 1 ? t + (o - t) * e * 6 : e < .5 ? o : 3 * e < 2 ? t + (o - t) * (2 / 3 - e) * 6 : t) + .5 | 0
                }

                function s(e, t) {
                    var o, s, i, n = e.match(me) || [],
                        l = 0,
                        r = n.length ? "" : e;
                    for (o = 0; o < n.length; o++) s = n[o], l += (i = e.substr(l, e.indexOf(s, l) - l)).length + s.length, 3 === (s = pe(s, t)).length && s.push(1), r += i + (t ? "hsla(" + s[0] + "," + s[1] + "%," + s[2] + "%," + s[3] : "rgba(" + s.join(",")) + ")";
                    return r + e.substr(l)
                }
                var H, v, P, A, T, C, E, O, o, i, k = /(?:\-|\.|\b)(\d|\.|e\-)+/g,
                    L = /(?:\d|\-\d|\.\d|\-\.\d|\+=\d|\-=\d|\+=.\d|\-=\.\d)+/g,
                    I = /(?:\+=|\-=|\-|\b)[\d\-\.]+[a-zA-Z0-9]*(?:%|\b)/gi,
                    u = /(?![+-]?\d*\.?\d+|[+-]|e[+-]\d+)[^0-9]/g,
                    x = /(?:\d|\-|\+|=|#|\.)*/g,
                    R = /opacity *= *([^)]*)/i,
                    Y = /opacity:([^;]*)/i,
                    a = /alpha\(opacity *=.+?\)/i,
                    X = /^(rgb|hsl)/,
                    c = /([A-Z])/g,
                    h = /-([a-z])/gi,
                    V = /(^(?:url\(\"|url\())|(?:(\"\))$|\)$)/gi,
                    j = /(?:Left|Right|Width)/i,
                    z = /(M11|M12|M21|M22)=[\d\-\.e]+/gi,
                    G = /progid\:DXImageTransform\.Microsoft\.Matrix\(.+?\)/i,
                    q = /,(?=[^\)]*(?:\(|$))/gi,
                    K = /[\s,\(]/i,
                    J = Math.PI / 180,
                    Q = 180 / Math.PI,
                    $ = {},
                    Z = document,
                    ee = t("div"),
                    te = t("img"),
                    oe = N._internals = {
                        _specialProps: m
                    },
                    se = navigator.userAgent,
                    ie = (o = se.indexOf("Android"), i = t("a"), P = -1 !== se.indexOf("Safari") && -1 === se.indexOf("Chrome") && (-1 === o || 3 < Number(se.substr(o + 8, 1))), T = P && Number(se.substr(se.indexOf("Version/") + 8, 1)) < 6, A = -1 !== se.indexOf("Firefox"), (/MSIE ([0-9]{1,}[\.0-9]{0,})/.exec(se) || /Trident\/.*rv:([0-9]{1,}[\.0-9]{0,})/.exec(se)) && (C = parseFloat(RegExp.$1)), !!i && (i.style.cssText = "top:1px;opacity:.55;", /^0.55/.test(i.style.opacity))),
                    ne = "",
                    le = "",
                    re = Z.defaultView ? Z.defaultView.getComputedStyle : function() {},
                    ae = N.getStyle = function(e, t, o, s, i) {
                        var n;
                        return ie || "opacity" !== t ? (!s && e.style[t] ? n = e.style[t] : (o = o || re(e)) ? n = o[t] || o.getPropertyValue(t) || o.getPropertyValue(t.replace(c, "-$1").toLowerCase()) : e.currentStyle && (n = e.currentStyle[t]), null == i || n && "none" !== n && "auto" !== n && "auto auto" !== n ? n : i) : r(e)
                    },
                    de = oe.convertToPixels = function(e, t, o, s, i) {
                        if ("px" === s || !s) return o;
                        if ("auto" === s || !o) return 0;
                        var n, l, r, a = j.test(t),
                            d = e,
                            u = ee.style,
                            c = o < 0,
                            h = 1 === o;
                        if (c && (o = -o), h && (o *= 100), "%" === s && -1 !== t.indexOf("border")) n = o / 100 * (a ? e.clientWidth : e.clientHeight);
                        else {
                            if (u.cssText = "border:0 solid red;position:" + ae(e, "position") + ";line-height:0;", "%" !== s && d.appendChild && "v" !== s.charAt(0) && "rem" !== s) u[a ? "borderLeftWidth" : "borderTopWidth"] = o + s;
                            else {
                                if (l = (d = e.parentNode || Z.body)._gsCache, r = U.ticker.frame, l && a && l.time === r) return l.width * o / 100;
                                u[a ? "width" : "height"] = o + s
                            }
                            d.appendChild(ee), n = parseFloat(ee[a ? "offsetWidth" : "offsetHeight"]), d.removeChild(ee), a && "%" === s && !1 !== N.cacheWidths && ((l = d._gsCache = d._gsCache || {}).time = r, l.width = n / o * 100), 0 !== n || i || (n = de(e, t, o, s, !0))
                        }
                        return h && (n /= 100), c ? -n : n
                    },
                    ue = oe.calculateOffset = function(e, t, o) {
                        if ("absolute" !== ae(e, "position", o)) return 0;
                        var s = "left" === t ? "Left" : "Top",
                            i = ae(e, "margin" + s, o);
                        return e["offset" + s] - (de(e, t, parseFloat(i), i.replace(x, "")) || 0)
                    },
                    ce = {
                        width: ["Left", "Right"],
                        height: ["Top", "Bottom"]
                    },
                    he = ["marginLeft", "marginRight", "marginTop", "marginBottom"],
                    _e = function(e, t) {
                        if ("contain" === e || "auto" === e || "auto auto" === e) return e + " ";
                        null != e && "" !== e || (e = "0 0");
                        var o, s = e.split(" "),
                            i = -1 !== e.indexOf("left") ? "0%" : -1 !== e.indexOf("right") ? "100%" : s[0],
                            n = -1 !== e.indexOf("top") ? "0%" : -1 !== e.indexOf("bottom") ? "100%" : s[1];
                        if (3 < s.length && !t) {
                            for (s = e.split(", ").join(",").split(","), e = [], o = 0; o < s.length; o++) e.push(_e(s[o]));
                            return e.join(",")
                        }
                        return null == n ? n = "center" === i ? "50%" : "0" : "center" === n && (n = "50%"), ("center" === i || isNaN(parseFloat(i)) && -1 === (i + "").indexOf("=")) && (i = "50%"), e = i + " " + n + (2 < s.length ? " " + s[2] : ""), t && (t.oxp = -1 !== i.indexOf("%"), t.oyp = -1 !== n.indexOf("%"), t.oxr = "=" === i.charAt(1), t.oyr = "=" === n.charAt(1), t.ox = parseFloat(i.replace(u, "")), t.oy = parseFloat(n.replace(u, "")), t.v = e), t || e
                    },
                    fe = {
                        aqua: [0, 255, 255],
                        lime: [0, 255, 0],
                        silver: [192, 192, 192],
                        black: [0, 0, 0],
                        maroon: [128, 0, 0],
                        teal: [0, 128, 128],
                        blue: [0, 0, 255],
                        navy: [0, 0, 128],
                        white: [255, 255, 255],
                        fuchsia: [255, 0, 255],
                        olive: [128, 128, 0],
                        yellow: [255, 255, 0],
                        orange: [255, 165, 0],
                        gray: [128, 128, 128],
                        purple: [128, 0, 128],
                        green: [0, 128, 0],
                        red: [255, 0, 0],
                        pink: [255, 192, 203],
                        cyan: [0, 255, 255],
                        transparent: [255, 255, 255, 0]
                    },
                    pe = N.parseColor = function(e, t) {
                        var o, s, i, n, l, r, a, d, u, c, h;
                        if (e)
                            if ("number" == typeof e) o = [e >> 16, e >> 8 & 255, 255 & e];
                            else {
                                if ("," === e.charAt(e.length - 1) && (e = e.substr(0, e.length - 1)), fe[e]) o = fe[e];
                                else if ("#" === e.charAt(0)) 4 === e.length && (e = "#" + (s = e.charAt(1)) + s + (i = e.charAt(2)) + i + (n = e.charAt(3)) + n), o = [(e = parseInt(e.substr(1), 16)) >> 16, e >> 8 & 255, 255 & e];
                                else if ("hsl" === e.substr(0, 3))
                                    if (o = h = e.match(k), t) {
                                        if (-1 !== e.indexOf("=")) return e.match(L)
                                    } else l = Number(o[0]) % 360 / 360, r = Number(o[1]) / 100, s = 2 * (a = Number(o[2]) / 100) - (i = a <= .5 ? a * (r + 1) : a + r - a * r), 3 < o.length && (o[3] = Number(e[3])), o[0] = _(l + 1 / 3, s, i), o[1] = _(l, s, i), o[2] = _(l - 1 / 3, s, i);
                                else o = e.match(k) || fe.transparent;
                                o[0] = Number(o[0]), o[1] = Number(o[1]), o[2] = Number(o[2]), 3 < o.length && (o[3] = Number(o[3]))
                            }
                        else o = fe.black;
                        return t && !h && (s = o[0] / 255, i = o[1] / 255, n = o[2] / 255, a = ((d = Math.max(s, i, n)) + (u = Math.min(s, i, n))) / 2, d === u ? l = r = 0 : (c = d - u, r = .5 < a ? c / (2 - d - u) : c / (d + u), l = d === s ? (i - n) / c + (i < n ? 6 : 0) : d === i ? (n - s) / c + 2 : (s - i) / c + 4, l *= 60), o[0] = l + .5 | 0, o[1] = 100 * r + .5 | 0, o[2] = 100 * a + .5 | 0), o
                    },
                    me = "(?:\\b(?:(?:rgb|rgba|hsl|hsla)\\(.+?\\))|\\B#(?:[0-9a-f]{3}){1,2}\\b";
                for (e in fe) me += "|" + e + "\\b";
                me = new RegExp(me + ")", "gi"), N.colorStringFilter = function(e) {
                    var t, o = e[0] + e[1];
                    me.test(o) && (t = -1 !== o.indexOf("hsl(") || -1 !== o.indexOf("hsla("), e[0] = s(e[0], t), e[1] = s(e[1], t)), me.lastIndex = 0
                }, U.defaultStringFilter || (U.defaultStringFilter = N.colorStringFilter);

                function be(e, t, n, l) {
                    if (null == e) return function(e) {
                        return e
                    };
                    var r, a = t ? (e.match(me) || [""])[0] : "",
                        d = e.split(a).join("").match(I) || [],
                        u = e.substr(0, e.indexOf(d[0])),
                        c = ")" === e.charAt(e.length - 1) ? ")" : "",
                        h = -1 !== e.indexOf(" ") ? " " : ",",
                        _ = d.length,
                        f = 0 < _ ? d[0].replace(k, "") : "";
                    return _ ? r = t ? function(e) {
                        var t, o, s, i;
                        if ("number" == typeof e) e += f;
                        else if (l && q.test(e)) {
                            for (i = e.replace(q, "|").split("|"), s = 0; s < i.length; s++) i[s] = r(i[s]);
                            return i.join(",")
                        }
                        if (t = (e.match(me) || [a])[0], s = (o = e.split(t).join("").match(I) || []).length, _ > s--)
                            for (; ++s < _;) o[s] = n ? o[(s - 1) / 2 | 0] : d[s];
                        return u + o.join(h) + h + t + c + (-1 !== e.indexOf("inset") ? " inset" : "")
                    } : function(e) {
                        var t, o, s;
                        if ("number" == typeof e) e += f;
                        else if (l && q.test(e)) {
                            for (o = e.replace(q, "|").split("|"), s = 0; s < o.length; s++) o[s] = r(o[s]);
                            return o.join(",")
                        }
                        if (s = (t = e.match(I) || []).length, _ > s--)
                            for (; ++s < _;) t[s] = n ? t[(s - 1) / 2 | 0] : d[s];
                        return u + t.join(h) + c
                    } : function(e) {
                        return e
                    }
                }

                function ge(d) {
                    return d = d.split(","),
                        function(e, t, o, s, i, n, l) {
                            var r, a = (t + "").split(" ");
                            for (l = {}, r = 0; r < 4; r++) l[d[r]] = a[r] = a[r] || a[(r - 1) / 2 >> 0];
                            return s.parse(e, l, i, n)
                        }
                }
                oe._setPluginRatio = function(e) {
                    this.plugin.setRatio(e);
                    for (var t, o, s, i, n, l = this.data, r = l.proxy, a = l.firstMPT; a;) t = r[a.v], a.r ? t = Math.round(t) : t < 1e-6 && -1e-6 < t && (t = 0), a.t[a.p] = t, a = a._next;
                    if (l.autoRotate && (l.autoRotate.rotation = l.mod ? l.mod(r.rotation, this.t) : r.rotation), 1 === e || 0 === e)
                        for (a = l.firstMPT, n = 1 === e ? "e" : "b"; a;) {
                            if ((o = a.t).type) {
                                if (1 === o.type) {
                                    for (i = o.xs0 + o.s + o.xs1, s = 1; s < o.l; s++) i += o["xn" + s] + o["xs" + (s + 1)];
                                    o[n] = i
                                }
                            } else o[n] = o.s + o.xs0;
                            a = a._next
                        }
                };

                function Se(e, t, o, s, i, n) {
                    var l = new ve(e, t, o, s - o, i, -1, n);
                    return l.b = o, l.e = l.xs0 = s, l
                }
                var ye = function(e, t, o, s, i) {
                        this.t = e, this.p = t, this.v = o, this.r = i, s && ((s._prev = this)._next = s)
                    },
                    ve = (oe._parseToProxy = function(e, t, o, s, i, n) {
                        var l, r, a, d, u, c = s,
                            h = {},
                            _ = {},
                            f = o._transform,
                            p = $;
                        for (o._transform = null, $ = t, s = u = o.parse(e, t, s, i), $ = p, n && (o._transform = f, c && (c._prev = null, c._prev && (c._prev._next = null))); s && s !== c;) {
                            if (s.type <= 1 && (_[r = s.p] = s.s + s.c, h[r] = s.s, n || (d = new ye(s, "s", r, d, s.r), s.c = 0), 1 === s.type))
                                for (l = s.l; 0 < --l;) a = "xn" + l, _[r = s.p + "_" + a] = s.data[a], h[r] = s[a], n || (d = new ye(s, a, r, d, s.rxp[a]));
                            s = s._next
                        }
                        return {
                            proxy: h,
                            end: _,
                            firstMPT: d,
                            pt: u
                        }
                    }, oe.CSSPropTween = function(e, t, o, s, i, n, l, r, a, d, u) {
                        this.t = e, this.p = t, this.s = o, this.c = s, this.n = l || t, e instanceof ve || p.push(this.n), this.r = r, this.type = n || 0, a && (this.pr = a, f = !0), this.b = void 0 === d ? o : d, this.e = void 0 === u ? o + s : u, i && ((this._next = i)._prev = this)
                    }),
                    Pe = N.parseComplex = function(e, t, o, s, i, n, l, r, a, d) {
                        o = o || n || "", "function" == typeof s && (s = s(O, E)), l = new ve(e, t, 0, 0, l, d ? 2 : 1, null, !1, r, o, s), s += "", i && me.test(s + o) && (s = [o, s], N.colorStringFilter(s), o = s[0], s = s[1]);
                        var u, c, h, _, f, p, m, b, g, S, y, v, P, T = o.split(", ").join(",").split(" "),
                            w = s.split(", ").join(",").split(" "),
                            D = T.length,
                            B = !1 !== H;
                        for (-1 === s.indexOf(",") && -1 === o.indexOf(",") || (T = T.join(" ").replace(q, ", ").split(" "), w = w.join(" ").replace(q, ", ").split(" "), D = T.length), D !== w.length && (D = (T = (n || "").split(" ")).length), l.plugin = a, l.setRatio = d, u = me.lastIndex = 0; u < D; u++)
                            if (_ = T[u], f = w[u], (b = parseFloat(_)) || 0 === b) l.appendXtra("", b, M(f, b), f.replace(L, ""), B && -1 !== f.indexOf("px"), !0);
                            else if (i && me.test(_)) v = ")" + ((v = f.indexOf(")") + 1) ? f.substr(v) : ""), P = -1 !== f.indexOf("hsl") && ie, _ = pe(_, P), f = pe(f, P), (g = 6 < _.length + f.length) && !ie && 0 === f[3] ? (l["xs" + l.l] += l.l ? " transparent" : "transparent", l.e = l.e.split(w[u]).join("transparent")) : (ie || (g = !1), P ? l.appendXtra(g ? "hsla(" : "hsl(", _[0], M(f[0], _[0]), ",", !1, !0).appendXtra("", _[1], M(f[1], _[1]), "%,", !1).appendXtra("", _[2], M(f[2], _[2]), g ? "%," : "%" + v, !1) : l.appendXtra(g ? "rgba(" : "rgb(", _[0], f[0] - _[0], ",", !0, !0).appendXtra("", _[1], f[1] - _[1], ",", !0).appendXtra("", _[2], f[2] - _[2], g ? "," : v, !0), g && (_ = _.length < 4 ? 1 : _[3], l.appendXtra("", _, (f.length < 4 ? 1 : f[3]) - _, v, !1))), me.lastIndex = 0;
                        else if (p = _.match(k)) {
                            if (!(m = f.match(L)) || m.length !== p.length) return l;
                            for (c = h = 0; c < p.length; c++) y = p[c], S = _.indexOf(y, h), l.appendXtra(_.substr(h, S - h), Number(y), M(m[c], y), "", B && "px" === _.substr(S + y.length, 2), 0 === c), h = S + y.length;
                            l["xs" + l.l] += _.substr(h)
                        } else l["xs" + l.l] += l.l || l["xs" + l.l] ? " " + f : f;
                        if (-1 !== s.indexOf("=") && l.data) {
                            for (v = l.xs0 + l.data.s, u = 1; u < l.l; u++) v += l["xs" + u] + l.data["xn" + u];
                            l.e = v + l["xs" + u]
                        }
                        return l.l || (l.type = -1, l.xs0 = l.e), l.xfirst || l
                    },
                    Te = 9;
                for ((e = ve.prototype).l = e.pr = 0; 0 < --Te;) e["xn" + Te] = 0, e["xs" + Te] = "";
                e.xs0 = "", e._next = e._prev = e.xfirst = e.data = e.plugin = e.setRatio = e.rxp = null, e.appendXtra = function(e, t, o, s, i, n) {
                    var l = this,
                        r = l.l;
                    return l["xs" + r] += n && (r || l["xs" + r]) ? " " + e : e || "", o || 0 === r || l.plugin ? (l.l++, l.type = l.setRatio ? 2 : 1, l["xs" + l.l] = s || "", 0 < r ? (l.data["xn" + r] = t + o, l.rxp["xn" + r] = i, l["xn" + r] = t, l.plugin || (l.xfirst = new ve(l, "xn" + r, t, o, l.xfirst || l, 0, l.n, i, l.pr), l.xfirst.xs0 = 0)) : (l.data = {
                        s: t + o
                    }, l.rxp = {}, l.s = t, l.c = o, l.r = i), l) : (l["xs" + r] += t + (s || ""), l)
                };

                function we(e, t) {
                    t = t || {}, this.p = t.prefix && B(e) || e, m[e] = m[this.p] = this, this.format = t.formatter || be(t.defaultValue, t.color, t.collapsible, t.multi), t.parser && (this.parse = t.parser), this.clrs = t.color, this.multi = t.multi, this.keyword = t.keyword, this.dflt = t.defaultValue, this.pr = t.priority || 0
                }
                var De = oe._registerComplexSpecialProp = function(e, t, o) {
                        "object" != typeof t && (t = {
                            parser: o
                        });
                        var s, i = e.split(","),
                            n = t.defaultValue;
                        for (o = o || [n], s = 0; s < i.length; s++) t.prefix = 0 === s && t.prefix, t.defaultValue = o[s] || n, new we(i[s], t)
                    },
                    Be = oe._registerPluginProp = function(e) {
                        if (!m[e]) {
                            var a = e.charAt(0).toUpperCase() + e.substr(1) + "Plugin";
                            De(e, {
                                parser: function(e, t, o, s, i, n, l) {
                                    var r = d.com.greensock.plugins[a];
                                    return r ? (r._cssRegister(), m[o].parse(e, t, o, s, i, n, l)) : (b("Error: " + a + " js file not loaded."), i)
                                }
                            })
                        }
                    };
                (e = we.prototype).parseComplex = function(e, t, o, s, i, n) {
                    var l, r, a, d, u, c, h = this.keyword;
                    if (this.multi && (q.test(o) || q.test(t) ? (r = t.replace(q, "|").split("|"), a = o.replace(q, "|").split("|")) : h && (r = [t], a = [o])), a) {
                        for (d = a.length > r.length ? a.length : r.length, l = 0; l < d; l++) t = r[l] = r[l] || this.dflt, o = a[l] = a[l] || this.dflt, h && (u = t.indexOf(h)) !== (c = o.indexOf(h)) && (-1 === c ? r[l] = r[l].split(h).join("") : -1 === u && (r[l] += " " + h));
                        t = r.join(", "), o = a.join(", ")
                    }
                    return Pe(e, this.p, t, o, this.clrs, this.dflt, s, this.pr, i, n)
                }, e.parse = function(e, t, o, s, i, n, l) {
                    return this.parseComplex(e.style, this.format(ae(e, this.p, D, !1, this.dflt)), this.format(t), i, n)
                }, N.registerSpecialProp = function(e, a, d) {
                    De(e, {
                        parser: function(e, t, o, s, i, n, l) {
                            var r = new ve(e, o, 0, 0, i, 2, o, !1, d);
                            return r.plugin = n, r.setRatio = a(e, t, s._tween, o), r
                        },
                        priority: d
                    })
                }, N.useSVGTransformAttr = P || A;

                function Me(e, t, o) {
                    var s, i = Z.createElementNS("http://www.w3.org/2000/svg", e),
                        n = /([a-z])([A-Z])/g;
                    for (s in o) i.setAttributeNS(null, s.replace(n, "$1-$2").toLowerCase(), o[s]);
                    return t.appendChild(i), i
                }

                function Fe(e, t, o, s, i, n) {
                    var l, r, a, d, u, c, h, _, f, p, m, b, g, S, y = e._gsTransform,
                        v = ze(e, !0);
                    y && (g = y.xOrigin, S = y.yOrigin), (!s || (l = s.split(" ")).length < 2) && (h = e.getBBox(), l = [(-1 !== (t = _e(t).split(" "))[0].indexOf("%") ? parseFloat(t[0]) / 100 * h.width : parseFloat(t[0])) + h.x, (-1 !== t[1].indexOf("%") ? parseFloat(t[1]) / 100 * h.height : parseFloat(t[1])) + h.y]), o.xOrigin = d = parseFloat(l[0]), o.yOrigin = u = parseFloat(l[1]), s && v !== je && (c = v[0], h = v[1], _ = v[2], f = v[3], p = v[4], r = d * (f / (b = c * f - h * _)) + u * (-_ / b) + (_ * (m = v[5]) - f * p) / b, a = d * (-h / b) + u * (c / b) - (c * m - h * p) / b, d = o.xOrigin = l[0] = r, u = o.yOrigin = l[1] = a), y && (n && (o.xOffset = y.xOffset, o.yOffset = y.yOffset, y = o), i || !1 !== i && !1 !== N.defaultSmoothOrigin ? (r = d - g, a = u - S, y.xOffset += r * v[0] + a * v[2] - r, y.yOffset += r * v[1] + a * v[3] - a) : y.xOffset = y.yOffset = 0), n || e.setAttribute("data-svg-origin", l.join(" "))
                }

                function We(e) {
                    var t, o, s = this.data,
                        i = -s.rotation * J,
                        n = i + s.skewX * J,
                        l = 1e5,
                        r = (Math.cos(i) * s.scaleX * l | 0) / l,
                        a = (Math.sin(i) * s.scaleX * l | 0) / l,
                        d = (Math.sin(n) * -s.scaleY * l | 0) / l,
                        u = (Math.cos(n) * s.scaleY * l | 0) / l,
                        c = this.t.style,
                        h = this.t.currentStyle;
                    if (h) {
                        o = a, a = -d, d = -o, t = h.filter, c.filter = "";
                        var _, f, p = this.t.offsetWidth,
                            m = this.t.offsetHeight,
                            b = "absolute" !== h.position,
                            g = "progid:DXImageTransform.Microsoft.Matrix(M11=" + r + ", M12=" + a + ", M21=" + d + ", M22=" + u,
                            S = s.x + p * s.xPercent / 100,
                            y = s.y + m * s.yPercent / 100;
                        if (null != s.ox && (S += (_ = (s.oxp ? p * s.ox * .01 : s.ox) - p / 2) - (_ * r + (f = (s.oyp ? m * s.oy * .01 : s.oy) - m / 2) * a), y += f - (_ * d + f * u)), g += b ? ", Dx=" + ((_ = p / 2) - (_ * r + (f = m / 2) * a) + S) + ", Dy=" + (f - (_ * d + f * u) + y) + ")" : ", sizingMethod='auto expand')", -1 !== t.indexOf("DXImageTransform.Microsoft.Matrix(") ? c.filter = t.replace(G, g) : c.filter = g + " " + t, 0 !== e && 1 !== e || 1 == r && 0 === a && 0 === d && 1 == u && (b && -1 === g.indexOf("Dx=0, Dy=0") || R.test(t) && 100 !== parseFloat(RegExp.$1) || -1 === t.indexOf(t.indexOf("Alpha")) && c.removeAttribute("filter")), !b) {
                            var v, P, T, w = C < 8 ? 1 : -1;
                            for (_ = s.ieOffsetX || 0, f = s.ieOffsetY || 0, s.ieOffsetX = Math.round((p - ((r < 0 ? -r : r) * p + (a < 0 ? -a : a) * m)) / 2 + S), s.ieOffsetY = Math.round((m - ((u < 0 ? -u : u) * m + (d < 0 ? -d : d) * p)) / 2 + y), Te = 0; Te < 4; Te++) T = (o = -1 !== (v = h[P = he[Te]]).indexOf("px") ? parseFloat(v) : de(this.t, P, parseFloat(v), v.replace(x, "")) || 0) !== s[P] ? Te < 2 ? -s.ieOffsetX : -s.ieOffsetY : Te < 2 ? _ - s.ieOffsetX : f - s.ieOffsetY, c[P] = (s[P] = Math.round(o - T * (0 === Te || 2 === Te ? 1 : w))) + "px"
                        }
                    }
                }
                var He, Ce, Ee, Oe, ke, Le = "scaleX,scaleY,scaleZ,x,y,z,skewX,skewY,rotation,rotationX,rotationY,perspective,xPercent,yPercent".split(","),
                    Ie = B("transform"),
                    xe = ne + "transform",
                    Ae = B("transformOrigin"),
                    Re = null !== B("perspective"),
                    Ue = oe.Transform = function() {
                        this.perspective = parseFloat(N.defaultTransformPerspective) || 0, this.force3D = !(!1 === N.defaultForce3D || !Re) && (N.defaultForce3D || "auto")
                    },
                    Ne = window.SVGElement,
                    Ye = Z.documentElement,
                    Xe = (ke = C || /Android/i.test(se) && !window.chrome, Z.createElementNS && !ke && (Ce = Me("svg", Ye), Oe = (Ee = Me("rect", Ce, {
                        width: 100,
                        height: 50,
                        x: 100
                    })).getBoundingClientRect().width, Ee.style[Ae] = "50% 50%", Ee.style[Ie] = "scaleX(0.5)", ke = Oe === Ee.getBoundingClientRect().width && !(A && Re), Ye.removeChild(Ce)), ke),
                    Ve = function(e) {
                        return !!(Ne && e.getBBox && e.getCTM && function(e) {
                            try {
                                return e.getBBox()
                            } catch (e) {}
                        }(e) && (!e.parentNode || e.parentNode.getBBox && e.parentNode.getCTM))
                    },
                    je = [1, 0, 0, 1, 0, 0],
                    ze = function(e, t) {
                        var o, s, i, n, l, r, a = e._gsTransform || new Ue,
                            d = e.style;
                        if (Ie ? s = ae(e, xe, null, !0) : e.currentStyle && (s = (s = e.currentStyle.filter.match(z)) && 4 === s.length ? [s[0].substr(4), Number(s[2].substr(4)), Number(s[1].substr(4)), s[3].substr(4), a.x || 0, a.y || 0].join(",") : ""), (o = !s || "none" === s || "matrix(1, 0, 0, 1, 0, 0)" === s) && Ie && ((r = "none" === re(e).display) || !e.parentNode) && (r && (n = d.display, d.display = "block"), e.parentNode || (l = 1, Ye.appendChild(e)), o = !(s = ae(e, xe, null, !0)) || "none" === s || "matrix(1, 0, 0, 1, 0, 0)" === s, n ? d.display = n : r && Qe(d, "display"), l && Ye.removeChild(e)), (a.svg || e.getBBox && Ve(e)) && (o && -1 !== (d[Ie] + "").indexOf("matrix") && (s = d[Ie], o = 0), i = e.getAttribute("transform"), o && i && (-1 !== i.indexOf("matrix") ? (s = i, o = 0) : -1 !== i.indexOf("translate") && (s = "matrix(1,0,0,1," + i.match(/(?:\-|\b)[\d\-\.e]+\b/gi).join(",") + ")", o = 0))), o) return je;
                        for (i = (s || "").match(k) || [], Te = i.length; - 1 < --Te;) n = Number(i[Te]), i[Te] = (l = n - (n |= 0)) ? (1e5 * l + (l < 0 ? -.5 : .5) | 0) / 1e5 + n : n;
                        return t && 6 < i.length ? [i[0], i[1], i[4], i[5], i[12], i[13]] : i
                    },
                    Ge = oe.getTransform = function(e, t, o, s) {
                        if (e._gsTransform && o && !s) return e._gsTransform;
                        var i, n, l, r, a, d, u = o && e._gsTransform || new Ue,
                            c = u.scaleX < 0,
                            h = Re && (parseFloat(ae(e, Ae, t, !1, "0 0 0").split(" ")[2]) || u.zOrigin) || 0,
                            _ = parseFloat(N.defaultTransformPerspective) || 0;
                        if (u.svg = !(!e.getBBox || !Ve(e)), u.svg && (Fe(e, ae(e, Ae, t, !1, "50% 50%") + "", u, e.getAttribute("data-svg-origin")), He = N.useSVGTransformAttr || Xe), (i = ze(e)) !== je) {
                            if (16 === i.length) {
                                var f, p, m, b, g, S = i[0],
                                    y = i[1],
                                    v = i[2],
                                    P = i[3],
                                    T = i[4],
                                    w = i[5],
                                    D = i[6],
                                    B = i[7],
                                    M = i[8],
                                    F = i[9],
                                    W = i[10],
                                    H = i[12],
                                    C = i[13],
                                    E = i[14],
                                    O = i[11],
                                    k = Math.atan2(D, W);
                                u.zOrigin && (H = M * (E = -u.zOrigin) - i[12], C = F * E - i[13], E = W * E + u.zOrigin - i[14]), u.rotationX = k * Q, k && (f = T * (b = Math.cos(-k)) + M * (g = Math.sin(-k)), p = w * b + F * g, m = D * b + W * g, M = T * -g + M * b, F = w * -g + F * b, W = D * -g + W * b, O = B * -g + O * b, T = f, w = p, D = m), k = Math.atan2(-v, W), u.rotationY = k * Q, k && (p = y * (b = Math.cos(-k)) - F * (g = Math.sin(-k)), m = v * b - W * g, F = y * g + F * b, W = v * g + W * b, O = P * g + O * b, S = f = S * b - M * g, y = p, v = m), k = Math.atan2(y, S), u.rotation = k * Q, k && (S = S * (b = Math.cos(-k)) + T * (g = Math.sin(-k)), p = y * b + w * g, w = y * -g + w * b, D = v * -g + D * b, y = p), u.rotationX && 359.9 < Math.abs(u.rotationX) + Math.abs(u.rotation) && (u.rotationX = u.rotation = 0, u.rotationY = 180 - u.rotationY), u.scaleX = (1e5 * Math.sqrt(S * S + y * y) + .5 | 0) / 1e5, u.scaleY = (1e5 * Math.sqrt(w * w + F * F) + .5 | 0) / 1e5, u.scaleZ = (1e5 * Math.sqrt(D * D + W * W) + .5 | 0) / 1e5, u.rotationX || u.rotationY ? u.skewX = 0 : (u.skewX = T || w ? Math.atan2(T, w) * Q + u.rotation : u.skewX || 0, 90 < Math.abs(u.skewX) && Math.abs(u.skewX) < 270 && (c ? (u.scaleX *= -1, u.skewX += u.rotation <= 0 ? 180 : -180, u.rotation += u.rotation <= 0 ? 180 : -180) : (u.scaleY *= -1, u.skewX += u.skewX <= 0 ? 180 : -180))), u.perspective = O ? 1 / (O < 0 ? -O : O) : 0, u.x = H, u.y = C, u.z = E, u.svg && (u.x -= u.xOrigin - (u.xOrigin * S - u.yOrigin * T), u.y -= u.yOrigin - (u.yOrigin * y - u.xOrigin * w))
                            } else if (!Re || s || !i.length || u.x !== i[4] || u.y !== i[5] || !u.rotationX && !u.rotationY) {
                                var L = 6 <= i.length,
                                    I = L ? i[0] : 1,
                                    x = i[1] || 0,
                                    A = i[2] || 0,
                                    R = L ? i[3] : 1;
                                u.x = i[4] || 0, u.y = i[5] || 0, l = Math.sqrt(I * I + x * x), r = Math.sqrt(R * R + A * A), a = I || x ? Math.atan2(x, I) * Q : u.rotation || 0, d = A || R ? Math.atan2(A, R) * Q + a : u.skewX || 0, 90 < Math.abs(d) && Math.abs(d) < 270 && (c ? (l *= -1, d += a <= 0 ? 180 : -180, a += a <= 0 ? 180 : -180) : (r *= -1, d += d <= 0 ? 180 : -180)), u.scaleX = l, u.scaleY = r, u.rotation = a, u.skewX = d, Re && (u.rotationX = u.rotationY = u.z = 0, u.perspective = _, u.scaleZ = 1), u.svg && (u.x -= u.xOrigin - (u.xOrigin * I + u.yOrigin * A), u.y -= u.yOrigin - (u.xOrigin * x + u.yOrigin * R))
                            }
                            for (n in u.zOrigin = h, u) u[n] < 2e-5 && -2e-5 < u[n] && (u[n] = 0)
                        }
                        return o && (e._gsTransform = u).svg && (He && e.style[Ie] ? U.delayedCall(.001, function() {
                            Qe(e.style, Ie)
                        }) : !He && e.getAttribute("transform") && U.delayedCall(.001, function() {
                            e.removeAttribute("transform")
                        })), u
                    },
                    qe = oe.set3DTransformRatio = oe.setTransformRatio = function(e) {
                        var t, o, s, i, n, l, r, a, d, u, c, h, _, f, p, m, b, g, S, y, v, P, T, w = this.data,
                            D = this.t.style,
                            B = w.rotation,
                            M = w.rotationX,
                            F = w.rotationY,
                            W = w.scaleX,
                            H = w.scaleY,
                            C = w.scaleZ,
                            E = w.x,
                            O = w.y,
                            k = w.z,
                            L = w.svg,
                            I = w.perspective,
                            x = w.force3D;
                        if (!((1 !== e && 0 !== e || "auto" !== x || this.tween._totalTime !== this.tween._totalDuration && this.tween._totalTime) && x || k || I || F || M || 1 !== C) || He && L || !Re) B || w.skewX || L ? (B *= J, P = w.skewX * J, T = 1e5, t = Math.cos(B) * W, i = Math.sin(B) * W, o = Math.sin(B - P) * -H, n = Math.cos(B - P) * H, P && "simple" === w.skewType && (b = Math.tan(P - w.skewY * J), o *= b = Math.sqrt(1 + b * b), n *= b, w.skewY && (b = Math.tan(w.skewY * J), t *= b = Math.sqrt(1 + b * b), i *= b)), L && (E += w.xOrigin - (w.xOrigin * t + w.yOrigin * o) + w.xOffset, O += w.yOrigin - (w.xOrigin * i + w.yOrigin * n) + w.yOffset, He && (w.xPercent || w.yPercent) && (f = this.t.getBBox(), E += .01 * w.xPercent * f.width, O += .01 * w.yPercent * f.height), E < (f = 1e-6) && -f < E && (E = 0), O < f && -f < O && (O = 0)), S = (t * T | 0) / T + "," + (i * T | 0) / T + "," + (o * T | 0) / T + "," + (n * T | 0) / T + "," + E + "," + O + ")", L && He ? this.t.setAttribute("transform", "matrix(" + S) : D[Ie] = (w.xPercent || w.yPercent ? "translate(" + w.xPercent + "%," + w.yPercent + "%) matrix(" : "matrix(") + S) : D[Ie] = (w.xPercent || w.yPercent ? "translate(" + w.xPercent + "%," + w.yPercent + "%) matrix(" : "matrix(") + W + ",0,0," + H + "," + E + "," + O + ")";
                        else {
                            if (A && (W < (f = 1e-4) && -f < W && (W = C = 2e-5), H < f && -f < H && (H = C = 2e-5), !I || w.z || w.rotationX || w.rotationY || (I = 0)), B || w.skewX) B *= J, p = t = Math.cos(B), m = i = Math.sin(B), w.skewX && (B -= w.skewX * J, p = Math.cos(B), m = Math.sin(B), "simple" === w.skewType && (b = Math.tan((w.skewX - w.skewY) * J), p *= b = Math.sqrt(1 + b * b), m *= b, w.skewY && (b = Math.tan(w.skewY * J), t *= b = Math.sqrt(1 + b * b), i *= b))), o = -m, n = p;
                            else {
                                if (!(F || M || 1 !== C || I || L)) return void(D[Ie] = (w.xPercent || w.yPercent ? "translate(" + w.xPercent + "%," + w.yPercent + "%) translate3d(" : "translate3d(") + E + "px," + O + "px," + k + "px)" + (1 !== W || 1 !== H ? " scale(" + W + "," + H + ")" : ""));
                                t = n = 1, o = i = 0
                            }
                            d = 1, s = l = r = a = u = c = 0, h = I ? -1 / I : 0, _ = w.zOrigin, f = 1e-6, y = ",", v = "0", (B = F * J) && (p = Math.cos(B), u = h * (r = -(m = Math.sin(B))), s = t * m, l = i * m, h *= d = p, t *= p, i *= p), (B = M * J) && (b = o * (p = Math.cos(B)) + s * (m = Math.sin(B)), g = n * p + l * m, a = d * m, c = h * m, s = o * -m + s * p, l = n * -m + l * p, d *= p, h *= p, o = b, n = g), 1 !== C && (s *= C, l *= C, d *= C, h *= C), 1 !== H && (o *= H, n *= H, a *= H, c *= H), 1 !== W && (t *= W, i *= W, r *= W, u *= W), (_ || L) && (_ && (E += s * -_, O += l * -_, k += d * -_ + _), L && (E += w.xOrigin - (w.xOrigin * t + w.yOrigin * o) + w.xOffset, O += w.yOrigin - (w.xOrigin * i + w.yOrigin * n) + w.yOffset), E < f && -f < E && (E = v), O < f && -f < O && (O = v), k < f && -f < k && (k = 0)), S = w.xPercent || w.yPercent ? "translate(" + w.xPercent + "%," + w.yPercent + "%) matrix3d(" : "matrix3d(", S += (t < f && -f < t ? v : t) + y + (i < f && -f < i ? v : i) + y + (r < f && -f < r ? v : r), S += y + (u < f && -f < u ? v : u) + y + (o < f && -f < o ? v : o) + y + (n < f && -f < n ? v : n), M || F || 1 !== C ? (S += y + (a < f && -f < a ? v : a) + y + (c < f && -f < c ? v : c) + y + (s < f && -f < s ? v : s), S += y + (l < f && -f < l ? v : l) + y + (d < f && -f < d ? v : d) + y + (h < f && -f < h ? v : h) + y) : S += ",0,0,0,0,1,0,", S += E + y + O + y + k + y + (I ? 1 + -k / I : 1) + ")", D[Ie] = S
                        }
                    };
                (e = Ue.prototype).x = e.y = e.z = e.skewX = e.skewY = e.rotation = e.rotationX = e.rotationY = e.zOrigin = e.xPercent = e.yPercent = e.xOffset = e.yOffset = 0, e.scaleX = e.scaleY = e.scaleZ = 1, De("transform,scale,scaleX,scaleY,scaleZ,x,y,z,rotation,rotationX,rotationY,rotationZ,skewX,skewY,shortRotation,shortRotationX,shortRotationY,shortRotationZ,transformOrigin,svgOrigin,transformPerspective,directionalRotation,parseTransform,force3D,skewType,xPercent,yPercent,smoothOrigin", {
                    parser: function(e, t, o, s, i, n, l) {
                        if (s._lastParsedTransform === l) return i;
                        var r;
                        "function" == typeof(s._lastParsedTransform = l)[o] && (r = l[o], l[o] = t);
                        var a, d, u, c, h, _, f, p, m, b = e._gsTransform,
                            g = e.style,
                            S = Le.length,
                            y = l,
                            v = {},
                            P = "transformOrigin",
                            T = Ge(e, D, !0, y.parseTransform),
                            w = y.transform && ("function" == typeof y.transform ? y.transform(O, E) : y.transform);
                        if (s._transform = T, w && "string" == typeof w && Ie)(d = ee.style)[Ie] = w, d.display = "block", d.position = "absolute", Z.body.appendChild(ee), a = Ge(ee, null, !1), T.svg && (_ = T.xOrigin, f = T.yOrigin, a.x -= T.xOffset, a.y -= T.yOffset, (y.transformOrigin || y.svgOrigin) && (w = {}, Fe(e, _e(y.transformOrigin), w, y.svgOrigin, y.smoothOrigin, !0), _ = w.xOrigin, f = w.yOrigin, a.x -= w.xOffset - T.xOffset, a.y -= w.yOffset - T.yOffset), (_ || f) && (p = ze(ee, !0), a.x -= _ - (_ * p[0] + f * p[2]), a.y -= f - (_ * p[1] + f * p[3]))), Z.body.removeChild(ee), a.perspective || (a.perspective = T.perspective), null != y.xPercent && (a.xPercent = F(y.xPercent, T.xPercent)), null != y.yPercent && (a.yPercent = F(y.yPercent, T.yPercent));
                        else if ("object" == typeof y) {
                            if (a = {
                                    scaleX: F(null != y.scaleX ? y.scaleX : y.scale, T.scaleX),
                                    scaleY: F(null != y.scaleY ? y.scaleY : y.scale, T.scaleY),
                                    scaleZ: F(y.scaleZ, T.scaleZ),
                                    x: F(y.x, T.x),
                                    y: F(y.y, T.y),
                                    z: F(y.z, T.z),
                                    xPercent: F(y.xPercent, T.xPercent),
                                    yPercent: F(y.yPercent, T.yPercent),
                                    perspective: F(y.transformPerspective, T.perspective)
                                }, null != (h = y.directionalRotation))
                                if ("object" == typeof h)
                                    for (d in h) y[d] = h[d];
                                else y.rotation = h;
                            "string" == typeof y.x && -1 !== y.x.indexOf("%") && (a.x = 0, a.xPercent = F(y.x, T.xPercent)), "string" == typeof y.y && -1 !== y.y.indexOf("%") && (a.y = 0, a.yPercent = F(y.y, T.yPercent)), a.rotation = W("rotation" in y ? y.rotation : "shortRotation" in y ? y.shortRotation + "_short" : "rotationZ" in y ? y.rotationZ : T.rotation - T.skewY, T.rotation - T.skewY, "rotation", v), Re && (a.rotationX = W("rotationX" in y ? y.rotationX : "shortRotationX" in y ? y.shortRotationX + "_short" : T.rotationX || 0, T.rotationX, "rotationX", v), a.rotationY = W("rotationY" in y ? y.rotationY : "shortRotationY" in y ? y.shortRotationY + "_short" : T.rotationY || 0, T.rotationY, "rotationY", v)), a.skewX = W(y.skewX, T.skewX - T.skewY), (a.skewY = W(y.skewY, T.skewY)) && (a.skewX += a.skewY, a.rotation += a.skewY)
                        }
                        for (Re && null != y.force3D && (T.force3D = y.force3D, c = !0), T.skewType = y.skewType || T.skewType || N.defaultSkewType, (u = T.force3D || T.z || T.rotationX || T.rotationY || a.z || a.rotationX || a.rotationY || a.perspective) || null == y.scale || (a.scaleZ = 1); - 1 < --S;)(1e-6 < (w = a[m = Le[S]] - T[m]) || w < -1e-6 || null != y[m] || null != $[m]) && (c = !0, i = new ve(T, m, T[m], w, i), m in v && (i.e = v[m]), i.xs0 = 0, i.plugin = n, s._overwriteProps.push(i.n));
                        return w = y.transformOrigin, T.svg && (w || y.svgOrigin) && (_ = T.xOffset, f = T.yOffset, Fe(e, _e(w), a, y.svgOrigin, y.smoothOrigin), i = Se(T, "xOrigin", (b ? T : a).xOrigin, a.xOrigin, i, P), i = Se(T, "yOrigin", (b ? T : a).yOrigin, a.yOrigin, i, P), _ === T.xOffset && f === T.yOffset || (i = Se(T, "xOffset", b ? _ : T.xOffset, T.xOffset, i, P), i = Se(T, "yOffset", b ? f : T.yOffset, T.yOffset, i, P)), w = He ? null : "0px 0px"), (w || Re && u && T.zOrigin) && (Ie ? (c = !0, m = Ae, w = (w || ae(e, m, D, !1, "50% 50%")) + "", (i = new ve(g, m, 0, 0, i, -1, P)).b = g[m], i.plugin = n, Re ? (d = T.zOrigin, w = w.split(" "), T.zOrigin = (2 < w.length && (0 === d || "0px" !== w[2]) ? parseFloat(w[2]) : d) || 0, i.xs0 = i.e = w[0] + " " + (w[1] || "50%") + " 0px", (i = new ve(T, "zOrigin", 0, 0, i, -1, i.n)).b = d, i.xs0 = i.e = T.zOrigin) : i.xs0 = i.e = w) : _e(w + "", T)), c && (s._transformType = T.svg && He || !u && 3 !== this._transformType ? 2 : 3), r && (l[o] = r), i
                    },
                    prefix: !0
                }), De("boxShadow", {
                    defaultValue: "0px 0px 0px 0px #999",
                    prefix: !0,
                    color: !0,
                    multi: !0,
                    keyword: "inset"
                }), De("borderRadius", {
                    defaultValue: "0px",
                    parser: function(e, t, o, s, i, n) {
                        t = this.format(t);
                        var l, r, a, d, u, c, h, _, f, p, m, b, g, S, y, v, P = ["borderTopLeftRadius", "borderTopRightRadius", "borderBottomRightRadius", "borderBottomLeftRadius"],
                            T = e.style;
                        for (f = parseFloat(e.offsetWidth), p = parseFloat(e.offsetHeight), l = t.split(" "), r = 0; r < P.length; r++) this.p.indexOf("border") && (P[r] = B(P[r])), -1 !== (u = d = ae(e, P[r], D, !1, "0px")).indexOf(" ") && (u = (d = u.split(" "))[0], d = d[1]), c = a = l[r], h = parseFloat(u), b = u.substr((h + "").length), "" === (m = (g = "=" === c.charAt(1)) ? (_ = parseInt(c.charAt(0) + "1", 10), c = c.substr(2), _ *= parseFloat(c), c.substr((_ + "").length - (_ < 0 ? 1 : 0)) || "") : (_ = parseFloat(c), c.substr((_ + "").length))) && (m = w[o] || b), m !== b && (S = de(e, "borderLeft", h, b), y = de(e, "borderTop", h, b), d = "%" === m ? (u = S / f * 100 + "%", y / p * 100 + "%") : "em" === m ? (u = S / (v = de(e, "borderLeft", 1, "em")) + "em", y / v + "em") : (u = S + "px", y + "px"), g && (c = parseFloat(u) + _ + m, a = parseFloat(d) + _ + m)), i = Pe(T, P[r], u + " " + d, c + " " + a, !1, "0px", i);
                        return i
                    },
                    prefix: !0,
                    formatter: be("0px 0px 0px 0px", !1, !0)
                }), De("borderBottomLeftRadius,borderBottomRightRadius,borderTopLeftRadius,borderTopRightRadius", {
                    defaultValue: "0px",
                    parser: function(e, t, o, s, i, n) {
                        return Pe(e.style, o, this.format(ae(e, o, D, !1, "0px 0px")), this.format(t), !1, "0px", i)
                    },
                    prefix: !0,
                    formatter: be("0px 0px", !1, !0)
                }), De("backgroundPosition", {
                    defaultValue: "0 0",
                    parser: function(e, t, o, s, i, n) {
                        var l, r, a, d, u, c, h = "background-position",
                            _ = D || re(e, null),
                            f = this.format((_ ? C ? _.getPropertyValue(h + "-x") + " " + _.getPropertyValue(h + "-y") : _.getPropertyValue(h) : e.currentStyle.backgroundPositionX + " " + e.currentStyle.backgroundPositionY) || "0 0"),
                            p = this.format(t);
                        if (-1 !== f.indexOf("%") != (-1 !== p.indexOf("%")) && p.split(",").length < 2 && (c = ae(e, "backgroundImage").replace(V, "")) && "none" !== c) {
                            for (l = f.split(" "), r = p.split(" "), te.setAttribute("src", c), a = 2; - 1 < --a;)(d = -1 !== (f = l[a]).indexOf("%")) != (-1 !== r[a].indexOf("%")) && (u = 0 === a ? e.offsetWidth - te.width : e.offsetHeight - te.height, l[a] = d ? parseFloat(f) / 100 * u + "px" : parseFloat(f) / u * 100 + "%");
                            f = l.join(" ")
                        }
                        return this.parseComplex(e.style, f, p, i, n)
                    },
                    formatter: _e
                }), De("backgroundSize", {
                    defaultValue: "0 0",
                    formatter: function(e) {
                        return _e(-1 === (e += "").indexOf(" ") ? e + " " + e : e)
                    }
                }), De("perspective", {
                    defaultValue: "0px",
                    prefix: !0
                }), De("perspectiveOrigin", {
                    defaultValue: "50% 50%",
                    prefix: !0
                }), De("transformStyle", {
                    prefix: !0
                }), De("backfaceVisibility", {
                    prefix: !0
                }), De("userSelect", {
                    prefix: !0
                }), De("margin", {
                    parser: ge("marginTop,marginRight,marginBottom,marginLeft")
                }), De("padding", {
                    parser: ge("paddingTop,paddingRight,paddingBottom,paddingLeft")
                }), De("clip", {
                    defaultValue: "rect(0px,0px,0px,0px)",
                    parser: function(e, t, o, s, i, n) {
                        var l, r, a;
                        return t = C < 9 ? (r = e.currentStyle, a = C < 8 ? " " : ",", l = "rect(" + r.clipTop + a + r.clipRight + a + r.clipBottom + a + r.clipLeft + ")", this.format(t).split(",").join(a)) : (l = this.format(ae(e, this.p, D, !1, this.dflt)), this.format(t)), this.parseComplex(e.style, l, t, i, n)
                    }
                }), De("textShadow", {
                    defaultValue: "0px 0px 0px #999",
                    color: !0,
                    multi: !0
                }), De("autoRound,strictUnits", {
                    parser: function(e, t, o, s, i) {
                        return i
                    }
                }), De("border", {
                    defaultValue: "0px solid #000",
                    parser: function(e, t, o, s, i, n) {
                        var l = ae(e, "borderTopWidth", D, !1, "0px"),
                            r = this.format(t).split(" "),
                            a = r[0].replace(x, "");
                        return "px" !== a && (l = parseFloat(l) / de(e, "borderTopWidth", 1, a) + a), this.parseComplex(e.style, this.format(l + " " + ae(e, "borderTopStyle", D, !1, "solid") + " " + ae(e, "borderTopColor", D, !1, "#000")), r.join(" "), i, n)
                    },
                    color: !0,
                    formatter: function(e) {
                        var t = e.split(" ");
                        return t[0] + " " + (t[1] || "solid") + " " + (e.match(me) || ["#000"])[0]
                    }
                }), De("borderWidth", {
                    parser: ge("borderTopWidth,borderRightWidth,borderBottomWidth,borderLeftWidth")
                }), De("float,cssFloat,styleFloat", {
                    parser: function(e, t, o, s, i, n) {
                        var l = e.style,
                            r = "cssFloat" in l ? "cssFloat" : "styleFloat";
                        return new ve(l, r, 0, 0, i, -1, o, !1, 0, l[r], t)
                    }
                });

                function Ke(e) {
                    var t, o = this.t,
                        s = o.filter || ae(this.data, "filter") || "",
                        i = this.s + this.c * e | 0;
                    100 == i && (t = -1 === s.indexOf("atrix(") && -1 === s.indexOf("radient(") && -1 === s.indexOf("oader(") ? (o.removeAttribute("filter"), !ae(this.data, "filter")) : (o.filter = s.replace(a, ""), !0)), t || (this.xn1 && (o.filter = s = s || "alpha(opacity=" + i + ")"), -1 === s.indexOf("pacity") ? 0 == i && this.xn1 || (o.filter = s + " alpha(opacity=" + i + ")") : o.filter = s.replace(R, "opacity=" + i))
                }
                De("opacity,alpha,autoAlpha", {
                    defaultValue: "1",
                    parser: function(e, t, o, s, i, n) {
                        var l = parseFloat(ae(e, "opacity", D, !1, "1")),
                            r = e.style,
                            a = "autoAlpha" === o;
                        return "string" == typeof t && "=" === t.charAt(1) && (t = ("-" === t.charAt(0) ? -1 : 1) * parseFloat(t.substr(2)) + l), a && 1 === l && "hidden" === ae(e, "visibility", D) && 0 !== t && (l = 0), ie ? i = new ve(r, "opacity", l, t - l, i) : ((i = new ve(r, "opacity", 100 * l, 100 * (t - l), i)).xn1 = a ? 1 : 0, r.zoom = 1, i.type = 2, i.b = "alpha(opacity=" + i.s + ")", i.e = "alpha(opacity=" + (i.s + i.c) + ")", i.data = e, i.plugin = n, i.setRatio = Ke), a && ((i = new ve(r, "visibility", 0, 0, i, -1, null, !1, 0, 0 !== l ? "inherit" : "hidden", 0 === t ? "hidden" : "inherit")).xs0 = "inherit", s._overwriteProps.push(i.n), s._overwriteProps.push(o)), i
                    }
                });

                function Je(e) {
                    if (this.t._gsClassPT = this, 1 === e || 0 === e) {
                        this.t.setAttribute("class", 0 === e ? this.b : this.e);
                        for (var t = this.data, o = this.t.style; t;) t.v ? o[t.p] = t.v : Qe(o, t.p), t = t._next;
                        1 === e && this.t._gsClassPT === this && (this.t._gsClassPT = null)
                    } else this.t.getAttribute("class") !== this.e && this.t.setAttribute("class", this.e)
                }
                var Qe = function(e, t) {
                    t && (e.removeProperty ? ("ms" !== t.substr(0, 2) && "webkit" !== t.substr(0, 6) || (t = "-" + t), e.removeProperty(t.replace(c, "-$1").toLowerCase())) : e.removeAttribute(t))
                };
                De("className", {
                    parser: function(e, t, o, s, i, n, l) {
                        var r, a, d, u, c, h = e.getAttribute("class") || "",
                            _ = e.style.cssText;
                        if ((i = s._classNamePT = new ve(e, o, 0, 0, i, 2)).setRatio = Je, i.pr = -11, f = !0, i.b = h, a = g(e, D), d = e._gsClassPT) {
                            for (u = {}, c = d.data; c;) u[c.p] = 1, c = c._next;
                            d.setRatio(1)
                        }
                        return (e._gsClassPT = i).e = "=" !== t.charAt(1) ? t : h.replace(new RegExp("(?:\\s|^)" + t.substr(2) + "(?![\\w-])"), "") + ("+" === t.charAt(0) ? " " + t.substr(2) : ""), e.setAttribute("class", i.e), r = S(e, a, g(e), l, u), e.setAttribute("class", h), i.data = r.firstMPT, e.style.cssText = _, i = i.xfirst = s.parse(e, r.difs, i, n)
                    }
                });

                function $e(e) {
                    if ((1 === e || 0 === e) && this.data._totalTime === this.data._totalDuration && "isFromStart" !== this.data.data) {
                        var t, o, s, i, n, l = this.t.style,
                            r = m.transform.parse;
                        if ("all" === this.e) i = !(l.cssText = "");
                        else
                            for (s = (t = this.e.split(" ").join("").split(",")).length; - 1 < --s;) o = t[s], m[o] && (m[o].parse === r ? i = !0 : o = "transformOrigin" === o ? Ae : m[o].p), Qe(l, o);
                        i && (Qe(l, Ie), (n = this.t._gsTransform) && (n.svg && (this.t.removeAttribute("data-svg-origin"), this.t.removeAttribute("transform")), delete this.t._gsTransform))
                    }
                }
                for (De("clearProps", {
                        parser: function(e, t, o, s, i) {
                            return (i = new ve(e, o, 0, 0, i, 2)).setRatio = $e, i.e = t, i.pr = -10, i.data = s._tween, f = !0, i
                        }
                    }), e = "bezier,throwProps,physicsProps,physics2D".split(","), Te = e.length; Te--;) Be(e[Te]);
                (e = N.prototype)._firstPT = e._lastParsedTransform = e._transform = null, e._onInitTween = function(e, t, o, s) {
                    if (!e.nodeType) return !1;
                    this._target = E = e, this._tween = o, this._vars = t, O = s, H = t.autoRound, f = !1, w = t.suffixMap || N.suffixMap, D = re(e, ""), p = this._overwriteProps;
                    var i, n, l, r, a, d, u, c, h, _ = e.style;
                    if (v && "" === _.zIndex && ("auto" !== (i = ae(e, "zIndex", D)) && "" !== i || this._addLazySet(_, "zIndex", 0)), "string" == typeof t && (r = _.cssText, i = g(e, D), _.cssText = r + ";" + t, i = S(e, i, g(e)).difs, !ie && Y.test(t) && (i.opacity = parseFloat(RegExp.$1)), t = i, _.cssText = r), t.className ? this._firstPT = n = m.className.parse(e, t.className, "className", this, null, null, t) : this._firstPT = n = this.parse(e, t, null), this._transformType) {
                        for (h = 3 === this._transformType, Ie ? P && (v = !0, "" === _.zIndex && ("auto" !== (u = ae(e, "zIndex", D)) && "" !== u || this._addLazySet(_, "zIndex", 0)), T && this._addLazySet(_, "WebkitBackfaceVisibility", this._vars.WebkitBackfaceVisibility || (h ? "visible" : "hidden"))) : _.zoom = 1, l = n; l && l._next;) l = l._next;
                        c = new ve(e, "transform", 0, 0, null, 2), this._linkCSSP(c, null, l), c.setRatio = Ie ? qe : We, c.data = this._transform || Ge(e, D, !0), c.tween = o, c.pr = -1, p.pop()
                    }
                    if (f) {
                        for (; n;) {
                            for (d = n._next, l = r; l && l.pr > n.pr;) l = l._next;
                            (n._prev = l ? l._prev : a) ? n._prev._next = n: r = n, (n._next = l) ? l._prev = n : a = n, n = d
                        }
                        this._firstPT = r
                    }
                    return !0
                }, e.parse = function(e, t, o, s) {
                    var i, n, l, r, a, d, u, c, h, _, f = e.style;
                    for (i in t) "function" == typeof(d = t[i]) && (d = d(O, E)), (n = m[i]) ? o = n.parse(e, d, i, this, o, s, t) : (a = ae(e, i, D) + "", h = "string" == typeof d, "color" === i || "fill" === i || "stroke" === i || -1 !== i.indexOf("Color") || h && X.test(d) ? (h || (d = (3 < (d = pe(d)).length ? "rgba(" : "rgb(") + d.join(",") + ")"), o = Pe(f, i, a, d, !0, "transparent", o, 0, s)) : h && K.test(d) ? o = Pe(f, i, a, d, !0, null, o, 0, s) : (u = (l = parseFloat(a)) || 0 === l ? a.substr((l + "").length) : "", "" !== a && "auto" !== a || (u = "width" === i || "height" === i ? (l = y(e, i, D), "px") : "left" === i || "top" === i ? (l = ue(e, i, D), "px") : (l = "opacity" !== i ? 0 : 1, "")), "" === (c = (_ = h && "=" === d.charAt(1)) ? (r = parseInt(d.charAt(0) + "1", 10), d = d.substr(2), r *= parseFloat(d), d.replace(x, "")) : (r = parseFloat(d), h ? d.replace(x, "") : "")) && (c = i in w ? w[i] : u), d = r || 0 === r ? (_ ? r + l : r) + c : t[i], u !== c && "" !== c && (r || 0 === r) && l && (l = de(e, i, l, u), "%" === c ? (l /= de(e, i, 100, "%") / 100, !0 !== t.strictUnits && (a = l + "%")) : "em" === c || "rem" === c || "vw" === c || "vh" === c ? l /= de(e, i, 1, c) : "px" !== c && (r = de(e, i, r, c), c = "px"), _ && (!r && 0 !== r || (d = r + l + c))), _ && (r += l), !l && 0 !== l || !r && 0 !== r ? void 0 !== f[i] && (d || d + "" != "NaN" && null != d) ? (o = new ve(f, i, r || l || 0, 0, o, -1, i, !1, 0, a, d)).xs0 = "none" !== d || "display" !== i && -1 === i.indexOf("Style") ? d : a : b("invalid " + i + " tween value: " + t[i]) : (o = new ve(f, i, l, r - l, o, 0, i, !1 !== H && ("px" === c || "zIndex" === i), 0, a, d)).xs0 = c)), s && o && !o.plugin && (o.plugin = s);
                    return o
                }, e.setRatio = function(e) {
                    var t, o, s, i = this._firstPT;
                    if (1 !== e || this._tween._time !== this._tween._duration && 0 !== this._tween._time)
                        if (e || this._tween._time !== this._tween._duration && 0 !== this._tween._time || -1e-6 === this._tween._rawPrevTime)
                            for (; i;) {
                                if (t = i.c * e + i.s, i.r ? t = Math.round(t) : t < 1e-6 && -1e-6 < t && (t = 0), i.type)
                                    if (1 === i.type)
                                        if (2 === (s = i.l)) i.t[i.p] = i.xs0 + t + i.xs1 + i.xn1 + i.xs2;
                                        else if (3 === s) i.t[i.p] = i.xs0 + t + i.xs1 + i.xn1 + i.xs2 + i.xn2 + i.xs3;
                                else if (4 === s) i.t[i.p] = i.xs0 + t + i.xs1 + i.xn1 + i.xs2 + i.xn2 + i.xs3 + i.xn3 + i.xs4;
                                else if (5 === s) i.t[i.p] = i.xs0 + t + i.xs1 + i.xn1 + i.xs2 + i.xn2 + i.xs3 + i.xn3 + i.xs4 + i.xn4 + i.xs5;
                                else {
                                    for (o = i.xs0 + t + i.xs1, s = 1; s < i.l; s++) o += i["xn" + s] + i["xs" + (s + 1)];
                                    i.t[i.p] = o
                                } else -1 === i.type ? i.t[i.p] = i.xs0 : i.setRatio && i.setRatio(e);
                                else i.t[i.p] = t + i.xs0;
                                i = i._next
                            } else
                                for (; i;) 2 !== i.type ? i.t[i.p] = i.b : i.setRatio(e), i = i._next;
                        else
                            for (; i;) {
                                if (2 !== i.type)
                                    if (i.r && -1 !== i.type)
                                        if (t = Math.round(i.s + i.c), i.type) {
                                            if (1 === i.type) {
                                                for (s = i.l, o = i.xs0 + t + i.xs1, s = 1; s < i.l; s++) o += i["xn" + s] + i["xs" + (s + 1)];
                                                i.t[i.p] = o
                                            }
                                        } else i.t[i.p] = t + i.xs0;
                                else i.t[i.p] = i.e;
                                else i.setRatio(e);
                                i = i._next
                            }
                }, e._enableTransforms = function(e) {
                    this._transform = this._transform || Ge(this._target, D, !0), this._transformType = this._transform.svg && He || !e && 3 !== this._transformType ? 2 : 3
                };

                function Ze(e) {
                    this.t[this.p] = this.e, this.data._linkCSSP(this, this._next, null, !0)
                }
                e._addLazySet = function(e, t, o) {
                    var s = this._firstPT = new ve(e, t, 0, 0, this._firstPT, 2);
                    s.e = o, s.setRatio = Ze, s.data = this
                }, e._linkCSSP = function(e, t, o, s) {
                    return e && (t && (t._prev = e), e._next && (e._next._prev = e._prev), e._prev ? e._prev._next = e._next : this._firstPT === e && (this._firstPT = e._next, s = !0), o ? o._next = e : s || null !== this._firstPT || (this._firstPT = e), e._next = t, e._prev = o), e
                }, e._mod = function(e) {
                    for (var t = this._firstPT; t;) "function" == typeof e[t.p] && e[t.p] === Math.round && (t.r = 1), t = t._next
                }, e._kill = function(e) {
                    var t, o, s, i = e;
                    if (e.autoAlpha || e.alpha) {
                        for (o in i = {}, e) i[o] = e[o];
                        i.opacity = 1, i.autoAlpha && (i.visibility = 1)
                    }
                    for (e.className && (t = this._classNamePT) && ((s = t.xfirst) && s._prev ? this._linkCSSP(s._prev, t._next, s._prev._prev) : s === this._firstPT && (this._firstPT = t._next), t._next && this._linkCSSP(t._next, t._next._next, s._prev), this._classNamePT = null), t = this._firstPT; t;) t.plugin && t.plugin !== o && t.plugin._kill && (t.plugin._kill(e), o = t.plugin), t = t._next;
                    return n.prototype._kill.call(this, i)
                };
                var et = function(e, t, o) {
                    var s, i, n, l;
                    if (e.slice)
                        for (i = e.length; - 1 < --i;) et(e[i], t, o);
                    else
                        for (i = (s = e.childNodes).length; - 1 < --i;) l = (n = s[i]).type, n.style && (t.push(g(n)), o && o.push(n)), 1 !== l && 9 !== l && 11 !== l || !n.childNodes.length || et(n, t, o)
                };
                return N.cascadeTo = function(e, t, o) {
                    var s, i, n, l, r = U.to(e, t, o),
                        a = [r],
                        d = [],
                        u = [],
                        c = [],
                        h = U._internals.reservedProps;
                    for (e = r._targets || r.target, et(e, d, c), r.render(t, !0, !0), et(e, u), r.render(0, !0, !0), r._enabled(!0), s = c.length; - 1 < --s;)
                        if ((i = S(c[s], d[s], u[s])).firstMPT) {
                            for (n in i = i.difs, o) h[n] && (i[n] = o[n]);
                            for (n in l = {}, i) l[n] = d[s][n];
                            a.push(U.fromTo(c[s], t, l, i))
                        } return a
                }, n.activate([N]), N
            }, !0), t = _fwd_gsScope._gsDefine.plugin({
                propName: "roundProps",
                version: "1.6.0",
                priority: -1,
                API: 2,
                init: function(e, t, o) {
                    return this._tween = o, !0
                }
            }), (s = t.prototype)._onInitAllProps = function() {
                for (var e, t, o, s = this._tween, i = s.vars.roundProps.join ? s.vars.roundProps : s.vars.roundProps.split(","), n = i.length, l = {}, r = s._propLookup.roundProps; - 1 < --n;) l[i[n]] = Math.round;
                for (n = i.length; - 1 < --n;)
                    for (e = i[n], t = s._firstPT; t;) o = t._next, t.pg ? t.t._mod(l) : t.n === e && (2 === t.f && t.t ? a(t.t._firstPT) : (this._add(t.t, e, t.s, t.c), o && (o._prev = t._prev), t._prev ? t._prev._next = o : s._firstPT === t && (s._firstPT = o), t._next = t._prev = null, s._propLookup[e] = r)), t = o;
                return !1
            }, s._add = function(e, t, o, s) {
                this._addTween(e, t, o, o + s, t, Math.round), this._overwriteProps.push(t)
            }, _fwd_gsScope._gsDefine.plugin({
                propName: "attr",
                API: 2,
                version: "0.6.0",
                init: function(e, t, o, s) {
                    var i, n;
                    if ("function" != typeof e.setAttribute) return !1;
                    for (i in t) "function" == typeof(n = t[i]) && (n = n(s, e)), this._addTween(e, "setAttribute", e.getAttribute(i) + "", n + "", i, !1, i), this._overwriteProps.push(i);
                    return !0
                }
            }), _fwd_gsScope._gsDefine.plugin({
                propName: "directionalRotation",
                version: "0.3.0",
                API: 2,
                init: function(e, t, o, s) {
                    "object" != typeof t && (t = {
                        rotation: t
                    }), this.finals = {};
                    var i, n, l, r, a, d, u = !0 === t.useRadians ? 2 * Math.PI : 360;
                    for (i in t) "useRadians" !== i && ("function" == typeof(r = t[i]) && (r = r(s, e)), n = (d = (r + "").split("_"))[0], l = parseFloat("function" != typeof e[i] ? e[i] : e[i.indexOf("set") || "function" != typeof e["get" + i.substr(3)] ? i : "get" + i.substr(3)]()), a = (r = this.finals[i] = "string" == typeof n && "=" === n.charAt(1) ? l + parseInt(n.charAt(0) + "1", 10) * Number(n.substr(2)) : Number(n) || 0) - l, d.length && (-1 !== (n = d.join("_")).indexOf("short") && (a %= u) !== a % (u / 2) && (a = a < 0 ? a + u : a - u), -1 !== n.indexOf("_cw") && a < 0 ? a = (a + 9999999999 * u) % u - (a / u | 0) * u : -1 !== n.indexOf("ccw") && 0 < a && (a = (a - 9999999999 * u) % u - (a / u | 0) * u)), (1e-6 < a || a < -1e-6) && (this._addTween(e, i, l, l + a, i), this._overwriteProps.push(i)));
                    return !0
                },
                set: function(e) {
                    var t;
                    if (1 !== e) this._super.setRatio.call(this, e);
                    else
                        for (t = this._firstPT; t;) t.f ? t.t[t.p](this.finals[t.p]) : t.t[t.p] = this.finals[t.p], t = t._next
                }
            })._autoCSS = !0, _fwd_gsScope._gsDefine("easing.Back", ["easing.Ease"], function(m) {
                function e(e, t) {
                    var o = u("easing." + e, function() {}, !0),
                        s = o.prototype = new m;
                    return s.constructor = o, s.getRatio = t, o
                }

                function t(e, t, o, s, i) {
                    var n = u("easing." + e, {
                        easeOut: new t,
                        easeIn: new o,
                        easeInOut: new s
                    }, !0);
                    return c(n, e), n
                }

                function b(e, t, o) {
                    this.t = e, this.v = t, o && (((this.next = o).prev = this).c = o.v - t, this.gap = o.t - e)
                }

                function o(e, t) {
                    var o = u("easing." + e, function(e) {
                            this._p1 = e || 0 === e ? e : 1.70158, this._p2 = 1.525 * this._p1
                        }, !0),
                        s = o.prototype = new m;
                    return s.constructor = o, s.getRatio = t, s.config = function(e) {
                        return new o(e)
                    }, o
                }
                var s, i, n, l = _fwd_gsScope.GreenSockGlobals || _fwd_gsScope,
                    r = l.com.greensock,
                    a = 2 * Math.PI,
                    d = Math.PI / 2,
                    u = r._class,
                    c = m.register || function() {},
                    h = t("Back", o("BackOut", function(e) {
                        return --e * e * ((this._p1 + 1) * e + this._p1) + 1
                    }), o("BackIn", function(e) {
                        return e * e * ((this._p1 + 1) * e - this._p1)
                    }), o("BackInOut", function(e) {
                        return (e *= 2) < 1 ? .5 * e * e * ((this._p2 + 1) * e - this._p2) : .5 * ((e -= 2) * e * ((this._p2 + 1) * e + this._p2) + 2)
                    })),
                    _ = u("easing.SlowMo", function(e, t, o) {
                        t = t || 0 === t ? t : .7, null == e ? e = .7 : 1 < e && (e = 1), this._p = 1 !== e ? t : 0, this._p1 = (1 - e) / 2, this._p2 = e, this._p3 = this._p1 + this._p2, this._calcEnd = !0 === o
                    }, !0),
                    f = _.prototype = new m;
                return f.constructor = _, f.getRatio = function(e) {
                    var t = e + (.5 - e) * this._p;
                    return e < this._p1 ? this._calcEnd ? 1 - (e = 1 - e / this._p1) * e : t - (e = 1 - e / this._p1) * e * e * e * t : e > this._p3 ? this._calcEnd ? 1 - (e = (e - this._p3) / this._p1) * e : t + (e - t) * (e = (e - this._p3) / this._p1) * e * e * e : this._calcEnd ? 1 : t
                }, _.ease = new _(.7, .7), f.config = _.config = function(e, t, o) {
                    return new _(e, t, o)
                }, (f = (s = u("easing.SteppedEase", function(e) {
                    e = e || 1, this._p1 = 1 / e, this._p2 = e + 1
                }, !0)).prototype = new m).constructor = s, f.getRatio = function(e) {
                    return e < 0 ? e = 0 : 1 <= e && (e = .999999999), (this._p2 * e >> 0) * this._p1
                }, f.config = s.config = function(e) {
                    return new s(e)
                }, (f = (i = u("easing.RoughEase", function(e) {
                    for (var t, o, s, i, n, l, r = (e = e || {}).taper || "none", a = [], d = 0, u = 0 | (e.points || 20), c = u, h = !1 !== e.randomize, _ = !0 === e.clamp, f = e.template instanceof m ? e.template : null, p = "number" == typeof e.strength ? .4 * e.strength : .4; - 1 < --c;) t = h ? Math.random() : 1 / u * c, o = f ? f.getRatio(t) : t, s = "none" === r ? p : "out" === r ? (i = 1 - t) * i * p : "in" === r ? t * t * p : t < .5 ? (i = 2 * t) * i * .5 * p : (i = 2 * (1 - t)) * i * .5 * p, h ? o += Math.random() * s - .5 * s : c % 2 ? o += .5 * s : o -= .5 * s, _ && (1 < o ? o = 1 : o < 0 && (o = 0)), a[d++] = {
                        x: t,
                        y: o
                    };
                    for (a.sort(function(e, t) {
                            return e.x - t.x
                        }), l = new b(1, 1, null), c = u; - 1 < --c;) n = a[c], l = new b(n.x, n.y, l);
                    this._prev = new b(0, 0, 0 !== l.t ? l : l.next)
                }, !0)).prototype = new m).constructor = i, f.getRatio = function(e) {
                    var t = this._prev;
                    if (e > t.t) {
                        for (; t.next && e >= t.t;) t = t.next;
                        t = t.prev
                    } else
                        for (; t.prev && e <= t.t;) t = t.prev;
                    return (this._prev = t).v + (e - t.t) / t.gap * t.c
                }, f.config = function(e) {
                    return new i(e)
                }, i.ease = new i, t("Bounce", e("BounceOut", function(e) {
                    return e < 1 / 2.75 ? 7.5625 * e * e : e < 2 / 2.75 ? 7.5625 * (e -= 1.5 / 2.75) * e + .75 : e < 2.5 / 2.75 ? 7.5625 * (e -= 2.25 / 2.75) * e + .9375 : 7.5625 * (e -= 2.625 / 2.75) * e + .984375
                }), e("BounceIn", function(e) {
                    return (e = 1 - e) < 1 / 2.75 ? 1 - 7.5625 * e * e : e < 2 / 2.75 ? 1 - (7.5625 * (e -= 1.5 / 2.75) * e + .75) : e < 2.5 / 2.75 ? 1 - (7.5625 * (e -= 2.25 / 2.75) * e + .9375) : 1 - (7.5625 * (e -= 2.625 / 2.75) * e + .984375)
                }), e("BounceInOut", function(e) {
                    var t = e < .5;
                    return (e = t ? 1 - 2 * e : 2 * e - 1) < 1 / 2.75 ? e *= 7.5625 * e : e = e < 2 / 2.75 ? 7.5625 * (e -= 1.5 / 2.75) * e + .75 : e < 2.5 / 2.75 ? 7.5625 * (e -= 2.25 / 2.75) * e + .9375 : 7.5625 * (e -= 2.625 / 2.75) * e + .984375, t ? .5 * (1 - e) : .5 * e + .5
                })), t("Circ", e("CircOut", function(e) {
                    return Math.sqrt(1 - --e * e)
                }), e("CircIn", function(e) {
                    return -(Math.sqrt(1 - e * e) - 1)
                }), e("CircInOut", function(e) {
                    return (e *= 2) < 1 ? -.5 * (Math.sqrt(1 - e * e) - 1) : .5 * (Math.sqrt(1 - (e -= 2) * e) + 1)
                })), t("Elastic", (n = function(e, t, o) {
                    var s = u("easing." + e, function(e, t) {
                            this._p1 = 1 <= e ? e : 1, this._p2 = (t || o) / (e < 1 ? e : 1), this._p3 = this._p2 / a * (Math.asin(1 / this._p1) || 0), this._p2 = a / this._p2
                        }, !0),
                        i = s.prototype = new m;
                    return i.constructor = s, i.getRatio = t, i.config = function(e, t) {
                        return new s(e, t)
                    }, s
                })("ElasticOut", function(e) {
                    return this._p1 * Math.pow(2, -10 * e) * Math.sin((e - this._p3) * this._p2) + 1
                }, .3), n("ElasticIn", function(e) {
                    return -this._p1 * Math.pow(2, 10 * --e) * Math.sin((e - this._p3) * this._p2)
                }, .3), n("ElasticInOut", function(e) {
                    return (e *= 2) < 1 ? this._p1 * Math.pow(2, 10 * --e) * Math.sin((e - this._p3) * this._p2) * -.5 : this._p1 * Math.pow(2, -10 * --e) * Math.sin((e - this._p3) * this._p2) * .5 + 1
                }, .45)), t("Expo", e("ExpoOut", function(e) {
                    return 1 - Math.pow(2, -10 * e)
                }), e("ExpoIn", function(e) {
                    return Math.pow(2, 10 * (e - 1)) - .001
                }), e("ExpoInOut", function(e) {
                    return (e *= 2) < 1 ? .5 * Math.pow(2, 10 * (e - 1)) : .5 * (2 - Math.pow(2, -10 * (e - 1)))
                })), t("Sine", e("SineOut", function(e) {
                    return Math.sin(e * d)
                }), e("SineIn", function(e) {
                    return 1 - Math.cos(e * d)
                }), e("SineInOut", function(e) {
                    return -.5 * (Math.cos(Math.PI * e) - 1)
                })), u("easing.EaseLookup", {
                    find: function(e) {
                        return m.map[e]
                    }
                }, !0), c(l.SlowMo, "SlowMo", "ease,"), c(i, "RoughEase", "ease,"), c(s, "SteppedEase", "ease,"), h
            }, !0)
        }), _fwd_gsScope._gsDefine && _fwd_gsScope._fwd_gsQueue.pop()(),
        function(_, f) {
            "use strict";
            var p = {},
                m = _.GreenSockGlobals = _.GreenSockGlobals || _;
            if (!m.FWDTweenLite) {
                var e, t, o, b, g, s, i, S = function(e) {
                        var t, o = e.split("."),
                            s = m;
                        for (t = 0; t < o.length; t++) s[o[t]] = s = s[o[t]] || {};
                        return s
                    },
                    c = S("com.greensock"),
                    y = 1e-10,
                    a = function(e) {
                        var t, o = [],
                            s = e.length;
                        for (t = 0; t !== s; o.push(e[t++]));
                        return o
                    },
                    v = function() {},
                    P = (s = Object.prototype.toString, i = s.call([]), function(e) {
                        return null != e && (e instanceof Array || "object" == typeof e && !!e.push && s.call(e) === i)
                    }),
                    T = {},
                    w = function(a, d, u, c) {
                        this.sc = T[a] ? T[a].sc : [], (T[a] = this).gsClass = null, this.func = u;
                        var h = [];
                        this.check = function(e) {
                            for (var t, o, s, i, n, l = d.length, r = l; - 1 < --l;)(t = T[d[l]] || new w(d[l], [])).gsClass ? (h[l] = t.gsClass, r--) : e && t.sc.push(this);
                            if (0 === r && u) {
                                if (s = (o = ("com.greensock." + a).split(".")).pop(), i = S(o.join("."))[s] = this.gsClass = u.apply(u, h), c)
                                    if (m[s] = p[s] = i, !(n = "undefined" != typeof fwd_module && fwd_module.exports) && "function" == typeof define && define.amd) define((_.GreenSockAMDPath ? _.GreenSockAMDPath + "/" : "") + a.split(".").pop(), [], function() {
                                        return i
                                    });
                                    else if (n)
                                    if (a === f)
                                        for (l in fwd_module.exports = p[f] = i, p) i[l] = p[l];
                                    else p[f] && (p[f][s] = i);
                                for (l = 0; l < this.sc.length; l++) this.sc[l].check()
                            }
                        }, this.check(!0)
                    },
                    n = _._gsDefine = function(e, t, o, s) {
                        return new w(e, t, o, s)
                    },
                    h = c._class = function(e, t, o) {
                        return t = t || function() {}, n(e, [], function() {
                            return t
                        }, o), t
                    };
                n.globals = m;
                var l = [0, 0, 1, 1],
                    D = h("easing.Ease", function(e, t, o, s) {
                        this._func = e, this._type = o || 0, this._power = s || 0, this._params = t ? l.concat(t) : l
                    }, !0),
                    B = D.map = {},
                    r = D.register = function(e, t, o, s) {
                        for (var i, n, l, r, a = t.split(","), d = a.length, u = (o || "easeIn,easeOut,easeInOut").split(","); - 1 < --d;)
                            for (n = a[d], i = s ? h("easing." + n, null, !0) : c.easing[n] || {}, l = u.length; - 1 < --l;) r = u[l], B[n + "." + r] = B[r + n] = i[r] = e.getRatio ? e : e[r] || new e
                    };
                for ((o = D.prototype)._calcEnd = !1, o.getRatio = function(e) {
                        if (this._func) return this._params[0] = e, this._func.apply(null, this._params);
                        var t = this._type,
                            o = this._power,
                            s = 1 === t ? 1 - e : 2 === t ? e : e < .5 ? 2 * e : 2 * (1 - e);
                        return 1 === o ? s *= s : 2 === o ? s *= s * s : 3 === o ? s *= s * s * s : 4 === o && (s *= s * s * s * s), 1 === t ? 1 - s : 2 === t ? s : e < .5 ? s / 2 : 1 - s / 2
                    }, t = (e = ["Linear", "Quad", "Cubic", "Quart", "Quint,Strong"]).length; - 1 < --t;) o = e[t] + ",Power" + t, r(new D(null, null, 1, t), o, "easeOut", !0), r(new D(null, null, 2, t), o, "easeIn" + (0 === t ? ",easeNone" : "")), r(new D(null, null, 3, t), o, "easeInOut");
                B.linear = c.easing.Linear.easeIn, B.swing = c.easing.Quad.easeInOut;
                var M = h("events.EventDispatcher", function(e) {
                    this._listeners = {}, this._eventTarget = e || this
                });
                (o = M.prototype).addEventListener = function(e, t, o, s, i) {
                    i = i || 0;
                    var n, l, r = this._listeners[e],
                        a = 0;
                    for (this !== b || g || b.wake(), null == r && (this._listeners[e] = r = []), l = r.length; - 1 < --l;)(n = r[l]).c === t && n.s === o ? r.splice(l, 1) : 0 === a && n.pr < i && (a = l + 1);
                    r.splice(a, 0, {
                        c: t,
                        s: o,
                        up: s,
                        pr: i
                    })
                }, o.removeEventListener = function(e, t) {
                    var o, s = this._listeners[e];
                    if (s)
                        for (o = s.length; - 1 < --o;)
                            if (s[o].c === t) return void s.splice(o, 1)
                }, o.dispatchEvent = function(e) {
                    var t, o, s, i = this._listeners[e];
                    if (i)
                        for (1 < (t = i.length) && (i = i.slice(0)), o = this._eventTarget; - 1 < --t;)(s = i[t]) && (s.up ? s.c.call(s.s || o, {
                            type: e,
                            target: o
                        }) : s.c.call(s.s || o))
                };
                var F = _.requestAnimationFrame,
                    W = _.cancelAnimationFrame,
                    H = Date.now || function() {
                        return (new Date).getTime()
                    },
                    C = H();
                for (t = (e = ["ms", "moz", "webkit", "o"]).length; - 1 < --t && !F;) F = _[e[t] + "RequestAnimationFrame"], W = _[e[t] + "CancelAnimationFrame"] || _[e[t] + "CancelRequestAnimationFrame"];
                h("Ticker", function(e, t) {
                    var i, n, l, r, a, d = this,
                        u = H(),
                        o = !(!1 === t || !F) && "auto",
                        c = 500,
                        h = 33,
                        _ = function(e) {
                            var t, o, s = H() - C;
                            c < s && (u += s - h), C += s, d.time = (C - u) / 1e3, t = d.time - a, (!i || 0 < t || !0 === e) && (d.frame++, a += t + (r <= t ? .004 : r - t), o = !0), !0 !== e && (l = n(_)), o && d.dispatchEvent("tick")
                        };
                    M.call(d), d.time = d.frame = 0, d.tick = function() {
                        _(!0)
                    }, d.lagSmoothing = function(e, t) {
                        c = e || 1e10, h = Math.min(t, c, 0)
                    }, d.sleep = function() {
                        null != l && (o && W ? W(l) : clearTimeout(l), n = v, l = null, d === b && (g = !1))
                    }, d.wake = function(e) {
                        null !== l ? d.sleep() : e ? u += -C + (C = H()) : 10 < d.frame && (C = H() - c + 5), n = 0 === i ? v : o && F ? F : function(e) {
                            return setTimeout(e, 1e3 * (a - d.time) + 1 | 0)
                        }, d === b && (g = !0), _(2)
                    }, d.fps = function(e) {
                        if (!arguments.length) return i;
                        r = 1 / ((i = e) || 60), a = this.time + r, d.wake()
                    }, d.useRAF = function(e) {
                        if (!arguments.length) return o;
                        d.sleep(), o = e, d.fps(i)
                    }, d.fps(e), setTimeout(function() {
                        "auto" === o && d.frame < 5 && "hidden" !== document.visibilityState && d.useRAF(!1)
                    }, 1500)
                }), (o = c.Ticker.prototype = new c.events.EventDispatcher).constructor = c.Ticker;
                var d = h("core.Animation", function(e, t) {
                    if (this.vars = t = t || {}, this._duration = this._totalDuration = e || 0, this._delay = Number(t.delay) || 0, this._timeScale = 1, this._active = !0 === t.immediateRender, this.data = t.data, this._reversed = !0 === t.reversed, q) {
                        g || b.wake();
                        var o = this.vars.useFrames ? G : q;
                        o.add(this, o._time), this.vars.paused && this.paused(!0)
                    }
                });
                b = d.ticker = new c.Ticker, (o = d.prototype)._dirty = o._gc = o._initted = o._paused = !1, o._totalTime = o._time = 0, o._rawPrevTime = -1, o._next = o._last = o._onUpdate = o._timeline = o.timeline = null, o._paused = !1;
                var u = function() {
                    g && 2e3 < H() - C && b.wake(), setTimeout(u, 2e3)
                };
                u(), o.play = function(e, t) {
                    return null != e && this.seek(e, t), this.reversed(!1).paused(!1)
                }, o.pause = function(e, t) {
                    return null != e && this.seek(e, t), this.paused(!0)
                }, o.resume = function(e, t) {
                    return null != e && this.seek(e, t), this.paused(!1)
                }, o.seek = function(e, t) {
                    return this.totalTime(Number(e), !1 !== t)
                }, o.restart = function(e, t) {
                    return this.reversed(!1).paused(!1).totalTime(e ? -this._delay : 0, !1 !== t, !0)
                }, o.reverse = function(e, t) {
                    return null != e && this.seek(e || this.totalDuration(), t), this.reversed(!0).paused(!1)
                }, o.render = function(e, t, o) {}, o.invalidate = function() {
                    return this._time = this._totalTime = 0, this._initted = this._gc = !1, this._rawPrevTime = -1, !this._gc && this.timeline || this._enabled(!0), this
                }, o.isActive = function() {
                    var e, t = this._timeline,
                        o = this._startTime;
                    return !t || !this._gc && !this._paused && t.isActive() && (e = t.rawTime()) >= o && e < o + this.totalDuration() / this._timeScale
                }, o._enabled = function(e, t) {
                    return g || b.wake(), this._gc = !e, this._active = this.isActive(), !0 !== t && (e && !this.timeline ? this._timeline.add(this, this._startTime - this._delay) : !e && this.timeline && this._timeline._remove(this, !0)), !1
                }, o._kill = function(e, t) {
                    return this._enabled(!1, !1)
                }, o.kill = function(e, t) {
                    return this._kill(e, t), this
                }, o._uncache = function(e) {
                    for (var t = e ? this : this.timeline; t;) t._dirty = !0, t = t.timeline;
                    return this
                }, o._swapSelfInParams = function(e) {
                    for (var t = e.length, o = e.concat(); - 1 < --t;) "{self}" === e[t] && (o[t] = this);
                    return o
                }, o._callback = function(e) {
                    var t = this.vars,
                        o = t[e],
                        s = t[e + "Params"],
                        i = t[e + "Scope"] || t.callbackScope || this;
                    switch (s ? s.length : 0) {
                        case 0:
                            o.call(i);
                            break;
                        case 1:
                            o.call(i, s[0]);
                            break;
                        case 2:
                            o.call(i, s[0], s[1]);
                            break;
                        default:
                            o.apply(i, s)
                    }
                }, o.eventCallback = function(e, t, o, s) {
                    if ("on" === (e || "").substr(0, 2)) {
                        var i = this.vars;
                        if (1 === arguments.length) return i[e];
                        null == t ? delete i[e] : (i[e] = t, i[e + "Params"] = P(o) && -1 !== o.join("").indexOf("{self}") ? this._swapSelfInParams(o) : o, i[e + "Scope"] = s), "onUpdate" === e && (this._onUpdate = t)
                    }
                    return this
                }, o.delay = function(e) {
                    return arguments.length ? (this._timeline.smoothChildTiming && this.startTime(this._startTime + e - this._delay), this._delay = e, this) : this._delay
                }, o.duration = function(e) {
                    return arguments.length ? (this._duration = this._totalDuration = e, this._uncache(!0), this._timeline.smoothChildTiming && 0 < this._time && this._time < this._duration && 0 !== e && this.totalTime(this._totalTime * (e / this._duration), !0), this) : (this._dirty = !1, this._duration)
                }, o.totalDuration = function(e) {
                    return this._dirty = !1, arguments.length ? this.duration(e) : this._totalDuration
                }, o.time = function(e, t) {
                    return arguments.length ? (this._dirty && this.totalDuration(), this.totalTime(e > this._duration ? this._duration : e, t)) : this._time
                }, o.totalTime = function(e, t, o) {
                    if (g || b.wake(), !arguments.length) return this._totalTime;
                    if (this._timeline) {
                        if (e < 0 && !o && (e += this.totalDuration()), this._timeline.smoothChildTiming) {
                            this._dirty && this.totalDuration();
                            var s = this._totalDuration,
                                i = this._timeline;
                            if (s < e && !o && (e = s), this._startTime = (this._paused ? this._pauseTime : i._time) - (this._reversed ? s - e : e) / this._timeScale, i._dirty || this._uncache(!1), i._timeline)
                                for (; i._timeline;) i._timeline._time !== (i._startTime + i._totalTime) / i._timeScale && i.totalTime(i._totalTime, !0), i = i._timeline
                        }
                        this._gc && this._enabled(!0, !1), this._totalTime === e && 0 !== this._duration || (L.length && J(), this.render(e, t, !1), L.length && J())
                    }
                    return this
                }, o.progress = o.totalProgress = function(e, t) {
                    var o = this.duration();
                    return arguments.length ? this.totalTime(o * e, t) : o ? this._time / o : this.ratio
                }, o.startTime = function(e) {
                    return arguments.length ? (e !== this._startTime && (this._startTime = e, this.timeline && this.timeline._sortChildren && this.timeline.add(this, e - this._delay)), this) : this._startTime
                }, o.endTime = function(e) {
                    return this._startTime + (0 != e ? this.totalDuration() : this.duration()) / this._timeScale
                }, o.timeScale = function(e) {
                    if (!arguments.length) return this._timeScale;
                    if (e = e || y, this._timeline && this._timeline.smoothChildTiming) {
                        var t = this._pauseTime,
                            o = t || 0 === t ? t : this._timeline.totalTime();
                        this._startTime = o - (o - this._startTime) * this._timeScale / e
                    }
                    return this._timeScale = e, this._uncache(!1)
                }, o.reversed = function(e) {
                    return arguments.length ? (e != this._reversed && (this._reversed = e, this.totalTime(this._timeline && !this._timeline.smoothChildTiming ? this.totalDuration() - this._totalTime : this._totalTime, !0)), this) : this._reversed
                }, o.paused = function(e) {
                    if (!arguments.length) return this._paused;
                    var t, o, s = this._timeline;
                    return e != this._paused && s && (g || e || b.wake(), o = (t = s.rawTime()) - this._pauseTime, !e && s.smoothChildTiming && (this._startTime += o, this._uncache(!1)), this._pauseTime = e ? t : null, this._paused = e, this._active = this.isActive(), !e && 0 != o && this._initted && this.duration() && (t = s.smoothChildTiming ? this._totalTime : (t - this._startTime) / this._timeScale, this.render(t, t === this._totalTime, !0))), this._gc && !e && this._enabled(!0, !1), this
                };
                var E = h("core.SimpleTimeline", function(e) {
                    d.call(this, 0, e), this.autoRemoveChildren = this.smoothChildTiming = !0
                });
                (o = E.prototype = new d).constructor = E, o.kill()._gc = !1, o._first = o._last = o._recent = null, o._sortChildren = !1, o.add = o.insert = function(e, t, o, s) {
                    var i, n;
                    if (e._startTime = Number(t || 0) + e._delay, e._paused && this !== e._timeline && (e._pauseTime = e._startTime + (this.rawTime() - e._startTime) / e._timeScale), e.timeline && e.timeline._remove(e, !0), e.timeline = e._timeline = this, e._gc && e._enabled(!0, !0), i = this._last, this._sortChildren)
                        for (n = e._startTime; i && i._startTime > n;) i = i._prev;
                    return i ? (e._next = i._next, i._next = e) : (e._next = this._first, this._first = e), e._next ? e._next._prev = e : this._last = e, e._prev = i, this._recent = e, this._timeline && this._uncache(!0), this
                }, o._remove = function(e, t) {
                    return e.timeline === this && (t || e._enabled(!1, !0), e._prev ? e._prev._next = e._next : this._first === e && (this._first = e._next), e._next ? e._next._prev = e._prev : this._last === e && (this._last = e._prev), e._next = e._prev = e.timeline = null, e === this._recent && (this._recent = this._last), this._timeline && this._uncache(!0)), this
                }, o.render = function(e, t, o) {
                    var s, i = this._first;
                    for (this._totalTime = this._time = this._rawPrevTime = e; i;) s = i._next, (i._active || e >= i._startTime && !i._paused) && (i._reversed ? i.render((i._dirty ? i.totalDuration() : i._totalDuration) - (e - i._startTime) * i._timeScale, t, o) : i.render((e - i._startTime) * i._timeScale, t, o)), i = s
                }, o.rawTime = function() {
                    return g || b.wake(), this._totalTime
                };
                var O = h("FWDTweenLite", function(e, t, o) {
                        if (d.call(this, t, o), this.render = O.prototype.render, null == e) throw "Cannot tween a null target.";
                        this.target = e = "string" != typeof e ? e : O.selector(e) || e;
                        var s, i, n, l = e.jquery || e.length && e !== _ && e[0] && (e[0] === _ || e[0].nodeType && e[0].style && !e.nodeType),
                            r = this.vars.overwrite;
                        if (this._overwrite = r = null == r ? z[O.defaultOverwrite] : "number" == typeof r ? r >> 0 : z[r], (l || e instanceof Array || e.push && P(e)) && "number" != typeof e[0])
                            for (this._targets = n = a(e), this._propLookup = [], this._siblings = [], s = 0; s < n.length; s++)(i = n[s]) ? "string" != typeof i ? i.length && i !== _ && i[0] && (i[0] === _ || i[0].nodeType && i[0].style && !i.nodeType) ? (n.splice(s--, 1), this._targets = n = n.concat(a(i))) : (this._siblings[s] = Q(i, this, !1), 1 === r && 1 < this._siblings[s].length && Z(i, this, null, 1, this._siblings[s])) : "string" == typeof(i = n[s--] = O.selector(i)) && n.splice(s + 1, 1) : n.splice(s--, 1);
                        else this._propLookup = {}, this._siblings = Q(e, this, !1), 1 === r && 1 < this._siblings.length && Z(e, this, null, 1, this._siblings);
                        (this.vars.immediateRender || 0 === t && 0 === this._delay && !1 !== this.vars.immediateRender) && (this._time = -y, this.render(Math.min(0, -this._delay)))
                    }, !0),
                    k = function(e) {
                        return e && e.length && e !== _ && e[0] && (e[0] === _ || e[0].nodeType && e[0].style && !e.nodeType)
                    };
                (o = O.prototype = new d).constructor = O, o.kill()._gc = !1, o.ratio = 0, o._firstPT = o._targets = o._overwrittenProps = o._startAt = null, o._notifyPluginsOfEnabled = o._lazy = !1, O.version = "1.19.0", O.defaultEase = o._ease = new D(null, null, 1, 1), O.defaultOverwrite = "auto", O.ticker = b, O.autoSleep = 120, O.lagSmoothing = function(e, t) {
                    b.lagSmoothing(e, t)
                }, O.selector = _.$ || _.jQuery || function(e) {
                    var t = _.$ || _.jQuery;
                    return t ? (O.selector = t)(e) : "undefined" == typeof document ? e : document.querySelectorAll ? document.querySelectorAll(e) : document.getElementById("#" === e.charAt(0) ? e.substr(1) : e)
                };
                var L = [],
                    I = {},
                    x = /(?:(-|-=|\+=)?\d*\.?\d*(?:e[\-+]?\d+)?)[0-9]/gi,
                    A = function(e) {
                        for (var t, o = this._firstPT; o;) t = o.blob ? e ? this.join("") : this.start : o.c * e + o.s, o.m ? t = o.m(t, this._target || o.t) : t < 1e-6 && -1e-6 < t && (t = 0), o.f ? o.fp ? o.t[o.p](o.fp, t) : o.t[o.p](t) : o.t[o.p] = t, o = o._next
                    },
                    R = function(e, t, o, s) {
                        var i, n, l, r, a, d, u, c = [e, t],
                            h = 0,
                            _ = "",
                            f = 0;
                        for (c.start = e, o && (o(c), e = c[0], t = c[1]), c.length = 0, i = e.match(x) || [], n = t.match(x) || [], s && (s._next = null, s.blob = 1, c._firstPT = c._applyPT = s), a = n.length, r = 0; r < a; r++) u = n[r], _ += (d = t.substr(h, t.indexOf(u, h) - h)) || !r ? d : ",", h += d.length, f ? f = (f + 1) % 5 : "rgba(" === d.substr(-5) && (f = 1), u === i[r] || i.length <= r ? _ += u : (_ && (c.push(_), _ = ""), l = parseFloat(i[r]), c.push(l), c._firstPT = {
                            _next: c._firstPT,
                            t: c,
                            p: c.length - 1,
                            s: l,
                            c: ("=" === u.charAt(1) ? parseInt(u.charAt(0) + "1", 10) * parseFloat(u.substr(2)) : parseFloat(u) - l) || 0,
                            f: 0,
                            m: f && f < 4 ? Math.round : 0
                        }), h += u.length;
                        return (_ += t.substr(h)) && c.push(_), c.setRatio = A, c
                    },
                    U = function(e, t, o, s, i, n, l, r, a) {
                        "function" == typeof s && (s = s(a || 0, e));
                        var d, u = "get" === o ? e[t] : o,
                            c = typeof e[t],
                            h = "string" == typeof s && "=" === s.charAt(1),
                            _ = {
                                t: e,
                                p: t,
                                s: u,
                                f: "function" == c,
                                pg: 0,
                                n: i || t,
                                m: n ? "function" == typeof n ? n : Math.round : 0,
                                pr: 0,
                                c: h ? parseInt(s.charAt(0) + "1", 10) * parseFloat(s.substr(2)) : parseFloat(s) - u || 0
                            };
                        if ("number" != c && ("function" == c && "get" === o && (d = t.indexOf("set") || "function" != typeof e["get" + t.substr(3)] ? t : "get" + t.substr(3), _.s = u = l ? e[d](l) : e[d]()), "string" == typeof u && (l || isNaN(u)) ? (_.fp = l, _ = {
                                t: R(u, s, r || O.defaultStringFilter, _),
                                p: "setRatio",
                                s: 0,
                                c: 1,
                                f: 2,
                                pg: 0,
                                n: i || t,
                                pr: 0,
                                m: 0
                            }) : h || (_.s = parseFloat(u), _.c = parseFloat(s) - _.s || 0)), _.c) return (_._next = this._firstPT) && (_._next._prev = _), this._firstPT = _
                    },
                    N = O._internals = {
                        isArray: P,
                        isSelector: k,
                        lazyTweens: L,
                        blobDif: R
                    },
                    Y = O._plugins = {},
                    X = N.tweenLookup = {},
                    V = 0,
                    j = N.reservedProps = {
                        ease: 1,
                        delay: 1,
                        overwrite: 1,
                        onComplete: 1,
                        onCompleteParams: 1,
                        onCompleteScope: 1,
                        useFrames: 1,
                        runBackwards: 1,
                        startAt: 1,
                        onUpdate: 1,
                        onUpdateParams: 1,
                        onUpdateScope: 1,
                        onStart: 1,
                        onStartParams: 1,
                        onStartScope: 1,
                        onReverseComplete: 1,
                        onReverseCompleteParams: 1,
                        onReverseCompleteScope: 1,
                        onRepeat: 1,
                        onRepeatParams: 1,
                        onRepeatScope: 1,
                        easeParams: 1,
                        yoyo: 1,
                        immediateRender: 1,
                        repeat: 1,
                        repeatDelay: 1,
                        data: 1,
                        paused: 1,
                        reversed: 1,
                        autoCSS: 1,
                        lazy: 1,
                        onOverwrite: 1,
                        callbackScope: 1,
                        stringFilter: 1,
                        id: 1
                    },
                    z = {
                        none: 0,
                        all: 1,
                        auto: 2,
                        concurrent: 3,
                        allOnStart: 4,
                        preexisting: 5,
                        true: 1,
                        false: 0
                    },
                    G = d._rootFramesTimeline = new E,
                    q = d._rootTimeline = new E,
                    K = 30,
                    J = N.lazyRender = function() {
                        var e, t = L.length;
                        for (I = {}; - 1 < --t;)(e = L[t]) && !1 !== e._lazy && (e.render(e._lazy[0], e._lazy[1], !0), e._lazy = !1);
                        L.length = 0
                    };
                q._startTime = b.time, G._startTime = b.frame, q._active = G._active = !0, setTimeout(J, 1), d._updateRoot = O.render = function() {
                    var e, t, o;
                    if (L.length && J(), q.render((b.time - q._startTime) * q._timeScale, !1, !1), G.render((b.frame - G._startTime) * G._timeScale, !1, !1), L.length && J(), b.frame >= K) {
                        for (o in K = b.frame + (parseInt(O.autoSleep, 10) || 120), X) {
                            for (e = (t = X[o].tweens).length; - 1 < --e;) t[e]._gc && t.splice(e, 1);
                            0 === t.length && delete X[o]
                        }
                        if ((!(o = q._first) || o._paused) && O.autoSleep && !G._first && 1 === b._listeners.tick.length) {
                            for (; o && o._paused;) o = o._next;
                            o || b.sleep()
                        }
                    }
                }, b.addEventListener("tick", d._updateRoot);
                var Q = function(e, t, o) {
                        var s, i, n = e._gsTweenID;
                        if (X[n || (e._gsTweenID = n = "t" + V++)] || (X[n] = {
                                target: e,
                                tweens: []
                            }), t && ((s = X[n].tweens)[i = s.length] = t, o))
                            for (; - 1 < --i;) s[i] === t && s.splice(i, 1);
                        return X[n].tweens
                    },
                    $ = function(e, t, o, s) {
                        var i, n, l = e.vars.onOverwrite;
                        return l && (i = l(e, t, o, s)), (l = O.onOverwrite) && (n = l(e, t, o, s)), !1 !== i && !1 !== n
                    },
                    Z = function(e, t, o, s, i) {
                        var n, l, r, a;
                        if (1 === s || 4 <= s) {
                            for (a = i.length, n = 0; n < a; n++)
                                if ((r = i[n]) !== t) r._gc || r._kill(null, e, t) && (l = !0);
                                else if (5 === s) break;
                            return l
                        }
                        var d, u = t._startTime + y,
                            c = [],
                            h = 0,
                            _ = 0 === t._duration;
                        for (n = i.length; - 1 < --n;)(r = i[n]) === t || r._gc || r._paused || (r._timeline !== t._timeline ? (d = d || ee(t, 0, _), 0 === ee(r, d, _) && (c[h++] = r)) : r._startTime <= u && r._startTime + r.totalDuration() / r._timeScale > u && ((_ || !r._initted) && u - r._startTime <= 2e-10 || (c[h++] = r)));
                        for (n = h; - 1 < --n;)
                            if (r = c[n], 2 === s && r._kill(o, e, t) && (l = !0), 2 !== s || !r._firstPT && r._initted) {
                                if (2 !== s && !$(r, t)) continue;
                                r._enabled(!1, !1) && (l = !0)
                            } return l
                    },
                    ee = function(e, t, o) {
                        for (var s = e._timeline, i = s._timeScale, n = e._startTime; s._timeline;) {
                            if (n += s._startTime, i *= s._timeScale, s._paused) return -100;
                            s = s._timeline
                        }
                        return t < (n /= i) ? n - t : o && n === t || !e._initted && n - t < 2 * y ? y : (n += e.totalDuration() / e._timeScale / i) > t + y ? 0 : n - t - y
                    };
                o._init = function() {
                    var e, t, o, s, i, n, l = this.vars,
                        r = this._overwrittenProps,
                        a = this._duration,
                        d = !!l.immediateRender,
                        u = l.ease;
                    if (l.startAt) {
                        for (s in this._startAt && (this._startAt.render(-1, !0), this._startAt.kill()), i = {}, l.startAt) i[s] = l.startAt[s];
                        if (i.overwrite = !1, i.immediateRender = !0, i.lazy = d && !1 !== l.lazy, i.startAt = i.delay = null, this._startAt = O.to(this.target, 0, i), d)
                            if (0 < this._time) this._startAt = null;
                            else if (0 !== a) return
                    } else if (l.runBackwards && 0 !== a)
                        if (this._startAt) this._startAt.render(-1, !0), this._startAt.kill(), this._startAt = null;
                        else {
                            for (s in 0 !== this._time && (d = !1), o = {}, l) j[s] && "autoCSS" !== s || (o[s] = l[s]);
                            if (o.overwrite = 0, o.data = "isFromStart", o.lazy = d && !1 !== l.lazy, o.immediateRender = d, this._startAt = O.to(this.target, 0, o), d) {
                                if (0 === this._time) return
                            } else this._startAt._init(), this._startAt._enabled(!1), this.vars.immediateRender && (this._startAt = null)
                        } if (this._ease = u = u ? u instanceof D ? u : "function" == typeof u ? new D(u, l.easeParams) : B[u] || O.defaultEase : O.defaultEase, l.easeParams instanceof Array && u.config && (this._ease = u.config.apply(u, l.easeParams)), this._easeType = this._ease._type, this._easePower = this._ease._power, this._firstPT = null, this._targets)
                        for (n = this._targets.length, e = 0; e < n; e++) this._initProps(this._targets[e], this._propLookup[e] = {}, this._siblings[e], r ? r[e] : null, e) && (t = !0);
                    else t = this._initProps(this.target, this._propLookup, this._siblings, r, 0);
                    if (t && O._onPluginEvent("_onInitAllProps", this), r && (this._firstPT || "function" != typeof this.target && this._enabled(!1, !1)), l.runBackwards)
                        for (o = this._firstPT; o;) o.s += o.c, o.c = -o.c, o = o._next;
                    this._onUpdate = l.onUpdate, this._initted = !0
                }, o._initProps = function(e, t, o, s, i) {
                    var n, l, r, a, d, u;
                    if (null == e) return !1;
                    for (n in I[e._gsTweenID] && J(), this.vars.css || e.style && e !== _ && e.nodeType && Y.css && !1 !== this.vars.autoCSS && function(e, t) {
                            var o, s = {};
                            for (o in e) j[o] || o in t && "transform" !== o && "x" !== o && "y" !== o && "width" !== o && "height" !== o && "className" !== o && "border" !== o || !(!Y[o] || Y[o] && Y[o]._autoCSS) || (s[o] = e[o], delete e[o]);
                            e.css = s
                        }(this.vars, e), this.vars)
                        if (u = this.vars[n], j[n]) u && (u instanceof Array || u.push && P(u)) && -1 !== u.join("").indexOf("{self}") && (this.vars[n] = u = this._swapSelfInParams(u, this));
                        else if (Y[n] && (a = new Y[n])._onInitTween(e, this.vars[n], this, i)) {
                        for (this._firstPT = d = {
                                _next: this._firstPT,
                                t: a,
                                p: "setRatio",
                                s: 0,
                                c: 1,
                                f: 1,
                                n: n,
                                pg: 1,
                                pr: a._priority,
                                m: 0
                            }, l = a._overwriteProps.length; - 1 < --l;) t[a._overwriteProps[l]] = this._firstPT;
                        (a._priority || a._onInitAllProps) && (r = !0), (a._onDisable || a._onEnable) && (this._notifyPluginsOfEnabled = !0), d._next && (d._next._prev = d)
                    } else t[n] = U.call(this, e, n, "get", u, n, 0, null, this.vars.stringFilter, i);
                    return s && this._kill(s, e) ? this._initProps(e, t, o, s, i) : 1 < this._overwrite && this._firstPT && 1 < o.length && Z(e, this, t, this._overwrite, o) ? (this._kill(t, e), this._initProps(e, t, o, s, i)) : (this._firstPT && (!1 !== this.vars.lazy && this._duration || this.vars.lazy && !this._duration) && (I[e._gsTweenID] = !0), r)
                }, o.render = function(e, t, o) {
                    var s, i, n, l, r = this._time,
                        a = this._duration,
                        d = this._rawPrevTime;
                    if (a - 1e-7 <= e) this._totalTime = this._time = a, this.ratio = this._ease._calcEnd ? this._ease.getRatio(1) : 1, this._reversed || (s = !0, i = "onComplete", o = o || this._timeline.autoRemoveChildren), 0 === a && (!this._initted && this.vars.lazy && !o || (this._startTime === this._timeline._duration && (e = 0), (d < 0 || e <= 0 && -1e-7 <= e || d === y && "isPause" !== this.data) && d !== e && (o = !0, y < d && (i = "onReverseComplete")), this._rawPrevTime = l = !t || e || d === e ? e : y));
                    else if (e < 1e-7) this._totalTime = this._time = 0, this.ratio = this._ease._calcEnd ? this._ease.getRatio(0) : 0, (0 !== r || 0 === a && 0 < d) && (i = "onReverseComplete", s = this._reversed), e < 0 && (this._active = !1, 0 === a && (!this._initted && this.vars.lazy && !o || (0 <= d && (d !== y || "isPause" !== this.data) && (o = !0), this._rawPrevTime = l = !t || e || d === e ? e : y))), this._initted || (o = !0);
                    else if (this._totalTime = this._time = e, this._easeType) {
                        var u = e / a,
                            c = this._easeType,
                            h = this._easePower;
                        (1 === c || 3 === c && .5 <= u) && (u = 1 - u), 3 === c && (u *= 2), 1 === h ? u *= u : 2 === h ? u *= u * u : 3 === h ? u *= u * u * u : 4 === h && (u *= u * u * u * u), this.ratio = 1 === c ? 1 - u : 2 === c ? u : e / a < .5 ? u / 2 : 1 - u / 2
                    } else this.ratio = this._ease.getRatio(e / a);
                    if (this._time !== r || o) {
                        if (!this._initted) {
                            if (this._init(), !this._initted || this._gc) return;
                            if (!o && this._firstPT && (!1 !== this.vars.lazy && this._duration || this.vars.lazy && !this._duration)) return this._time = this._totalTime = r, this._rawPrevTime = d, L.push(this), void(this._lazy = [e, t]);
                            this._time && !s ? this.ratio = this._ease.getRatio(this._time / a) : s && this._ease._calcEnd && (this.ratio = this._ease.getRatio(0 === this._time ? 0 : 1))
                        }
                        for (!1 !== this._lazy && (this._lazy = !1), this._active || !this._paused && this._time !== r && 0 <= e && (this._active = !0), 0 === r && (this._startAt && (0 <= e ? this._startAt.render(e, t, o) : i = i || "_dummyGS"), this.vars.onStart && (0 === this._time && 0 !== a || t || this._callback("onStart"))), n = this._firstPT; n;) n.f ? n.t[n.p](n.c * this.ratio + n.s) : n.t[n.p] = n.c * this.ratio + n.s, n = n._next;
                        this._onUpdate && (e < 0 && this._startAt && -1e-4 !== e && this._startAt.render(e, t, o), t || (this._time !== r || s || o) && this._callback("onUpdate")), i && (this._gc && !o || (e < 0 && this._startAt && !this._onUpdate && -1e-4 !== e && this._startAt.render(e, t, o), s && (this._timeline.autoRemoveChildren && this._enabled(!1, !1), this._active = !1), !t && this.vars[i] && this._callback(i), 0 === a && this._rawPrevTime === y && l !== y && (this._rawPrevTime = 0)))
                    }
                }, o._kill = function(e, t, o) {
                    if ("all" === e && (e = null), null == e && (null == t || t === this.target)) return this._lazy = !1, this._enabled(!1, !1);
                    t = "string" != typeof t ? t || this._targets || this.target : O.selector(t) || t;
                    var s, i, n, l, r, a, d, u, c, h = o && this._time && o._startTime === this._startTime && this._timeline === o._timeline;
                    if ((P(t) || k(t)) && "number" != typeof t[0])
                        for (s = t.length; - 1 < --s;) this._kill(e, t[s], o) && (a = !0);
                    else {
                        if (this._targets) {
                            for (s = this._targets.length; - 1 < --s;)
                                if (t === this._targets[s]) {
                                    r = this._propLookup[s] || {}, this._overwrittenProps = this._overwrittenProps || [], i = this._overwrittenProps[s] = e ? this._overwrittenProps[s] || {} : "all";
                                    break
                                }
                        } else {
                            if (t !== this.target) return !1;
                            r = this._propLookup, i = this._overwrittenProps = e ? this._overwrittenProps || {} : "all"
                        }
                        if (r) {
                            if (d = e || r, u = e !== i && "all" !== i && e !== r && ("object" != typeof e || !e._tempKill), o && (O.onOverwrite || this.vars.onOverwrite)) {
                                for (n in d) r[n] && (c = c || []).push(n);
                                if ((c || !e) && !$(this, o, t, c)) return !1
                            }
                            for (n in d)(l = r[n]) && (h && (l.f ? l.t[l.p](l.s) : l.t[l.p] = l.s, a = !0), l.pg && l.t._kill(d) && (a = !0), l.pg && 0 !== l.t._overwriteProps.length || (l._prev ? l._prev._next = l._next : l === this._firstPT && (this._firstPT = l._next), l._next && (l._next._prev = l._prev), l._next = l._prev = null), delete r[n]), u && (i[n] = 1);
                            !this._firstPT && this._initted && this._enabled(!1, !1)
                        }
                    }
                    return a
                }, o.invalidate = function() {
                    return this._notifyPluginsOfEnabled && O._onPluginEvent("_onDisable", this), this._firstPT = this._overwrittenProps = this._startAt = this._onUpdate = null, this._notifyPluginsOfEnabled = this._active = this._lazy = !1, this._propLookup = this._targets ? {} : [], d.prototype.invalidate.call(this), this.vars.immediateRender && (this._time = -y, this.render(Math.min(0, -this._delay))), this
                }, o._enabled = function(e, t) {
                    if (g || b.wake(), e && this._gc) {
                        var o, s = this._targets;
                        if (s)
                            for (o = s.length; - 1 < --o;) this._siblings[o] = Q(s[o], this, !0);
                        else this._siblings = Q(this.target, this, !0)
                    }
                    return d.prototype._enabled.call(this, e, t), !(!this._notifyPluginsOfEnabled || !this._firstPT) && O._onPluginEvent(e ? "_onEnable" : "_onDisable", this)
                }, O.to = function(e, t, o) {
                    return new O(e, t, o)
                }, O.from = function(e, t, o) {
                    return o.runBackwards = !0, o.immediateRender = 0 != o.immediateRender, new O(e, t, o)
                }, O.fromTo = function(e, t, o, s) {
                    return s.startAt = o, s.immediateRender = 0 != s.immediateRender && 0 != o.immediateRender, new O(e, t, s)
                }, O.delayedCall = function(e, t, o, s, i) {
                    return new O(t, 0, {
                        delay: e,
                        onComplete: t,
                        onCompleteParams: o,
                        callbackScope: s,
                        onReverseComplete: t,
                        onReverseCompleteParams: o,
                        immediateRender: !1,
                        lazy: !1,
                        useFrames: i,
                        overwrite: 0
                    })
                }, O.set = function(e, t) {
                    return new O(e, 0, t)
                }, O.getTweensOf = function(e, t) {
                    if (null == e) return [];
                    var o, s, i, n;
                    if (e = "string" != typeof e ? e : O.selector(e) || e, (P(e) || k(e)) && "number" != typeof e[0]) {
                        for (o = e.length, s = []; - 1 < --o;) s = s.concat(O.getTweensOf(e[o], t));
                        for (o = s.length; - 1 < --o;)
                            for (n = s[o], i = o; - 1 < --i;) n === s[i] && s.splice(o, 1)
                    } else
                        for (o = (s = Q(e).concat()).length; - 1 < --o;)(s[o]._gc || t && !s[o].isActive()) && s.splice(o, 1);
                    return s
                }, O.killTweensOf = O.killDelayedCallsTo = function(e, t, o) {
                    "object" == typeof t && (o = t, t = !1);
                    for (var s = O.getTweensOf(e, t), i = s.length; - 1 < --i;) s[i]._kill(o, e)
                };
                var te = h("plugins.TweenPlugin", function(e, t) {
                    this._overwriteProps = (e || "").split(","), this._propName = this._overwriteProps[0], this._priority = t || 0, this._super = te.prototype
                }, !0);
                if (o = te.prototype, te.version = "1.19.0", te.API = 2, o._firstPT = null, o._addTween = U, o.setRatio = A, o._kill = function(e) {
                        var t, o = this._overwriteProps,
                            s = this._firstPT;
                        if (null != e[this._propName]) this._overwriteProps = [];
                        else
                            for (t = o.length; - 1 < --t;) null != e[o[t]] && o.splice(t, 1);
                        for (; s;) null != e[s.n] && (s._next && (s._next._prev = s._prev), s._prev ? (s._prev._next = s._next, s._prev = null) : this._firstPT === s && (this._firstPT = s._next)), s = s._next;
                        return !1
                    }, o._mod = o._roundProps = function(e) {
                        for (var t, o = this._firstPT; o;)(t = e[this._propName] || null != o.n && e[o.n.split(this._propName + "_").join("")]) && "function" == typeof t && (2 === o.f ? o.t._applyPT.m = t : o.m = t), o = o._next
                    }, O._onPluginEvent = function(e, t) {
                        var o, s, i, n, l, r = t._firstPT;
                        if ("_onInitAllProps" === e) {
                            for (; r;) {
                                for (l = r._next, s = i; s && s.pr > r.pr;) s = s._next;
                                (r._prev = s ? s._prev : n) ? r._prev._next = r: i = r, (r._next = s) ? s._prev = r : n = r, r = l
                            }
                            r = t._firstPT = i
                        }
                        for (; r;) r.pg && "function" == typeof r.t[e] && r.t[e]() && (o = !0), r = r._next;
                        return o
                    }, te.activate = function(e) {
                        for (var t = e.length; - 1 < --t;) e[t].API === te.API && (Y[(new e[t])._propName] = e[t]);
                        return !0
                    }, n.plugin = function(e) {
                        if (!(e && e.propName && e.init && e.API)) throw "illegal plugin definition.";
                        var t, o = e.propName,
                            s = e.priority || 0,
                            i = e.overwriteProps,
                            n = {
                                init: "_onInitTween",
                                set: "setRatio",
                                kill: "_kill",
                                round: "_mod",
                                mod: "_mod",
                                initAll: "_onInitAllProps"
                            },
                            l = h("plugins." + o.charAt(0).toUpperCase() + o.substr(1) + "Plugin", function() {
                                te.call(this, o, s), this._overwriteProps = i || []
                            }, !0 === e.fwd_global),
                            r = l.prototype = new te(o);
                        for (t in (r.constructor = l).API = e.API, n) "function" == typeof e[t] && (r[n[t]] = e[t]);
                        return l.version = e.version, te.activate([l]), l
                    }, e = _._fwd_gsQueue) {
                    for (t = 0; t < e.length; t++) e[t]();
                    for (o in T) T[o].func || _.console.log("GSAP encountered missing dependency: " + o)
                }
                g = !1
            }
        }("undefined" != typeof fwd_module && fwd_module.exports && "undefined" != typeof fwd_global ? fwd_global : this || window, "FWDAnimation")
}
if (! function(e) {
        var t = function() {
            var i = this;
            t.prototype;
            this.main_do = null, this.init = function() {
                this.setupScreen(), e.onerror = this.showError, this.screen.style.zIndex = 100000009, setTimeout(this.addConsoleToDom, 100), setInterval(this.position, 100)
            }, this.position = function() {
                var e = FWDMSPUtils.getScrollOffsets();
                i.setX(e.x + 200), i.setY(e.y)
            }, this.addConsoleToDom = function() {
                -1 != navigator.userAgent.toLowerCase().indexOf("msie 7") ? document.getElementsByTagName("body")[0].appendChild(i.screen) : document.documentElement.appendChild(i.screen)
            }, this.setupScreen = function() {
                this.main_do = new FWDMSPDisplayObject("div", "absolute"), this.main_do.setOverflow("auto"), this.main_do.setWidth(200), this.main_do.setHeight(300), this.setWidth(200), this.setHeight(300), this.main_do.setBkColor("#FFFFFF"), this.addChild(this.main_do)
            }, this.showError = function(e, t, o) {
                var s = i.main_do.getInnerHTML() + "<br>JavaScript error: " + e + " on line " + o + " for " + t;
                i.main_do.setInnerHTML(s), i.main_do.screen.scrollTop = i.main_do.screen.scrollHeight
            }, this.log = function(e) {
                var t = i.main_do.getInnerHTML() + "<br>" + e;
                i.main_do.setInnerHTML(t), i.main_do.getScreen().scrollTop = 1e4
            }, this.init()
        };
        t.setPrototype = function() {
            t.prototype = new FWDMSPDisplayObject("div", "absolute")
        }, t.prototype = null, e.FWDConsole = t
    }(window), void 0 === asual) var asual = {};
void 0 === asual.util && (asual.util = {}), asual.util.Browser = new function() {
    var e = navigator.userAgent.toLowerCase(),
        t = /webkit/.test(e),
        o = /opera/.test(e),
        s = /msie/.test(e) && !/opera/.test(e),
        i = /mozilla/.test(e) && !/(compatible|webkit)/.test(e),
        n = parseFloat(s ? e.substr(e.indexOf("msie") + 4) : (e.match(/.+(?:rv|it|ra|ie)[\/: ]([\d.]+)/) || [0, "0"])[1]);
    this.toString = function() {
        return "[class Browser]"
    }, this.getVersion = function() {
        return n
    }, this.isMSIE = function() {
        return s
    }, this.isSafari = function() {
        return t
    }, this.isOpera = function() {
        return o
    }, this.isMozilla = function() {
        return i
    }
}, asual.util.Events = new function() {
    var n = "DOMContentLoaded",
        t = "onstop",
        o = window,
        s = document,
        l = [],
        i = asual.util,
        e = i.Browser,
        r = e.isMSIE(),
        a = e.isSafari();
    this.toString = function() {
        return "[class Events]"
    }, this.addListener = function(e, t, o) {
        l.push({
            o: e,
            t: t,
            l: o
        }), t == n && (r || a) || (e.addEventListener ? e.addEventListener(t, o, !1) : e.attachEvent && e.attachEvent("on" + t, o))
    }, this.removeListener = function(e, t, o) {
        for (var s, i = 0; s = l[i]; i++)
            if (s.o == e && s.t == t && s.l == o) {
                l.splice(i, 1);
                break
            } t == n && (r || a) || (e.removeEventListener ? e.removeEventListener(t, o, !1) : e.detachEvent && e.detachEvent("on" + t, o))
    };

    function d() {
        for (var e, t = 0; e = l[t]; t++) e.t != n && i.Events.removeListener(e.o, e.t, e.l)
    }(r || a) && function() {
        try {
            (r && s.body || !/loaded|complete/.test(s.readyState)) && s.documentElement.doScroll("left")
        } catch (e) {
            return setTimeout(arguments.callee, 0)
        }
        for (var e, t = 0; e = l[t]; t++) e.t == n && e.l.call(null)
    }(), r && o.attachEvent && o.attachEvent("onbeforeunload", function() {
        if ("interactive" == s.readyState) {
            function e() {
                s.detachEvent(t, e), d()
            }
            s.attachEvent(t, e), o.setTimeout(function() {
                s.detachEvent(t, e)
            }, 0)
        }
    }), this.addListener(o, "unload", d)
}, asual.util.Functions = new function() {
    this.toString = function() {
        return "[class Functions]"
    }, this.bind = function(e, t, o) {
        for (var s, i = 2, n = []; s = arguments[i]; i++) n.push(s);
        return function() {
            return e.apply(t, n)
        }
    }
};
var FWDAddressEvent = function(e) {
    this.toString = function() {
        return "[object FWDAddressEvent]"
    }, this.type = e, this.target = [FWDAddress][0], this.value = FWDAddress.getValue(), this.path = FWDAddress.getPath(), this.pathNames = FWDAddress.getPathNames(), this.parameters = {};
    for (var t = FWDAddress.getParameterNames(), o = 0, s = t.length; o < s; o++) this.parameters[t[o]] = FWDAddress.getParameter(t[o]);
    this.parameterNames = t
};
FWDAddressEvent.INIT = "init", FWDAddressEvent.CHANGE = "change", FWDAddressEvent.INTERNAL_CHANGE = "internalChange", FWDAddressEvent.EXTERNAL_CHANGE = "externalChange";
var FWDAddress = new function() {
        var _getHash = function() {
                var e = _l.href.indexOf("#");
                return -1 != e ? _ec(_dc(_l.href.substr(e + 1))) : ""
            },
            _getWindow = function() {
                try {
                    return top.document, top
                } catch (e) {
                    return window
                }
            },
            _strictCheck = function(e, t) {
                return _opts.strict && (e = t ? "/" != e.substr(0, 1) ? "/" + e : e : "" == e ? "/" : e), e
            },
            _ieLocal = function(e, t) {
                return _msie && "file:" == _l.protocol ? t ? _value.replace(/\?/, "%3F") : _value.replace(/%253F/, "?") : e
            },
            _searchScript = function(e) {
                if (e.childNodes)
                    for (var t, o = 0, s = e.childNodes.length; o < s; o++)
                        if (e.childNodes[o].src && (_url = String(e.childNodes[o].src)), t = _searchScript(e.childNodes[o])) return t
            },
            _titleCheck = function() {
                _d.title != _title && -1 != _d.title.indexOf("#") && (_d.title = _title)
            },
            _listen = function() {
                if (!_silent) {
                    var e = _getHash(),
                        t = !(_value == e);
                    _safari && _version < 523 ? _length != _h.length && (_length = _h.length, typeof _stack[_length - 1] != UNDEFINED && (_value = _stack[_length - 1]), _update.call(this, !1)) : _msie && t ? _version < 7 ? _l.reload() : this.setValue(e) : t && (_value = e, _update.call(this, !1)), _msie && _titleCheck.call(this)
                }
            },
            _bodyClick = function(e) {
                if (0 < _popup.length) {
                    var popup = window.open(_popup[0], _popup[1], eval(_popup[2]));
                    typeof _popup[3] != UNDEFINED && eval(_popup[3])
                }
                _popup = []
            },
            _swfChange = function() {
                for (var e, t, o = 0, s = FWDAddress.getValue(), i = "setFWDAddressValue"; e = _ids[o]; o++)
                    if (t = document.getElementById(e))
                        if (t.parentNode && typeof t.parentNode.so != UNDEFINED) t.parentNode.so.call(i, s);
                        else {
                            if (!t || typeof t[i] == UNDEFINED) {
                                var n = t.getElementsByTagName("object"),
                                    l = t.getElementsByTagName("embed");
                                t = n[0] && typeof n[0][i] != UNDEFINED ? n[0] : l[0] && typeof l[0][i] != UNDEFINED ? l[0] : null
                            }
                            t && t[i](s)
                        }
                else(t = document[e]) && typeof t[i] != UNDEFINED && t[i](s)
            },
            _jsDispatch = function(e) {
                this.dispatchEvent(new FWDAddressEvent(e)), typeof this["on" + (e = e.substr(0, 1).toUpperCase() + e.substr(1))] == FUNCTION && this["on" + e]()
            },
            _jsInit = function() {
                _util.Browser.isSafari() && _d.body.addEventListener("click", _bodyClick), _jsDispatch.call(this, "init")
            },
            _jsChange = function() {
                _swfChange(), _jsDispatch.call(this, "change")
            },
            _update = function(e) {
                _jsChange.call(this), e ? _jsDispatch.call(this, "internalChange") : _jsDispatch.call(this, "externalChange"), _st(_functions.bind(_track, this), 10)
            },
            _track = function() {
                var e = (_l.pathname + (/\/$/.test(_l.pathname) ? "" : "/") + this.getValue()).replace(/\/\//, "/").replace(/^\/$/, ""),
                    t = _t[_opts.tracker];
                typeof t == FUNCTION ? t(e) : typeof _t.pageTracker != UNDEFINED && typeof _t.pageTracker._trackPageview == FUNCTION ? _t.pageTracker._trackPageview(e) : typeof _t.urchinTracker == FUNCTION && _t.urchinTracker(e)
            },
            _htmlWrite = function() {
                var e = _frame.contentWindow.document;
                e.open(), e.write("<html><head><title>" + _d.title + "</title><script>var " + ID + ' = "' + _getHash() + '";<\/script></head></html>'), e.close()
            },
            _htmlLoad = function() {
                var e = _frame.contentWindow;
                e.location.href;
                (_value = typeof e[ID] != UNDEFINED ? e[ID] : "") != _getHash() && (_update.call(FWDAddress, !1), _l.hash = _ieLocal(_value, TRUE))
            },
            _load = function() {
                if (!_loaded) {
                    if (_loaded = TRUE, _msie && _version < 8) {
                        var e = _d.getElementsByTagName("frameset")[0];
                        _frame = _d.createElement((e ? "" : "i") + "frame"), e ? (e.insertAdjacentElement("beforeEnd", _frame), e[e.cols ? "cols" : "rows"] += ",0", _frame.src = "javascript:false", _frame.noResize = !0, _frame.frameBorder = _frame.frameSpacing = 0) : (_frame.src = "javascript:false", _frame.style.display = "none", _d.body.insertAdjacentElement("afterBegin", _frame)), _st(function() {
                            _events.addListener(_frame, "load", _htmlLoad), typeof _frame.contentWindow[ID] == UNDEFINED && _htmlWrite()
                        }, 50)
                    } else _safari && (_version < 418 && (_d.body.innerHTML += '<form id="' + ID + '" style="position:absolute;top:-9999px;" method="get"></form>', _form = _d.getElementById(ID)), typeof _l[ID] == UNDEFINED && (_l[ID] = {}), typeof _l[ID][_l.pathname] != UNDEFINED && (_stack = _l[ID][_l.pathname].split(",")));
                    _st(_functions.bind(function() {
                        _jsInit.call(this), _jsChange.call(this), _track.call(this)
                    }, this), 1), _msie && 8 <= _version ? (_d.body.onhashchange = _functions.bind(_listen, this), _si(_functions.bind(_titleCheck, this), 50)) : _si(_functions.bind(_listen, this), 50)
                }
            },
            ID = "swfaddress",
            FUNCTION = "function",
            UNDEFINED = "undefined",
            TRUE = !0,
            FALSE = !1,
            _util = asual.util,
            _browser = _util.Browser,
            _events = _util.Events,
            _functions = _util.Functions,
            _version = _browser.getVersion(),
            _msie = _browser.isMSIE(),
            _mozilla = _browser.isMozilla(),
            _opera = _browser.isOpera(),
            _safari = _browser.isSafari(),
            _supported = FALSE,
            _t = _getWindow(),
            _d = _t.document,
            _h = _t.history,
            _l = _t.location,
            _si = setInterval,
            _st = setTimeout,
            _dc = decodeURI,
            _ec = encodeURI,
            _frame, _form, _url, _title = _d.title,
            _length = _h.length,
            _silent = FALSE,
            _loaded = FALSE,
            _justset = TRUE,
            _juststart = TRUE,
            _ref = this,
            _stack = [],
            _ids = [],
            _popup = [],
            _listeners = {},
            _value = _getHash(),
            _opts = {
                history: TRUE,
                strict: TRUE
            };
        if (_msie && _d.documentMode && _d.documentMode != _version && (_version = 8 != _d.documentMode ? 7 : 8), _supported = _mozilla && 1 <= _version || _msie && 6 <= _version || _opera && 9.5 <= _version || _safari && 312 <= _version, _supported) {
            _opera && (history.navigationMode = "compatible");
            for (var i = 1; i < _length; i++) _stack.push("");
            _stack.push(_getHash()), _msie && _l.hash != _getHash() && (_l.hash = "#" + _ieLocal(_getHash(), TRUE)), _searchScript(document);
            var _qi = _url ? _url.indexOf("?") : -1;
            if (-1 != _qi)
                for (var param, params = _url.substr(_qi + 1).split("&"), i = 0, p; p = params[i]; i++) param = p.split("="), /^(history|strict)$/.test(param[0]) && (_opts[param[0]] = isNaN(param[1]) ? /^(true|yes)$/i.test(param[1]) : 0 != parseInt(param[1])), /^tracker$/.test(param[0]) && (_opts[param[0]] = param[1]);
            _msie && _titleCheck.call(this), window == _t && _events.addListener(document, "DOMContentLoaded", _functions.bind(_load, this)), _events.addListener(_t, "load", _functions.bind(_load, this))
        } else !_supported && -1 != _l.href.indexOf("#") || _safari && _version < 418 && -1 != _l.href.indexOf("#") && "" != _l.search ? (_d.open(), _d.write('<html><head><meta http-equiv="refresh" content="0;url=' + _l.href.substr(0, _l.href.indexOf("#")) + '" /></head></html>'), _d.close()) : _track();
        this.toString = function() {
                return "[class FWDAddress]"
            }, this.back = function() {
                _h.back()
            }, this.forward = function() {
                _h.forward()
            }, this.up = function() {
                var e = this.getPath();
                this.setValue(e.substr(0, e.lastIndexOf("/", e.length - 2) + ("/" == e.substr(e.length - 1) ? 1 : 0)))
            }, this.go = function(e) {
                _h.go(e)
            }, this.href = function(e, t) {
                "_self" == (t = typeof t != UNDEFINED ? t : "_self") ? self.location.href = e: "_top" == t ? _l.href = e : "_blank" == t ? window.open(e) : _t.frames[t].location.href = e
            }, this.popup = function(url, name, options, handler) {
                try {
                    var popup = window.open(url, name, eval(options));
                    typeof handler != UNDEFINED && eval(handler)
                } catch (e) {}
                _popup = arguments
            }, this.getIds = function() {
                return _ids
            }, this.getId = function(e) {
                return _ids[0]
            }, this.setId = function(e) {
                _ids[0] = e
            }, this.addId = function(e) {
                this.removeId(e), _ids.push(e)
            }, this.removeId = function(e) {
                for (var t = 0; t < _ids.length; t++)
                    if (e == _ids[t]) {
                        _ids.splice(t, 1);
                        break
                    }
            }, this.addEventListener = function(e, t) {
                typeof _listeners[e] == UNDEFINED && (_listeners[e] = []), _listeners[e].push(t)
            }, this.removeEventListener = function(e, t) {
                if (typeof _listeners[e] != UNDEFINED) {
                    for (var o, s = 0;
                        (o = _listeners[e][s]) && o != t; s++);
                    _listeners[e].splice(s, 1)
                }
            }, this.dispatchEvent = function(e) {
                if (this.hasEventListener(e.type)) {
                    e.target = this;
                    for (var t, o = 0; t = _listeners[e.type][o]; o++) t(e);
                    return TRUE
                }
                return FALSE
            }, this.hasEventListener = function(e) {
                return typeof _listeners[e] != UNDEFINED && 0 < _listeners[e].length
            }, this.getBaseURL = function() {
                var e = _l.href;
                return -1 != e.indexOf("#") && (e = e.substr(0, e.indexOf("#"))), "/" == e.substr(e.length - 1) && (e = e.substr(0, e.length - 1)), e
            }, this.getStrict = function() {
                return _opts.strict
            }, this.setStrict = function(e) {
                _opts.strict = e
            }, this.getHistory = function() {
                return _opts.history
            }, this.setHistory = function(e) {
                _opts.history = e
            }, this.getTracker = function() {
                return _opts.tracker
            }, this.setTracker = function(e) {
                _opts.tracker = e
            }, this.getTitle = function() {
                return _d.title
            }, this.setTitle = function(e) {
                if (!_supported) return null;
                typeof e != UNDEFINED && ("null" == e && (e = ""), e = _dc(e), _st(function() {
                    _title = _d.title = e, _juststart && _frame && _frame.contentWindow && _frame.contentWindow.document && (_frame.contentWindow.document.title = e, _juststart = FALSE), !_justset && _mozilla && _l.replace(-1 != _l.href.indexOf("#") ? _l.href : _l.href + "#"), _justset = FALSE
                }, 10))
            }, this.getStatus = function() {
                return _t.status
            }, this.setStatus = function(e) {
                if (!_supported) return null;
                if (typeof e != UNDEFINED && ("null" == e && (e = ""), e = _dc(e), !_safari)) {
                    if ("/" == (e = _strictCheck("null" != e ? e : "", TRUE)) && (e = ""), !/http(s)?:\/\//.test(e)) {
                        var t = _l.href.indexOf("#");
                        e = (-1 == t ? _l.href : _l.href.substr(0, t)) + "#" + e
                    }
                    _t.status = e
                }
            }, this.resetStatus = function() {
                _t.status = ""
            }, this.getValue = function() {
                return _supported ? _dc(_strictCheck(_ieLocal(_value, FALSE), FALSE)) : null
            }, this.setValue = function(e) {
                if (!_supported) return null;
                if (typeof e != UNDEFINED && ("null" == e && (e = ""), "/" == (e = _ec(_dc(_strictCheck(e, TRUE)))) && (e = ""), _value != e)) {
                    if (_value = e, _silent = _justset = TRUE, _update.call(FWDAddress, !0), _stack[_h.length] = _value, _safari)
                        if (_opts.history)
                            if (_l[ID][_l.pathname] = _stack.toString(), _length = _h.length + 1, _version < 418) "" == _l.search && (_form.action = "#" + _value, _form.submit());
                            else if (_version < 523 || "" == _value) {
                        var t = _d.createEvent("MouseEvents");
                        t.initEvent("click", TRUE, TRUE);
                        var o = _d.createElement("a");
                        o.href = "#" + _value, o.dispatchEvent(t)
                    } else _l.hash = "#" + _value;
                    else _l.replace("#" + _value);
                    else _value != _getHash() && (_opts.history ? _l.hash = "#" + _dc(_ieLocal(_value, TRUE)) : _l.replace("#" + _dc(_value)));
                    _msie && _version < 8 && _opts.history && _st(_htmlWrite, 50), _safari ? _st(function() {
                        _silent = FALSE
                    }, 1) : _silent = FALSE
                }
            }, this.getPath = function() {
                var e = this.getValue();
                return -1 != e.indexOf("?") ? e.split("?")[0] : -1 != e.indexOf("#") ? e.split("#")[0] : e
            }, this.getPathNames = function() {
                var e = this.getPath(),
                    t = e.split("/");
                return "/" != e.substr(0, 1) && 0 != e.length || t.splice(0, 1), "/" == e.substr(e.length - 1, 1) && t.splice(t.length - 1, 1), t
            }, this.getQueryString = function() {
                var e = this.getValue(),
                    t = e.indexOf("?");
                if (-1 != t && t < e.length) return e.substr(t + 1)
            }, this.getParameter = function(e) {
                var t = this.getValue(),
                    o = t.indexOf("?");
                if (-1 != o) {
                    for (var s, i = (t = t.substr(o + 1)).split("&"), n = i.length, l = []; n--;)(s = i[n].split("="))[0] == e && l.push(s[1]);
                    if (0 != l.length) return 1 != l.length ? l : l[0]
                }
            }, this.getParameterNames = function() {
                var e = this.getValue(),
                    t = e.indexOf("?"),
                    o = [];
                if (-1 != t && "" != (e = e.substr(t + 1)) && -1 != e.indexOf("="))
                    for (var s = e.split("&"), i = 0; i < s.length;) o.push(s[i].split("=")[0]), i++;
                return o
            }, this.onInit = null, this.onChange = null, this.onInternalChange = null, this.onExternalChange = null,
            function() {
                var o;
                if (typeof FlashObject != UNDEFINED && (SWFObject = FlashObject), typeof SWFObject != UNDEFINED && SWFObject.prototype && SWFObject.prototype.write) {
                    var t = SWFObject.prototype.write;
                    SWFObject.prototype.write = function() {
                        var e;
                        return o = arguments, this.getAttribute("version").major < 8 && (this.addVariable("$swfaddress", FWDAddress.getValue()), ("string" == typeof o[0] ? document.getElementById(o[0]) : o[0]).so = this), (e = t.apply(this, o)) && _ref.addId(this.getAttribute("id")), e
                    }
                }
                if (typeof swfobject != UNDEFINED) {
                    var e = swfobject.registerObject;
                    swfobject.registerObject = function() {
                        o = arguments, e.apply(this, o), _ref.addId(o[0])
                    };
                    var s = swfobject.createSWF;
                    swfobject.createSWF = function() {
                        o = arguments;
                        var e = s.apply(this, o);
                        return e && _ref.addId(o[0].id), e
                    };
                    var i = swfobject.embedSWF;
                    swfobject.embedSWF = function() {
                        typeof(o = arguments)[8] == UNDEFINED && (o[8] = {}), typeof o[8].id == UNDEFINED && (o[8].id = o[1]), i.apply(this, o), _ref.addId(o[8].id)
                    }
                }
                if (typeof UFO != UNDEFINED) {
                    var n = UFO.create;
                    UFO.create = function() {
                        o = arguments, n.apply(this, o), _ref.addId(o[0].id)
                    }
                }
                if (typeof AC_FL_RunContent != UNDEFINED) {
                    var l = AC_FL_RunContent;
                    AC_FL_RunContent = function() {
                        o = arguments, l.apply(this, o);
                        for (var e = 0, t = o.length; e < t; e++) "id" == o[e] && _ref.addId(o[e + 1])
                    }
                }
            }()
    },
    FWDFlashTest = function() {
        var u = "undefined",
            c = "object",
            h = "Shockwave Flash",
            _ = "application/x-shockwave-flash",
            f = window,
            p = document,
            m = navigator,
            s = function() {
                var e = typeof p.getElementById != u && typeof p.getElementsByTagName != u && typeof p.createElement != u,
                    t = m.userAgent.toLowerCase(),
                    o = m.platform.toLowerCase(),
                    s = /win/.test(o || t),
                    i = /mac/.test(o || t),
                    n = !!/webkit/.test(t) && parseFloat(t.replace(/^.*webkit\/(\d+(\.\d+)?).*$/, "$1")),
                    l = !1,
                    r = [0, 0, 0],
                    a = null;
                if (typeof m.plugins != u && typeof m.plugins[h] == c) !(a = m.plugins[h].description) || typeof m.mimeTypes != u && m.mimeTypes[_] && !m.mimeTypes[_].enabledPlugin || (l = !!0, a = a.replace(/^.*\s+(\S+\s+\S+$)/, "$1"), r[0] = parseInt(a.replace(/^(.*)\..*$/, "$1"), 10), r[1] = parseInt(a.replace(/^.*\.(.*)\s.*$/, "$1"), 10), r[2] = /[a-zA-Z]/.test(a) ? parseInt(a.replace(/^.*[a-zA-Z]+(.*)$/, "$1"), 10) : 0);
                else if (typeof f.ActiveXObject != u) try {
                    var d = new ActiveXObject("ShockwaveFlash.ShockwaveFlash");
                    d && (a = d.GetVariable("$version")) && (l = !0, a = a.split(" ")[1].split(","), r = [parseInt(a[0], 10), parseInt(a[1], 10), parseInt(a[2], 10)])
                } catch (e) {}
                return {
                    w3: e,
                    pv: r,
                    wk: n,
                    ie: l,
                    win: s,
                    mac: i
                }
            }();

        function e(e) {
            var t = s.pv,
                o = e.split(".");
            return o[0] = parseInt(o[0], 10), o[1] = parseInt(o[1], 10) || 0, o[2] = parseInt(o[2], 10) || 0, t[0] > o[0] || t[0] == o[0] && t[1] > o[1] || t[0] == o[0] && t[1] == o[1] && t[2] >= o[2]
        }
        return {
            hasFlashPlayerVersion: e
        }
    }();
