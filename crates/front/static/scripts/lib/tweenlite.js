
(function(e) {
   "use strict";
   var t = e.GreenSockGlobals || e,
       n = function(e) {
           var n = e.split("."),
               r = t,
               i;
           for (i = 0; i < n.length; i++) {
               r[n[i]] = r = r[n[i]] || {}
           }
           return r
       },
       r = n("com.greensock"),
       i = [].slice,
       s = function() {},
       o, u, a, f, l, c = {},
       h = function(r, i, s, o) {
           this.sc = c[r] ? c[r].sc : [];
           c[r] = this;
           this.gsClass = null;
           this.func = s;
           var u = [];
           this.check = function(a) {
               var f = i.length,
                   l = f,
                   p, d, v, m;
               while (--f > -1) {
                   if ((p = c[i[f]] || new h(i[f], [])).gsClass) {
                       u[f] = p.gsClass;
                       l--
                   } else if (a) {
                       p.sc.push(this)
                   }
               }
               if (l === 0 && s) {
                   d = ("com.greensock." + r).split(".");
                   v = d.pop();
                   m = n(d.join("."))[v] = this.gsClass = s.apply(s, u);
                   if (o) {
                       t[v] = m;
                       if (typeof define === "function" && define.amd) {
                           define((e.GreenSockAMDPath ? e.GreenSockAMDPath + "/" : "") + r.split(".").join("/"), [], function() {
                               return m
                           })
                       } else if (typeof module !== "undefined" && module.exports) {
                           module.exports = m
                       }
                   }
                   for (f = 0; f < this.sc.length; f++) {
                       this.sc[f].check()
                   }
               }
           };
           this.check(true)
       },
       p = e._gsDefine = function(e, t, n, r) {
           return new h(e, t, n, r)
       },
       d = r._class = function(e, t, n) {
           t = t || function() {};
           p(e, [], function() {
               return t
           }, n);
           return t
       };
   p.globals = t;
   var v = [0, 0, 1, 1],
       m = [],
       g = d("easing.Ease", function(e, t, n, r) {
           this._func = e;
           this._type = n || 0;
           this._power = r || 0;
           this._params = t ? v.concat(t) : v
       }, true),
       y = g.map = {},
       b = g.register = function(e, t, n, i) {
           var s = t.split(","),
               o = s.length,
               u = (n || "easeIn,easeOut,easeInOut").split(","),
               a, f, l, c;
           while (--o > -1) {
               f = s[o];
               a = i ? d("easing." + f, null, true) : r.easing[f] || {};
               l = u.length;
               while (--l > -1) {
                   c = u[l];
                   y[f + "." + c] = y[c + f] = a[c] = e.getRatio ? e : e[c] || new e
               }
           }
       };
   a = g.prototype;
   a._calcEnd = false;
   a.getRatio = function(e) {
       if (this._func) {
           this._params[0] = e;
           return this._func.apply(null, this._params)
       }
       var t = this._type,
           n = this._power,
           r = t === 1 ? 1 - e : t === 2 ? e : e < .5 ? e * 2 : (1 - e) * 2;
       if (n === 1) {
           r *= r
       } else if (n === 2) {
           r *= r * r
       } else if (n === 3) {
           r *= r * r * r
       } else if (n === 4) {
           r *= r * r * r * r
       }
       return t === 1 ? 1 - r : t === 2 ? r : e < .5 ? r / 2 : 1 - r / 2
   };
   o = ["Linear", "Quad", "Cubic", "Quart", "Quint,Strong"];
   u = o.length;
   while (--u > -1) {
       a = o[u] + ",Power" + u;
       b(new g(null, null, 1, u), a, "easeOut", true);
       b(new g(null, null, 2, u), a, "easeIn" + (u === 0 ? ",easeNone" : ""));
       b(new g(null, null, 3, u), a, "easeInOut")
   }
   y.linear = r.easing.Linear.easeIn;
   y.swing = r.easing.Quad.easeInOut;
   var w = d("events.EventDispatcher", function(e) {
       this._listeners = {};
       this._eventTarget = e || this
   });
   a = w.prototype;
   a.addEventListener = function(e, t, n, r, i) {
       i = i || 0;
       var s = this._listeners[e],
           o = 0,
           u, a;
       if (s == null) {
           this._listeners[e] = s = []
       }
       a = s.length;
       while (--a > -1) {
           u = s[a];
           if (u.c === t && u.s === n) {
               s.splice(a, 1)
           } else if (o === 0 && u.pr < i) {
               o = a + 1
           }
       }
       s.splice(o, 0, {
           c: t,
           s: n,
           up: r,
           pr: i
       });
       if (this === f && !l) {
           f.wake()
       }
   };
   a.removeEventListener = function(e, t) {
       var n = this._listeners[e],
           r;
       if (n) {
           r = n.length;
           while (--r > -1) {
               if (n[r].c === t) {
                   n.splice(r, 1);
                   return
               }
           }
       }
   };
   a.dispatchEvent = function(e) {
       var t = this._listeners[e],
           n, r, i;
       if (t) {
           n = t.length;
           r = this._eventTarget;
           while (--n > -1) {
               i = t[n];
               if (i.up) {
                   i.c.call(i.s || r, {
                       type: e,
                       target: r
                   })
               } else {
                   i.c.call(i.s || r)
               }
           }
       }
   };
   var E = e.requestAnimationFrame,
       S = e.cancelAnimationFrame,
       x = Date.now || function() {
           return (new Date).getTime()
       };
   o = ["ms", "moz", "webkit", "o"];
   u = o.length;
   while (--u > -1 && !E) {
       E = e[o[u] + "RequestAnimationFrame"];
       S = e[o[u] + "CancelAnimationFrame"] || e[o[u] + "CancelRequestAnimationFrame"]
   }
   d("Ticker", function(e, t) {
       var n = this,
           r = x(),
           i = t !== false && E,
           o, u, a, c, h, p = function(e) {
               n.time = (x() - r) / 1e3;
               var t = a,
                   i = n.time - h;
               if (!o || i > 0 || e === true) {
                   n.frame++;
                   h += i + (i >= c ? .004 : c - i);
                   n.dispatchEvent("tick")
               }
               if (e !== true && t === a) {
                   a = u(p)
               }
           };
       w.call(n);
       this.time = this.frame = 0;
       this.tick = function() {
           p(true)
       };
       this.sleep = function() {
           if (a == null) {
               return
           }
           if (!i || !S) {
               clearTimeout(a)
           } else {
               S(a)
           }
           u = s;
           a = null;
           if (n === f) {
               l = false
           }
       };
       this.wake = function() {
           if (a !== null) {
               n.sleep()
           }
           u = o === 0 ? s : !i || !E ? function(e) {
               return setTimeout(e, (h - n.time) * 1e3 + 1 | 0)
           } : E;
           if (n === f) {
               l = true
           }
           p(2)
       };
       this.fps = function(e) {
           if (!arguments.length) {
               return o
           }
           o = e;
           c = 1 / (o || 60);
           h = this.time + c;
           n.wake()
       };
       this.useRAF = function(e) {
           if (!arguments.length) {
               return i
           }
           n.sleep();
           i = e;
           n.fps(o)
       };
       n.fps(e);
       setTimeout(function() {
           if (i && (!a || n.frame < 5)) {
               n.useRAF(false)
           }
       }, 1500)
   });
   a = r.Ticker.prototype = new r.events.EventDispatcher;
   a.constructor = r.Ticker;
   var T = d("core.Animation", function(e, t) {
       this.vars = t || {};
       this._duration = this._totalDuration = e || 0;
       this._delay = Number(this.vars.delay) || 0;
       this._timeScale = 1;
       this._active = this.vars.immediateRender === true;
       this.data = this.vars.data;
       this._reversed = this.vars.reversed === true;
       if (!B) {
           return
       }
       if (!l) {
           f.wake()
       }
       var n = this.vars.useFrames ? H : B;
       n.add(this, n._time);
       if (this.vars.paused) {
           this.paused(true)
       }
   });
   f = T.ticker = new r.Ticker;
   a = T.prototype;
   a._dirty = a._gc = a._initted = a._paused = false;
   a._totalTime = a._time = 0;
   a._rawPrevTime = -1;
   a._next = a._last = a._onUpdate = a._timeline = a.timeline = null;
   a._paused = false;
   a.play = function(e, t) {
       if (arguments.length) {
           this.seek(e, t)
       }
       return this.reversed(false).paused(false)
   };
   a.pause = function(e, t) {
       if (arguments.length) {
           this.seek(e, t)
       }
       return this.paused(true)
   };
   a.resume = function(e, t) {
       if (arguments.length) {
           this.seek(e, t)
       }
       return this.paused(false)
   };
   a.seek = function(e, t) {
       return this.totalTime(Number(e), t !== false)
   };
   a.restart = function(e, t) {
       return this.reversed(false).paused(false).totalTime(e ? -this._delay : 0, t !== false, true)
   };
   a.reverse = function(e, t) {
       if (arguments.length) {
           this.seek(e || this.totalDuration(), t)
       }
       return this.reversed(true).paused(false)
   };
   a.render = function() {};
   a.invalidate = function() {
       return this
   };
   a._enabled = function(e, t) {
       if (!l) {
           f.wake()
       }
       this._gc = !e;
       this._active = e && !this._paused && this._totalTime > 0 && this._totalTime < this._totalDuration;
       if (t !== true) {
           if (e && !this.timeline) {
               this._timeline.add(this, this._startTime - this._delay)
           } else if (!e && this.timeline) {
               this._timeline._remove(this, true)
           }
       }
       return false
   };
   a._kill = function(e, t) {
       return this._enabled(false, false)
   };
   a.kill = function(e, t) {
       this._kill(e, t);
       return this
   };
   a._uncache = function(e) {
       var t = e ? this : this.timeline;
       while (t) {
           t._dirty = true;
           t = t.timeline
       }
       return this
   };
   a.eventCallback = function(e, t, n, r) {
       if (e == null) {
           return null
       } else if (e.substr(0, 2) === "on") {
           var i = this.vars,
               s;
           if (arguments.length === 1) {
               return i[e]
           }
           if (t == null) {
               delete i[e]
           } else {
               i[e] = t;
               i[e + "Params"] = n;
               i[e + "Scope"] = r;
               if (n) {
                   s = n.length;
                   while (--s > -1) {
                       if (n[s] === "{self}") {
                           n = i[e + "Params"] = n.concat();
                           n[s] = this
                       }
                   }
               }
           }
           if (e === "onUpdate") {
               this._onUpdate = t
           }
       }
       return this
   };
   a.delay = function(e) {
       if (!arguments.length) {
           return this._delay
       }
       if (this._timeline.smoothChildTiming) {
           this.startTime(this._startTime + e - this._delay)
       }
       this._delay = e;
       return this
   };
   a.duration = function(e) {
       if (!arguments.length) {
           this._dirty = false;
           return this._duration
       }
       this._duration = this._totalDuration = e;
       this._uncache(true);
       if (this._timeline.smoothChildTiming)
           if (this._time > 0)
               if (this._time < this._duration)
                   if (e !== 0) {
                       this.totalTime(this._totalTime * (e / this._duration), true)
                   } return this
   };
   a.totalDuration = function(e) {
       this._dirty = false;
       return !arguments.length ? this._totalDuration : this.duration(e)
   };
   a.time = function(e, t) {
       if (!arguments.length) {
           return this._time
       }
       if (this._dirty) {
           this.totalDuration()
       }
       return this.totalTime(e > this._duration ? this._duration : e, t)
   };
   a.totalTime = function(e, t, n) {
       if (!l) {
           f.wake()
       }
       if (!arguments.length) {
           return this._totalTime
       }
       if (this._timeline) {
           if (e < 0 && !n) {
               e += this.totalDuration()
           }
           if (this._timeline.smoothChildTiming) {
               if (this._dirty) {
                   this.totalDuration()
               }
               var r = this._totalDuration,
                   i = this._timeline;
               if (e > r && !n) {
                   e = r
               }
               this._startTime = (this._paused ? this._pauseTime : i._time) - (!this._reversed ? e : r - e) / this._timeScale;
               if (!i._dirty) {
                   this._uncache(false)
               }
               if (!i._active) {
                   while (i._timeline) {
                       i.totalTime(i._totalTime, true);
                       i = i._timeline
                   }
               }
           }
           if (this._gc) {
               this._enabled(true, false)
           }
           if (this._totalTime !== e) {
               this.render(e, t, false)
           }
       }
       return this
   };
   a.startTime = function(e) {
       if (!arguments.length) {
           return this._startTime
       }
       if (e !== this._startTime) {
           this._startTime = e;
           if (this.timeline)
               if (this.timeline._sortChildren) {
                   this.timeline.add(this, e - this._delay)
               }
       }
       return this
   };
   a.timeScale = function(e) {
       if (!arguments.length) {
           return this._timeScale
       }
       e = e || 1e-6;
       if (this._timeline && this._timeline.smoothChildTiming) {
           var t = this._pauseTime,
               n = t || t === 0 ? t : this._timeline.totalTime();
           this._startTime = n - (n - this._startTime) * this._timeScale / e
       }
       this._timeScale = e;
       return this._uncache(false)
   };
   a.reversed = function(e) {
       if (!arguments.length) {
           return this._reversed
       }
       if (e != this._reversed) {
           this._reversed = e;
           this.totalTime(this._totalTime, true)
       }
       return this
   };
   a.paused = function(e) {
       if (!arguments.length) {
           return this._paused
       }
       if (e != this._paused)
           if (this._timeline) {
               if (!l && !e) {
                   f.wake()
               }
               var t = this._timeline.rawTime(),
                   n = t - this._pauseTime;
               if (!e && this._timeline.smoothChildTiming) {
                   this._startTime += n;
                   this._uncache(false)
               }
               this._pauseTime = e ? t : null;
               this._paused = e;
               this._active = !e && this._totalTime > 0 && this._totalTime < this._totalDuration;
               if (!e && n !== 0 && this._duration !== 0) {
                   this.render(this._totalTime, true, true)
               }
           } if (this._gc && !e) {
           this._enabled(true, false)
       }
       return this
   };
   var N = d("core.SimpleTimeline", function(e) {
       T.call(this, 0, e);
       this.autoRemoveChildren = this.smoothChildTiming = true
   });
   a = N.prototype = new T;
   a.constructor = N;
   a.kill()._gc = false;
   a._first = a._last = null;
   a._sortChildren = false;
   a.add = a.insert = function(e, t, n, r) {
       var i, s;
       e._startTime = Number(t || 0) + e._delay;
       if (e._paused)
           if (this !== e._timeline) {
               e._pauseTime = e._startTime + (this.rawTime() - e._startTime) / e._timeScale
           } if (e.timeline) {
           e.timeline._remove(e, true)
       }
       e.timeline = e._timeline = this;
       if (e._gc) {
           e._enabled(true, true)
       }
       i = this._last;
       if (this._sortChildren) {
           s = e._startTime;
           while (i && i._startTime > s) {
               i = i._prev
           }
       }
       if (i) {
           e._next = i._next;
           i._next = e
       } else {
           e._next = this._first;
           this._first = e
       }
       if (e._next) {
           e._next._prev = e
       } else {
           this._last = e
       }
       e._prev = i;
       if (this._timeline) {
           this._uncache(true)
       }
       return this
   };
   a._remove = function(e, t) {
       if (e.timeline === this) {
           if (!t) {
               e._enabled(false, true)
           }
           e.timeline = null;
           if (e._prev) {
               e._prev._next = e._next
           } else if (this._first === e) {
               this._first = e._next
           }
           if (e._next) {
               e._next._prev = e._prev
           } else if (this._last === e) {
               this._last = e._prev
           }
           if (this._timeline) {
               this._uncache(true)
           }
       }
       return this
   };
   a.render = function(e, t, n) {
       var r = this._first,
           i;
       this._totalTime = this._time = this._rawPrevTime = e;
       while (r) {
           i = r._next;
           if (r._active || e >= r._startTime && !r._paused) {
               if (!r._reversed) {
                   r.render((e - r._startTime) * r._timeScale, t, n)
               } else {
                   r.render((!r._dirty ? r._totalDuration : r.totalDuration()) - (e - r._startTime) * r._timeScale, t, n)
               }
           }
           r = i
       }
   };
   a.rawTime = function() {
       if (!l) {
           f.wake()
       }
       return this._totalTime
   };
   var C = d("TweenLite", function(e, t, n) {
           T.call(this, t, n);
           if (e == null) {
               throw "Cannot tween a null target."
           }
           this.target = e = typeof e !== "string" ? e : C.selector(e) || e;
           var r = e.jquery || e.length && e[0] && e[0].nodeType && e[0].style,
               s = this.vars.overwrite,
               o, u, a;
           this._overwrite = s = s == null ? P[C.defaultOverwrite] : typeof s === "number" ? s >> 0 : P[s];
           if ((r || e instanceof Array) && typeof e[0] !== "number") {
               this._targets = a = i.call(e, 0);
               this._propLookup = [];
               this._siblings = [];
               for (o = 0; o < a.length; o++) {
                   u = a[o];
                   if (!u) {
                       a.splice(o--, 1);
                       continue
                   } else if (typeof u === "string") {
                       u = a[o--] = C.selector(u);
                       if (typeof u === "string") {
                           a.splice(o + 1, 1)
                       }
                       continue
                   } else if (u.length && u[0] && u[0].nodeType && u[0].style) {
                       a.splice(o--, 1);
                       this._targets = a = a.concat(i.call(u, 0));
                       continue
                   }
                   this._siblings[o] = j(u, this, false);
                   if (s === 1)
                       if (this._siblings[o].length > 1) {
                           F(u, this, null, 1, this._siblings[o])
                       }
               }
           } else {
               this._propLookup = {};
               this._siblings = j(e, this, false);
               if (s === 1)
                   if (this._siblings.length > 1) {
                       F(e, this, null, 1, this._siblings)
                   }
           }
           if (this.vars.immediateRender || t === 0 && this._delay === 0 && this.vars.immediateRender !== false) {
               this.render(-this._delay, false, true)
           }
       }, true),
       k = function(e) {
           return e.length && e[0] && e[0].nodeType && e[0].style
       },
       L = function(e, t) {
           var n = {},
               r;
           for (r in e) {
               if (!D[r] && (!(r in t) || r === "x" || r === "y" || r === "width" || r === "height" || r === "className") && (!O[r] || O[r] && O[r]._autoCSS)) {
                   n[r] = e[r];
                   delete e[r]
               }
           }
           e.css = n
       };
   a = C.prototype = new T;
   a.constructor = C;
   a.kill()._gc = false;
   a.ratio = 0;
   a._firstPT = a._targets = a._overwrittenProps = a._startAt = null;
   a._notifyPluginsOfEnabled = false;
   C.version = "1.9.7";
   C.defaultEase = a._ease = new g(null, null, 1, 1);
   C.defaultOverwrite = "auto";
   C.ticker = f;
   C.autoSleep = true;
   C.selector = e.$ || e.jQuery || function(t) {
       if (e.$) {
           C.selector = e.$;
           return e.$(t)
       }
       return e.document ? e.document.getElementById(t.charAt(0) === "#" ? t.substr(1) : t) : t
   };
   var A = C._internals = {},
       O = C._plugins = {},
       M = C._tweenLookup = {},
       _ = 0,
       D = A.reservedProps = {
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
           autoCSS: 1
       },
       P = {
           none: 0,
           all: 1,
           auto: 2,
           concurrent: 3,
           allOnStart: 4,
           preexisting: 5,
           "true": 1,
           "false": 0
       },
       H = T._rootFramesTimeline = new N,
       B = T._rootTimeline = new N;
   B._startTime = f.time;
   H._startTime = f.frame;
   B._active = H._active = true;
   T._updateRoot = function() {
       B.render((f.time - B._startTime) * B._timeScale, false, false);
       H.render((f.frame - H._startTime) * H._timeScale, false, false);
       if (!(f.frame % 120)) {
           var e, t, n;
           for (n in M) {
               t = M[n].tweens;
               e = t.length;
               while (--e > -1) {
                   if (t[e]._gc) {
                       t.splice(e, 1)
                   }
               }
               if (t.length === 0) {
                   delete M[n]
               }
           }
           n = B._first;
           if (!n || n._paused)
               if (C.autoSleep && !H._first && f._listeners.tick.length === 1) {
                   while (n && n._paused) {
                       n = n._next
                   }
                   if (!n) {
                       f.sleep()
                   }
               }
       }
   };
   f.addEventListener("tick", T._updateRoot);
   var j = function(e, t, n) {
           var r = e._gsTweenID,
               i, s;
           if (!M[r || (e._gsTweenID = r = "t" + _++)]) {
               M[r] = {
                   target: e,
                   tweens: []
               }
           }
           if (t) {
               i = M[r].tweens;
               i[s = i.length] = t;
               if (n) {
                   while (--s > -1) {
                       if (i[s] === t) {
                           i.splice(s, 1)
                       }
                   }
               }
           }
           return M[r].tweens
       },
       F = function(e, t, n, r, i) {
           var s, o, u, a;
           if (r === 1 || r >= 4) {
               a = i.length;
               for (s = 0; s < a; s++) {
                   if ((u = i[s]) !== t) {
                       if (!u._gc)
                           if (u._enabled(false, false)) {
                               o = true
                           }
                   } else if (r === 5) {
                       break
                   }
               }
               return o
           }
           var f = t._startTime + 1e-10,
               l = [],
               c = 0,
               h = t._duration === 0,
               p;
           s = i.length;
           while (--s > -1) {
               if ((u = i[s]) === t || u._gc || u._paused) {} else if (u._timeline !== t._timeline) {
                   p = p || I(t, 0, h);
                   if (I(u, p, h) === 0) {
                       l[c++] = u
                   }
               } else if (u._startTime <= f)
                   if (u._startTime + u.totalDuration() / u._timeScale + 1e-10 > f)
                       if (!((h || !u._initted) && f - u._startTime <= 2e-10)) {
                           l[c++] = u
                       }
           }
           s = c;
           while (--s > -1) {
               u = l[s];
               if (r === 2)
                   if (u._kill(n, e)) {
                       o = true
                   } if (r !== 2 || !u._firstPT && u._initted) {
                   if (u._enabled(false, false)) {
                       o = true
                   }
               }
           }
           return o
       },
       I = function(e, t, n) {
           var r = e._timeline,
               i = r._timeScale,
               s = e._startTime,
               o = 1e-10;
           while (r._timeline) {
               s += r._startTime;
               i *= r._timeScale;
               if (r._paused) {
                   return -100
               }
               r = r._timeline
           }
           s /= i;
           return s > t ? s - t : n && s === t || !e._initted && s - t < 2 * o ? o : (s += e.totalDuration() / e._timeScale / i) > t + o ? 0 : s - t - o
       };
   a._init = function() {
       var e = this.vars,
           t = this._overwrittenProps,
           n = this._duration,
           r = e.ease,
           i, s, o, u;
       if (e.startAt) {
           e.startAt.overwrite = 0;
           e.startAt.immediateRender = true;
           this._startAt = C.to(this.target, 0, e.startAt);
           if (e.immediateRender) {
               this._startAt = null;
               if (this._time === 0 && n !== 0) {
                   return
               }
           }
       } else if (e.runBackwards && e.immediateRender && n !== 0) {
           if (this._startAt) {
               this._startAt.render(-1, true);
               this._startAt = null
           } else if (this._time === 0) {
               o = {};
               for (u in e) {
                   if (!D[u] || u === "autoCSS") {
                       o[u] = e[u]
                   }
               }
               o.overwrite = 0;
               this._startAt = C.to(this.target, 0, o);
               return
           }
       }
       if (!r) {
           this._ease = C.defaultEase
       } else if (r instanceof g) {
           this._ease = e.easeParams instanceof Array ? r.config.apply(r, e.easeParams) : r
       } else {
           this._ease = typeof r === "function" ? new g(r, e.easeParams) : y[r] || C.defaultEase
       }
       this._easeType = this._ease._type;
       this._easePower = this._ease._power;
       this._firstPT = null;
       if (this._targets) {
           i = this._targets.length;
           while (--i > -1) {
               if (this._initProps(this._targets[i], this._propLookup[i] = {}, this._siblings[i], t ? t[i] : null)) {
                   s = true
               }
           }
       } else {
           s = this._initProps(this.target, this._propLookup, this._siblings, t)
       }
       if (s) {
           C._onPluginEvent("_onInitAllProps", this)
       }
       if (t)
           if (!this._firstPT)
               if (typeof this.target !== "function") {
                   this._enabled(false, false)
               } if (e.runBackwards) {
           o = this._firstPT;
           while (o) {
               o.s += o.c;
               o.c = -o.c;
               o = o._next
           }
       }
       this._onUpdate = e.onUpdate;
       this._initted = true
   };
   a._initProps = function(e, t, n, r) {
       var i, s, o, u, a, f, l;
       if (e == null) {
           return false
       }
       if (!this.vars.css)
           if (e.style)
               if (e.nodeType)
                   if (O.css)
                       if (this.vars.autoCSS !== false) {
                           L(this.vars, e)
                       } for (i in this.vars) {
           if (D[i]) {
               if (i === "onStartParams" || i === "onUpdateParams" || i === "onCompleteParams" || i === "onReverseCompleteParams" || i === "onRepeatParams")
                   if (a = this.vars[i]) {
                       s = a.length;
                       while (--s > -1) {
                           if (a[s] === "{self}") {
                               a = this.vars[i] = a.concat();
                               a[s] = this
                           }
                       }
                   }
           } else if (O[i] && (u = new O[i])._onInitTween(e, this.vars[i], this)) {
               this._firstPT = f = {
                   _next: this._firstPT,
                   t: u,
                   p: "setRatio",
                   s: 0,
                   c: 1,
                   f: true,
                   n: i,
                   pg: true,
                   pr: u._priority
               };
               s = u._overwriteProps.length;
               while (--s > -1) {
                   t[u._overwriteProps[s]] = this._firstPT
               }
               if (u._priority || u._onInitAllProps) {
                   o = true
               }
               if (u._onDisable || u._onEnable) {
                   this._notifyPluginsOfEnabled = true
               }
           } else {
               this._firstPT = t[i] = f = {
                   _next: this._firstPT,
                   t: e,
                   p: i,
                   f: typeof e[i] === "function",
                   n: i,
                   pg: false,
                   pr: 0
               };
               f.s = !f.f ? parseFloat(e[i]) : e[i.indexOf("set") || typeof e["get" + i.substr(3)] !== "function" ? i : "get" + i.substr(3)]();
               l = this.vars[i];
               f.c = typeof l === "string" && l.charAt(1) === "=" ? parseInt(l.charAt(0) + "1", 10) * Number(l.substr(2)) : Number(l) - f.s || 0
           }
           if (f)
               if (f._next) {
                   f._next._prev = f
               }
       }
       if (r)
           if (this._kill(r, e)) {
               return this._initProps(e, t, n, r)
           } if (this._overwrite > 1)
           if (this._firstPT)
               if (n.length > 1)
                   if (F(e, this, t, this._overwrite, n)) {
                       this._kill(t, e);
                       return this._initProps(e, t, n, r)
                   } return o
   };
   a.render = function(e, t, n) {
       var r = this._time,
           i, s, o;
       if (e >= this._duration) {
           this._totalTime = this._time = this._duration;
           this.ratio = this._ease._calcEnd ? this._ease.getRatio(1) : 1;
           if (!this._reversed) {
               i = true;
               s = "onComplete"
           }
           if (this._duration === 0) {
               if (e === 0 || this._rawPrevTime < 0)
                   if (this._rawPrevTime !== e) {
                       n = true;
                       if (this._rawPrevTime > 0) {
                           s = "onReverseComplete";
                           if (t) {
                               e = -1
                           }
                       }
                   } this._rawPrevTime = e
           }
       } else if (e < 1e-7) {
           this._totalTime = this._time = 0;
           this.ratio = this._ease._calcEnd ? this._ease.getRatio(0) : 0;
           if (r !== 0 || this._duration === 0 && this._rawPrevTime > 0) {
               s = "onReverseComplete";
               i = this._reversed
           }
           if (e < 0) {
               this._active = false;
               if (this._duration === 0) {
                   if (this._rawPrevTime >= 0) {
                       n = true
                   }
                   this._rawPrevTime = e
               }
           } else if (!this._initted) {
               n = true
           }
       } else {
           this._totalTime = this._time = e;
           if (this._easeType) {
               var u = e / this._duration,
                   a = this._easeType,
                   f = this._easePower;
               if (a === 1 || a === 3 && u >= .5) {
                   u = 1 - u
               }
               if (a === 3) {
                   u *= 2
               }
               if (f === 1) {
                   u *= u
               } else if (f === 2) {
                   u *= u * u
               } else if (f === 3) {
                   u *= u * u * u
               } else if (f === 4) {
                   u *= u * u * u * u
               }
               if (a === 1) {
                   this.ratio = 1 - u
               } else if (a === 2) {
                   this.ratio = u
               } else if (e / this._duration < .5) {
                   this.ratio = u / 2
               } else {
                   this.ratio = 1 - u / 2
               }
           } else {
               this.ratio = this._ease.getRatio(e / this._duration)
           }
       }
       if (this._time === r && !n) {
           return
       } else if (!this._initted) {
           this._init();
           if (!this._initted) {
               return
           }
           if (this._time && !i) {
               this.ratio = this._ease.getRatio(this._time / this._duration)
           } else if (i && this._ease._calcEnd) {
               this.ratio = this._ease.getRatio(this._time === 0 ? 0 : 1)
           }
       }
       if (!this._active)
           if (!this._paused) {
               this._active = true
           } if (r === 0) {
           if (this._startAt) {
               if (e >= 0) {
                   this._startAt.render(e, t, n)
               } else if (!s) {
                   s = "_dummyGS"
               }
           }
           if (this.vars.onStart)
               if (this._time !== 0 || this._duration === 0)
                   if (!t) {
                       this.vars.onStart.apply(this.vars.onStartScope || this, this.vars.onStartParams || m)
                   }
       }
       o = this._firstPT;
       while (o) {
           if (o.f) {
               o.t[o.p](o.c * this.ratio + o.s)
           } else {
               o.t[o.p] = o.c * this.ratio + o.s
           }
           o = o._next
       }
       if (this._onUpdate) {
           if (e < 0)
               if (this._startAt) {
                   this._startAt.render(e, t, n)
               } if (!t) {
               this._onUpdate.apply(this.vars.onUpdateScope || this, this.vars.onUpdateParams || m)
           }
       }
       if (s)
           if (!this._gc) {
               if (e < 0 && this._startAt && !this._onUpdate) {
                   this._startAt.render(e, t, n)
               }
               if (i) {
                   if (this._timeline.autoRemoveChildren) {
                       this._enabled(false, false)
                   }
                   this._active = false
               }
               if (!t && this.vars[s]) {
                   this.vars[s].apply(this.vars[s + "Scope"] || this, this.vars[s + "Params"] || m)
               }
           }
   };
   a._kill = function(e, t) {
       if (e === "all") {
           e = null
       }
       if (e == null)
           if (t == null || t === this.target) {
               return this._enabled(false, false)
           } t = typeof t !== "string" ? t || this._targets || this.target : C.selector(t) || t;
       var n, r, i, s, o, u, a, f;
       if ((t instanceof Array || k(t)) && typeof t[0] !== "number") {
           n = t.length;
           while (--n > -1) {
               if (this._kill(e, t[n])) {
                   u = true
               }
           }
       } else {
           if (this._targets) {
               n = this._targets.length;
               while (--n > -1) {
                   if (t === this._targets[n]) {
                       o = this._propLookup[n] || {};
                       this._overwrittenProps = this._overwrittenProps || [];
                       r = this._overwrittenProps[n] = e ? this._overwrittenProps[n] || {} : "all";
                       break
                   }
               }
           } else if (t !== this.target) {
               return false
           } else {
               o = this._propLookup;
               r = this._overwrittenProps = e ? this._overwrittenProps || {} : "all"
           }
           if (o) {
               a = e || o;
               f = e !== r && r !== "all" && e !== o && (e == null || e._tempKill !== true);
               for (i in a) {
                   if (s = o[i]) {
                       if (s.pg && s.t._kill(a)) {
                           u = true
                       }
                       if (!s.pg || s.t._overwriteProps.length === 0) {
                           if (s._prev) {
                               s._prev._next = s._next
                           } else if (s === this._firstPT) {
                               this._firstPT = s._next
                           }
                           if (s._next) {
                               s._next._prev = s._prev
                           }
                           s._next = s._prev = null
                       }
                       delete o[i]
                   }
                   if (f) {
                       r[i] = 1
                   }
               }
               if (!this._firstPT && this._initted) {
                   this._enabled(false, false)
               }
           }
       }
       return u
   };
   a.invalidate = function() {
       if (this._notifyPluginsOfEnabled) {
           C._onPluginEvent("_onDisable", this)
       }
       this._firstPT = null;
       this._overwrittenProps = null;
       this._onUpdate = null;
       this._startAt = null;
       this._initted = this._active = this._notifyPluginsOfEnabled = false;
       this._propLookup = this._targets ? {} : [];
       return this
   };
   a._enabled = function(e, t) {
       if (!l) {
           f.wake()
       }
       if (e && this._gc) {
           var n = this._targets,
               r;
           if (n) {
               r = n.length;
               while (--r > -1) {
                   this._siblings[r] = j(n[r], this, true)
               }
           } else {
               this._siblings = j(this.target, this, true)
           }
       }
       T.prototype._enabled.call(this, e, t);
       if (this._notifyPluginsOfEnabled)
           if (this._firstPT) {
               return C._onPluginEvent(e ? "_onEnable" : "_onDisable", this)
           } return false
   };
   C.to = function(e, t, n) {
       return new C(e, t, n)
   };
   C.from = function(e, t, n) {
       n.runBackwards = true;
       n.immediateRender = n.immediateRender != false;
       return new C(e, t, n)
   };
   C.fromTo = function(e, t, n, r) {
       r.startAt = n;
       r.immediateRender = r.immediateRender != false && n.immediateRender != false;
       return new C(e, t, r)
   };
   C.delayedCall = function(e, t, n, r, i) {
       return new C(t, 0, {
           delay: e,
           onComplete: t,
           onCompleteParams: n,
           onCompleteScope: r,
           onReverseComplete: t,
           onReverseCompleteParams: n,
           onReverseCompleteScope: r,
           immediateRender: false,
           useFrames: i,
           overwrite: 0
       })
   };
   C.set = function(e, t) {
       return new C(e, 0, t)
   };
   C.killTweensOf = C.killDelayedCallsTo = function(e, t) {
       var n = C.getTweensOf(e),
           r = n.length;
       while (--r > -1) {
           n[r]._kill(t, e)
       }
   };
   C.getTweensOf = function(e) {
       if (e == null) {
           return []
       }
       e = typeof e !== "string" ? e : C.selector(e) || e;
       var t, n, r, i;
       if ((e instanceof Array || k(e)) && typeof e[0] !== "number") {
           t = e.length;
           n = [];
           while (--t > -1) {
               n = n.concat(C.getTweensOf(e[t]))
           }
           t = n.length;
           while (--t > -1) {
               i = n[t];
               r = t;
               while (--r > -1) {
                   if (i === n[r]) {
                       n.splice(t, 1)
                   }
               }
           }
       } else {
           n = j(e).concat();
           t = n.length;
           while (--t > -1) {
               if (n[t]._gc) {
                   n.splice(t, 1)
               }
           }
       }
       return n
   };
   var q = d("plugins.TweenPlugin", function(e, t) {
       this._overwriteProps = (e || "").split(",");
       this._propName = this._overwriteProps[0];
       this._priority = t || 0;
       this._super = q.prototype
   }, true);
   a = q.prototype;
   q.version = "1.9.1";
   q.API = 2;
   a._firstPT = null;
   a._addTween = function(e, t, n, r, i, s) {
       var o, u;
       if (r != null && (o = typeof r === "number" || r.charAt(1) !== "=" ? Number(r) - n : parseInt(r.charAt(0) + "1", 10) * Number(r.substr(2)))) {
           this._firstPT = u = {
               _next: this._firstPT,
               t: e,
               p: t,
               s: n,
               c: o,
               f: typeof e[t] === "function",
               n: i || t,
               r: s
           };
           if (u._next) {
               u._next._prev = u
           }
       }
   };
   a.setRatio = function(e) {
       var t = this._firstPT,
           n = 1e-6,
           r;
       while (t) {
           r = t.c * e + t.s;
           if (t.r) {
               r = r + (r > 0 ? .5 : -.5) >> 0
           } else if (r < n)
               if (r > -n) {
                   r = 0
               } if (t.f) {
               t.t[t.p](r)
           } else {
               t.t[t.p] = r
           }
           t = t._next
       }
   };
   a._kill = function(e) {
       var t = this._overwriteProps,
           n = this._firstPT,
           r;
       if (e[this._propName] != null) {
           this._overwriteProps = []
       } else {
           r = t.length;
           while (--r > -1) {
               if (e[t[r]] != null) {
                   t.splice(r, 1)
               }
           }
       }
       while (n) {
           if (e[n.n] != null) {
               if (n._next) {
                   n._next._prev = n._prev
               }
               if (n._prev) {
                   n._prev._next = n._next;
                   n._prev = null
               } else if (this._firstPT === n) {
                   this._firstPT = n._next
               }
           }
           n = n._next
       }
       return false
   };
   a._roundProps = function(e, t) {
       var n = this._firstPT;
       while (n) {
           if (e[this._propName] || n.n != null && e[n.n.split(this._propName + "_").join("")]) {
               n.r = t
           }
           n = n._next
       }
   };
   C._onPluginEvent = function(e, t) {
       var n = t._firstPT,
           r, i, s, o, u;
       if (e === "_onInitAllProps") {
           while (n) {
               u = n._next;
               i = s;
               while (i && i.pr > n.pr) {
                   i = i._next
               }
               if (n._prev = i ? i._prev : o) {
                   n._prev._next = n
               } else {
                   s = n
               }
               if (n._next = i) {
                   i._prev = n
               } else {
                   o = n
               }
               n = u
           }
           n = t._firstPT = s
       }
       while (n) {
           if (n.pg)
               if (typeof n.t[e] === "function")
                   if (n.t[e]()) {
                       r = true
                   } n = n._next
       }
       return r
   };
   q.activate = function(e) {
       var t = e.length;
       while (--t > -1) {
           if (e[t].API === q.API) {
               O[(new e[t])._propName] = e[t]
           }
       }
       return true
   };
   p.plugin = function(e) {
       if (!e || !e.propName || !e.init || !e.API) {
           throw "illegal plugin definition."
       }
       var t = e.propName,
           n = e.priority || 0,
           r = e.overwriteProps,
           i = {
               init: "_onInitTween",
               set: "setRatio",
               kill: "_kill",
               round: "_roundProps",
               initAll: "_onInitAllProps"
           },
           s = d("plugins." + t.charAt(0).toUpperCase() + t.substr(1) + "Plugin", function() {
               q.call(this, t, n);
               this._overwriteProps = r || []
           }, e.global === true),
           o = s.prototype = new q(t),
           u;
       o.constructor = s;
       s.API = e.API;
       for (u in i) {
           if (typeof e[u] === "function") {
               o[i[u]] = e[u]
           }
       }
       s.version = e.version;
       q.activate([s]);
       return s
   };
   o = e._gsQueue;
   if (o) {
       for (u = 0; u < o.length; u++) {
           o[u]()
       }
       for (a in c) {
           if (!c[a].func) {
               e.console.log("GSAP encountered missing dependency: com.greensock." + a)
           }
       }
   }
   l = false
})(window);

gsap.timeline({repeat: 1, yoyo: true});
