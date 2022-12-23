function A(t, e, o) {
	var s = e || 0,
		i = 0;
	"string" == typeof t ? (i = o || t.length, this.a = function(e) {
		return 255 & t.charCodeAt(e + s)
	}) : "unknown" == typeof t && (i = o || IEBinary_getLength(t), this.a = function(e) {
		return IEBinary_getByteAt(t, e + s)
	}), this.l = function(e, t) {
		for (var o = Array(t), s = 0; s < t; s++) o[s] = this.a(e + s);
		return o
	}, this.h = function() {
		return i
	}, this.d = function(e, t) {
		return 0 != (this.a(e) & 1 << t)
	}, this.w = function(e) {
		return (e = (this.a(e + 1) << 8) + this.a(e)) < 0 && (e += 65536), e
	}, this.i = function(e) {
		var t = this.a(e);
		return (t = (((t << 8) + this.a(e + 1) << 8) + this.a(e + 2) << 8) + (e = this.a(e + 3))) < 0 && (t += 4294967296), t
	}, this.o = function(e) {
		var t = this.a(e);
		return (t = ((t << 8) + this.a(e + 1) << 8) + (e = this.a(e + 2))) < 0 && (t += 16777216), t
	}, this.c = function(e, t) {
		for (var o = [], s = e, i = 0; s < e + t; s++, i++) o[i] = String.fromCharCode(this.a(s));
		return o.join("")
	}, this.e = function(e, t, o) {
		switch (e = this.l(e, t), o.toLowerCase()) {
			case "utf-16":
			case "utf-16le":
			case "utf-16be":
				t = o;
				var s, i = 0,
					n = 1;
				o = 0, s = Math.min(s || e.length, e.length),
					254 == e[0] && 255 == e[1] ? (t = !0, i = 2) : 255 == e[0] && 254 == e[1] && (t = !1, i = 2),
					t && (n = 0, o = 1), t = [];
				for (var l = 0; i < s; l++) {
					var r = e[i + n],
						a = (r << 8) + e[i + o];
					i = i + 2;
					if (0 == a) break;
					r < 216 || 224 <= r ? t[l] = String.fromCharCode(a) : (r = (e[i + n] << 8) + e[i + o], i += 2, t[l] = String.fromCharCode(a, r))
				}(e = new String(t.join(""))).g = i;
				break;
			case "utf-8":
				for (s = 0, i = Math.min(i || e.length, e.length), 239 == e[0] && 187 == e[1] && 191 == e[2] && (s = 3), n = [], o = 0; s < i && 0 != (t = e[s++]); o++) t < 128 ? n[o] = String.fromCharCode(t) : 194 <= t && t < 224 ? (l = e[s++], n[o] = String.fromCharCode(((31 & t) << 6) + (63 & l))) : 224 <= t && t < 240 ? (l = e[s++], a = e[s++], n[o] = String.fromCharCode(((255 & t) << 12) + ((63 & l) << 6) + (63 & a))) : 240 <= t && t < 245 && (t = ((7 & t) << 18) + ((63 & (l = e[s++])) << 12) + ((63 & (a = e[s++])) << 6) + (63 & (r = e[s++])) - 65536, n[o] = String.fromCharCode(55296 + (t >> 10), 56320 + (1023 & t)));
				(e = new String(n.join(""))).g = s;
				break;
			default:
				for (i = [], n = n || e.length, s = 0; s < n && 0 != (o = e[s++]);) i[s - 1] = String.fromCharCode(o);
				(e = new String(i.join(""))).g = s
		}
		return e
	}, this.f = function(e, t) {
		t()
	}
}

function B(t, o, p) {
	function m() {
		var e = null;
		return window.XMLHttpRequest ? e = new XMLHttpRequest : window.ActiveXObject && (e = new ActiveXObject("Microsoft.XMLHTTP")), e
	}

	function s(u, c) {
		var h, o;

		function s(e) {
			var t = ~~(e[0] / h) - o;
			return t < 0 && (t = 0), (e = 1 + ~~(e[1] / h) + o) >= blockTotal && (e = blockTotal - 1), [t, e]
		}

		function i(o, s) {
			for (; f[o[0]];)
				if (o[0]++, o[0] > o[1]) return void(s && s());
			for (; f[o[1]];)
				if (o[1]--, o[0] > o[1]) return void(s && s());
			var e, t, i, n, l, r, a, d = [o[0] * h, (o[1] + 1) * h - 1];
			e = u, t = function(e) {
				parseInt(e.getResponseHeader("Content-Length"), 10) == c && (o[0] = 0, o[1] = blockTotal - 1, d[0] = 0, d[1] = c - 1), e = {
					data: e.N || e.responseText,
					offset: d[0]
				};
				for (var t = o[0]; t <= o[1]; t++) f[t] = e;
				s && s()
			}, i = p, n = d, l = _, r = !!s, (a = m()) ? (void 0 === r && (r = !0), t && (void 0 !== a.onload ? a.onload = function() {
				"200" == a.status || "206" == a.status ? (a.fileSize = l || a.getResponseHeader("Content-Length"), t(a)) : i && i(), a = null
			} : a.onreadystatechange = function() {
				4 == a.readyState && ("200" == a.status || "206" == a.status ? (a.fileSize = l || a.getResponseHeader("Content-Length"), t(a)) : i && i(), a = null)
			}), a.open("GET", e, r), a.overrideMimeType && a.overrideMimeType("text/plain; charset=x-user-defined"), n && a.setRequestHeader("Range", "bytes=" + n[0] + "-" + n[1]), a.setRequestHeader("If-Modified-Since", "Sat, 1 Jan 1970 00:00:00 GMT"), a.send(null)) : i && i()
		}
		var _, e = new A("", 0, c),
			f = [];
		for (var t in o = void 0 === o ? 0 : o, blockTotal = 1 + ~~((c - 1) / (h = h || 2048)), e) e.hasOwnProperty(t) && "function" == typeof e[t] && (this[t] = e[t]);
		this.a = function(e) {
			var t;
			return i(s([e, e])), "string" == typeof(t = f[~~(e / h)]).data ? 255 & t.data.charCodeAt(e - t.offset) : "unknown" == typeof t.data ? IEBinary_getByteAt(t.data, e - t.offset) : void 0
		}, this.f = function(e, t) {
			i(s(e), t)
		}
	}
	var e, i, n;
	e = t, i = function(e) {
		e = parseInt(e.getResponseHeader("Content-Length"), 10) || -1, o(new s(t, e))
	}, (n = m()) && (i && (void 0 !== n.onload ? n.onload = function() {
		"200" == n.status && i(this), n = null
	} : n.onreadystatechange = function() {
		4 == n.readyState && ("200" == n.status && i(this), n = null)
	}), n.open("HEAD", e, !0), n.send(null))
}
document.write("<script type='text/vbscript'>\r\nFunction IEBinary_getByteAt(strBinary, iOffset)\r\n\tIEBinary_getByteAt = AscB(MidB(strBinary,iOffset+1,1))\r\nEnd Function\r\nFunction IEBinary_getLength(strBinary)\r\n\tIEBinary_getLength = LenB(strBinary)\r\nEnd Function\r\n<\/script>\r\n"),
	function(e) {
		e.FileAPIReader = function(s, i) {
			return function(e, t) {
				var o = i || new FileReader;
				o.onload = function(e) {
					t(new A(e.target.result))
				}, o.readAsBinaryString(s)
			}
		}
	}(this),
	function(e) {
		var t = e.p = {},
			a = {},
			o = [0, 7];
		t.t = function(e) {
			delete a[e]
		}, t.s = function() {
			a = {}
		}, t.B = function(n, l, r) {
			((r = r || {}).dataReader || B)(n, function(i) {
				i.f(o, function() {
					var s = "ftypM4A" == i.c(4, 7) ? ID4 : "ID3" == i.c(0, 3) ? ID3v2 : ID3v1;
					s.m(i, function() {
						var e, t = r.tags,
							o = s.n(i, t);
						t = a[n] || {};
						for (e in o) o.hasOwnProperty(e) && (t[e] = o[e]);
						a[n] = t, l && l()
					})
				})
			})
		}, t.v = function(e) {
			if (!a[e]) return null;
			var t, o = {};
			for (t in a[e]) a[e].hasOwnProperty(t) && (o[t] = a[e]
				[t]);
			return o
		}, t.A = function(e, t) {
			return a[e] ? a[e][t] : null
		}, e.ID3 = e.p, t.loadTags = t.B, t.getAllTags = t.v, t.getTag = t.A, t.clearTags = t.t, t.clearAll = t.s
	}(this),
	function(e) {
		var t = e.q = {},
			a = "Blues;Classic Rock;Country;Dance;Disco;Funk;Grunge;Hip-Hop;Jazz;Metal;New Age;Oldies;Other;Pop;R&B;Rap;Reggae;Rock;Techno;Industrial;Alternative;Ska;Death Metal;Pranks;Soundtrack;Euro-Techno;Ambient;Trip-Hop;Vocal;Jazz+Funk;Fusion;Trance;Classical;Instrumental;Acid;House;Game;Sound Clip;Gospel;Noise;AlternRock;Bass;Soul;Punk;Space;Meditative;Instrumental Pop;Instrumental Rock;Ethnic;Gothic;Darkwave;Techno-Industrial;Electronic;Pop-Folk;Eurodance;Dream;Southern Rock;Comedy;Cult;Gangsta;Top 40;Christian Rap;Pop/Funk;Jungle;Native American;Cabaret;New Wave;Psychadelic;Rave;Showtunes;Trailer;Lo-Fi;Tribal;Acid Punk;Acid Jazz;Polka;Retro;Musical;Rock & Roll;Hard Rock;Folk;Folk-Rock;National Folk;Swing;Fast Fusion;Bebob;Latin;Revival;Celtic;Bluegrass;Avantgarde;Gothic Rock;Progressive Rock;Psychedelic Rock;Symphonic Rock;Slow Rock;Big Band;Chorus;Easy Listening;Acoustic;Humour;Speech;Chanson;Opera;Chamber Music;Sonata;Symphony;Booty Bass;Primus;Porn Groove;Satire;Slow Jam;Club;Tango;Samba;Folklore;Ballad;Power Ballad;Rhythmic Soul;Freestyle;Duet;Punk Rock;Drum Solo;Acapella;Euro-House;Dance Hall".split(";");
		t.m = function(e, t) {
			var o = e.h();
			e.f([o - 128 - 1, o], t)
		}, t.n = function(e) {
			var t = e.h() - 128;
			if ("TAG" != e.c(t, 3)) return {};
			var o = e.c(3 + t, 30).replace(/\0/g, ""),
				s = e.c(33 + t, 30).replace(/\0/g, ""),
				i = e.c(63 + t, 30).replace(/\0/g, ""),
				n = e.c(93 + t, 4).replace(/\0/g, "");
			if (0 == e.a(97 + t + 28)) var l = e.c(97 + t, 28).replace(/\0/g, ""),
				r = e.a(97 + t + 29);
			else l = "", r = 0;
			return {
				version: "1.1",
				title: o,
				artist: s,
				album: i,
				year: n,
				comment: l,
				track: r,
				genre: (e = e.a(97 + t + 30)) < 255 ? a[e] : ""
			}
		}, e.ID3v1 = e.q
	}(this),
	function(e) {
		function g(e, t) {
			var o = t.a(e),
				s = t.a(e + 1),
				i = t.a(e + 2);
			return 127 & t.a(e + 3) | (127 & i) << 7 | (127 & s) << 14 | (127 & o) << 21
		}
		var S = e.D = {};
		S.b = {}, S.frames = {

		};
		var y = {
				title: ["TIT2", "TT2"],
				artist: ["TPE1", "TP1"],
				album: ["TALB", "TAL"],
				year: ["TYER", "TYE"],
				comment: ["COMM", "COM"],
				track: ["TRCK", "TRK"],
				genre: ["TCON", "TCO"],
				picture: ["APIC", "PIC"],
				lyrics: ["USLT", "ULT"]
			},
			v = ["title", "artist", "album", "track"];
		S.m = function(e, t) {
			e.f([0, g(6, e)], t)
		}, S.n = function(e, t) {
			var o = 0;
			if (4 < (d = e.a(o + 3))) return {
				version: ">2.4"
			};
			var s = e.a(o + 4),
				i = e.d(o + 5, 7),
				n = e.d(o + 5, 6),
				l = e.d(o + 5, 5),
				r = g(o + 6, e);
			o += 10;
			if (n) o = o + ((c = e.i(o)) + 4);
			var a, d = {
				version: "2." + d + "." + s,
				major: d,
				revision: s,
				flags: {
					unsynchronisation: i,
					extended_header: n,
					experimental_indicator: l
				},
				size: r
			};
			if (i) a = {};
			else {
				r = r - 10, i = e, s = t, n = {}, l = d.major;
				for (var u, c = [], h = 0; u = (s || v)[h]; h++) c = c.concat(y[u] || [u]);
				for (s = c; o < r;) {
					h = i, u = o;
					var _ = c = null;
					switch (l) {
						case 2:
							a = h.c(u, 3);
							var f = h.o(u + 3),
								p = 6;
							break;
						case 3:
							a = h.c(u, 4), f = h.i(u + 4), p = 10;
							break;
						case 4:
							a = h.c(u, 4), f = g(u + 4, h), p = 10
					}
					if ("" == a) break;
					o += p + f, s.indexOf(a) < 0 || (2 < l && (_ = {
						message: {
							P: h.d(u + 8, 6),
							I: h.d(u + 8, 5),
							M: h.d(u + 8, 4)
						},
						k: {
							K: h.d(u + 8 + 1, 7),
							F: h.d(u + 8 + 1, 3),
							H: h.d(u + 8 + 1, 2),
							C: h.d(u + 8 + 1, 1),
							u: h.d(u + 8 + 1, 0)
						}
					}), u += p, _ && _.k.u && (g(u, h), u += 4, f -= 4), _ && _.k.C || (a in S.b ? c = S.b[a] : "T" == a[0] && (c = S.b["T*"]), c = c ? c(u, f, h, _) : void 0, c = {
						id: a,
						size: f,
						description: a in S.frames ? S.frames[a] : "Unknown",
						data: c
					}, a in n ? (n[a].id && (n[a] = [
						n[a]
					]), n[a].push(c)) : n[a] = c))
				}
				a = n
			}
			for (var m in y)
				if (y.hasOwnProperty(m)) {
					e: {
						for ("string" == typeof(f = y[m]) && (f = [f]), o = void(p = 0); o = f[p]; p++)
							if (o in a) {
								e = a[o].data;
								break e
							} e = void 0
					}
					e && (d[m] = e)
				} for (var b in a) a.hasOwnProperty(b) && (d[b] = a[b]);
			return d
		}, e.ID3v2 = S
	}(this),
	function() {
		function r(e) {
			var t;
			switch (e) {
				case 0:
					t = "iso-8859-1";
					break;
				case 1:
					t = "utf-16";
					break;
				case 2:
					t = "utf-16be";
					break;
				case 3:
					t = "utf-8"
			}
			return t
		}
		var a = "32x32 pixels 'file icon' (PNG only);Other file icon;Cover (front);Cover (back);Leaflet page;Media (e.g. lable side of CD);Lead artist/lead performer/soloist;Artist/performer;Conductor;Band/Orchestra;Composer;Lyricist/text writer;Recording Location;During recording;During performance;Movie/video screen capture;A bright coloured fish;Illustration;Band/artist logotype;Publisher/Studio logotype".split(";");
		ID3v2.b.APIC = function(e, t, o, s, i) {
			i = i || "3", s = e;
			var n = r(o.a(e));
			switch (i) {
				case "2":
					var l = o.c(e + 1, 3);
					e += 4;
					break;
				case "3":
				case "4":
					e += 1 + (l = o.e(e + 1, t - (e - s), n)).g
			}
			return i = o.a(e, 1), i = a[i], e += 1 + (n = o.e(e + 1, t - (e - s), n)).g, {
				format: l.toString(),
				type: i,
				description: n.toString(),
				data: o.l(e, s + t - e)
			}
		}, ID3v2.b.COMM = function(e, t, o) {
			var s = e,
				i = r(o.a(e)),
				n = o.c(e + 1, 3),
				l = o.e(e + 4, t - 4, i);
			return e += 4 + l.g, e = o.e(e, s + t - e, i), {
				language: n,
				O: l.toString(),
				text: e.toString()
			}
		}, ID3v2.b.COM = ID3v2.b.COMM, ID3v2.b.PIC = function(e, t, o, s) {
			return ID3v2.b.APIC(e, t, o, s, "2")
		}, ID3v2.b.PCNT = function(e, t, o) {
			return o.J(e)
		}, ID3v2.b.CNT = ID3v2.b.PCNT, ID3v2.b["T*"] = function(e, t, o) {
			var s = r(o.a(e));
			return o.e(e + 1, t - 1, s).toString()
		}, ID3v2.b.TCON = function(e, t, o) {
			return ID3v2.b["T*"].apply(this, arguments).replace(/^\(\d+\)/, "")
		}, ID3v2.b.TCO = ID3v2.b.TCON, ID3v2.b.USLT = function(e, t, o) {
			var s = e,
				i = r(o.a(e)),
				n = o.c(e + 1, 3),
				l = o.e(e + 4, t - 4, i);
			return e += 4 + l.g, e = o.e(e, s + t - e, i), {
				language: n,
				G: l.toString(),
				L: e.toString()
			}
		}, ID3v2.b.ULT = ID3v2.b.USLT
	}(),
	function(e) {
		var _ = e.r = {};
		_.types = {
			0: "uint8",
			1: "text",
			13: "jpeg",
			14: "png",
			21: "uint8"
		}, _.j = {
			"Â©alb": ["album"],
			"Â©art": ["artist"],
			"Â©ART": ["artist"],
			aART: ["artist"],
			"Â©day": ["year"],
			"Â©nam": ["title"],
			"Â©gen": ["genre"],
			trkn: ["track"],
			"Â©wrt": ["composer"],
			"Â©too": ["encoder"],
			cprt: ["copyright"],
			covr: ["picture"],
			"Â©grp": ["grouping"],
			keyw: ["keyword"],
			"Â©lyr": ["lyrics"],
			"Â©cmt": ["comment"],
			tmpo: ["tempo"],
			cpil: ["compilation"],
			disk: ["disc"]
		}, _.m = function(e, t) {
			e.f([0, 7], function() {
				! function e(t, o, s, i) {
					var n = t.i(o);
					if (0 == n) i();
					else {
						var l = t.c(o + 4, 4); - 1 < ["moov", "udta", "meta", "ilst"].indexOf(l) ? ("meta" == l && (o += 4), t.f([o + 8, o + 8 + 8], function() {
							e(t, o + 8, n - 8, i)
						})) : t.f([o + (l in _.j ? 0 : n), o + n + 8], function() {
							e(t, o + n, s, i)
						})
					}
				}(e, 0, e.h(), t)
			})
		}, _.n = function(e) {
			var t = {};
			return function e(t, o, s, i, n) {
				n = void 0 === n ? "" : n + "  ";
				for (var l = s; l < s + i;) {
					var r = o.i(l);
					if (0 == r) break;
					var a = o.c(l + 4, 4);
					if (-1 < ["moov", "udta", "meta", "ilst"].indexOf(a)) {
						"meta" == a && (l += 4), e(t, o, l + 8, r - 8, n);
						break
					}
					if (_.j[a]) {
						var d = o.o(l + 16 + 1),
							u = _.j[a];
						if (d = _.types[d], "trkn" == a) t[u[0]] = o.a(l + 16 + 11), t.count = o.a(l + 16 + 13);
						else {
							a = l + 16 + 4 + 4;
							var c, h = r - 16 - 4 - 4;
							switch (d) {
								case "text":
									c = o.e(a, h, "UTF-8");
									break;
								case "uint8":
									c = o.w(a);
									break;
								case "jpeg":
								case "png":
									c = {
										k: "image/" + d,
										data: o.l(a, h)
									}
							}
							t[u[0]] = "comment" === u[0] ? {
								text: c
							} : c
						}
					}
					l += r
				}
			}(t, e, 0, e.h()), t
		}, e.ID4 = e.r
	}(this),
	function(a) {
		var e = navigator.platform,
			t = !1;
		if ("iPad" != e && "iPhone" != e || (t = !0), t) {
			var o = !1;
			if (-1 != navigator.userAgent.indexOf("6") && (o = !0), o) {
				var s = {},
					i = {},
					n = a.setTimeout,
					d = a.setInterval,
					l = a.clearTimeout,
					r = a.clearInterval;
				a.setTimeout = function() {
					return u(n, s, arguments)
				}, a.setInterval = function() {
					return u(d, i, arguments)
				}, a.clearTimeout = function(e) {
					var t = s[e];
					t && (delete s[e], l(t.id))
				}, a.clearInterval = function(e) {
					var t = i[e];
					t && (delete i[e], r(t.id))
				}, a.addEventListener("scroll", function() {
					var e;
					for (e in s) c(n, l, s, e);
					for (e in i) c(d, r, i, e)
				})
			}
		}

		function u(e, t, o) {
			var s, i = o[0],
				n = e === d;
			return o[0] = function() {
				i && (i.apply(a, arguments), n || (delete t[s], i = null))
			}, s = e.apply(a, o), t[s] = {
				args: o,
				created: Date.now(),
				cb: i,
				id: s
			}, s
		}

		function c(e, t, o, s) {
			var i = o[s];
			if (i) {
				var n = e === d;
				if (t(i.id), !n) {
					var l = i.args[1],
						r = Date.now() - i.created;
					r < 0 && (r = 0), (l -= r) < 0 && (l = 0), i.args[1] = l
				}
				i.args[0] = function() {
					i.cb && (i.cb.apply(a, arguments), n || (delete o[s], i.cb = null))
				}, i.created = Date.now(), i.id = e.apply(a, i.args)
			}
		}
	}(window),
	function() {
		var s, _, e;
		s = ("undefined" != typeof window && null !== window ? window.DOMParser : void 0) || ("function" == typeof require ? require("xmldom").DOMParser : void 0) || function() {}, _ = function(e, t) {
			var o, s, i, n, l, r, a, d, u, c, h;
			if (e.hasChildNodes())
				for (l = d = 0, c = (n = e.childNodes).length; 0 <= c ? d < c : c < d; l = 0 <= c ?
					++d : --d)
					if (i = (s = n[l]).nodeName, /REF/i.test(i)) {
						for (a = u = 0, h = (o = s.attributes).length; 0 <= h ? u < h : h < u; a = 0 <= h ? ++u : --u)
							if (r = o[a].nodeName.match(/HREF/i)) {
								t.push({
									file: s.getAttribute(r[0]).trim()
								});
								break
							}
					}
			else "#text" !== i && _(s, t);
			return null
		}, e = function(e) {
			var t, o;
			return o = [], (t = (new s).parseFromString(e, "text/xml").documentElement) && _(t, o), o
		}, ("undefined" != typeof module && null !== module ? module.exports : window).ASX = {
			name: "asx",
			parse: e
		}
	}.call(this),
	function() {
		var o, s, i, n, e, l;
		o = /:(?:(-?\d+),(.+)\s*-\s*(.+)|(.+))\n(.+)/, n = function(e) {
			var t;
			return (t = e.match(o)) && 6 === t.length ? {
				length: t[1] || 0,
				artist: t[2] || "",
				title: t[4] || t[3],
				file: t[5].trim()
			} : void 0
		}, l = function(e) {
			return {
				file: e.trim()
			}
		}, i = function(e) {
			return !!e.trim().length
		}, s = function(e) {
			return "#" !== e[0]
		}, e = function(e) {
			var t;
			return t = (e = e.replace(/\r/g, "")).search("\n"), "#EXTM3U" === e.substr(0, t) ? e.substr(t).split("\n#").filter(i).map(n) : e.split("\n").filter(i).filter(s).map(l)
		}, ("undefined" != typeof module && null !== module ? module.exports : window).M3U = {
			name: "m3u",
			parse: e
		}
	}.call(this),
	function() {
		var d, e;
		d = /(file|title|length)(\d+)=(.+)\r?/i, e = function(e) {
			var t, o, s, i, n, l, r, a;
			for (i = [], l = 0, r = (a = e.trim().split("\n")).length; l < r; l++)(s = a[l].match(d)) && 4 === s.length && (s[0], o = s[1], t = s[2], n = s[3], i[t] || (i[t] = {}), i[t][o.toLowerCase()] = n);
			return i.filter(function(e) {
				return null != e
			})
		}, ("undefined" != typeof module && null !== module ? module.exports : window).PLS = {
			name: "pls",
			parse: e
		}
	}.call(this),
	function(window) {
		var FWDMSP = function(props) {
				var self = this;
				if (FWDMSP.instaces_ar.push(this),
            self.mainFolderPath_str = props.mainFolderPath,
            self.mainFolderPath_str.lastIndexOf("/") + 1 != self.mainFolderPath_str.length && (self.mainFolderPath_str += "/"),
            this.skinPath_str = props.skinPath,
            self.skinPath_str.lastIndexOf("/") + 1 != self.skinPath_str.length && (self.skinPath_str += "/"),
            this.warningIconPath_str = self.mainFolderPath_str + this.skinPath_str + "warningIcon.png",
            this.instanceName_str = props.instanceName,
            this.instanceName_str)
            {
					if (window[this.instanceName_str]) alert("FWDMSP instance name " + this.instanceName_str + " is already defined and contains a different instance reference, set a different instance name.");
					else if (window[this.instanceName_str] = this, this.listeners = {
							events_ar: []
						}, window[this.instanceName_str].addListener = function() {}, !document.cookie || -1 == document.cookie.indexOf("FWDMSP=" + self.instanceName_str) || self.isMobile_bl) {
						var recoverDecodingErrorDate,
							recoverSwapAudioCodecDate;
						if (self.init = function() {
								if (FWDTweenLite.ticker.useRAF(!1), this.props_obj = props, this.props_obj) {
									this.position_str = self.props_obj.verticalPosition,
										this.position_str || (this.position_str = FWDMSP.POSITION_TOP), "bottom" == this.position_str ? this.position_str = FWDMSP.POSITION_BOTTOM : this.position_str = FWDMSP.POSITION_TOP,
										this.horizontalPosition_str = self.props_obj.horizontalPosition,
										this.horizontalPosition_str || (this.horizontalPosition_str = FWDMSP.CENTER), "center" == this.horizontalPosition_str ? this.horizontalPosition_str = FWDMSP.CENTER : "left" == this.horizontalPosition_str ? this.horizontalPosition_str = FWDMSP.LEFT : "right" == this.horizontalPosition_str ? this.horizontalPosition_str = FWDMSP.RIGHT : this.horizontalPosition_str = FWDMSP.CENTER,
										this.stageContainer = document.createElement("div"),
										this.stageContainer.style.position = "fixed",
										self.stageContainer.style.width = "100%",
										FWDMSPUtils.isIEAndLessThen9 ? this.stageContainer.style.zIndex = "21474836" : this.stageContainer.style.zIndex = "21474835",
										this.stageContainer.style.overflow = "visible",
										self.stageContainer.style.height = "0px",
										FWDMSPUtils.isIE ? document.getElementsByTagName("body")[0].appendChild(this.stageContainer) : document.documentElement.appendChild(this.stageContainer),
										this.ws = null,
										this.so = null,
										this.data = null,
										this.opener_do = null,
										this.customContextMenu_do = null,
										this.info_do = null,
										this.main_do = null,
										this.background_do = null,
										this.preloader_do = null,
										this.controller_do = null,
										this.categories_do = null,
										this.playlist_do = null,
										this.audioScreen_do = null,
										this.flash_do = null,
										this.flashObject = null,
										this.flashObjectMarkup_str = null,
										this.prevCatId = -1,
										this.catId = -1,
										this.id = -1,
										this.prevId = -1,
										this.totalAudio = 0,
										this.stageWidth = 0,
										this.stageHeight = 0,
										this.maxWidth = self.props_obj.maxWidth || 2e3,
										this.maxHeight = 0,
										this.prevAddToHeight = -1,
										this.lastPercentPlayed = 0,
										this.resizeHandlerId_to,
										this.resizeHandler2Id_to,
										this.hidePreloaderId_to,
										this.orientationChangeId_to,
										this.showCatWidthDelayId_to,
										this.showPlaylistWithDelayId_to,
										this.disablePlaylistForAWhileId_to,
										this.allowToResizeAndPosition_bl = !1,
										this.isAPIReady_bl = !1,
										this.isPlaylistLoaded_bl = !1,
										this.isFlashScreenReady_bl = !1,
										this.orintationChangeComplete_bl = !0,
										this.animate_bl = !1,
										this.isFirstPlaylistLoaded_bl = !1,
										this.showedFirstTime_bl = !1,
										self.isPlaylistShowed_bl = !1,
										this.useDeepLinking_bl = self.props_obj.useDeepLinking,
										this.useDeepLinking_bl = "yes" == self.useDeepLinking_bl,
										this.showMainBackground_bl = "no" != self.props_obj.showMainBackground,
										this.isMobile_bl = FWDMSPUtils.isMobile,
										this.hasPointerEvent_bl = FWDMSPUtils.hasPointerEvent;
									try {
										window.opener && window.opener[this.instanceName_str]
										              && window.opener[this.instanceName_str].instanceName_str == this.instanceName_str
																	&& (window.opener[this.instanceName_str].removeAndDisablePlayer(),
										self.isMobile_bl || (document.cookie = "FWDMSP=" + self.instanceName_str + "; expires=Thu, 18 Dec 2030 12:00:00 UTC; path=/", window.onbeforeunload = function(e) {
											document.cookie = "FWDMSP=; expires=Thu, 01-Jan-70 00:00:01 GMT; path=/"
										}))
									}
									catch (e) {}

									var e, t, o, s, i;
									this.setupMainDo(),
									this.startResizeHandler(),
									this.setupInfo(),
									this.setupData()
								} else alert("FWDMSP constructor properties object is not defined!")
							},
							this.removeAndDisablePlayer = function() {
								self.stageContainer.style.display = "none"
							},
							self.setupMainDo = function() {
								self.showMainBackground_bl && (self.background_do = new FWDMSPDisplayObject("div"),
								                               self.background_do.getStyle().width = "100%"),
								self.main_do = new FWDMSPDisplayObject("div"),
								self.main_do.getStyle().msTouchAction = "none",
								self.main_do.getStyle().webkitTapHighlightColor = "rgba(0, 0, 0, 0)",
								(!FWDMSPUtils.isMobile || FWDMSPUtils.isMobile && FWDMSPUtils.hasPointerEvent) && self.main_do.setSelectable(!1),
								self.background_do && self.stageContainer.appendChild(self.background_do.screen),
								self.stageContainer.appendChild(self.main_do.screen)
							},
							self.setupInfo = function() {
								FWDMSPInfo.setPrototype(),
								self.info_do = new FWDMSPInfo(self, self.warningIconPath_str),
								FWDMSPUtils.isIEAndLessThen9 ? self.info_do.getStyle().zIndex = "2147483632" : self.info_do.getStyle().zIndex = "99999999992"
							},
							self.startResizeHandler = function() {
								window.addEventListener ? (window.addEventListener("resize", self.onResizeHandler),
								                           FWDMSPUtils.isAndroid && window.addEventListener("orientationchange", self.orientationChange))
																				: window.attachEvent && window.attachEvent("onresize", self.onResizeHandler)
							},
							self.stopResizeHandler = function() {
								clearTimeout(self.resizeHandlerId_to),
								clearTimeout(self.resizeHandler2Id_to),
								clearTimeout(self.orientationChangeId_to),
								window.removeEventListener ? (window.removeEventListener("resize", self.onResizeHandler),
																							window.removeEventListener("orientationchange", self.orientationChange))
																					 : window.detachEvent && window.detachEvent("onresize", self.onResizeHandler)
							},
							self.onScrollHandler = function() {
								self.onResizeHandler()
							},
							self.onResizeHandler = function(e) {
								self.resizeHandler()
							},
							this.orientationChange = function() {
								self.orintationChangeComplete_bl = !1,
								clearTimeout(self.resizeHandlerId_to),
								clearTimeout(self.resizeHandler2Id_to),
								clearTimeout(self.orientationChangeId_to),
								self.orientationChangeId_to = setTimeout(function() {
									self.orintationChangeComplete_bl = !0, self.resizeHandler(!0)
									}, 1e3),
							  self.stageContainer.style.left = "-5000px",
								self.preloader_do && self.preloader_do.setX(-5e3)
							},
							self.resizeHandler = function(e, t) {
								self.orintationChangeComplete_bl && (self.ws = FWDMSPUtils.getViewportSize(),
								                                     self.stageWidth = document.documentElement.offsetWidth,
																										 self.stageContainer.style.left = "0px",
																										 self.stageWidth > self.maxWidth && (self.stageWidth = self.maxWidth),
																										 self.controller_do && (self.maxHeight = self.controller_do.h),
																										 self.stageHeight = self.maxHeight,
																										 self.main_do.setWidth(self.stageWidth),
																										 self.preloader_do && self.positionPreloader(),
																										 self.controller_do && self.controller_do.resizeAndPosition(e),
																										 self.categories_do && self.categories_do.resizeAndPosition(),
																										 self.playlist_do && self.playlist_do.resizeAndPosition(),
																										 self.isFirstPlaylistLoaded_bl && self.setStageContainerFinalHeightAndPosition(!1),
																										 self.info_do && self.info_do.isShowed_bl && self.info_do.positionAndResize(),
																										 self.atb_do && self.atb_do.isShowed_bl && self.atb_do.positionAndResize(),
																										 self.passWindow_do && self.passWindow_do.isShowed_bl && self.passWindow_do.positionAndResize(),
																										 self.playbackRateWindow_do && self.playbackRateWindow_do.isShowed_bl && self.playbackRateWindow_do.positionAndResize())
							},
							this.setStageContainerFinalHeightAndPosition = function(e) {
								if (self.ws || (self.ws = FWDMSPUtils.getViewportSize()),
								    self.controller_do && self.allowToResizeAndPosition_bl) {
									if (self.openInPopup_bl) return self.main_do.setX(0),
										self.main_do.setY(0),
										self.main_do.getStyle().width = "100%",
										self.main_do.setHeight(self.ws.h),
										self.controller_do.setX(0),
										FWDAnimation.killTweensOf(self.controller_do),
										e ? 0 != self.controller_do.y && FWDAnimation.to(self.controller_do, .8, {
												y: 0,
												ease: Expo.easeInOut
												})
											: self.controller_do.setY(0),
									self.isFullScreen_bl || self.controller_do.setY(0),
									void(self.playlist_do && (FWDAnimation.killTweensOf(self.playlist_do),
																						self.playlist_do.setX(0),
																						self.playlist_do.setY(self.controller_do.h)));
									clearTimeout(self.showPlaylistWithDelayId_to),
									self.playlist_do && self.playlist_do.isShowed_bl && (addToHeight = self.playlist_do.h),
									self.position_str == FWDMSP.POSITION_TOP ? self.playlist_do
									                                         ? (self.background_do && self.background_do.setHeight(self.playlist_do.h + self.controller_do.h),
																													    self.playlist_do.setY(0),
																															self.isFullScreen_bl ? self.controller_do.setY(0) : self.controller_do.setY(self.playlist_do.h),
																															self.main_do.setHeight(self.playlist_do.h + self.controller_do.h))
																													 : (self.background_do && self.background_do.setHeight(self.controller_do.h),
																													    self.controller_do.setY(0),
																															self.main_do.setHeight(self.controller_do.h))
																													 : self.playlist_do ? (self.background_do && self.background_do.setHeight(self.playlist_do.h + self.controller_do.h + 150),
																													   self.playlist_do.setY(self.controller_do.h),
																														 self.controller_do.setY(0),
																														 self.main_do.setHeight(self.playlist_do.h + self.controller_do.h))
																													 : (self.background_do && self.background_do.setHeight(self.controller_do.h),
																													   self.controller_do.setY(0),
																														 self.main_do.setHeight(self.controller_do.h)),
										self.horizontalPosition_str == FWDMSP.LEFT ? (self.main_do.setX(0),
										                                              self.opener_do && ("right" == self.data.openerAlignment_str ? self.opener_do.setX(Math.round(self.stageWidth - self.opener_do.w)) : self.opener_do.setX(0)))
																															 : self.horizontalPosition_str == FWDMSP.CENTER
																															 ? (self.main_do.setX(Math.round((self.ws.w - self.stageWidth) / 2)),
																															    self.opener_do && ("right" == self.data.openerAlignment_str
																																	? self.opener_do.setX(parseInt((self.ws.w - self.stageWidth) / 2) + self.stageWidth - self.opener_do.w)
																																	: self.opener_do.setX(self.main_do.x)))
																															 : self.horizontalPosition_str == FWDMSP.RIGHT && (self.main_do.setX(Math.round(self.ws.w - self.stageWidth)),
																															   																								 "right" == self.data.openerAlignment_str
																																																								 ? self.opener_do.setX(Math.round(self.ws.w - self.opener_do.w))
																																																								 : self.opener_do.setX(Math.round(self.ws.w - self.stageWidth))),
										FWDAnimation.killTweensOf(self.stageContainer),
										self.background_do && FWDAnimation.killTweensOf(self.background_do),
										FWDAnimation.killTweensOf(self.controller_do),
										FWDAnimation.killTweensOf(self.opener_do),
										self.center(),
										e ? self.position_str == FWDMSP.POSITION_TOP ? self.playlist_do && self.playlist_do.isShowed_bl && self.controller_do.isShowed_bl ? (FWDAnimation.to(self.stageContainer, .8, {
											css: {
												top: 0
											},
											ease: Expo.easeInOut
										}), FWDAnimation.to(self.opener_do, .8, {
											y: self.playlist_do.h + self.controller_do.h,
											ease: Expo.easeInOut
										})) : self.controller_do.isShowed_bl && self.playlist_do ? (FWDAnimation.to(self.stageContainer, .8, {
											css: {
												top: -self.playlist_do.h
											},
											ease: Expo.easeInOut
										}), FWDAnimation.to(self.opener_do, .8, {
											y: self.playlist_do.h + self.controller_do.h,
											ease: Expo.easeInOut
										})) : !self.controller_do.isShowed_bl && self.playlist_do ? (FWDAnimation.to(self.stageContainer, .8, {
											css: {
												top: -self.playlist_do.h - self.controller_do.h
											},
											ease: Expo.easeInOut
										}), FWDAnimation.to(self.opener_do, .8, {
											y: self.playlist_do.h + self.controller_do.h,
											ease: Expo.easeInOut,
											onComplete: self.moveWheyLeft
										})) : (self.controller_do.isShowed_bl ? FWDAnimation.to(self.stageContainer, .8, {
											css: {
												top: 0
											},
											ease: Expo.easeInOut
										}) : FWDAnimation.to(self.stageContainer, .8, {
											css: {
												top: -self.controller_do.h
											},
											ease: Expo.easeInOut
										}), FWDAnimation.to(self.opener_do, .8, {
											y: self.controller_do.h,
											ease: Expo.easeInOut
										})) : (self.playlist_do && self.playlist_do.isShowed_bl && self.controller_do.isShowed_bl ? FWDAnimation.to(self.stageContainer, .8, {
											css: {
												top: self.ws.h - self.controller_do.h - self.playlist_do.h
											},
											ease: Expo.easeInOut
										}) : self.controller_do.isShowed_bl && self.playlist_do ? FWDAnimation.to(self.stageContainer, .8, {
											css: {
												top: self.ws.h - self.controller_do.h
											},
											ease: Expo.easeInOut
										}) : self.controller_do.isShowed_bl ? FWDAnimation.to(self.stageContainer, .8, {
											css: {
												top: self.ws.h - self.controller_do.h
											},
											ease: Expo.easeInOut
										}) : self.controller_do.isShowed_bl ? FWDAnimation.to(self.stageContainer, .8, {
											css: {
												top: 0
											},
											ease: Expo.easeInOut
										}) : FWDAnimation.to(self.stageContainer, .8, {
											css: {
												top: self.ws.h
											},
											ease: Expo.easeInOut,
											onComplete: self.moveWheyLeft
										}), FWDAnimation.to(self.opener_do, .8, {
											y: -self.opener_do.h,
											ease: Expo.easeInOut
										})) : self.position_str == FWDMSP.POSITION_TOP ? self.playlist_do && self.playlist_do.isShowed_bl && self.controller_do.isShowed_bl ? (self.stageContainer.style.top = "0px", self.opener_do.setY(self.playlist_do.h + self.controller_do.h)) : self.controller_do.isShowed_bl && self.playlist_do ? (self.stageContainer.style.top = -self.playlist_do.h + "px", self.opener_do.setY(self.playlist_do.h + self.controller_do.h)) : !self.controller_do.isShowed_bl && self.playlist_do ? (self.stageContainer.style.top = -self.playlist_do.h - self.controller_do.h + "px", self.opener_do.setY(self.playlist_do.h + self.controller_do.h)) : self.controller_do.isShowed_bl ? (self.stageContainer.style.top = "0px", self.opener_do.setY(self.controller_do.h)) : (self.stageContainer.style.top = -self.controller_do.h + "px", self.opener_do.setY(self.controller_do.h), self.moveWheyLeft()) : (self.playlist_do && self.playlist_do.isShowed_bl && self.controller_do.isShowed_bl ? self.stageContainer.style.top = self.ws.h - self.controller_do.h - self.playlist_do.h + "px" : self.controller_do.isShowed_bl && self.playlist_do ? self.stageContainer.style.top = self.ws.h - self.controller_do.h + "px" : self.controller_do.isShowed_bl ? self.stageContainer.style.top = self.ws.h - self.controller_do.h + "px" : (self.stageContainer.style.top = self.ws.h + "px", self.moveWheyLeft()), self.opener_do.setY(-self.opener_do.h))
								}
							},
							this.moveWheyLeft = function() {
								self.main_do.setX(-5e3),
								self.background_do && self.background_do.setWidth(0)
							},
							this.center = function() {
								self.isFullScreen_bl && self.main_do.setX(0),
								self.background_do && (self.background_do.getStyle().width = "100%")
							},
							this.setupContextMenu = function() {
								self.customContextMenu_do = new FWDMSPContextMenu(self.main_do, self.data.rightClickContextMenu_str)
							},
							this.setupMainInstances = function() {
								self.controller_do || (FWDMSP.hasHTML5Audio && self.setupAudioScreen(),
								                      self.data.showPlaylistsButtonAndPlaylists_bl && self.setupCategories(),
																			self.data.showPlayListButtonAndPlaylist_bl && self.setupPlaylist(),
																			self.setupController(),
																			self.data.showPlaybackRateButton_bl && self.setupPlaybackRateWindow(),
																			self.setupOpener(),
																			self.controller_do.resizeAndPosition(),
																			self.data.addKeyboardSupport_bl && self.addKeyboardSupport())
							},
							this.setInputs = function() {
								for (var e = document.querySelectorAll("input"),
										 t = 0;
										 t < e.length; t++)
										 	self.hasPointerEvent_bl ? e[t].addEventListener("pointerdown", self.inputFocusInHandler)
										                                            : e[t].addEventListener && (e[t].addEventListener("mousedown", self.inputFocusInHandler),
																																                            e[t].addEventListener("touchstart", self.inputFocusInHandler))
							},
							this.inputFocusInHandler = function(e) {
								self.curInput = e.target,
								setTimeout(function() {
									self.hasPointerEvent_bl ? window.addEventListener("pointerdown", self.inputFocusOutHandler)
									                        : window.addEventListener && (window.addEventListener("mousedown", self.inputFocusOutHandler),
																					                              window.addEventListener("touchstart", self.inputFocusOutHandler)),
									FWDMSP.isSearchedFocused_bl = !0
								}, 50)
							},
							this.inputFocusOutHandler = function(e) {
								var t = FWDUVPUtils.getViewportMouseCoordinates(e);
								if (!FWDUVPUtils.hitTest(self.curInput, t.screenX, t.screenY))
								   return self.hasPointerEvent_bl ? window.removeEventListener("pointerdown", self.inputFocusOutHandler)
									                                : window.removeEventListener && (window.removeEventListener("mousedown", self.inputFocusOutHandler),
																																									 window.removeEventListener("touchstart", self.inputFocusOutHandler)),
																									void(FWDMSP.isSearchedFocused_bl = !1)
							},
							this.addKeyboardSupport = function() {
								self.setInputs(),
								document.addEventListener("keydown", this.onKeyDownHandler),
								document.addEventListener("keyup", this.onKeyUpHandler)
							},
							this.onKeyDownHandler = function(e) {
								if (!self.isSpaceDown_bl && self.hasStartedToPlay_bl
									                       && !FWDMSP.isSearchedFocused_bl
																				 && (self.isSpaceDown_bl = !0,
										e.preventDefault && e.preventDefault(),
										self == FWDMSP.keyboardCurInstance)) {
									if (32 == e.keyCode) {
											if (!self.audioScreen_do.isSafeToBeControlled_bl)
											  return;
											self.audioScreen_do.togglePlayPause()
										}
										return e.preventDefault && e.preventDefault(), !1
									}
									if (77 == e.keyCode) 0 != self.volume && (self.lastVolume = self.volume),
									0 != self.volume ? self.volume = 0 : self.volume = self.lastVolume,
									self.setVolume(self.volume);
									else if (38 == e.keyCode) self.volume += .1,
										1 < self.volume && (self.volume = 1),
										self.setVolume(self.volume);
									else if (40 == e.keyCode) self.volume -= .1,
										self.volume < 0 && (self.volume = 0),
										self.setVolume(self.volume);
									else if (77 == e.keyCode) self.volume < 0 && (self.volume = 0),
									  self.setVolume(self.volume);
									else if (39 != e.keyCode || self.isAdd_bl) {
										if (37 == e.keyCode && !self.isAdd_bl) {
											5 == (t = self.getCurrentTime()).length && (t = "00:" + t), 7 == t.length
											                                        && (t = "0" + t),
											t = FWDMSPUtils.getSecondsFromString(t),
											t -= 5, 5 == (t = FWDMSPUtils.formatTime(t)).length && (t = "00:" + t), 7 == t.length && (t = "0" + t),
											self.scrubbAtTime(t)
										}
									} else {
										var t;
										5 == (t = self.getCurrentTime()).length && (t = "00:" + t),
										7 == t.length && (t = "0" + t),
										t = FWDMSPUtils.getSecondsFromString(t),
										t += 5, 5 == (t = FWDMSPUtils.formatTime(t)).length && (t = "00:" + t),
										7 == t.length && (t = "0" + t),
										self.scrubbAtTime(t)
									}

							},
              this.onKeyUpHandler = function(e) {
								self.isSpaceDown_bl = !1
							},
              this.setupAopw = function() {
								FWDMSPOPWindow.setPrototype(),
                self.popw_do = new FWDMSPOPWindow(self.data, self)
							},

              this.setupAtbWindow = function() {
								FWDMSPATB.setPrototype(),
                self.atb_do = new FWDMSPATB(self.controller_do, self),
                self.atb_do.addListener(FWDMSPATB.HIDE_COMPLETE, self.atbWindowHideCompleteHandler)
							},
              this.atbWindowHideCompleteHandler = function() {
								self.controller_do && !self.isMobile_bl && (self.controller_do.atbButton_do.isDisabled_bl = !1, self.controller_do.atbButton_do.setNormalState())
							},
              this.setupPlaybackRateWindow = function() {
								FWDMSPPlaybackRateWindow.setPrototype(),
                self.playbackRateWindow_do = new FWDMSPPlaybackRateWindow(self.data, self),
                self.playbackRateWindow_do.addListener(FWDMSPPlaybackRateWindow.HIDE_COMPLETE, self.playbackRateWindowHideCompleteHandler),
                self.playbackRateWindow_do.addListener(FWDMSPPlaybackRateWindow.SET_PLAYBACK_RATE, self.playbackRateWindowSetPlaybackRateHandler)
							},
              this.playbackRateWindowHideCompleteHandler = function() {
								self.controller_do && !self.isMobile_bl && (self.controller_do.playbackRateButton_do.isDisabled_bl = !1, self.controller_do.playbackRateButton_do.setNormalState())
							},
              this.playbackRateWindowSetPlaybackRateHandler = function(e) {
								self.setPlaybackRate(e.rate)
							},
              this.setupContinousPlayback = function() {
								self.data.useContinuousPlayback_bl && (self.ppPplayedOnce = !1, window.onbeforeunload = function(e) {
									var t = new Date;
									t.setTime(t.getTime() + 2e4);
									var o, s = 0;
									self.audioScreen_do && (s = self.audioScreen_do.lastPercentPlayed, o = self.audioScreen_do.isPlaying_bl),
										document.cookie = "FWDMSPusePP=true; expires=" + t.toGMTString() + ", 01-Jan-70 00:00:01 GMT; path=/",
										document.cookie = "FWDMSPVolume=" + self.volume + "; expires=" + t.toGMTString() + ", 01-Jan-70 00:00:01 GMT; path=/",
										document.cookie = "FWDMSPpp=" + s + "; expires=" + t.toGMTString() + ", 01-Jan-70 00:00:01 GMT; path=/",
										document.cookie = "FWDMSPppPlay=" + o + "; expires=" + t.toGMTString() + ", 01-Jan-70 00:00:01 GMT; path=/",
										document.cookie = "FWDMSPcatId=" + self.catId + "; expires=" + t.toGMTString() + ", 01-Jan-70 00:00:01 GMT; path=/",
										document.cookie = "FWDMSPid=" + self.id + "; expires=" + t.toGMTString() + ", 01-Jan-70 00:00:01 GMT; path=/"
								})
							}, this.setupData = function() {
								FWDMSPAudioData.setPrototype(),
                self.data = new FWDMSPAudioData(self.props_obj, self.rootElement_el, self),
                self.data.addListener(FWDMSPAudioData.UPDATE_IMAGE, self.onImageUpdate),
                self.data.addListener(FWDMSPAudioData.PRELOADER_LOAD_DONE, self.onPreloaderLoadDone),
                self.data.addListener(FWDMSPAudioData.SOUNDCLOUD_TRACK_READY, self.onSoundClooudReady),
                self.data.addListener(FWDMSPAudioData.RADIO_TRACK_READY, self.onRadioReady),
                self.data.addListener(FWDMSPAudioData.RADIO_TRACK_UPDATE, self.onRadioTrackUpdate),
                self.data.addListener(FWDMSPAudioData.LOAD_ERROR, self.dataLoadError),
                self.data.addListener(FWDMSPAudioData.SKIN_LOAD_COMPLETE, self.dataSkinLoadComplete),
                self.data.addListener(FWDMSPAudioData.PLAYLIST_LOAD_COMPLETE, self.dataPlayListLoadComplete)
							},
              self.onImageUpdate = function(e) {
								self.controller_do.loadThumb(e.image)
							},
              self.onRadioReady = function(e) {
								self.isShoutcast_bl || self.isIcecast_bl ? (self.radioSource_str = e.source, self.data.playlist_ar[self.id].title = e.songTitle, self.controller_do.setTitle(e.songTitle), self.prevAudioPath != self.audioPath && (self.setSource(), self.isPlaylistItemClicked_bl && self.play(), self.prevAudioPath = self.audioPath)) : self.data.closeJsonPLoader()
							},
              self.onRadioTrackUpdate = function(e) {
								self.curTitle = e.songTitle,
                self.curTitle != self.prevTitle && (self.controller_do.setTitle(e.songTitle), self.prevTitle = self.curTitle)
							},
              self.onSoundClooudReady = function(e) {
								self.data.playlist_ar[self.id].source = e.source,
                self.setSource(),
                self.isPlaylistItemClicked_bl && self.play()
							},
              self.onPreloaderLoadDone = function() {
								!self.data.useContinuousPlayback_bl && !self.data.autoPlay_bl || FWDMSP.iFrame || !FWDMSPUtils.isChrome || FWDMSPUtils.isMobile || (FWDMSP.iFrame = document.createElement("iframe"), FWDMSP.iFrame.src = self.data.mainFolderPath_str + "audio/silent.mp3", FWDMSP.iFrame.style.position = "absolute", FWDMSP.iFrame.style.top = "-500px", document.documentElement.appendChild(FWDMSP.iFrame)),
									self.maxHeight = 32, self.usePlaylistsSelectBox_bl = self.data.usePlaylistsSelectBox_bl,
									self.background_do && (self.background_do.getStyle().background = "url('" + self.data.skinPath_str + "main-background.png')"), self.setupPreloader(), !self.isMobile_bl && self.data.showContextMenu_bl && self.setupContextMenu(), self.resizeHandler(), self.main_do.setHeight(self.stageHeight), self.main_do.setHeight(3e3)
							},
              self.dataLoadError = function(e) {
								self.maxHeight = 120,
                self.preloader_do && self.preloader_do.hide(!1),
                self.main_do.addChild(self.info_do),
                self.info_do.showText(e.text),
                self.controller_do || (self.ws || (self.ws = FWDMSPUtils.getViewportSize()),
                self.position_str == FWDMSP.POSITION_TOP ? self.stageContainer.style.top = "0px" : self.stageContainer.style.top = self.ws.h - self.maxHeight + "px", self.main_do.setHeight(self.maxHeight)),
                self.resizeHandler(),
                self.dispatchEvent(FWDMSP.ERROR, {
									error: e.text
								})
							},
              self.dataSkinLoadComplete = function() {
								self.animate_bl = self.data.animate_bl,
								self.lastVolume = self.volume = self.data.volume,
                self.setupContinousPlayback(),
                self.initPlaylist()
							},
              self.initPlaylist = function() {
								self.useDeepLinking_bl ? setTimeout(function() {
									self.setupDL()
								}, 200) :
                (FWDMSPUtils.getCookie("FWDMSPusePP") ? (self.catId = FWDMSPUtils.getCookie("FWDMSPcatId"), self.id = FWDMSPUtils.getCookie("FWDMSPid")) : (self.catId = self.data.startAtPlaylist, self.id = self.data.startAtTrack),
                  self.loadInternalPlaylist()
                )
							},
              this.dataPlayListLoadComplete = function() {
								self.isAPIReady_bl || self.dispatchEvent(FWDMSP.READY),
                self.data.randomizePlaylist_bl && (self.data.playlist_ar = FWDMSPUtils.randomizeArray(self.data.playlist_ar)),
								self.isAPIReady_bl = !0,
                self.isPlaylistLoaded_bl = !0,
                self.data.startAtRandomTrack_bl && (self.id = Math.max(0, parseInt(Math.random() * self.data.playlist_ar.length) - 1),
                self.startAtTrack = self.id,
                self.useDeepLinking_bl && (self.preventFWDDLchange_bl = !0, FWDAddress.setValue(self.instanceName_str + "?catid=" + self.catId + "&trackid=" + self.id), setTimeout(function() {
										self.preventFWDDLchange_bl = !1
									}, 250))),
                self.setupMainInstances(),
                self.updatePlaylist(),
								//self.setStageContainerFinalHeightAndPosition(!0),
                self.dispatchEvent(FWDMSP.LOAD_PLAYLIST_COMPLETE)
							},
              this.updatePlaylist = function() {
								if (self.main_do && self.main_do.contains(self.info_do) && self.main_do.removeChild(self.info_do),
                    self.id > self.data.playlist_ar.length && (self.id = 0),
                    self.data.playlist_ar
										  &&
											(self.preloader_do.hide(!0),
                    	 self.prevId = -1,
                    	 self.totalAudio = self.data.playlist_ar.length,
                    	 self.controller_do.enableControllerWhileLoadingPlaylist(),
                    	 self.controller_do.cleanThumbnails(!0),
                       self.playlist_do && (self.playlist_do.updatePlaylist(self.data.playlist_ar),
                                         self.playlist_do.resizeAndPosition(),
                                         self.playlist_do.isShowed_bl && self.controller_do.setPlaylistButtonState("selected")
                                        ),
                    	self.openInPopup_bl && self.popupWindow.audioScreen_do && (self.lastPercentPlayed = self.popupWindow.audioScreen_do.lastPercentPlayed),
                    	self.playlist_do && self.playlist_do.comboBox_do && self.playlist_do.comboBox_do.setButtonsStateBasedOnId(self.catId),
                    	self.setSource(),
                    	(self.data.autoPlay_bl || self.data.playTrackAfterPlaylistLoad_bl) && setTimeout(self.play, 1e3),
											self.openInPopup_bl && !self.showedFirstTime_bl ? (self.controller_do.setY(-self.controller_do.h),
										                                                   self.playlist_do && self.playlist_do.setY(-self.playlist_do.h))
																																		: self.playlist_do && self.playlist_do.setY(-self.playlist_do.h + self.controller_do.h),
											self.setStageContainerFinalHeightAndPosition(!0),
											self.openInPopup_bl
										))
										return clearTimeout(self.showPlaylistWithDelayId_to),
									self.showedFirstTime_bl ? self.showPlaylistWithDelayId_to = setTimeout(function() {
																						self.setStageContainerFinalHeightAndPosition(!0)
																						}, 100)
																					: self.showPlaylistWithDelayId_to = setTimeout(function() {
																						self.setStageContainerFinalHeightAndPosition(!0)
																					}, 900),
                self.showedFirstTime_bl = !0,
								void(self.allowToResizeAndPosition_bl = !0);
								self.allowToResizeAndPosition_bl = !0,
								self.position_str == FWDMSP.POSITION_TOP ? self.playlist_do && self.controller_do.isShowed_bl ? self.showedFirstTime_bl ? (self.stageContainer.style.top = -self.playlist_do.h + "px", self.opener_do.setY(self.controller_do.h + self.playlist_do.h))
                : (self.stageContainer.style.top = -self.controller_do.h - self.playlist_do.h + "px", self.opener_do.setY(self.controller_do.h + self.playlist_do.h - self.opener_do.h))
                : self.controller_do.isShowed_bl ? self.playlist_do ? (self.stageContainer.style.top = self.controller_do.h + "px", self.opener_do.setY(self.controller_do.h + self.playlist_do.h - self.opener_do.h))
                : self.showedFirstTime_bl || (self.stageContainer.style.top = -self.controller_do.h + "px", self.opener_do.setY(self.controller_do.h - self.opener_do.h))
                : self.playlist_do ? (self.stageContainer.style.top = -self.controller_do.h - self.playlist_do.h + "px", self.opener_do.setY(0))
                : self.showedFirstTime_bl ? (self.stageContainer.style.top = -self.controller_do.h + "px", self.opener_do.setY(0))
                : (self.stageContainer.style.top = -self.controller_do.h + "px", self.opener_do.setY(-self.opener_do.h))
                : self.controller_do.isShowed_bl || self.playlist_do && self.controller_do.isShowed_bl ? self.showedFirstTime_bl ? (self.stageContainer.style.top = self.ws.h - self.controller_do.h + "px", self.opener_do.setY(-self.opener_do.h))
                : (self.stageContainer.style.top = self.ws.h + "px", self.opener_do.setY(0))
                : self.showedFirstTime_bl ? (self.stageContainer.style.top = self.ws.h + "px", self.opener_do.setY(-self.opener_do.h))
                : (self.stageContainer.style.top = self.ws.h + "px", self.opener_do.setY(0)),
								clearTimeout(self.showPlaylistWithDelayId_to),
								self.showPlaylistWithDelayId_to = setTimeout(function() {
										self.setStageContainerFinalHeightAndPosition(!0)
									}, 900),
                self.showedFirstTime_bl = !0
							},
              this.loadInternalPlaylist = function() {
								self.isPlaylistLoaded_bl = !1,
                self.data.loadPlaylist(self.catId),
								self.isPlaylistItemClicked_bl = !1,
                self.stop(),
                self.playbackRateWindow_do && self.playbackRateWindow_do.hide(),
                self.preloader_do.show(!0),
                self.controller_do && (self.controller_do.disableControllerWhileLoadingPlaylist(), self.controller_do.loadThumb()),
                self.hider && (self.hider.reset(), self.hider.stop()),
								self.playlist_do && self.playlist_do.destroyPlaylist(),
								self.positionPreloader(),
                self.setStageContainerFinalHeightAndPosition(!1),
                self.dispatchEvent(FWDMSP.START_TO_LOAD_PLAYLIST)
							},
              this.setupDL = function() {
								self.setOnceDL = !0,
                self.dlChangeHandler(),
                FWDAddress.onChange = self.dlChangeHandler
							},
              this.dlChangeHandler = function() {
								var e = !1;
								if (!self.preventFWDDLchange_bl)
									if (self.categories_do && self.categories_do.isOnDOM_bl) self.categories_do.hide();
									else {
										if (self.catId = parseInt(FWDAddress.getParameter("catid")), self.id = parseInt(FWDAddress.getParameter("trackid")), "true" == FWDMSPUtils.getCookie("FWDMSPusePP") && self.setOnceDL && -1 == location.hash.indexOf("catid=")) return self.catId = FWDMSPUtils.getCookie("FWDMSPcatId"),
										  self.id = FWDMSPUtils.getCookie("FWDMSPid"),
                      self.setOnceDL = !1,
                      void(location.hash = self.instanceName_str + "?catid=" + self.catId + "&trackid=" + self.id);
										(void 0 === self.catId || void 0 === self.id || isNaN(self.catId) || isNaN(self.id)) && (self.catId = self.data.startAtPlaylist, self.id = self.data.startAtTrack, e = !0),
                    (self.catId < 0 || self.catId > self.data.totalCategories - 1 && !e) && (self.catId = self.data.startAtPlaylist, self.id = self.data.startAtTrack, e = !0),
										self.data.playlist_ar && (self.id < 0 && !e ? (self.id = self.data.startAtTrack, e = !0) : self.prevCatId == self.catId && self.id > self.data.playlist_ar.length - 1 && !e && (self.id = self.data.playlist_ar.length - 1, e = !0)),
                    e ? location.hash = self.instanceName_str + "?catid=" + self.catId + "&trackid=" + self.id : self.prevCatId != self.catId ? (self.loadInternalPlaylist(), self.prevCatId = self.catId) : (self.isPlaylistItemClicked_bl = !0, self.setSource(!1), self.isShoutcast_bl || self.isIcecast_bl || self.play())
									}
							}, this.setupPreloader = function() {
								FWDMSPPreloader.setPrototype(),
                self.preloader_do = new FWDMSPPreloader(self.data.preloaderPath_str, 53, 34, 30, 80),
                self.preloader_do.addListener(FWDMSPPreloader.HIDE_COMPLETE, self.preloaderHideComplete),
								FWDMSPUtils.isIEAndLessThen9 ? self.preloader_do.getStyle().zIndex = "2147483633" : self.preloader_do.getStyle().zIndex = "99999999993",
                self.preloader_do.setPosition("fixed"),
                self.preloader_do.setForFixedPosition(),
                self.preloader_do.show(!0),
								document.documentElement.appendChild(self.preloader_do.screen)
							},
              this.positionPreloader = function() {
								self.preloader_do.setX(parseInt((self.ws.w - self.preloader_do.w) / 2)),
                self.openInPopup_bl ? self.controller_do ? self.preloader_do.setY(parseInt((self.controller_do.h - self.preloader_do.h) / 2)) : self.preloader_do.setY(0) : self.position_str == FWDMSP.POSITION_TOP ? self.controller_do && !self.controller_do.isShowed_bl ? self.preloader_do.setY(-200) : self.controller_do ? self.preloader_do.setY(parseInt((self.controller_do.h - self.preloader_do.h) / 2)) : self.preloader_do.setY(parseInt((self.stageHeight - self.preloader_do.h) / 2)) : self.controller_do && !self.controller_do.isShowed_bl ? self.preloader_do.setY(self.ws.h) : self.controller_do ? self.preloader_do.setY(self.ws.h - self.controller_do.h + parseInt((self.controller_do.h - self.preloader_do.h) / 2)) : self.preloader_do.setY(self.ws.h - self.preloader_do.h)
							},
              this.preloaderHideComplete = function() {
								self.controller_do.show(),
                self.opener_do.show(),
                self.playlist_do && self.playlist_do.show(),
                self.isFirstPlaylistLoaded_bl = !0,
								self.allowToResizeAndPosition_bl = !0,
								self.animate_bl || self.setStageContainerFinalHeightAndPosition(!1)
							},
              this.setupOpener = function() {
								FWDMSPOpener.setPrototype(),
                self.opener_do = new FWDMSPOpener(self.data, self.position_str, self.controller_do.isShowed_bl),
								FWDMSPUtils.isIEAndLessThen9 ? self.opener_do.getStyle().zIndex = "2147483634" : self.opener_do.getStyle().zIndex = "99999999994", self.opener_do.setX(-1e3), self.controller_do.isShowed_bl ? self.opener_do.showCloseButton() : self.opener_do.showOpenButton(), self.opener_do.addListener(FWDMSPOpener.SHOW, self.openerShowHandler), self.opener_do.addListener(FWDMSPOpener.HIDE, self.openerHideHandler), self.opener_do.addListener(FWDMSPController.PLAY, self.controllerOnPlayHandler),
								self.opener_do.addListener(FWDMSPController.PAUSE, self.controllerOnPauseHandler),
								self.data.showOpener_bl && self.stageContainer.appendChild(self.opener_do.screen)
							},
              this.openerShowHandler = function() {
								self.showPlayer()
							},
              this.openerHideHandler = function() {
								self.hidePlayer()
							},
              this.setupCategories = function() {
								FWDMSPCategories.setPrototype(),
                self.categories_do = new FWDMSPCategories(self.data),
								FWDMSPUtils.isIEAndLessThen9 ? self.categories_do.getStyle().zIndex = "2147483635" : self.categories_do.getStyle().zIndex = "99999999995",
                self.categories_do.addListener(FWDMSPCategories.HIDE_COMPLETE, self.categoriesHideCompleteHandler),
                self.data.showPlaylistsByDefault_bl && (self.showCatWidthDelayId_to = setTimeout(function() {
										self.showCategories()
									}, 1400))
							},
              this.categoriesHideCompleteHandler = function(e) {
								if (self.controller_do.setCategoriesButtonState("unselected"),
                self.customContextMenu_do && self.customContextMenu_do.updateParent(self.main_do),
                self.useDeepLinking_bl) self.categories_do.id != self.catId && (
                                                                                  self.catId = self.categories_do.id, self.id = 0, FWDAddress.setValue(self.instanceName_str + "?catid=" + self.catId + "&trackid=" + self.id)
                                                                                );
								else {
									if (self.catId == self.categories_do.id) return;
									self.catId = self.categories_do.id,
                  self.id = 0,
                  self.loadInternalPlaylist(self.catId)
								}
							},
              this.setupPlaylist = function() {
								FWDMSPPlaylist.setPrototype(),
                self.playlist_do = new FWDMSPPlaylist(self.data, self),
                self.playlist_do.addListener(FWDMSPPlaylist.CHANGE_PLAYLIST, self.playlistChangePlaylistHandler),
                self.playlist_do.addListener(FWDMSPPlaylistItem.MOUSE_UP, self.palylistItemOnUpHandler),
								self.playlist_do.addListener(FWDMSPPlaylistItem.BUY, self.palylistItemBuyHandler),
                self.playlist_do.addListener(FWDMSPPlaylist.UPDATE_TRACK_TITLE_if_FOLDER, self.palylistUpdateFolderTrackTitle),
                self.main_do.addChild(self.playlist_do)
							},
              this.playlistChangePlaylistHandler = function(e) {
								if (self.controller_do.setCategoriesButtonState("unselected"), self.customContextMenu_do && self.customContextMenu_do.updateParent(self.main_do), self.useDeepLinking_bl)
                   e.id != self.catId && (self.catId = e.id, self.id = 0, FWDAddress.setValue(self.instanceName_str + "?catid=" + self.catId + "&trackid=" + self.id));
								else {
									if (self.catId == e.id) return;
									self.catId = e.id, self.id = 0,
										self.loadInternalPlaylist(self.catId)
								}
							},
              this.palylistItemOnUpHandler = function(e) {
								self.isPlaylistItemClicked_bl = !0,
								e.id == self.id ? self.audioType_str == FWDMSP.AUDIO && self.audioScreen_do.isPlaying_bl
																	? self.pause()
																	: self.audioType_str != FWDMSP.AUDIO || self.audioScreen_do.isStopped_bl && !self.audioScreen_do.isStopped_bl
																	? self.audioType_str != FWDMSP.HLS
														  		: self.play()
																: self.useDeepLinking_bl && self.id != e.id
																  ? (FWDAddress.setValue(self.instanceName_str + "?catid=" + self.catId + "&trackid=" + e.id), self.id = e.id)
																	: (self.id = e.id, self.setSource(!0), self.play()
									)

							},
              this.palylistUpdateFolderTrackTitle = function(e) {
								self.controller_do.setTitle(e.title)
							},
              this.palylistItemBuyHandler = function(e) {
								self.buy(e.id)
							},
              this.setupController = function() {
								FWDMSPController.setPrototype(), self.controller_do = new FWDMSPController(self.data, self), self.controller_do.addListener(FWDMSPController.POPUP, self.controllerOnPopupHandler),
									self.controller_do.addListener(FWDMSPController.PLAY, self.controllerOnPlayHandler),
									self.controller_do.addListener(FWDMSPController.PLAY_NEXT, self.controllerPlayNextHandler),
                  self.controller_do.addListener(FWDMSPController.PLAY_PREV, self.controllerPlayPrevHandler),
									self.controller_do.addListener(FWDMSPController.PAUSE, self.controllerOnPauseHandler),
									self.controller_do.addListener(FWDMSPController.CHANGE_VOLUME, self.controllerChangeVolumeHandler),
                  self.controller_do.addListener(FWDMSPController.VOLUME_START_TO_SCRUB, self.volumeStartToScrubbHandler),
									self.controller_do.addListener(FWDMSPController.VOLUME_STOP_TO_SCRUB, self.volumeStopToScrubbHandler),
									self.controller_do.addListener(FWDMSPController.START_TO_SCRUB, self.controllerStartToScrubbHandler),
                  self.controller_do.addListener(FWDMSPController.SCRUB, self.controllerScrubbHandler),
									self.controller_do.addListener(FWDMSPController.SCRUB_PLAYLIST_ITEM, self.controllerPlaylistItemScrubbHandler),
                  self.controller_do.addListener(FWDMSPController.STOP_TO_SCRUB, self.controllerStopToScrubbHandler),
                  self.controller_do.addListener(FWDMSPController.SHOW_CATEGORIES, self.showCategoriesHandler),
                  self.controller_do.addListener(FWDMSPController.SHOW_PLAYLIST, self.showPlaylistHandler),
                  self.controller_do.addListener(FWDMSPController.HIDE_PLAYLIST, self.hidePlaylistHandler),
                  self.controller_do.addListener(FWDMSPController.ENABLE_LOOP, self.enableLoopHandler),
                  self.controller_do.addListener(FWDMSPController.DISABLE_LOOP, self.disableLoopHandler),
                  self.controller_do.addListener(FWDMSPController.ENABLE_SHUFFLE, self.enableShuffleHandler),
                  self.controller_do.addListener(FWDMSPController.DISABLE_SHUFFLE, self.disableShuffleHandler),
                  self.controller_do.addListener(FWDMSPController.BUY, self.controllerButtonBuyHandler),
									self.controller_do.addListener(FWDMSPController.REPOST, self.repostHandler),
                  self.controller_do.addListener(FWDMSPController.SHOW_PLAYBACKRATE, self.showPlaybacrateWindowHandler),
                  self.controller_do.addListener(FWDMSPController.SHOW_ATOB, self.showAtobWindowHandler),
                  self.main_do.addChild(self.controller_do),
                  self.openInPopup_bl && self.data.showPlaylistsButtonAndPlaylists_bl && (self.controller_do.setPlaylistButtonState("selected"), self.controller_do.playlistButton_do && self.controller_do.playlistButton_do.disableForGood())
							},
              this.controllerOnPopupHandler = function() {
								self.popup()
							},
              this.controllerOnPlayHandler = function(e) {
								self.play()
							},
              this.controllerPlayNextHandler = function(e) {
								self.isPlaylistItemClicked_bl = !0,
									self.data.shuffle_bl ? self.playShuffle() : self.playNext()
							},
              this.controllerPlayPrevHandler = function(e) {
								self.isPlaylistItemClicked_bl = !0,
								self.data.shuffle_bl ? self.playShuffle() : self.playPrev()
							},
              this.controllerOnPauseHandler = function(e) {
								self.isPlaylistItemClicked_bl = !0,
									self.pause()
							},
              this.volumeStartToScrubbHandler = function(e) {
								self.playlist_do && self.playlist_do.showDisable()
							},
              this.volumeStopToScrubbHandler = function(e) {
								self.playlist_do && self.playlist_do.hideDisable()
							},
              this.controllerStartToScrubbHandler = function(e) {
								self.playlist_do && self.playlist_do.showDisable(),
								self.audioScreen_do.startToScrub()
							},
              this.controllerScrubbHandler = function(e) {
								self.audioScreen_do.scrub(e.percent)
							},
              this.controllerPlaylistItemScrubbHandler = function(e) {
								self.playlist_do && self.playlist_do.updateCurItemProgress(e.percent)
							},
              this.controllerStopToScrubbHandler = function(e) {
								self.playlist_do && self.playlist_do.hideDisable(),
								self.audioScreen_do.stopToScrub()
							},
              this.controllerChangeVolumeHandler = function(e) {
								self.setVolume(e.percent)
							},
              this.showCategoriesHandler = function(e) {
                self.showCategories(), self.controller_do.setCategoriesButtonState("selected")
							},
              this.showPlaylistHandler = function(e) {
                self.showPlaylist()
							},
              this.hidePlaylistHandler = function(e) {
                self.hidePlaylist()
							},
              this.enableLoopHandler = function(e) {
								self.data.loop_bl = !0, self.data.shuffle_bl = !1, self.controller_do.setLoopStateButton("selected"), self.controller_do.setShuffleButtonState("unselected")
							},
              this.disableLoopHandler = function(e) {
								self.data.loop_bl = !1, self.controller_do.setLoopStateButton("unselected")
							},
              this.enableShuffleHandler = function(e) {
								self.data.shuffle_bl = !0, self.data.loop_bl = !1, self.controller_do.setShuffleButtonState("selected"), self.controller_do.setLoopStateButton("unselected")
							},
              this.disableShuffleHandler = function(e) {
								self.data.shuffle_bl = !1, self.controller_do.setShuffleButtonState("unselected")
							},
              this.repostHandler = function(e) {
								console.log("Кнопка поделиться нажата!")
							},
              this.showPlaybacrateWindowHandler = function(e) {
								self.resizeHandler(),
                self.playbackRateWindow_do.show(),
								self.controller_do && !self.isMobile_bl && (self.controller_do.playbackRateButton_do.setSelectedState(), self.controller_do.playbackRateButton_do.isDisabled_bl = !0)
							},
              this.showAtobWindowHandler = function(e) {
								self.resizeHandler(),
                self.atb_do.positionAndResize(),
                self.atb_do.show(!0),
                self.controller_do && !self.isMobile_bl && (self.controller_do.atbButton_do.setSelectedState(), self.controller_do.atbButton_do.isDisabled_bl = !0)
							},
              this.controllerButtonBuyHandler = function() {
								self.buy();
							},
              this.setupAudioScreen = function() {
								FWDMSPAudioScreen.setPrototype(), self.audioScreen_do = new FWDMSPAudioScreen(self.data.volume, self.data.autoPlay_bl, self.data.loop_bl), self.audioScreen_do.addListener(FWDMSPAudioScreen.ERROR, self.audioScreenErrorHandler),
									self.audioScreen_do.addListener(FWDMSPAudioScreen.START, self.audioScreenSatrtHandler),
									self.audioScreen_do.addListener(FWDMSPAudioScreen.SAFE_TO_SCRUBB, self.audioScreenSafeToScrubbHandler),
                  self.audioScreen_do.addListener(FWDMSPAudioScreen.STOP, self.audioScreenStopHandler),
                  self.audioScreen_do.addListener(FWDMSPAudioScreen.PLAY, self.audioScreenPlayHandler),
                  self.audioScreen_do.addListener(FWDMSPAudioScreen.PAUSE, self.audioScreenPauseHandler),
									self.audioScreen_do.addListener(FWDMSPAudioScreen.UPDATE, self.audioScreenUpdateHandler),
									self.audioScreen_do.addListener(FWDMSPAudioScreen.UPDATE_TIME, self.audioScreenUpdateTimeHandler),
                  self.audioScreen_do.addListener(FWDMSPAudioScreen.LOAD_PROGRESS, self.audioScreenLoadProgressHandler),
                  self.audioScreen_do.addListener(FWDMSPAudioScreen.PLAY_COMPLETE, self.audioScreenPlayCompleteHandler),
                  self.useOnlyAPI_bl ? document.documentElement.appendChild(self.audioScreen_do.screen) : self.main_do.addChild(self.audioScreen_do)
							},
              this.audioScreenErrorHandler = function(e) {
								var t; - 1 == e.text.indexOf(">null<") && (t = FWDMSP.hasHTML5Audio ? e.text : e, self.main_do && self.main_do.addChild(self.info_do), self.info_do && self.info_do.showText(t), self.position_str == FWDMSP.POSITION_TOP && self.playlist_do && (self.info_do.setY(self.playlist_do.h), self.info_do.setHeight(self.controller_do.h)), self.hider && (self.hider.reset(), self.hider.stop()), self.dispatchEvent(FWDMSP.ERROR, {
									error: t
								}))
							},
              this.audioScreenSatrtHandler = function() {
								self.dispatchEvent(FWDMSP.START)
							},
              this.audioScreenSafeToScrubbHandler = function() {
								self.controller_do && self.controller_do.enableMainScrubber(), FWDMSPUtils.getCookie("FWDMSPusePP") && !self.playedOnceCP_bl && (self.setVolume(Number(FWDMSPUtils.getCookie("FWDMSPVolume"))), setTimeout(function() {
									self.scrub(Number(FWDMSPUtils.getCookie("FWDMSPpp")))
								}, 200)), self.playedOnceCP_bl = !0
							},
              this.audioScreenStopHandler = function(e) {
								self.main_do && self.main_do.contains(self.info_do) && self.main_do.removeChild(self.info_do), self.opener_do && self.opener_do.showPlayButton(), self.controller_do && (self.controller_do.showPlayButton(), self.controller_do.stopEqulizer(), self.controller_do.disableMainScrubber()), self.hider && (self.hider.reset(), self.hider.stop()), self.dispatchEvent(FWDMSP.STOP)
							},
              this.audioScreenPlayHandler = function() {
                (FWDMSP.keyboardCurInstance = self).controller_do && (self.controller_do.showPauseButton(),
																																			self.controller_do.startEqulizer()),
								self.opener_do && self.opener_do.showPauseButton(),
								self.playlist_do && self.playlist_do.setCurItemPauseState(),
								self.openInPopup_bl && setTimeout(function() {
									self.scrubbedFirstTimeInPopup_bl || self.scrub(self.lastPercentPlayed),
									self.scrubbedFirstTimeInPopup_bl = !0
								}, 600),
								self.hasStartedToPlay_bl || self.data.playlist_ar[self.id].startAtTime && self.scrubbAtTime(self.data.playlist_ar[self.id].startAtTime), setTimeout(function() {
									self.isPlaylistItemClicked_bl = !1
								}, 500),
								self.ppPplayedOnce = !0,
								self.hasStartedToPlay_bl = !0,
								self.dispatchEvent(FWDMSP.PLAY)
							},
							this.audioScreenPauseHandler = function() {
								self.isPlaying_bl = !1,
								self.opener_do && self.opener_do.showPlayButton(),
								self.largePlayButton_do && self.isFullScreen_bl && self.largePlayButton_do.show(),
								self.hider && (self.hider.reset(), self.hider.stop()),
								!FWDMSPUtils.isIphone && self.largePlayButton_do
								                      && self.isFullScreen_bl
																			&& (self.isMobile_bl || self.largePlayButton_do && self.isFullScreen_bl && self.largePlayButton_do.show()),
								self.showCursor(),
								self.controller_do && (self.controller_do.showPlayButton(),
																			 self.controller_do.stopEqulizer()),
								self.playlist_do && self.playlist_do.setCurItemPlayState(),
								self.dispatchEvent(FWDMSP.PAUSE)
							},
							this.audioScreenUpdateHandler = function(e) {
								var t;
								t = FWDMSP.hasHTML5Audio ? e.percent : e,
								self.controller_do && self.controller_do.updateMainScrubber(t),
								self.playlist_do && self.playlist_do.updateCurItemProgress(t),
								self.dispatchEvent(FWDMSP.UPDATE, {
									percent: t
								})
							},
							this.audioScreenUpdateTimeHandler = function(e, t) {
								if (self.prevSeconds != e.seconds && (self.totalTimePlayed += 1),
								    self.totalTimeInSeconds = e.totalTimeInSeconds,
										self.curTimeInSecond = e.seconds,
										self.totalTime = e.totalTime,
										self.curTime = e.curTime,
										self.prevSeconds = e.seconds,
										self.totalPercentPlayed = self.totalTimePlayed / e.totalTimeInSeconds,
										isFinite(self.totalPercentPlayed) || (self.totalPercentPlayed = 0),
										self.controller_do && !self.controller_do.isMainScrubberScrubbing_bl
										                   && self.atb_do
																			 && self.atb_do.isShowed_bl
																			 && !self.atb_do.scrub) {
																				 var o = self.totalTimeInSeconds * self.atb_do.pa,
																				 s = self.totalTimeInSeconds * self.atb_do.pb;
																				 self.prevCurTimeInSeconds != self.curTimeInSecond && (self.prevCurTimeInSeconds = self.curTimeInSecond,
																					                                                     self.curTimeInSecond < o ? self.scrub(self.atb_do.pa)
																																															                          : self.curTimeInSecond > s && self.scrub(self.atb_do.pa))
								}
								var i, n;
								FWDMSP.hasHTML5Audio ? (i = e.curTime, n = e.totalTime)
								                     : (i = e, (n = t).length > i.length && (i = parseInt(n.substring(0, 1)) - 1 + ":" + i)),
								self.controller_do && self.controller_do.updateTime(i, n),
								FWDMSPUtils.getSecondsFromString(self.data.playlist_ar[self.id].stopAtTime) <= e.seconds && self.stop(),
								5 < n.length ? self.totalDuration = FWDMSPUtils.getSecondsFromString(n)
								             : self.totalDuration = FWDMSPUtils.getSecondsFromString("00:" + n),
									self.dispatchEvent(FWDMSP.UPDATE_TIME, {
										curTime: i,
										totalTime: n
									})
							},
							this.audioScreenLoadProgressHandler = function(e) {
								FWDMSP.hasHTML5Audio ? self.controller_do && self.controller_do.updatePreloaderBar(e.percent)
								                     : self.controller_do && self.controller_do.updatePreloaderBar(e)
							},
							this.audioScreenPlayCompleteHandler = function() {
								self.data.playlist_ar
								FWDMSP.hasHTML5Audio && (self.data.loop_bl ? "hls_flash" == self.audioType_str
																													 ? setTimeout(function() {
																															self.scrub(0), self.resume()
																															}, 50)
																														: (self.scrub(0), self.play())
																														: self.data.shuffle_bl ? self.playShuffle() : 1 == self.playlist_do.items_ar.length
																														? (self.stop(), self.playlist_do && self.playlist_do.updateCurItemProgress(0))
																														: self.playNext()),
																														self.dispatchEvent(FWDMSP.PLAY_COMPLETE)
							},
							this.loadID3IfPlaylistDisabled = function() {
								var o = self.data.playlist_ar[self.id].source;
								"..." == self.data.playlist_ar[self.id].title && (o = o + "?rand=" + parseInt(99999999 * Math.random()), ID3.loadTags(o, function() {
									var e = self.data.playlist_ar[self.id],
										t = ID3.getAllTags(o);
									e.title = t.artist + " - " + t.title, e.titleText = e.title, self.controller_do.setTitle(e.title)
								}))
							},
							this.setSource = function(e) {
								if (self.stop(!0),
								    FWDMSPUtils.getCookie("FWDMSPusePP") && !self.playedOnceCP_bl
										                                     && self.setVolume(Number(FWDMSPUtils.getCookie("FWDMSPVolume"))))
										return self.main_do.addChild(self.info_do),
										self.info_do.showText(self.data.loggedInMessage_str),
									void(self.info_do.allowToRemove_bl = !1);

								else {
									if (e && (self.itemClicked = e),
											self.id < 0 ? self.id = 0 : self.id > self.totalAudio - 1 && (self.id = self.totalAudio - 1),
											self.audioPath = self.data.playlist_ar[self.id].source,
											self.isShoutcast_bl = self.data.playlist_ar[self.id].isShoutcast_bl,
											self.isIcecast_bl = self.data.playlist_ar[self.id].isIcecast_bl,
											self.data.shoutcastVersion = self.data.playlist_ar[self.id].shoutcastVersion,
											!self.isShoutcastLoaded_bl && self.isShoutcast_bl && self.prevAudioPath != self.audioPath)
											  return self.isShoutcastLoaded_bl = !0,
												self.playlist_do && self.playlist_do.activateItems(self.id, self.itemClicked),
												self.resizeHandler(),
												void self.data.getShoutcastRadioNameAndStream(self.audioPath);
									if (!self.isIcecastLoaded_bl && self.isIcecast_bl && self.prevAudioPath != self.audioPath)
									   return self.isIcecastLoaded_bl = !0,
									self.playlist_do && self.playlist_do.activateItems(self.id, self.itemClicked),
									self.resizeHandler(),
									void self.data.getIcecastRadioNameAndStream(self.audioPath);
									var t;
									if ((self.isShoutcast_bl || self.isIcecast_bl) && (self.audioPath = self.radioSource_str),
									    self.prevAudioPath = self.audioPath,
											self.data.playlist_ar[self.id].controlerThumbnailPath && self.controller_do.loadThumb(self.data.playlist_ar[self.id].controlerThumbnailPath),
											self.data.playlist_ar[self.id].title && self.controller_do.setTitle(self.data.playlist_ar[self.id].title),
											(self.isShoutcast_bl || self.isIcecast_bl) && (self.audioPath = self.radioSource_str),
											self.stop(),
											self.isShoutcast_bl = self.data.playlist_ar[self.id].isShoutcast_bl,
											self.isIcecast_bl = self.data.playlist_ar[self.id].isIcecast_bl,
											-1 != self.audioPath.indexOf("soundcloud.")
											&&
											-1 == self.audioPath.indexOf("https://api.soundcloud.") ? (self.data.getSoundcloudUrl(self.audioPath),
											                                                           self.isLoadingSoundcloudTrack_bl = !0,
																																								 self.audioType_str = FWDMSP.AUDIO)
																																							: (self.audioType_str = FWDMSP.AUDIO,
																																								 self.isLoadingSoundcloudTrack_bl = !1),
						          self.finalAudioPath_str = self.audioPath,
											FWDMSP.hasHTMLHLS || -1 == self.audioPath.indexOf(".m3u8") ? self.audioType_str = FWDMSP.AUDIO
											                                                           : self.audioType_str = FWDMSP.HLS,
											self.isMobile_bl ? self.largePlayButton_do && self.largePlayButton_do.hide()
											                 : self.largePlayButton_do && self.isFullScreen_bl && self.largePlayButton_do.show(),
											self.data.playlist_ar[self.id].atb && !self.isATBJsLoaded_bl)
											 return (t = document.createElement("script")).src = self.data.mainFolderPath_str + "java/FWDMSPATB.js",
										document.head.appendChild(t),
										t.onerror = function() {
											self.main_do.addChild(self.info_do),
												self.info_do.showText('A to B plugin js file named <font color="#FF0000">FWDMSPATB.js</font> file.'),
												self.preloader_do && self.preloader_do.hide()
										}, void(t.onload = function() {
											self.isATBJsLoaded_bl = !0, self.setupAtbWindow(),
												self.setSource(self.audioPath)
										});

									self.audioScreen_do.setSource(self.audioPath),
									(self.data.autoPlay_bl || self.isPlaylistItemClicked_bl) && self.play(),
									!Boolean("true" == FWDMSPUtils.getCookie("FWDMSPppPlay")) || self.isMobile_bl || self.ppPplayedOnce || self.play();
									self.controller_do.stopEqulizer(),
										self.controller_do.setTitle(self.data.playlist_ar[self.id].title), null == self.data.playlist_ar[self.id].duration ? self.controller_do.updateTime("00:00", "00:00") : self.controller_do.updateTime("00:00", FWDMSP.formatTotalTime(self.data.playlist_ar[self.id].duration)), self.controller_do.loadThumb(self.data.playlist_ar[self.id].thumbPath), self.playlist_do ? self.playlist_do.activateItems(self.id, self.itemClicked) : self.loadID3IfPlaylistDisabled(),
										self.setPlaybackRate(self.data.defaultPlaybackRate)
								}
							},
							this.setupClickScreen = function() {
								self.dumyClick_do = new FWDMSPDisplayObject("div"),
								self.dumyClick_do.getStyle().width = "100%",
								self.dumyClick_do.getStyle().height = "100%",
								FWDMSPUtils.isIE && (self.dumyClick_do.setBkColor("#00FF00"),
																		 self.dumyClick_do.setAlpha(1e-5)),
								self.dumyClick_do.screen.addEventListener ? self.dumyClick_do.screen.addEventListener("click", self.playPauseClickHandler)
								                                          : self.dumyClick_do.screen.attachEvent && self.dumyClick_do.screen.attachEvent("onclick", self.playPauseClickHandler)
							},
							this.playPauseClickHandler = function(e) {
								2 != e.button && (self.disableClick_bl || (self.firstTapPlaying_bl = self.isPlaying_bl))
							},
							this.addDoubleClickSupport = function() {
								!self.isMobile_bl && self.dumyClick_do.screen.addEventListener ? (self.dumyClick_do.screen.addEventListener("mousedown", self.onFirstDown), FWDMSPUtils.isIEWebKit && self.dumyClick_do.screen.addEventListener("dblclick", self.onSecondDown))
								                                                               : self.isMobile_bl
																																							 ? self.dumyClick_do.screen.addEventListener("touchstart", self.onFirstDown)
																																							 : self.dumyClick_do.screen.addEventListener && self.dumyClick_do.screen.addEventListener("mousedown", self.onFirstDown)
							},
							this.onFirstDown = function(e) {
								if (2 != e.button) {
									self.isFullscreen_bl && e.preventDefault && e.preventDefault();
									var t = FWDMSPUtils.getViewportMouseCoordinates(e);
									self.firstTapX = t.screenX,
									self.firstTapY = t.screenY,
									self.firstTapPlaying_bl = self.isPlaying_bl,
									FWDMSPUtils.isIEWebKit || (self.isMobile_bl ? (self.dumyClick_do.screen.addEventListener("touchstart", self.onSecondDown),
									                                               self.dumyClick_do.screen.removeEventListener("touchstart", self.onFirstDown))
																															: self.dumyClick_do.screen.addEventListener && (self.dumyClick_do.screen.addEventListener("mousedown", self.onSecondDown),
																															  																							self.dumyClick_do.screen.removeEventListener("mousedown", self.onFirstDown)),
																						clearTimeout(self.secondTapId_to),
																						self.secondTapId_to = setTimeout(self.doubleTapExpired, 250))
								}
							},
							this.doubleTapExpired = function() {
								clearTimeout(self.secondTapId_to),
									self.isMobile_bl ? (self.dumyClick_do.screen.removeEventListener("touchstart", self.onSecondDown),
									                    self.dumyClick_do.screen.addEventListener("touchstart", self.onFirstDown))
																	 : self.dumyClick_do.screen.addEventListener && (self.dumyClick_do.screen.removeEventListener("mousedown", self.onSecondDown),
																	 																								 self.dumyClick_do.screen.addEventListener("mousedown", self.onFirstDown))
							},
							this.onSecondDown = function(e) {
								e.preventDefault && e.preventDefault();
								var t, o, s = FWDMSPUtils.getViewportMouseCoordinates(e);
								FWDMSPUtils.isIEWebKit && (self.firstTapPlaying_bl = self.isPlaying_bl),
								e.touches && 1 != e.touches.length || (t = Math.abs(s.screenX - self.firstTapX),
								o = Math.abs(s.screenY - self.firstTapY),
								self.isMobile_bl && (10 < t || 10 < o) || !self.isMobile_bl && (2 < t || 2 < o) || (self.switchFullScreenOnDoubleClick(),
								FWDMSPUtils.isIEWebKit || (self.firstTapPlaying_bl ? self.play() : self.pause())))
							},
							this.hiderHideHandler = function() {
								FWDMSPUtils.isIphone
							},
							this.hiderHideCompleteHandler = function() {},

							this.setupDisableClick = function() {
								self.disableClick_do = new FWDMSPDisplayObject("div")
							},
							this.disableClick = function() {
								self.disableClick_bl = !0,
									clearTimeout(self.disableClickId_to),
									self.disableClick_do && (self.disableClick_do.getStyle().width = "5000px",
									self.disableClick_do.getStyle().height = "5000px"),
									self.disableClickId_to = setTimeout(function() {
										self.disableClick_do && (self.disableClick_do.setWidth(0), self.disableClick_do.setHeight(0)),
											self.disableClick_bl = !1
									}, 500)
							},
							this.disableFullScreenOnMobileHandler = function(e) {
								e.preventDefault && e.preventDefault()
							},
							this.addMainDoToTheOriginalParent = function() {
								self.isFullScreen_bl && (document.removeEventListener && (document.removeEventListener("fullscreenchange", self.onFullScreenChange),
								                        																	document.removeEventListener("mozfullscreenchange", self.onFullScreenChange),
																																					document.removeEventListener("webkitfullscreenchange", self.onFullScreenChange),
																																					document.removeEventListener("MSFullscreenChange", self.onFullScreenChange)),
																			  self.isFullScreen_bl = !1,
																				self.isEmbedded_bl || (FWDMSPUtils.isIEAndLessThen9 ? document.documentElement.style.overflow = "auto" : document.documentElement.style.overflow = "visible", self.main_do.getStyle().position = "relative"),
																				self.controller_do.setOverflow("hidden"),
																				self.controller_do.mainHolder_do.setOverflow("hidden"),
																				self.opener_do && self.opener_do.setVisible(!0),
																				document.documentElement.style.overflow = "visible",
																				self.main_do.getStyle().zIndex = 0,
																				self.playlist_do && (self.playlist_do.setVisible(!0),
																														 self.playlist_do.ascDscButton_do && self.playlist_do.ascDscButton_do.setAlpha(1)),
																				self.hideFullScreenButtonAndOverlay(!1),
																				self.fullScreenButtonOverlay_do.setVisible(!0),
																				self.checkShowFullScreenButtonHitTest(),
																				self.largePlayButton_do && self.largePlayButton_do.hide(),
																				self.hider && (self.hider.reset(), self.hider.stop()),
																				FWDMSP.setInstancesInvisible(this, !0),
																				self.resizeHandler(!0),
																				window.scrollTo(self.lastX, self.lastY),
																				FWDMSPUtils.isIE || setTimeout(function() {
																					window.scrollTo(self.lastX, self.lastY)
																				}, 150),
																				self.isMobile_bl && window.removeEventListener("touchmove", self.disableFullScreenOnMobileHandler))
							},
							this.onFullScreenChange = function(e) {
								document.fullScreen || document.msFullscreenElement
								                    || document.mozFullScreen
																		|| document.webkitIsFullScreen
																		|| document.msieFullScreen
																		|| (self.fullScreenButton_do.setButtonState(1),
																		    self.addMainDoToTheOriginalParent(),
																				self.isFullScreen_bl = !1,
																				self.resizeHandler(!0))
							},
							this.hideCursor = function() {
								document.documentElement.style.cursor = "none",
								self.dumyClick_do && (self.dumyClick_do.getStyle().cursor = "none"),
								document.getElementsByTagName("body")[0].style.cursor = "none"
							},
							this.showCursor = function() {
								document.documentElement.style.cursor = "auto",
								document.getElementsByTagName("body")[0].style.cursor = "auto",
								self.dumyClick_do && (self.dumyClick_do.getStyle().cursor = "auto")
							},
							this.showPlayer = function() {
								self.isAPIReady_bl && (self.controller_do.isShowed_bl = !0,
									                     self.opener_do.showCloseButton(),
																			 self.setStageContainerFinalHeightAndPosition(self.animate_bl),
																			 self.playlist_do && (clearTimeout(self.disablePlaylistForAWhileId_to),
																			 self.disablePlaylistForAWhileId_to = setTimeout(function() {
																				 self.playlist_do.hideDisable()
																			 }, 500),
																			 self.playlist_do.showDisable()))
							},
							this.hidePlayer = function() {
								self.isAPIReady_bl && (self.controller_do.isShowed_bl = !1,
									                     self.opener_do.showOpenButton(),
																			 self.setStageContainerFinalHeightAndPosition(self.animate_bl))
							},
							this.loadPlaylist = function(e) {
								self.isAPIReady_bl && self.data.prevId != e
								                   && (self.catId = e,
																		   self.categories_do && (self.categories_do.id = self.catId),
																			 self.id = 0,
																			 self.catId < 0 ? self.catId = 0 : self.catId > self.data.totalCategories - 1 && (self.catId = self.data.totalCategories - 1),
																			 self.useDeepLinking_bl ? FWDAddress.setValue(self.instanceName_str + "?catid=" + self.catId + "&trackid=" + self.id) : self.loadInternalPlaylist(),
																			 self.data.playlist_ar)
							},
							this.playNext = function() {
								self.isAPIReady_bl && self.isPlaylistLoaded_bl
								                   && (self.data.showPlayListButtonAndPlaylist_bl ? self.playlist_do.items_ar[self.playlist_do.curItem_do.sortId + 1]
																		                                              ? self.id = self.playlist_do.items_ar[self.playlist_do.curItem_do.sortId + 1].id
																																									: self.id = self.playlist_do.items_ar[0].id
																																									: (self.id++, self.id < 0 ? self.id = self.totalAudio - 1 : self.id > self.totalAudio - 1 && (self.id = 0)),
																		  self.useDeepLinking_bl ? FWDAddress.setValue(self.instanceName_str + "?catid=" + self.catId + "&trackid=" + self.id)
																			                       : (self.setSource(), self.play()),
																      self.prevId = self.id,
																			self.data.playlist_ar)
							},
							this.playPrev = function() {
								self.isAPIReady_bl && self.isPlaylistLoaded_bl
								                   && (self.data.showPlayListButtonAndPlaylist_bl ? self.playlist_do.items_ar[self.playlist_do.curItem_do.sortId - 1]
																		                                              ? self.id = self.playlist_do.items_ar[self.playlist_do.curItem_do.sortId - 1].id
																																									: self.id = self.playlist_do.items_ar[self.totalAudio - 1].id
																																									: (self.id--, self.id < 0 ? self.id = self.totalAudio - 1
																																										                        : self.id > self.totalAudio - 1 && (self.id = 0)),
								 self.useDeepLinking_bl ? FWDAddress.setValue(self.instanceName_str + "?catid=" + self.catId + "&trackid=" + self.id)
								                        : (self.setSource(), self.changeHLS_bl = !0, self.audioType_str != FWDMSP.HLS && self.play()),
								 self.prevId = self.id,
								 self.data.playlist_ar)
							}, this.playShuffle = function() {
								if (self.isAPIReady_bl && self.isPlaylistLoaded_bl) {
									self.isPlaylistItemClicked_bl = !0;
									for (var e = parseInt(Math.random() * self.data.playlist_ar.length); e == self.id;) e = parseInt(Math.random() * self.data.playlist_ar.length);
									self.id = e, self.id < 0 ? self.id = self.totalAudio - 1 : self.id > self.totalAudio - 1 && (self.id = 0), self.useDeepLinking_bl ? FWDAddress.setValue(self.instanceName_str + "?catid=" + self.catId + "&trackid=" + self.id) : (self.setSource(), self.changeHLS_bl = !0, self.audioType_str != FWDMSP.HLS && self.play()), self.prevId = self.id, self.data.playlist_ar
								}
							}, this.playSpecificTrack = function(e, t) {
								self.isAPIReady_bl && self.isPlaylistLoaded_bl
								                   && (self.isPlaylistItemClicked_bl = !0,
									                     self.catId = e,
																			 self.id = t,
																			 self.catId < 0 ? self.catId = 0 : self.catId > self.data.totalCategories - 1 && (self.catId = self.data.totalCategories - 1),
																			 self.id < 0 && (self.id = 0),
																			 self.useDeepLinking_bl ? FWDAddress.setValue(self.instanceName_str + "?catid=" + self.catId + "&trackid=" + self.id)
																			                        : (self.setSource(), self.play()),
																			 self.prevId = self.id,
																			 self.data.playlist_ar )
							},
							this.play = function() {
								if (self.isAPIReady_bl && self.isPlaylistLoaded_bl && !self.isLoadingSoundcloudTrack_bl) {
										FWDMSP.pauseAllAudio(self),
										self.audioScreen_do && self.audioScreen_do.play()
								}
							},
							this.resume = function() {
								self.isAPIReady_bl
							},
							this.pause = function() {
								self.isAPIReady_bl && self.isPlaylistLoaded_bl
								                   && (self.isPlaylistItemClicked_bl = !0,
																		   FWDMSP.hasHTML5Audio && self.audioScreen_do && self.audioScreen_do.pause())
							},
							this.stop = function(e) {
								self.isAPIReady_bl && (e || (self.isIcecastLoaded_bl = !1, self.isShoutcastLoaded_bl = !1),
								                       self.isRadioLoaded_bl = !1,
																			 self.hasStartedToPlay_bl = !1,
																			 self.isShoutcast_bl = !1,
																			 self.isIcecast_bl = !1,
																			 self.atb_do && self.atb_do.hide(!0),
																			 self.opener_do && self.opener_do.showPlayButton(),
																			 self.playlist_do && (self.playlist_do.setCurItemPlayState(), self.playlist_do.updateCurItemProgress(0)),
																			 self.controller_do && self.controller_do.ttm && self.controller_do.ttm.hide(),
																			 self.showCursor(),
																			 self.audioType_str == FWDMSP.hasHTML5Audio && self.audioScreen_do.stop(),
																				self.controller_do && self.controller_do.disableAtbButton(),
																				self.setPlaybackRate(self.data.defaultPlaybackRate),
																				self.hasHlsPlayedOnce_bl = !1,
																				self.isSafeToScrub_bl = !1,
																				self.hlsState = void 0,
																				self.changeHLS_bl = !1)
							},
							this.startToScrub = function() {
								self.isAPIReady_bl && self.isPlaylistLoaded_bl && self.audioScreen_do.startToScrub()
							},
							this.stopToScrub = function() {
								self.isAPIReady_bl && self.isPlaylistLoaded_bl && self.flashObject.stopToScrub()
							},
							this.scrub = function(e) {
								self.isAPIReady_bl && self.isPlaylistLoaded_bl
								                   && (isNaN(e) || (e < 0 ? e = 0 : 1 < e && (e = 1),
																	                  self.audioType_str == FWDMSP.hasHTML5Audio,
																										self.audioScreen_do && self.audioScreen_do.scrub(e)
																									))
							},
							this.setPlaybackRate = function(e) {
								self.isAPIReady_bl && (self.data.defaultPlaybackRate = e,
									                     self.audioType_str == FWDMSP.AUDIO && self.audioScreen_do,
																			 self.audioScreen_do.setPlaybackRate(e))
							},
							this.setVolume = function(e) {
								self.isAPIReady_bl && (self.volume = e,
									                     self.controller_do && self.controller_do.updateVolume(e, !0),
																			 self.audioType_str != FWDMSP.hasHTML5Audio && self.audioScreen_do && self.audioScreen_do.setVolume(e))
							},
							this.showCategories = function() {
								self.isAPIReady_bl && self.categories_do
								                   && (self.categories_do.show(self.catId),
																	     self.customContextMenu_do && self.customContextMenu_do.updateParent(self.categories_do),
																			 self.controller_do.setCategoriesButtonState("selected"))
							},
							this.hideCategories = function() {
								self.isAPIReady_bl && self.categories_do
								                   && (self.categories_do.hide(),
																	     self.controller_do.setCategoriesButtonState("unselected"))
							},
							this.showPlaylist = function() {
								self.isAPIReady_bl && (self.playlist_do && (self.isPlaylistShowed_bl = !0,
									                                          self.playlist_do.show(!0),
																														self.controller_do.setPlaylistButtonState("selected"),
																														clearTimeout(self.disablePlaylistForAWhileId_to),
																														self.disablePlaylistForAWhileId_to = setTimeout(function() {
																															self.playlist_do.hideDisable()
																														}, 150),
																														self.playlist_do.showDisable()),
																		self.setStageContainerFinalHeightAndPosition(self.animate_bl))
							},
							this.hidePlaylist = function() {
								self.isAPIReady_bl && (self.playlist_do && (self.isPlaylistShowed_bl = !1,
									                                          self.playlist_do.hide(),
																														self.controller_do.setPlaylistButtonState("unselected"),
																														self.setStageContainerFinalHeightAndPosition(self.animate_bl)))
							},
							this.getIsAPIReady = function() {
								return self.isAPIReady_bl
							},
							this.getCatId = function() {
								return self.catId
							},
							this.getTrackId = function() {
								return self.id
							},
							this.getTrackTitle = function() {
								if (self.isAPIReady_bl) return self.data.playlist_ar[self.id].title
							},
							this.getThumbnailPath = function() {
								return self.data.playlist_ar[self.id].thumbPath
							},
							this.getCurrentTime = function() {
								if (self.isAPIReady_bl)
								  return self.audioType_str == FWDMSP.AUDIO, self.audioScreen_do.getCurrentTime()
							},
							this.getDuration = function() {
								if (self.isAPIReady_bl)
								  return self.audioType_str == FWDMSP.AUDIO ,self.audioScreen_do.getDuration()
							},
							this.scrubbAtTime = function(e) {
								self.isAPIReady_bl && e
								                   && (-1 != String(e).indexOf(":") && (e = FWDMSPUtils.getSecondsFromString(e)),
																	    self.audioType_str == FWDMSP.AUDIO, self.audioScreen_do && self.audioScreen_do.scrubbAtTime(e))
							},
							this.buy = function(pId) {
								if (self.isAPIReady_bl) {
									null == pId && (pId = self.id);
									var track_id = self.data.playlist_ar[pId].buy;
									return track_id
								}
							},
							//this.getTrackPk = function() {
							//	var track_id = self.data.playlist_ar[self.id].buy;
							//},
							this.playFirstTrack = function() {
								self.playSpecificTrack(self.catId, 0)
							},
							this.playLastTrack = function() {
								self.playSpecificTrack(self.catId, self.data.playlist_ar.length - 1)
							},
							this.addTrack = function(e, t, o, s, i, n, l) {
								self.isReady_bl || (self.useDeepLinking_bl && (location.hash = self.instanceName_str + "?catid=" + self.catId + "&trackid=" + (self.id + 1)),
								                                               self.playlist_do && self.playlist_do.addTrack(e, t, o, s, i, n, l),
																															 self.useDeepLinking_bl && (location.hash = self.instanceName_str + "?catid=" + self.catId + "&trackid=0"))
							},
							this.updateHEXColors = function(e, t) {
								self.isAPIReady_bl && (self.controller_do.updateHEXColors(e, t),
								                       self.largePlayButton_do && self.largePlayButton_do.updateHEXColors(e, "#FFFFFF"),
																			 self.playlist_do && self.playlist_do.updateHEXColors(e, t),
																			 self.opener_do && self.opener_do.updateHEXColors(e, "#FFFFFF"),
																			 self.playbackRateWindow_do && self.playbackRateWindow_do.updateHEXColors(e, t))
							},
							this.addListener = function(e, t) {
								if (this.listeners) {
									if (null == e) throw Error("type is required.");
									if ("object" == typeof e) throw Error("type must be of type String.");
									if ("function" != typeof t) throw Error("listener must be of type Function.");
									var o = {};
									o.type = e,
									o.listener = t,
									(o.target = this).listeners.events_ar.push(o)
								}
							},
							this.dispatchEvent = function(e, t) {
								if (null != this.listeners) {
									if (null == e) throw Error("type is required.");
									if ("object" == typeof e) throw Error("type must be of type String.");
									for (var o = 0, s = this.listeners.events_ar.length; o < s; o++)
										if (this.listeners.events_ar[o].target === this && this.listeners.events_ar[o].type === e) {
											if (t)
												for (var i in t) this.listeners.events_ar[o][i] = t[i];
											this.listeners.events_ar[o].listener.call(this, this.listeners.events_ar[o])
										}
								}
							},
							this.removeListener = function(e, t) {
								if (null == e) throw Error("type is required.");
								if ("object" == typeof e) throw Error("type must be of type String.");
								if ("function" != typeof t) throw Error("listener must be of type Function." + e);
								for (var o = 0, s = this.listeners.events_ar.length; o < s; o++)
									if (this.listeners.events_ar[o].target === this && this.listeners.events_ar[o].type === e && this.listeners.events_ar[o].listener === t) {
										this.listeners.events_ar.splice(o, 1);
										break
									}
							},
							self.useYoutube_bl && (-1 != location.protocol.indexOf("file:") && FWDMSPUtils.isIE
							                       || -1 != location.protocol.indexOf("file:") && FWDMSPUtils.isOpera))
							return self.stageContainer = FWDMSPUtils.getChildById(props.parentId),
							self.setupMainDo(),
							self.setupInfo(),
							self.main_do.addChild(self.info_do),
							self.info_do.allowToRemove_bl = !1,
							self.info_do.showText('This Youtube:"no"</font>.'),
							void self.resizeHandler();
						setTimeout(FWDMSP.checkIfHasYoutube, 100)
					}
				} else alert("FWDMSP instance name is requires please make sure that the instanceName parameter exsists and it's value is uinique.");

				function handleMediaError() {
					if (autoRecoverError) {
						var e = performance.now();
						!recoverDecodingErrorDate || 3e3 < e - recoverDecodingErrorDate ? (recoverDecodingErrorDate = performance.now(), self.HLSError_str = "try to recover media Error ...", self.hlsJS.recoverMediaError()) : !recoverSwapAudioCodecDate || 3e3 < e - recoverSwapAudioCodecDate ? (recoverSwapAudioCodecDate = performance.now(), self.HLSError_str = "try to swap Audio Codec and recover media Error ...", self.hlsJS.swapAudioCodec(), self.hlsJS.recoverMediaError()) : self.HLSError_str = "cannot recover, last media error recovery failed ..."
					}
					self.HLSError_str && (console && console.log(self.HLSError_str), self.main_do.addChild(self.info_do), self.info_do.showText(self.HLSError_str), self.resizeHandler())
				}
			},
			TZ, UZ, f$, g$;

			FWDMSP.checkIfHasYoutube = function() {
						setTimeout(FWDMSP.setupAllInstances, 500)
				},

		FWDMSP.setupAllInstances = function() {
				if (!FWDMSP.areInstancesCreated_bl) {
					var e = FWDMSPUtils.getUrlArgs(window.location.search).MSPInstanceName;
					"pause" != FWDMSP.audioStartBehaviour && "stop" != FWDMSP.audioStartBehaviour && "none" != FWDMSP.audioStartBehaviour && (FWDMSP.audioStartBehaviour = "pause"),
						FWDMSPUtils.isMobile_bl && (FWDMSP.audioStartBehaviour = "stop"), FWDMSP.areInstancesCreated_bl = !0;
					var t, o = FWDMSP.instaces_ar.length,
						s = !1;
					if (e)
						for (var i = 0; i < o; i++)
							if ((t = FWDMSP.instaces_ar[i]).props.instanceName == e) return void(FWDMSP.isEmbedded_bl = !0);
					for (i = 0; i < o; i++) t = FWDMSP.instaces_ar[i],
						FWDMSP.instaces_ar[i - 1], t.init(), s && (t.data.autoPlay_bl = !1), 1 == t.data.autoPlay_bl && (s = !0)
				}
			}, FWDMSP.setInstancesInvisible = function(e, t) {
				for (var o = 0; o < FWDMSP.instaces_ar.length; o++) inst = FWDMSP.instaces_ar[o], e == inst || t ? (inst.stageContainer.style.overflow = "visible", inst.stageContainer.style.width = "100%") : (inst.stageContainer.style.overflow = "hidden", inst.stageContainer.style.width = "0px")
			}, FWDMSP.setPrototype = function() {
				FWDMSP.prototype = new FWDMSPEventDispatcher
			}, FWDMSP.pauseAllAudio = function(e) {
				for (var t, o = FWDMSP.instaces_ar.length, s = 0; s < o; s++)(t = FWDMSP.instaces_ar[s]) != e && t.stop()
			}, FWDMSP.stopAllAudio = function(e) {
				for (var t, o = FWDMSP.instaces_ar.length, s = 0; s < o; s++)(t = FWDMSP.instaces_ar[s]) != e && t.stop()
			}, FWDMSP.hasHTML5Audio = (TZ = document.createElement("audio"), UZ = !1, TZ.canPlayType && (UZ = Boolean("probably" == TZ.canPlayType("audio/mpeg") || "maybe" == TZ.canPlayType("audio/mpeg"))), !!self.isMobile_bl || UZ),
			FWDMSP.getAudioFormats = function() {
				var e = document.createElement("audio");
				if (e.canPlayType) {
					var t = "",
						o = [];
					return "probably" != e.canPlayType("audio/mpeg") && "maybe" != e.canPlayType("audio/mpeg") || (t += ".mp3"), "probably" != e.canPlayType("audio/ogg") && "maybe" != e.canPlayType("audio/ogg") || (t += ".ogg"), "probably" != e.canPlayType("audio/mp4") && "maybe" != e.canPlayType("audio/mp4") || (t += ".webm"), (o = t.split(".")).shift(), e = null, o
				}
			}(), FWDMSP.hasCanvas = Boolean(document.createElement("canvas")), FWDMSP.formatTotalTime = function(e) {
				if ("string" == typeof e && -1 != e.indexOf(":")) return e;
				e /= 1e3;
				var t = Math.floor(e / 3600),
					o = e % 3600,
					s = Math.floor(o / 60),
					i = o % 60,
					n = Math.ceil(i);
				return s = 10 <= s ? s : "0" + s, n = 10 <= n ? n : "0" + n, isNaN(n) ? "00:00/00:00" : 0 < t ? t + ":" + s + ":" + n : s + ":" + n
			}, FWDMSP.getAudioFormats = function() {
				var e = document.createElement("audio");
				if (e.canPlayType) {
					var t = "",
						o = [];
					return "probably" != e.canPlayType("audio/mpeg") && "maybe" != e.canPlayType("audio/mpeg") || (t += ".mp3"), "probably" != e.canPlayType("audio/ogg") && "maybe" != e.canPlayType("audio/ogg") || (t += ".ogg"), "probably" != e.canPlayType("audio/mp4") && "maybe" != e.canPlayType("audio/mp4") || (t += ".webm"), (o = t.split(".")).shift(), e = null, o
				}
			}(),

			FWDMSP.instaces_ar = [],
			FWDMSP.CENTER = "center",
			FWDMSP.LEFT = "left",
			FWDMSP.RIGHT = "right",
			FWDMSP.AUDIO = "audio",
			FWDMSP.POPUP = "popup",
			FWDMSP.POSITION_TOP = "positionTop",
			FWDMSP.POSITION_BOTTOM = "positionBottom",
			FWDMSP.READY = "ready",
			FWDMSP.START = "start",
			FWDMSP.START_TO_LOAD_PLAYLIST = "startToLoadPlaylist",
			FWDMSP.LOAD_PLAYLIST_COMPLETE = "loadPlaylistComplete",
			FWDMSP.STOP = "stop",
			FWDMSP.PLAY = "play",
			FWDMSP.PAUSE = "pause",
			FWDMSP.UPDATE = "update",
			FWDMSP.UPDATE_TIME = "updateTime",
			FWDMSP.ERROR = "error",
			FWDMSP.PLAY_COMPLETE = "playComplete",
			FWDMSP.PLAYLIST_LOAD_COMPLETE = "onPlayListLoadComplete",
			window.FWDMSP = FWDMSP
	}(window),
	function(window) {
		var FWDMSPAudioData = function(props, playListElement, parent) {
			var self = this,
			prototype = FWDMSPAudioData.prototype;
			this.xhr = null,
			this.playlist_ar = null,
			this.dlIframe = null,
			this.mainPreloader_img = null,
			this.bk_img = null,
			this.thumbnail_img = null,
			this.separator1_img = null,
			this.separator2_img = null,
			this.prevN_img = null,
			this.playN_img = null,
			this.pauseN_img = null,
			this.nextN_img = null,
			this.popupN_img = null,
			this.mainScrubberBkLeft_img = null,
			this.mainScrubberBkRight_img = null,
			this.mainScrubberDragLeft_img = null,
			this.mainScrubberLine_img = null,
			this.mainScrubberLeftProgress_img = null,
			this.volumeScrubberBkLeft_img = null,
			this.volumeScrubberBkRight_img = null,
			this.volumeScrubberDragLeft_img = null,
			this.volumeScrubberLine_img = null,
			this.volumeD_img = null,
			this.progressLeft_img = null,
			this.titleBarLeft_img = null,
			this.titleBarRigth_img = null,
			this.openerAnimation_img = null,
			this.openTopN_img = null,
			this.openTopS_img = null,
			this.openBottomN_img = null,
			this.openBottomS_img = null,
			this.closeN_img = null,
			this.closeS_img = null,
			this.openerPauseN_img = null,
			this.openerPauseS_img = null,
			this.openerPlayN_img = null,
			this.openerPlayS_img = null,
			this.categoriesN_img = null,
			this.replayN_img = null,
			this.playlistN_img = null,
			this.shuffleN_img = null,
			this.repostN_img = null,
			this.titlebarAnimBkPath_img = null,
			this.titlebarLeftPath_img = null,
			this.titlebarRightPath_img = null,
			this.soundAnimationPath_img = null,
			this.controllerBk_img = null,
			this.playlistItemBk1_img = null,
			this.playlistItemBk2_img = null,
			this.playlistSeparator_img = null,
			this.playlistScrBkTop_img = null,
			this.playlistScrBkMiddle_img = null,
			this.playlistScrBkBottom_img = null,
			this.playlistScrDragTop_img = null,
		  this.playlistScrDragMiddle_img = null,
			this.playlistScrDragBottom_img = null,
			this.playlistScrLines_img = null,
			this.playlistScrLinesOver_img = null,
			this.playlistPlayButtonN_img = null,
			this.playlistItemGrad1_img = null,
			this.playlistItemGrad2_img = null,
			this.playlistItemProgress1_img = null,
			this.playlistItemProgress2_img = null,
			this.catThumbBk_img = null,
			this.catThumbTextBk_img = null,
			this.catNextN_img = null,
			this.catNextS_img = null,
			this.catNextD_img = null,
		  this.catPrevN_img = null,
			this.catPrevS_img = null,
			this.catPrevD_img = null,
			this.catCloseN_img = null,
			this.catCloseS_img = null,
			this.categories_el = null,
			this.scs_el = null,
			this.props_obj = props,
			this.skinPaths_ar = [],
			this.images_ar = [],
			this.cats_ar = [],
			this.scClientId_str = props.soundCloudAPIKey || "0aff03b3b79c2ac02fd2283b300735bd",
			this.flashPath_str = null,
			this.proxyPath_str = null,
			this.proxyFolderPath_str = null,
			this.mailPath_str = null,
			this.skinPath_str = null,
			this.controllerBkPath_str = null,
			this.thumbnailBkPath_str = null,
			this.playlistIdOrPath_str = null,
			this.mainScrubberBkMiddlePath_str = null,
			this.volumeScrubberBkMiddlePath_str = null,
			this.mainScrubberDragMiddlePath_str = null,
			this.volumeScrubberDragMiddlePath_str = null,
			this.timeColor_str = null,
			this.titleColor_str = null,
			this.progressMiddlePath_str = null,
			this.sourceURL_str = null,
			this.titlebarBkMiddlePattern_str = null,
			this.playlistPlayButtonN_str = null,
			this.playlistPlayButtonS_str = null,
			this.playlistPauseButtonN_str = null,
			this.playlistPauseButtonS_str = null,
			this.trackTitleNormalColor_str = null,
			this.trackTitleSelected_str = null,
			this.trackDurationColor_str = null,
			this.categoriesId_str = null,
			this.thumbnailSelectedType_str = null,
			this.openerAlignment_str = null,
			this.prevId = -1,
			this.totalCats = 0,
			this.countLoadedSkinImages = 0,
			this.volume = 1,
			this.startSpaceBetweenButtons = 0,
			this.spaceBetweenButtons = 0,
			this.mainScrubberOffsetTop = 0,
			this.spaceBetweenMainScrubberAndTime = 0,
			this.startTimeSpace = 0,
			this.scrubbersOffsetWidth = 0,
			this.scrubbersOffestTotalWidth = 0,
			this.volumeButtonAndScrubberOffsetTop = 0,
			this.maxPlaylistItems = 0,
			this.separatorOffsetOutSpace = 0,
			this.separatorOffsetInSpace = 0,
			this.lastButtonsOffsetTop = 0,
			this.allButtonsOffsetTopAndBottom = 0,
			this.controllerHeight = 0,
			this.titleBarOffsetTop = 0,
			this.scrubberOffsetBottom = 0,
			this.equlizerOffsetLeft = 0,
			this.nrOfVisiblePlaylistItems = 0,
			this.trackTitleOffsetLeft = 0,
			this.playPauseButtonOffsetLeftAndRight = 0,
			this.durationOffsetRight = 0,
			this.scrollbarOffestWidth = 0,
			this.resetLoadIndex = -1,
			this.startAtPlaylist = 0,
			this.startAtTrack = 0,
			this.totalCategories = 0,
			this.thumbnailMaxWidth = 0,
			this.buttonsMargins = 0,
			this.thumbnailMaxHeight = 0,
			this.horizontalSpaceBetweenThumbnails = 0,
			this.verticalSpaceBetweenThumbnails = 0,
			this.openerEqulizerOffsetLeft = 0,
			this.openerEqulizerOffsetTop = 0,
			this.countID3 = 0,
			this.JSONPRequestTimeoutId_to,
			this.showLoadPlaylistErrorId_to,
			this.dispatchPlaylistLoadCompleteWidthDelayId_to,
			this.loadImageId_to,
			this.loadPreloaderId_to,
			this.isPlaylistDispatchingError_bl = !1,
			this.allowToChangeVolume_bl = !0,
			this.showContextMenu_bl = !1,
			this.autoPlay_bl = !1,
			this.loop_bl = !1,
			this.shuffle_bl = !1,
			this.showLoopButton_bl = !1,
			this.showShuffleButton_bl = !1,
			this.showPlaylistsButtonAndPlaylists_bl = !1,
			this.showPlaylistsByDefault_bl = !1,
			this.showPlayListButtonAndPlaylist_bl = !1,
			this.showRepostButton_bl = !1,
			this.showPopupButton_bl = !1,
			this.animate_bl = !1,
			this.showControllerByDefault_bl = !1,
			this.showPlayListByDefault_bl = !1,
			this.isDataLoaded_bl = !1,
			this.useDeepLinking_bl = !1,
			this.showSoundCloudUserNameInTitle_bl = !1,
			this.showThumbnail_bl = !1,
			this.showSoundAnimation_bl = !1,
			this.expandControllerBackground_bl = !1,
			this.showPlaylistItemPlayButton_bl = !1,
			this.loadFromFolder_bl = !1,
			this.isMobile_bl = FWDMSPUtils.isMobile,
			this.hasPointerEvent_bl = FWDMSPUtils.hasPointerEvent,
			self.init = function() {
					self.parseProperties()
				},
				self.parseProperties = function() {
					if (this.addKeyboardSupport_bl = self.props_obj.addKeyboardSupport || "no",
							this.addKeyboardSupport_bl = "yes" == self.addKeyboardSupport_bl,
							self.useHEXColorsForSkin_bl = self.props_obj.useHEXColorsForSkin,
							self.useHEXColorsForSkin_bl = "yes" == self.useHEXColorsForSkin_bl,
							-1 != location.protocol.indexOf("file:") && (self.useHEXColorsForSkin_bl = !1),
							self.categoriesId_str = self.props_obj.playlistsId, self.categoriesId_str)
						if (self.mainFolderPath_str = self.props_obj.mainFolderPath, self.mainFolderPath_str)
							if (self.mainFolderPath_str.lastIndexOf("/") + 1 != self.mainFolderPath_str.length && (self.mainFolderPath_str += "/"),
							    self.skinPath_str = self.props_obj.skinPath, self.skinPath_str)
								if (self.skinPath_str.lastIndexOf("/") + 1 != self.skinPath_str.length && (self.skinPath_str += "/"),
								    self.skinPath_str = self.mainFolderPath_str + self.skinPath_str,
										self.flashPath_str = self.mainFolderPath_str,
										self.proxyPath_str = self.mainFolderPath_str,
										self.proxyFolderPath_str = self.mainFolderPath_str,
										self.mailPath_str = self.mainFolderPath_str,
										self.hlsPath_str = self.mainFolderPath_str,
										self.categories_el = document.getElementById(self.categoriesId_str),
										self.categories_el)
										{
									var e = FWDMSPUtils.getChildren(self.categories_el);
									if (self.totalCats = e.length, self.categories_el = document.getElementById(self.categoriesId_str), 0 != self.totalCats) {
										for (var t = 0; t < self.totalCats; t++) {
											var o = {};
											if (child = e[t], !FWDMSPUtils.hasAttribute(child, "data-source")) return void setTimeout(function() {
												null != self && self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
													text: "Attribute <font color='#FF0000'>data-source</font> is required in the categories html element at position <font color='#FF0000'>" + (t + 1)
												})
											}, 50);
											if (!FWDMSPUtils.hasAttribute(child, "data-thumbnail-path")) return void setTimeout(function() {
												null != self && self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
													text: "Attribute <font color='#FF0000'>data-thumbnail-path</font> is required in the categories html element at position <font color='#FF0000'>" + (t + 1)
												})
											}, 50);
											o.playlistsName = FWDMSPUtils.getAttributeValue(child, "data-playlist-name"), o.source = FWDMSPUtils.getAttributeValue(child, "data-source"), o.thumbnailPath = FWDMSPUtils.getAttributeValue(child, "data-thumbnail-path"), o.htmlContent = child.innerHTML, o.htmlText_str = child.innerText,
												self.cats_ar[t] = o
										}
										self.playlistBackgroundColor_str = self.props_obj.playlistBackgroundColor || "transparent",
										self.searchInputColor_str = self.props_obj.searchInputColor || "#FF0000",
										self.openerAlignment_str = self.props_obj.openerAlignment || "right",
										"right" != self.openerAlignment_str && "left" != self.openerAlignment_str && (self.openerAlignment_str = "right"),
										self.totalCategories = self.cats_ar.length,
										self.playlistIdOrPath_str = self.props_obj.playlistIdOrPath || void 0,
										self.timeColor_str = self.props_obj.timeColor || "#FF0000",
										self.playbackRateWindowTextColor_str = self.props_obj.playbackRateWindowTextColor || "#FF0000",
										self.showPlaylistsSearchInput_bl = self.props_obj.showPlaylistsSearchInput,
										self.showPlaylistsSearchInput_bl = "yes" == self.showPlaylistsSearchInput_bl,
										self.trackTitleNormalColor_str = self.props_obj.trackTitleNormalColor || "#FF0000",
										self.trackTitleSelected_str = self.props_obj.trackTitleSelectedColor || "#FF0000",
										self.trackDurationColor_str = self.props_obj.trackDurationColor || "#FF0000",
										self.titleColor_str = self.props_obj.titleColor || "#FF0000",
										self.thumbnailSelectedType_str = self.props_obj.thumbnailSelectedType || "opacity",
										"blackAndWhite" != self.thumbnailSelectedType_str && "threshold" != self.thumbnailSelectedType_str && "opacity" != self.thumbnailSelectedType_str && (self.thumbnailSelectedType_str = "opacity"),
										(self.isMobile_bl || FWDMSPUtils.isIEAndLessThen9) && (self.thumbnailSelectedType_str = "opacity"),
										"file:" == document.location.protocol && (self.thumbnailSelectedType_str = "opacity"),
										self.searchInputColor_str = self.props_obj.searchInputColor || "#FF0000",
										self.playlistBackgroundColor_str = self.props_obj.playlistBackgroundColor || "transparent",
										self.startAtPlaylist = self.props_obj.startAtPlaylist || 0,
										isNaN(self.startAtPlaylist) && (self.startAtPlaylist = 0),
										self.startAtPlaylist < 0 ? self.startAtPlaylist = 0 : self.startAtPlaylist > self.totalCats - 1 && (self.startAtPlaylist = self.totalCats - 1),
										self.startAtRandomTrack_bl = self.props_obj.startAtRandomTrack,
										self.startAtRandomTrack_bl = "no" != self.startAtRandomTrack_bl,
										self.startAtTrack = self.props_obj.startAtTrack || 0,
										self.volume = self.props_obj.volume,
										self.volume || (self.volume = 1),
										isNaN(self.volume) && (volume = 1), 1 < self.volume ? self.volume = 1 : self.volume < 0 && (self.volume = 0),
										self.searchBarHeight = self.props_obj.searchBarHeight || 50,
										self.buttonsMargins = self.props_obj.buttonsMargins || 0,
										self.thumbnailMaxWidth = self.props_obj.thumbnailMaxWidth || 330,
										self.thumbnailMaxHeight = self.props_obj.thumbnailMaxHeight || 330,
										self.horizontalSpaceBetweenThumbnails = self.props_obj.horizontalSpaceBetweenThumbnails,
										null == self.horizontalSpaceBetweenThumbnails && (self.horizontalSpaceBetweenThumbnails = 40),
										self.verticalSpaceBetweenThumbnails = parseInt(self.props_obj.verticalSpaceBetweenThumbnails),
										null == self.verticalSpaceBetweenThumbnails && (self.verticalSpaceBetweenThumbnails = 40),
										self.openerEqulizerOffsetLeft = self.props_obj.openerEqulizerOffsetLeft || 0,
										self.openerEqulizerOffsetTop = self.props_obj.openerEqulizerOffsetTop || 0,
										self.inputSearchTextOffsetTop = self.props_obj.inputSearchTextOffsetTop,
										self.inputSearchOffsetLeft = self.props_obj.inputSearchOffsetLeft,
										self.startSpaceBetweenButtons = self.props_obj.startSpaceBetweenButtons || 0,
										self.spaceBetweenButtons = self.props_obj.spaceBetweenButtons || 0,
										self.mainScrubberOffsetTop = self.props_obj.mainScrubberOffsetTop || 100,
										self.spaceBetweenMainScrubberAndTime = self.props_obj.spaceBetweenMainScrubberAndTime,
										self.startTimeSpace = self.props_obj.startTimeSpace,
										self.scrubbersOffsetWidth = self.props_obj.scrubbersOffsetWidth || 0,
										self.scrubbersOffestTotalWidth = self.props_obj.scrubbersOffestTotalWidth || 0,
										self.volumeButtonAndScrubberOffsetTop = self.props_obj.volumeButtonAndScrubberOffsetTop || 0,
										self.spaceBetweenVolumeButtonAndScrubber = self.props_obj.spaceBetweenVolumeButtonAndScrubber || 0,
										self.volumeScrubberOffestWidth = self.props_obj.volumeScrubberOffestWidth || 0,
										self.scrubberOffsetBottom = self.props_obj.scrubberOffsetBottom || 0,
										self.equlizerOffsetLeft = self.props_obj.equlizerOffsetLeft || 0,
										self.nrOfVisiblePlaylistItems = self.props_obj.nrOfVisiblePlaylistItems || 0,
										self.trackTitleOffsetLeft = self.props_obj.trackTitleOffsetLeft || 0,
										self.playPauseButtonOffsetLeftAndRight = self.props_obj.playPauseButtonOffsetLeftAndRight || 0,
										self.durationOffsetRight = self.props_obj.durationOffsetRight || 0,
										self.scrollbarOffestWidth = self.props_obj.scrollbarOffestWidth || 0,
										self.maxPlaylistItems = self.props_obj.maxPlaylistItems || 200,
										self.controllerHeight = self.props_obj.controllerHeight || 200,
										self.titleBarOffsetTop = self.props_obj.titleBarOffsetTop || 0,
										self.separatorOffsetInSpace = self.props_obj.separatorOffsetInSpace || 0,
										self.lastButtonsOffsetTop = self.props_obj.lastButtonsOffsetTop || 0,
										self.allButtonsOffsetTopAndBottom = self.props_obj.allButtonsOffsetTopAndBottom || 0,
										self.separatorOffsetOutSpace = self.props_obj.separatorOffsetOutSpace || 0,
										self.volumeScrubberWidth = self.props_obj.volumeScrubberWidth || 10,
										200 < self.volumeScrubberWidth && (self.volumeScrubberWidth = 200),
										self.secondaryLabelsColor_str = self.props_obj.secondaryLabelsColor || "#FF0000",
										self.mainLabelsColor_str = self.props_obj.mainLabelsColor || "#FF0000",
										self.borderColor_str = self.props_obj.borderColor || "#FF0000",
										self.textColor_str = self.props_obj.textColor_str || "#FF0000",
										self.inputBackgroundColor_str = self.props_obj.inputBackgroundColor || "#FF0000",
										self.inputColor_str = self.props_obj.inputColor || "#FF0000",
										self.showContextMenu_bl = self.props_obj.showContextMenu,
										self.showContextMenu_bl = "no" != self.showContextMenu_bl,
										self.autoPlay_bl = self.props_obj.autoPlay,
										self.autoPlay_bl = "yes" == self.autoPlay_bl,
										self.loop_bl = self.props_obj.loop,
										self.loop_bl = "yes" == self.loop_bl,
										self.shuffle_bl = self.props_obj.shuffle,
										self.shuffle_bl = "yes" == self.shuffle_bl,
										self.useContinuousPlayback_bl = self.props_obj.useContinuousPlayback,
										self.useContinuousPlayback_bl = "yes" == self.useContinuousPlayback_bl,
										self.isLoggedIn_bl = self.props_obj.isLoggedIn,
										self.isLoggedIn_bl = "yes" == self.isLoggedIn_bl,
										self.useDeepLinking_bl = self.props_obj.useDeepLinking,
										self.useDeepLinking_bl = "yes" == self.useDeepLinking_bl,
										self.showSoundCloudUserNameInTitle_bl = self.props_obj.showSoundCloudUserNameInTitle,
										self.showSoundCloudUserNameInTitle_bl = "yes" == self.showSoundCloudUserNameInTitle_bl,
										self.showThumbnail_bl = self.props_obj.showThumbnail,
										self.showThumbnail_bl = "yes" == self.showThumbnail_bl,
										self.showNextAndPrevButtons_bl = self.props_obj.showNextAndPrevButtons,
										self.showNextAndPrevButtons_bl = "yes" == self.showNextAndPrevButtons_bl,
										self.showLoopButton_bl = self.props_obj.showLoopButton,
										self.showLoopButton_bl = "no" != self.props_obj.showLoopButton,
										self.showPlayListButtonAndPlaylist_bl = self.props_obj.showPlayListButtonAndPlaylist,
										self.showPlayListButtonAndPlaylist_bl = "no" != self.showPlayListButtonAndPlaylist_bl,
										FWDMSPUtils.isAndroid && self.showPlayListButtonAndPlaylist_bl && "no" == self.props_obj.showPlayListOnAndroid && (self.showPlayListButtonAndPlaylist_bl = !1),
										self.rightClickContextMenu_str = self.props_obj.rightClickContextMenu || "developer",
										test = "developer" == self.rightClickContextMenu_str || "disabled" == self.rightClickContextMenu_str || "default" == self.rightClickContextMenu_str,
											test || (self.rightClickContextMenu_str = "developer"), self.showPlaylistsButtonAndPlaylists_bl = self.props_obj.showPlaylistsButtonAndPlaylists,
											self.showPlaylistsButtonAndPlaylists_bl = "no" != self.showPlaylistsButtonAndPlaylists_bl,
											self.showPlaylistsByDefault_bl = self.props_obj.showPlaylistsByDefault,
											self.showPlaylistsByDefault_bl = "yes" == self.showPlaylistsByDefault_bl,
											self.showShuffleButton_bl = self.props_obj.showShuffleButton,
											self.showShuffleButton_bl = "no" != self.showShuffleButton_bl,
											self.randomizePlaylist_bl = self.props_obj.randomizePlaylist,
											self.randomizePlaylist_bl = "yes" == self.randomizePlaylist_bl,
											self.showBuyButton_bl = self.props_obj.showBuyButton,
											self.showBuyButton_bl = "no" != self.showBuyButton_bl,
											self.showRepostButton_bl = self.props_obj.showRepostButton,
											self.showRepostButton_bl = "no" != self.showRepostButton_bl,
											self.showPopupButton_bl = self.props_obj.showPopupButton,
											self.showPopupButton_bl = "no" != self.showPopupButton_bl,
											self.showOpenerPlayPauseButton_bl = self.props_obj.showOpenerPlayPauseButton,
											self.showOpenerPlayPauseButton_bl = "no" != self.showOpenerPlayPauseButton_bl,
											self.showPlaylistItemBuyButton_bl = self.props_obj.showPlaylistItemBuyButton,
											self.showPlaylistItemBuyButton_bl = "no" != self.showPlaylistItemBuyButton_bl,
											self.normalButtonsColor_str = self.props_obj.normalHEXButtonsColor || "#FF0000",
											self.selectedButtonsColor_str = self.props_obj.selectedHEXButtonsColor || "#00FF00",
											self.showOpener_bl = self.props_obj.showOpener,
											self.showOpener_bl = "no" != self.showOpener_bl,
											self.showTracksNumbers_bl = self.props_obj.showTracksNumbers,
											self.showTracksNumbers_bl = "yes" == self.showTracksNumbers_bl,
											self.disableScrubber_bl = self.props_obj.disableScrubber,
											self.disableScrubber_bl = "yes" == self.disableScrubber_bl,
											self.showPlaybackRateButton_bl = self.props_obj.showPlaybackRateButton,
											self.showPlaybackRateButton_bl = "yes" == self.showPlaybackRateButton_bl,
											self.playTrackAfterPlaylistLoad_bl = self.props_obj.playTrackAfterPlaylistLoad,
											self.playTrackAfterPlaylistLoad_bl = "yes" == self.playTrackAfterPlaylistLoad_bl,
											self.atbTimeBackgroundColor = self.props_obj.atbTimeBackgroundColor || "transparent",
											self.atbTimeTextColorNormal = self.props_obj.atbTimeTextColorNormal || "#888888",
											self.atbTimeTextColorSelected = self.props_obj.atbTimeTextColorSelected || "#FFFFFF",
											self.atbButtonTextNormalColor = self.props_obj.atbButtonTextNormalColor || "#888888",
											self.atbButtonTextSelectedColor = self.props_obj.atbButtonTextSelectedColor || "#FFFFFF",
											self.atbButtonBackgroundNormalColor = self.props_obj.atbButtonBackgroundNormalColor || "#FFFFFF",
											self.atbButtonBackgroundSelectedColor = self.props_obj.atbButtonBackgroundSelectedColor || "#000000",
											self.defaultPlaybackRate = parseFloat(self.props_obj.defaultPlaybackRate.toFixed(1)) || 1,
											isNaN(self.defaultPlaybackRate) && (self.defaultPlaybackRate = 1),
											self.defaultPlaybackRate < .5 ? self.defaultPlaybackRate = .5 : 2 < self.defaultPlaybackRate && (self.defaultPlaybackRate = 2),
											self.animate_bl = self.props_obj.animate,
											self.animate_bl = "yes" == self.animate_bl,
											self.showControllerByDefault_bl = self.props_obj.showControllerByDefault,
											self.showControllerByDefault_bl = "no" != self.showControllerByDefault_bl,
											self.showPlayListByDefault_bl = self.props_obj.showPlayListByDefault,
											self.showPlayListByDefault_bl = "no" != self.showPlayListByDefault_bl,
											self.showSoundAnimation_bl = self.props_obj.showSoundAnimation,
											self.showSoundAnimation_bl = "yes" == self.showSoundAnimation_bl,
											self.expandControllerBackground_bl = self.props_obj.expandBackground,
											self.expandControllerBackground_bl = "yes" == self.expandControllerBackground_bl,
											self.showPlaylistItemPlayButton_bl = self.props_obj.showPlaylistItemPlayButton,
											self.showPlaylistItemPlayButton_bl = "no" != self.showPlaylistItemPlayButton_bl,
											self.addScrollBarMouseWheelSupport_bl = self.props_obj.addScrollBarMouseWheelSupport,
											self.addScrollBarMouseWheelSupport_bl = "no" != self.addScrollBarMouseWheelSupport_bl,
											self.usePlaylistsSelectBox_bl = self.props_obj.usePlaylistsSelectBox,
											self.usePlaylistsSelectBox_bl = "yes" == self.usePlaylistsSelectBox_bl,
											self.showPlaylistsSelectBoxNumbers_bl = self.props_obj.showPlaylistsSelectBoxNumbers,
											self.showPlaylistsSelectBoxNumbers_bl = "yes" == self.showPlaylistsSelectBoxNumbers_bl,
											self.mainSelectorBackgroundSelectedColor = self.props_obj.mainSelectorBackgroundSelectedColor || "#FFFFFF",
											self.mainSelectorTextNormalColor = self.props_obj.mainSelectorTextNormalColor || "#FFFFFF",
											self.mainSelectorTextSelectedColor = self.props_obj.mainSelectorTextSelectedColor || "#000000",
											self.mainButtonBackgroundNormalColor = self.props_obj.mainButtonBackgroundNormalColor || "#212021",
											self.mainButtonBackgroundSelectedColor = self.props_obj.mainButtonBackgroundSelectedColor || "#FFFFFF",
											self.mainButtonTextNormalColor = self.props_obj.mainButtonTextNormalColor || "#FFFFFF",
											self.mainButtonTextSelectedColor = self.props_obj.mainButtonTextSelectedColor || "#000000",
											self.showSearchBar_bl = self.props_obj.showSearchBar,
											self.showSearchBar_bl = "no" != self.showSearchBar_bl,
											self.showSortButtons_bl = self.props_obj.showSortButtons,
											self.showSortButtons_bl = "no" != self.showSortButtons_bl,
											self.preloaderPath_str = self.skinPath_str + "preloader.png",
											self.animationPath_str = self.skinPath_str + "equalizer.png",
											self.arrowN_str = self.skinPath_str + "combobox-arrow-normal.png",
											self.arrowS_str = self.skinPath_str + "combobox-arrow-selected.png",
											self.comboboxBk1_str = self.skinPath_str + "combobox-item-background1.png",
											self.comboboxBk2_str = self.skinPath_str + "combobox-item-background2.png",
											self.mainPreloader_img = new Image,
											self.mainPreloader_img.onerror = self.onSkinLoadErrorHandler,
											self.mainPreloader_img.onload = self.onPreloaderLoadHandler,
											self.mainPreloader_img.src = self.skinPath_str + "preloader.png",
											self.skinPaths_ar = [{
												img: self.controllerBk_img = new Image,
												src: self.skinPath_str + "controller-background.png"
											}, {
												img: self.separator1_img = new Image,
												src: self.skinPath_str + "separator.png"
											}, {
												img: self.separator2_img = new Image,
												src: self.skinPath_str + "separator.png"
											}, {
												img: self.prevN_img = new Image,
												src: self.skinPath_str + "prev-button.png"
											}, {
												img: self.playN_img = new Image,
												src: self.skinPath_str + "play-button.png"
											}, {
												img: self.pauseN_img = new Image,
												src: self.skinPath_str + "pause-button.png"
											}, {
												img: self.nextN_img = new Image,
												src: self.skinPath_str + "next-button.png"
											}, {
												img: self.popupN_img = new Image,
												src: self.skinPath_str + "popup-button.png"
											}, {
												img: self.buyN_img = new Image,
												src: self.skinPath_str + "add.svg"
											}, {
												img: self.mainScrubberBkLeft_img = new Image,
												src: self.skinPath_str + "scrubber-left-background.png"
											}, {
												img: self.mainScrubberBkRight_img = new Image,
												src: self.skinPath_str + "scrubber-right-background.png"
											}, {
												img: self.mainScrubberDragLeft_img = new Image,
												src: self.skinPath_str + "scrubber-left-drag.png"
											}, {
												img: self.volumeScrubberDragLeft_img = new Image,
												src: self.skinPath_str + "scrubber-left-drag.png"
											}, {
												img: self.mainScrubberLine_img = new Image,
												src: self.skinPath_str + "scrubber-line.png"
											}, {
												img: self.mainScrubberLeftProgress_img = new Image,
												src: self.skinPath_str + "progress-left.png"
											}, {
												img: self.volumeN_img = new Image,
												src: self.skinPath_str + "volume-icon.png"
											}, {
												img: self.categoriesN_img = new Image,
												src: self.skinPath_str + "categories-button.png"
											}, {
												img: self.openTopN_img = new Image,
												src: self.skinPath_str + "open-button-normal-top.png"
											}, {
												img: self.openBottomN_img = new Image,
												src: self.skinPath_str + "open-button-normal-bottom.png"
											}, {
												img: self.closeN_img = new Image,
												src: self.skinPath_str + "close-button-normal.png"
											}, {
												img: self.openerPauseN_img = new Image,
												src: self.skinPath_str + "open-pause-button-normal.png"
											}, {
												img: self.openerPlayN_img = new Image,
												src: self.skinPath_str + "open-play-button-normal.png"
											}, {
												img: self.replayN_img = new Image,
												src: self.skinPath_str + "replay-button.png"
											}, {
												img: self.playlistN_img = new Image,
												src: self.skinPath_str + "playlist-button.png"
											}, {
												img: self.shuffleN_img = new Image,
												src: self.skinPath_str + "shuffle-button.png"
											}, {
												img: self.repostN_img = new Image,
												src: self.skinPath_str + "share.png"
											}, {
												img: self.titlebarAnimBkPath_img = new Image,
												src: self.skinPath_str + "titlebar-equlizer-background.png"
											}, {
												img: self.titlebarLeftPath_img = new Image,
												src: self.skinPath_str + "titlebar-grad-left.png"
											}, {
												img: self.playbackRateNormal_img = new Image,
												src: self.skinPath_str + "playback-rate-normal.png"
											}, {
												img: self.soundAnimationPath_img = new Image,
												src: self.skinPath_str + "equalizer.png"
											}, {
												img: self.titleBarLeft_img = new Image,
												src: self.skinPath_str + "titlebar-left-pattern.png"
											}, {
												img: self.titleBarRigth_img = new Image,
												src: self.skinPath_str + "titlebar-right-pattern.png"
											}, {
												img: self.atbNPath_img = new Image,
												src: self.skinPath_str + "a-to-b-button.png"
											}], self.skinPaths_ar.push({
												img: self.fullScreenN_img = new Image,
												src: self.skinPath_str + "full-screen.png"
											}, {
												img: self.normalScreenN_img = new Image,
												src: self.skinPath_str + "normal-screen.png"
											}, {
												img: self.largePlayN_img = new Image,
												src: self.skinPath_str + "large-play.png"
											}), self.largePlayS_str = self.skinPath_str + "large-play-over.png",
											self.fullScreenS_str = self.skinPath_str + "full-screen-over.png",
											self.normalScreenS_str = self.skinPath_str + "normal-screen-over.png",
											self.atbSPath_str = self.skinPath_str + "a-to-b-button-over.png",
											self.playbackRateSelectedPath_str = self.skinPath_str + "playback-rate-selected.png",
											self.prevSPath_str = self.skinPath_str + "prev-button-over.png",
											self.playSPath_str = self.skinPath_str + "play-button-over.png",
											self.pauseSPath_str = self.skinPath_str + "pause-button-over.png",
											self.nextSPath_str = self.skinPath_str + "next-button-over.png",
											self.popupSPath_str = self.skinPath_str + "popup-button-over.png",
											self.controllerBkPath_str = self.skinPath_str + "controller-background.png",
											self.thumbnailBkPath_str = self.skinPath_str + "thumbnail-background.png",
											self.mainScrubberBkMiddlePath_str = self.skinPath_str + "scrubber-middle-background.png",
											self.mainScrubberDragMiddlePath_str = self.skinPath_str + "scrubber-middle-drag.png",
											self.volumeScrubberBkMiddlePath_str = self.skinPath_str + "scrubber-middle-background.png",
											self.volumeScrubberDragMiddlePath_str = self.skinPath_str + "scrubber-middle-drag.png",
											self.volumeSPath_str = self.skinPath_str + "volume-icon-over.png",
											self.volumeDPath_str = self.skinPath_str + "volume-icon-disabled.png",
											self.openerAnimationPath_str = self.skinPath_str + "equalizer.png",
											self.openTopSPath_str = self.skinPath_str + "open-button-selected-top.png",
											self.openBottomSPath_str = self.skinPath_str + "open-button-selected-bottom.png",
											self.closeSPath_str = self.skinPath_str + "close-button-selected.png",
											self.openerPauseS_str = self.skinPath_str + "open-pause-button-selected.png",
											self.openerPlayS_str = self.skinPath_str + "open-play-button-selected.png",
											self.progressMiddlePath_str = self.skinPath_str + "progress-middle.png",
											self.buySPath_str = self.skinPath_str + "add_active.svg",
											self.showPlayListButtonAndPlaylist_bl && (self.skinPaths_ar.push({
												img: self.playlistItemBk1_img = new Image,
												src: self.skinPath_str + "playlist-item-background1.png"
											}, {
												img: self.playlistItemBk2_img = new Image,
												src: self.skinPath_str + "playlist-item-background2.png"
											}, {
												img: self.playlistSeparator_img = new Image,
												src: self.skinPath_str + "playlist-separator.png"
											}, {
												img: self.playlistScrBkTop_img = new Image,
												src: self.skinPath_str + "playlist-scrollbar-background-top.png"
											}, {
												img: self.playlistScrDragTop_img = new Image,
												src: self.skinPath_str + "playlist-scrollbar-drag-bottom.png"
											}, {
												img: self.playlistScrLines_img = new Image,
												src: self.skinPath_str + "playlist-scrollbar-lines.png"
											}, {
												img: self.playlistPlayButtonN_img = new Image,
												src: self.skinPath_str + "playlist-play-button.png"
											}, {
												img: self.playlistItemGrad1_img = new Image,
												src: self.skinPath_str + "playlist-item-grad1.png"
											}, {
												img: self.playlistItemGrad2_img = new Image,
												src: self.skinPath_str + "playlist-item-grad2.png"
											}, {
												img: self.playlistItemProgress1_img = new Image,
												src: self.skinPath_str + "playlist-item-progress1.png"
											}, {
												img: self.playlistItemProgress2_img = new Image,
												src: self.skinPath_str + "playlist-item-progress2.png"
											}, {
												img: self.playlistBuyButtonN_img = new Image,
												src: self.skinPath_str + "playlist_add.svg"
											}),
											self.scrBkMiddlePath_str = self.skinPath_str + "playlist-scrollbar-background-middle.png",
											self.scrBkBottomPath_str = self.skinPath_str + "playlist-scrollbar-background-bottom.png",
											self.scrDragMiddlePath_str = self.skinPath_str + "playlist-scrollbar-drag-middle.png",
											self.scrDragBottomPath_str = self.skinPath_str + "playlist-scrollbar-drag-top.png",
											self.scrLinesSPath_str = self.skinPath_str + "playlist-scrollbar-lines-over.png",
											self.playlistBuyButtonS_str = self.skinPath_str + "playlist_add_active.svg",
											self.playlistPlayButtonN_str = self.skinPath_str + "playlist-play-button.png",
											self.playlistPlayButtonS_str = self.skinPath_str + "playlist-play-button-over.png",
											self.playlistPauseButtonN_str = self.skinPath_str + "playlist-pause-button.png",
											self.playlistPauseButtonS_str = self.skinPath_str + "playlist-pause-button-over.png"),
											self.showPlaylistsButtonAndPlaylists_bl && (self.skinPaths_ar.push({
												img: self.catNextN_img = new Image,
												src: self.skinPath_str + "categories-next-button.png"
											}, {
												img: self.catPrevN_img = new Image,
												src: self.skinPath_str + "categories-prev-button.png"
											}, {
												img: self.catCloseN_img = new Image,
												src: self.skinPath_str + "categories-close-button.png"
											}, {
												img: new Image,
												src: self.skinPath_str + "categories-background.png"
											}), self.catBkPath_str = self.skinPath_str + "categories-background.png",
											self.catThumbBkPath_str = self.skinPath_str + "categories-thumbnail-background.png",
											self.catThumbBkTextPath_str = self.skinPath_str + "categories-thumbnail-text-backgorund.png",
											self.catNextSPath_str = self.skinPath_str + "categories-next-button-over.png",
											self.catNextDPath_str = self.skinPath_str + "categories-next-button-disabled.png",
											self.catPrevSPath_str = self.skinPath_str + "categories-prev-button-over.png",
											self.catPrevDPath_str = self.skinPath_str + "categories-prev-button-disabled.png",
											self.catCloseSPath_str = self.skinPath_str + "categories-close-button-over.png"),
											self.showSearchBar_bl && (self.skinPaths_ar.push({
												img: self.sortAN_img = new Image,
												src: self.skinPath_str + "sort-alphabetical-button.png"
											}, {
												img: self.sortNN_img = new Image,
												src: self.skinPath_str + "sort-numerical-button.png"
											}, {
												img: self.ascendingN_img = new Image,
												src: self.skinPath_str + "ascending-button.png"
											}, {
												img: self.decendingN_img = new Image,
												src: self.skinPath_str + "descending-button.png"
											}),
											self.sortASPath_str = self.skinPath_str + "sort-alphabetical-button-over.png",
											self.sortNSPath_str = self.skinPath_str + "sort-numerical-button-over.png",
											self.ascendingSpath_str = self.skinPath_str + "ascending-button-over.png",
											self.decendingSpath_str = self.skinPath_str + "descending-button-over.png",
											self.inputArrowPath_str = self.skinPath_str + "input-arrow.png"),
											self.categoriesSPath_str = self.skinPath_str + "categories-button-over.png",
										  self.replaySPath_str = self.skinPath_str + "replay-button-over.png";
										  self.skinPath_str;
										  self.playlistSPath_str = self.skinPath_str + "playlist-button-over.png",
											self.shuffleSPath_str = self.skinPath_str + "shuffle-button-over.png",
											self.repostSPath_str = self.skinPath_str + "share-over.png",
											self.animationPath_str = self.skinPath_str + "equalizer.png",
											self.titlebarBkMiddlePattern_str = self.skinPath_str + "titlebar-middle-pattern.png",
											self.embedWindowClosePathS_str = self.skinPath_str + "embed-close-button-over.png",
											self.showPlaybackRateButton_bl && (self.skinPaths_ar.push({
												img: self.playbackRateWindowClooseN_img = new Image,
												src: self.skinPath_str + "embed-close-button.png"
											}, {
												img: self.closeClooseN_img = new Image,
												src: self.skinPath_str + "embed-close-button.png"
											}),
											self.playbackRateClosePathS_str = self.skinPath_str + "embed-close-button-over.png"),
											self.totalGraphics = self.skinPaths_ar.length,
											self.loadSkin()
									} else setTimeout(function() {
										null != self && (errorMessage_str = "At least one category is required!", self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
											text: errorMessage_str
										}))
									}, 50)
								}
					else setTimeout(function() {
						null != self && (errorMessage_str = "The html element with id <font color='#FF0000'>" + self.categoriesId_str + "</font> is not found in the DOM, this html element represents the player categories.!", self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
							text: errorMessage_str
						}))
					}, 50);
					else setTimeout(function() {
						null != self && (errorMessage_str = "The <font color='#FF0000'>skinPath</font> property is not defined in the constructor function!", self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
							text: errorMessage_str
						}))
					}, 50);
					else setTimeout(function() {
						null != self && (errorMessage_str = "The <font color='#FF0000'>mainFolderPath</font> property is not defined in the constructor function!", self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
							text: errorMessage_str
						}))
					}, 50);
					else setTimeout(function() {
						null != self && (errorMessage_str = "The <font color='#FF0000'>playlistsId</font> property is not defined in the constructor function!", self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
							text: errorMessage_str
						}))
					}, 50)
				}, this.onPreloaderLoadHandler = function() {
					setTimeout(function() {
						self.dispatchEvent(FWDMSPAudioData.PRELOADER_LOAD_DONE)
					}, 50)
				}, self.loadSkin = function() {
					for (var e, t, o = 0; o < self.totalGraphics; o++) e = self.skinPaths_ar[o].img, t = self.skinPaths_ar[o].src, e.onload = self.onSkinLoadHandler, e.onerror = self.onSkinLoadErrorHandler, e.src = t
				}, this.onSkinLoadHandler = function(e) {
					self.countLoadedSkinImages++, self.countLoadedSkinImages == self.totalGraphics && setTimeout(function() {
						self.dispatchEvent(FWDMSPAudioData.SKIN_LOAD_COMPLETE)
					}, 50)
				}, self.onSkinLoadErrorHandler = function(e) {
					message = FWDMSPUtils.isIEAndLessThen9 ? "Graphics image not found!" : "The skin icon with label <font color='#FF0000'>" + e.target.src + "</font> can't be loaded, check path!",
						window.console && console.log(e);
					var t = {
						text: message
					};
					setTimeout(function() {
						self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, t)
					}, 50)
				}, self.showPropertyError = function(e) {
					self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
						text: "The property called <font color='#FF0000'>" + e + "</font> is not defined."
					})
				}, this.loadPlaylist = function(e) {
					if (!self.isPlaylistDispatchingError_bl) {
						clearTimeout(self.dispatchPlaylistLoadCompleteWidthDelayId_to);
						var t = self.cats_ar[e].source;
						if (!t)
						   return self.isPlaylistDispatchingError_bl = !0,
							 void(showLoadPlaylistErrorId_to = setTimeout(function() {
								 self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
									 text: "<font color='#FF0000'>loadPlaylist()</font> - Please specify an html elementid, podcast link, soudcloud link or xml path"
								 }),
								 self.isPlaylistDispatchingError_bl = !1
						}, 50));
						if (!isNaN(t))
						   return self.isPlaylistDispatchingError_bl = !0,
							 void(showLoadPlaylistErrorId_to = setTimeout(function() {
								 self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
									 text: "<font color='#FF0000'>loadPlaylist()</font> - The parameter must be of type string!"
								 }),
								 self.isPlaylistDispatchingError_bl = !1
						}, 50));
						self.closeData(),

						self.parseDOMPlaylist(t),
						self.prevId = e
					}
				}, this.loadSoundCloudList = function(e) {
					if (!self.isPlaylistDispatchingError_bl) {
						self.closeXHR(), self.sourceURL_str = e, -1 != self.sourceURL_str.indexOf("likes") && (self.sourceURL_str = self.sourceURL_str.replace(/\/likes$/, "/favorites")),
							e = -1 == self.sourceURL_str.indexOf("api.soundcloud.") ? "https://api.soundcloud.com/resolve?format=json&url=" + self.sourceURL_str + "&limit=100&client_id=" + self.scClientId_str : self.sourceURL_str + "?format=json&client_id=" + self.scClientId_str + "&limit=100", self.loadFromFolder_bl = !1, self.sourceURL_str = e, self.xhr = new XMLHttpRequest, self.xhr.onreadystatechange = self.ajaxOnLoadHandler, self.xhr.onerror = self.ajaxOnErrorHandler;
						try {
							self.xhr.open("GET", self.sourceURL_str, !0), self.xhr.send()
						} catch (e) {
							var t = e;
							e && e.message && (t = e.message),
								self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
									text: "Soundclud playlist can't be loaded! <font color='#FF0000'>" + self.sourceURL_str + "</font>. " + t
								})
						}
					}
				}, this.JSONPSoundcloudRequestTimeoutError = function() {
					self.isPlaylistDispatchingError_bl = !0,
						showLoadPlaylistErrorId_to = setTimeout(function() {
							self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
								text: "Error loading soundcloud url!<font color='#FF0000'>" + self.sourceURL_str + "</font>"
							}), self.isPlaylistDispatchingError_bl = !1
						}, 50)
				}, this.getSoundcloudUrl = function(e) {
					if (!self.isPlaylistDispatchingError_bl) {
						try {
							self.closeJsonPLoader()
						} catch (e) {}
						self.sourceURL_str = e, -1 != self.sourceURL_str.indexOf("likes") && (self.sourceURL_str = self.sourceURL_str.replace(/\/likes$/, "/favorites")), e = "https://api.soundcloud.com/resolve?format=json&url=" + self.sourceURL_str + "&limit=100&client_id=" + self.scClientId_str, self.isSCTrack = !0,
							self.loadFromFolder_bl = !1, self.sourceURL_str = e, self.xhr = new XMLHttpRequest, self.xhr.onreadystatechange = self.ajaxOnLoadHandler, self.xhr.onerror = self.ajaxOnErrorHandler;
						try {
							self.xhr.open("GET", self.sourceURL_str, !0), self.xhr.send()
						} catch (e) {
							var t = e;
							e && e.message && (t = e.message),
								self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
									text: "Soundclud track can't be loaded! <font color='#FF0000'>" + self.sourceURL_str + "</font>. " + t
								})
						}
					}
				}, this.parseSoundCloudURL = function(e) {
					var t;
					self.closeJsonPLoader(), e.stream_url ? (t = e.stream_url + "?consumer_key=" + self.scClientId_str, self.dispatchEvent(FWDMSPAudioData.SOUNDCLOUD_TRACK_READY, {
						source: t
					})) : self.loadSoundcloudTrackError()
				}, this.loadSoundcloudTrackError = function() {
					self.closeJsonPLoader(), self.isPlaylistDispatchingError_bl = !0,
						showLoadPlaylistErrorId_to = setTimeout(function() {
							self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
								text: "Error loading soundcloud track url!<font color='#FF0000'>" + self.sourceURL_str + "</font>"
							}), self.isPlaylistDispatchingError_bl = !1
						}, 50)
				}, this.closeJsonPLoader = function() {
					self.isSCTrack = !1, self.isLoadingShoutcast_bl = !1, self.isLoadingIcecast_bl = !1, clearTimeout(self.JSONPRequestTimeoutId_to),
						clearTimeout(self.updateRadioTitleId_to);
					try {
						self.icecastxmlHttp.abort()
					} catch (e) {}
					self.icecastxmlHttp = null;
					try {
						self.shoutcastxmlHttp.abort()
					} catch (e) {}
					self.shoutcastxmlHttp = null;
					try {
						document.documentElement.removeChild(self.scs_el)
					} catch (e) {}
					try {
						document.documentElement.removeChild(self.scs2_el)
					} catch (e) {}
					try {
						document.documentElement.removeChild(self.scs3_el)
					} catch (e) {}
				}, this.startToUpdateIcecastName = function() {
					self.closeJsonPLoader(), self.getIcecastRadioNameAndStream(self.sourceURL_str, !0)
				}, this.getIcecastRadioNameAndStream = function(e, t) {
					self.isPlaylistDispatchingError_bl || (self.sourceURL_str = e, "/" == self.sourceURL_str.substr(self.sourceURL_str.length - 1) && (self.sourceURL_str = self.sourceURL_str.substr(0, self.sourceURL_str.length - 1)), "/" != self.sourceURL_str.substr(self.sourceURL_str.length - 1) && (self.sourceURL_str += "/"), e = "https://cors-anywhere.herokuapp.com/" + self.sourceURL_str + "status-json.xsl", self.originalSourceURL_str = self.sourceURL_str, self.icecastxmlHttp = new XMLHttpRequest, self.icecastxmlHttp.onreadystatechange = function() {
						4 == self.icecastxmlHttp.readyState && 200 == self.icecastxmlHttp.status && self.parseIcecastRadioURL(self.icecastxmlHttp.responseText)
					}, self.icecastxmlHttp.open("GET", e, !0), self.icecastxmlHttp.send(null), t || (self.JSONPRequestTimeoutId_to = setTimeout(self.parseRadioErrorURL, 5e3)))
				}, this.parseIcecastRadioURL = function(e) {
					if ("/" == self.sourceURL_str.substr(self.sourceURL_str.length - 1) && (self.sourceURL_str = self.sourceURL_str.substr(0, self.sourceURL_str.length - 1)), e = JSON.parse(e), self.closeJsonPLoader(), e.icestats.source[0]) var t = e.icestats.source[0].listenurl,
						o = e.icestats.source[0].title;
					else t = e.icestats.source.listenurl, o = e.icestats.source.title;
					if (o = o || "title not defined", e.icestats.source[0]) self.stationLabelClassName,
						self.stationClassName, e.icestats.source[0].server_name, self.genreLabelClassName, self.genreClassName,
						e.icestats.source[0].genre, self.currentListenersLabelClassName, self.currentListenersClassName, e.icestats.source[0].listeners,
						self.bitrateLabelClassName, self.bitrateClassName, e.icestats.source[0].bitrate;
					else self.stationLabelClassName, self.stationClassName, e.icestats.source.server_name, self.genreLabelClassName,
						self.genreClassName, e.icestats.source.genre, self.currentListenersLabelClassName, self.currentListenersClassName, e.icestats.source.listeners, self.bitrateLabelClassName, self.bitrateClassName, e.icestats.source.bitrate;
					self.dispatchEvent(FWDMSPAudioData.RADIO_TRACK_READY, {
						source: t,
						songTitle: o
					}), self.updateRadioTitleId_to = setTimeout(function() {
						parent.isIcecast_bl && self.startToUpdateIcecastName()
					}, 5e3);
					var s = o,
						i = s.substr(0, s.indexOf("-") - 1),
						n = s.substr(s.indexOf("-") + 2);
					self.getImage(i, n)
				}, this.startToUpdateShoutcast = function() {
					self.closeJsonPLoader(), self.getShoutcastRadioNameAndStream(self.sourceURL_str, !0)
				}, this.getShoutcastRadioNameAndStream = function(e, t) {
					if (!self.isPlaylistDispatchingError_bl) {
						if (self.sourceURL_str = e, self.originalSourceURL_str = e, "/" == self.sourceURL_str.substr(self.sourceURL_str.length - 1) && (self.sourceURL_str = self.sourceURL_str.substr(0, self.sourceURL_str.length - 1)), 1 == self.shoutcastVersion) e = "https://cors-anywhere.herokuapp.com/" + self.sourceURL_str + "/7.html", self.originalSourceURL_str = e, self.shoutcastxmlHttp = new XMLHttpRequest, self.shoutcastxmlHttp.onreadystatechange = function() {
							if (4 == self.shoutcastxmlHttp.readyState && 200 == self.shoutcastxmlHttp.status) {
								var e = self.shoutcastxmlHttp.responseText.match(/<body>.*?<\/body>/im)[0];
								e = (e = (e = (e = e.replace("<body>", "")).replace("<body> ", "")).replace(" </body>", "")).replace("</body> ", "");
								var t = {
									streampath: "/;type=mp3",
									servertitle: "Shoutcast v1",
									servergenre: "Shoutcast v1"
								};
								t.songtitle = e.split(",")[6],
									t.currentlisteners = e.split(",")[0], t.bitrate = e.split(",")[5], self.parseShoutcastRadioURL(t)
							}
						}, self.shoutcastxmlHttp.open("GET", e, !0), self.shoutcastxmlHttp.send(null);
						else {
							e = self.sourceURL_str + "/stats?sid=1&json=1&callback=" + parent.instanceName_str + ".data.parseShoutcastRadioURL";
							try {
								document.documentElement.removeChild(self.scs_el)
							} catch (e) {}
							try {
								document.documentElement.removeChild(self.scs_el)
							} catch (e) {}
							try {
								self.scs_el = document.createElement("script"), self.scs_el.src = e, self.scs_el.id = parent.instanceName_str + ".data.parseRadioErrorURL",
									document.documentElement.appendChild(self.scs_el)
							} catch (e) {}
						}
						t || (self.JSONPRequestTimeoutId_to = setTimeout(self.parseRadioErrorURL, 5e3))
					}
				}, this.parseShoutcastRadioURL = function(e) {
					var t;
					if (parent.isShoutcast_bl || parent.isIcecast_bl)
						if (self.closeJsonPLoader(), e.streampath) {
							t = self.sourceURL_str + e.streampath, "/" == e.streampath && (t += ";.mp3"), songTitle = e.songtitle, self.prevSongTitle != songTitle && self.getSoutcastHistory();
							self.stationLabelClassName, self.stationClassName, e.servertitle,
								self.genreLabelClassName, self.genreClassName, e.servergenre,
								self.currentListenersLabelClassName,
								self.currentListenersClassName, e.currentlisteners, self.bitrateLabelClassName, self.bitrateClassName, e.bitrate;
							var o = songTitle.substr(0, songTitle.indexOf("-") - 1),
								s = songTitle.substr(songTitle.indexOf("-") + 2);
							self.getImage(o, s), self.dispatchEvent(FWDMSPAudioData.RADIO_TRACK_READY, {
								source: t,
								songTitle: songTitle
							}), self.updateRadioTitleId_to = setTimeout(function() {
								parent.isShoutcast_bl && self.startToUpdateShoutcast()
							}, 5e3)
						}
					else self.parseRadioErrorURL()
				}, this.parseRadioErrorURL = function() {
					(parent.isShoutcast_bl || parent.isIcecast_bl) && (self.closeJsonPLoader(), self.isPlaylistDispatchingError_bl = !0, showLoadPlaylistErrorId_to = setTimeout(function() {
						self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
							text: "Error loading radio track url!<font color='#FF0000'>" + self.sourceURL_str + "</font>"
						}), self.isPlaylistDispatchingError_bl = !1
					}, 50), parent.isShoutcast_bl && self.startToUpdateShoutcast())
				}, this.getSoutcastHistory = function() {
					if (parent.isShoutcast_bl || parent.isIcecast_bl) {
						"/" != self.sourceURL_str.substr(self.sourceURL_str.length - 1) && (self.sourceURL_str = self.sourceURL_str + "/"), url = self.sourceURL_str + "played?sid=1&type=json&callback=" + parent.instanceName_str + ".data.parseShoutcastRadioHisotry";
						try {
							document.documentElement.removeChild(self.scs2_el)
						} catch (e) {}
						try {
							self.scs2_el = document.createElement("script"), self.scs2_el.src = url, document.documentElement.appendChild(self.scs2_el)
						} catch (e) {}
					}
				}, this.parseShoutcastRadioHisotry = function(e) {
					if (self.prevObj != e[0].title) {
						var t;
						self.history_ar = [];
						for (var o = 0; o < e.length; o++) {
							t = e[o];
							var s = new Date(1e3 * t.playedat),
								i = String(s.getHours());
							1 == i.length && parseInt(i) <= 9 ? i = "0" + i : 1 == i.length && 9 < parseInt(i) && (i += "0");
							var n = String(s.getMinutes());
							1 == n.length && parseInt(n) <= 9 ? n = "0" + n : 1 == n.length && 9 < parseInt(n) && (n += "0");
							var l = String(s.getSeconds());
							1 == l.length && parseInt(l) <= 9 ? l = "0" + l : 1 == l.length && 9 < parseInt(l) && (l += "0"), s = i + ":" + n + ":" + l;
							var r = t.title;
							if (0 == o) var a = "<span class='" + self.titleClassNameSelected + "'>" + r + "</span><span class='" + self.lineClassNameSelected + "'> - </span><span class='" + self.playedAtClassNameSelected + "'>played at:</span> <span class='" + self.timeClassNameSelected + "'>" + s + "</span>";
							else a = "<span class='" + self.titleClassName + "'>" + r + "</span><span class='" + self.lineClassName + "'> - </span><span class='" + self.playedAtClassName + "'>played at</span> <span class='" + self.timeClassName + "'>" + s + "</span>";
							self.history_ar[o] = a, self.prevObj = e[0].title
						}
					}
				}, this.getImage = function(e, t) {
					if (parent.isShoutcast_bl || parent.isIcecast_bl) {
						var o = "http://itunes.apple.com/search?type=jsonp&term==" + (e = encodeURI(e)) + "-" + (t = encodeURI(t)) + "&media=music&limit=1&callback=" + parent.instanceName_str + ".data.parseImage";
						try {
							document.documentElement.removeChild(self.scs3_el)
						} catch (e) {}
						try {
							self.scs3_el = document.createElement("script"), self.scs3_el.src = o, document.documentElement.appendChild(self.scs3_el)
						} catch (e) {}
					}
				}, this.parseImage = function(e) {
					e.results && e.results[0] && self.dispatchEvent(FWDMSPAudioData.UPDATE_IMAGE, {
						image: e.results[0].artworkUrl100
					})
				}, this.loadFolderPlaylist = function(e) {
					if (!self.isPlaylistDispatchingError_bl) {
						if ("file:" == document.location.protocol && -1 == e.indexOf("official.fm")) return self.isPlaylistDispatchingError_bl = !0, void(showLoadPlaylistErrorId_to = setTimeout(function() {
							self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
								text: "Creating a mp3 playlist from a folder is not allowed or possible local! To function properly please test online."
							}), self.isPlaylistDispatchingError_bl = !1
						}, 50));
						self.closeXHR(), self.loadFromFolder_bl = !0, self.countID3 = 0, self.sourceURL_str = e.substr(e.indexOf(":") + 1), self.xhr = new XMLHttpRequest, self.xhr.onreadystatechange = self.ajaxOnLoadHandler, self.xhr.onerror = self.ajaxOnErrorHandler;
						try {
							self.xhr.open("get", self.proxyFolderPath_str + "?dir=" + encodeURIComponent(self.sourceURL_str) + "&rand=" + parseInt(9999999 * Math.random()), !0), self.xhr.send()
						} catch (e) {
							e && e.message && e.message, self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
								text: "Folder proxy file path is not found: <font color='#FF0000'>" + self.proxyFolderPath_str + "</font>"
							})
						}
					}
				}, this.ajaxOnLoadHandler = function(e) {
					var response, isXML = !1;
					if (4 == self.xhr.readyState)
						if (404 == self.xhr.status) self.loadFromFolder_bl ? self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
							text: "Folder proxy file path is not found: <font color='#FF0000'>" + self.proxyFolderPath_str + "</font>"
						}) : -1 != self.sourceURL_str.indexOf(".pls") ? self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
							text: "Error loading file <font color='#FF0000'>" + self.sourceURL_str + "</font>. Probably the file path is incorect."
						}) : self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
							text: "Proxy file path is not found: <font color='#FF0000'>" + self.proxyPath_str + "</font>"
						});
						else if (408 == self.xhr.status) self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
						text: "Proxy file request load timeout!"
					});
					else if (200 == self.xhr.status) {
						if (-1 != self.xhr.responseText.indexOf("<b>Warning</b>:")) return void self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
							text: "Error loading folder: <font color='#FF0000'>" + self.sourceURL_str + "</font>. Make sure that the folder path is correct!"
						});
						response = -1 != self.xhr.responseText.indexOf("NumberOfEntries") ? PLS.parse(this.response) : window.JSON ? JSON.parse(self.xhr.responseText) : eval("(" + self.xhr.responseText + ")"), -1 != self.xhr.responseText.indexOf("api.soundcloud.com") ? (self.isSCTrack ? self.parseSoundCloudURL(response) : self.parseSoundCloud(response), self.isSCTrack = !1) : response.channel ? self.parsePodcast(response) : response.folder ? self.parseFolderJSON(response) : response.li ? self.parseXML(response) : -1 != self.xhr.responseText.indexOf("NumberOfEntries") ? self.parsePLS(response) : response.error && self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
							text: "Error loading file: <font color='#FF0000'>" + self.sourceURL_str + "</font>. Make sure the file path (xml or podcast) is correct and well formatted!"
						})
					}
				}, this.ajaxOnErrorHandler = function(e) {
					try {
						window.console && console.log(e), window.console && console.log(e.message)
					} catch (e) {}
					self.loadFromFolder_bl ? self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
						text: "Error loading file : <font color='#FF0000'>" + self.proxyFolderPath_str + "</font>. Make sure the path is correct"
					}) : self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
						text: "Error loading file : <font color='#FF0000'>" + self.proxyPath_str + "</font>. Make sure the path is correct"
					})
				}, this.parseSoundCloud = function(e) {
					var t;
					if (self.closeJsonPLoader(), self.playlist_ar = [], e && e.uri) return "track" == e.kind ? void self.createSoundcloudPlaylist(e) : (t = -1 == e.uri.indexOf("/tracks") ? e.uri + "/tracks" : e.uri + "/favorites", void self.loadSoundCloudList(t));
					e.length || "track" == e.kind ? self.createSoundcloudPlaylist(e) : self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
						text: "Please provide a playlist or track URL : <font color='#FF0000'>" + self.sourceURL_str + "</font>."
					})
				}, this.createSoundcloudPlaylist = function(e) {
					if (e.length)
						for (var t = 0; t < e.length; t++) {
							if (track = e[t], obj = {}, obj.source = track.stream_url + "?consumer_key=" + self.scClientId_str, obj.buy = void 0, obj.thumbPath = track.artwork_url, self.showSoundCloudUserNameInTitle_bl) {
								var o = "";
								self.showTracksNumbers_bl ? (t < 9 && (o = "0"), o = o + (t + 1) + ". ", obj.title = o + "<span style='font-weight:bold;'>" + track.user.username + "</span> - " + track.title) : obj.title = "<span style='font-weight:bold;'>" + track.user.username + "</span> - " + track.title,
									obj.titleText = track.user.username + " - " + track.title
							} else {
								o = "";
								self.showTracksNumbers_bl ? (t < 9 && (o = "0"), o = o + (t + 1) + ". ", obj.title = o + track.title) : obj.title = track.title, obj.titleText = track.title
							}
							if (obj.duration = track.duration, track.streamable && self.playlist_ar.push(obj), t > self.maxPlaylistItems - 1) break
						}
					else track = e, obj = {}, obj.source = track.stream_url + "?consumer_key=" + self.scClientId_str, obj.buy = void 0, obj.thumbPath = track.artwork_url, self.showSoundCloudUserNameInTitle_bl ? (obj.title = "<span style='font-weight:bold;'>" + track.user.username + "</span> - " + track.title, obj.titleText = track.user.username + " - " + track.title) : (obj.title = track.title, obj.titleText = track.title), obj.duration = track.duration, track.streamable && self.playlist_ar.push(obj);
					clearTimeout(self.dispatchPlaylistLoadCompleteWidthDelayId_to), self.dispatchPlaylistLoadCompleteWidthDelayId_to = setTimeout(function() {
						self.dispatchEvent(FWDMSPAudioData.PLAYLIST_LOAD_COMPLETE)
					}, 50), self.isDataLoaded_bl = !0
				}, this.parsePodcast = function(e) {
					var t;
					self.playlist_ar = [];
					var o = e.channel.item,
						s = void 0;
					try {
						s = e.channel.image.url
					} catch (e) {}
					for (var i = 0; i < o.length; i++) {
						t = {}, o[i].enclosure ? t.source = encodeURI(o[i].enclosure["@attributes"].url) : t.source = encodeURI(o[i].link), t.buy = void 0,
							t.thumbPath = s;
						var n = "";
						if (self.showTracksNumbers_bl ? (i < 9 && (n = "0"), n = n + (i + 1) + ". ", t.title = n + o[i].title) : t.title = o[i].title, t.titleText = o[i].title, t.duration = void 0, self.playlist_ar[i] = t, i > self.maxPlaylistItems - 1) break
					}
					clearTimeout(self.dispatchPlaylistLoadCompleteWidthDelayId_to), self.dispatchPlaylistLoadCompleteWidthDelayId_to = setTimeout(function() {
						self.dispatchEvent(FWDMSPAudioData.PLAYLIST_LOAD_COMPLETE)
					}, 50), self.isDataLoaded_bl = !0
				},
				this.parseXML = function(e) {
					var t;
					self.playlist_ar = [];
					var o = e.li;
					o.length || (o = [o]);
					for (var s = 0; s < o.length; s++) {
						(t = {}).source = o[s]["@attributes"]["data-path"],
						-1 != t.source.indexOf("encrypt:") && (t.source = atob(t.source.substr(8)));
						var i = encodeURI(t.source.substr(0, t.source.lastIndexOf("/") + 1)),
							n = t.source.substr(t.source.lastIndexOf("/") + 1);
						n = -1 != n.indexOf(";.mp3") || FWDMSPUtils.isURLEncoded(n) ? t.source.substr(t.source.lastIndexOf("/") + 1)
						                                                            : encodeURIComponent(t.source.substr(t.source.lastIndexOf("/") + 1)),
																																				 t.source = i + n;
						t.buy = o[s]["@attributes"]["data-track-id"],
						t.can_add = o[s]["@attributes"]["data-add"],
						null == t.buy && (t.buy = ""),
						t.thumbPath = o[s]["@attributes"]["data-thumbpath"];
						var r = "";
						if (self.showTracksNumbers_bl ? (s < 9 && (r = "0"),
						                                 r = r + (s + 1) + ". ",
																						 t.title = r + o[s]["@attributes"]["data-title"])
						                              : t.title = o[s]["@attributes"]["data-title"],
																						t.titleText = o[s]["@attributes"]["data-title"],
																						t.duration = o[s]["@attributes"]["data-duration"],
																						t.atb = o[s]["@attributes"]["data-use-a-to-b"],
																						t.isPrivate = o[s]["@attributes"]["data-is-private"],
																						"yes" == t.isPrivate ? t.isPrivate = !0 : t.isPrivate = !1,
																						t.startAtTime = o[s]["@attributes"]["data-start-at-time"],
																						"00:00:00" != t.startAtTime
						        												&& FWDMSPUtils.checkTime(t.startAtTime) || (t.startAtTime = void 0),
										   										t.stopAtTime = o[s]["@attributes"]["data-stop-at-time"],
										   										"00:00:00" != t.stopAtTime && FWDMSPUtils.checkTime(t.stopAtTime) || (t.stopAtTime = void 0),
											 										t.isShoutcast_bl = o[s]["@attributes"]["data-type"],
											 										t.isShoutcast_bl && (-1 != t.isShoutcast_bl.toLowerCase().indexOf("shoutcastv1") ? (t.shoutcastVersion = 1, t.isShoutcast_bl = !0)
											                                                                                  									 : -1 != t.isShoutcast_bl.toLowerCase().indexOf("shoutcastv2")
																																																													 ? (t.shoutcastVersion = 2, t.isShoutcast_bl = !0)
																																																													 : t.isShoutcast_bl = !1),
											 t.isIcecast_bl = o[s]["@attributes"]["data-type"],
											 t.isIcecast_bl && (-1 != t.isIcecast_bl.toLowerCase().indexOf("icecast") ? t.isIcecast_bl = !0 : t.isIcecast_bl = !1),
											 self.playlist_ar[s] = t,
											 s > self.maxPlaylistItems - 1
								)
									break
					}
					clearTimeout(self.dispatchPlaylistLoadCompleteWidthDelayId_to),
					self.dispatchPlaylistLoadCompleteWidthDelayId_to = setTimeout(function() {
						self.dispatchEvent(FWDMSPAudioData.PLAYLIST_LOAD_COMPLETE)
					}, 50),
					self.isDataLoaded_bl = !0
				}, this.parsePLS = function(e) {
					var t;
					self.playlist_ar = [];
					for (var o = e, s = 0; s < o.length; s++) {
						(t = {}).source = o[s].file + "/;.mp3", t.buy = void 0, t.thumbPath = void 0, t.title = o[s].title, t.titleText = o[s].title;
						var i = "";
						if (self.showTracksNumbers_bl ? (s < 9 && (i = "0"), i = i + (s + 1) + ". ", t.title = i + " " + t.title) : (s < 9 && (i = "0"), i += s + 1, t.title = " " + i), t.titleText = t.title, self.playlist_ar[s] = t, s > self.maxPlaylistItems - 1) break
					}
					clearTimeout(self.dispatchPlaylistLoadCompleteWidthDelayId_to), self.dispatchPlaylistLoadCompleteWidthDelayId_to = setTimeout(function() {
						self.dispatchEvent(FWDMSPAudioData.PLAYLIST_LOAD_COMPLETE)
					}, 50), self.isDataLoaded_bl = !0
				}, this.parseFolderJSON = function(e) {
					var t;
					self.playlist_ar = [];
					for (var o = e.folder, s = 0; s < o.length; s++) {
						(t = {}).source = o[s]["@attributes"]["data-path"], -1 != t.source.indexOf("encrypt:") && (t.source = atob(t.source.substr(8)));
						var i = encodeURI(t.source.substr(0, t.source.lastIndexOf("/") + 1)),
							n = encodeURIComponent(t.source.substr(t.source.lastIndexOf("/") + 1));
						if (t.source = i + n, t.buy = void 0, t.thumbPath = o[s]["@attributes"]["data-thumbpath"], t.title = "...", t.titleText = "...", FWDMSPUtils.isIEAndLessThen9) {
							var l = "";
							self.showTracksNumbers_bl ? (s < 9 && (l = "0"), l = l + (s + 1) + ". ", t.title = l + "track ", t.titleText = "track") : (s < 9 && (l = "0"), l += s + 1, t.title = "track " + l, t.titleText = "track " + l)
						}
						if (self.playlist_ar[s] = t, s > self.maxPlaylistItems - 1) break
					}
					clearTimeout(self.dispatchPlaylistLoadCompleteWidthDelayId_to), self.dispatchPlaylistLoadCompleteWidthDelayId_to = setTimeout(function() {
						self.dispatchEvent(FWDMSPAudioData.PLAYLIST_LOAD_COMPLETE)
					}, 50), self.isDataLoaded_bl = !0
				}, this.parseDOMPlaylist = function(e) {
					if (!self.isPlaylistDispatchingError_bl) {
						var t;
						if (self.closeXHR(), !(t = document.getElementById(e))) return self.isPlaylistDispatchingError_bl = !0, void(showLoadPlaylistErrorId_to = setTimeout(function() {
							self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
								text: "The playlist with id <font color='#FF0000'>" + e + "</font> is not found in the DOM."
							}), self.isPlaylistDispatchingError_bl = !1
						}, 50));
						var o, s = FWDMSPUtils.getChildren(t),
							i = s.length;
						if (self.playlist_ar = [], 0 == i) return self.isPlaylistDispatchingError_bl = !0, void(showLoadPlaylistErrorId_to = setTimeout(function() {
							self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
								text: "The playlist whit the id  <font color='#FF0000'>" + e + "</font> must contain at least one track."
							}), self.isPlaylistDispatchingError_bl = !1
						}, 50));
						for (var n = 0; n < i; n++) {
							var l = {};
							if (o = s[n], !FWDMSPUtils.hasAttribute(o, "data-path")) return self.isPlaylistDispatchingError_bl = !0, void(showLoadPlaylistErrorId_to = setTimeout(function() {
								self.dispatchEvent(FWDMSPAudioData.LOAD_ERROR, {
									text: "Attribute <font color='#FF0000'>data-path</font> is required in the playlist at position <font color='#FF0000'>" + (n + 1)
								})
							}, 50));
							if (n > self.maxPlaylistItems - 1) break;
							if (l.isShoutcast_bl = FWDMSPUtils.getAttributeValue(o, "data-type"), l.isShoutcast_bl && (-1 != l.isShoutcast_bl.toLowerCase().indexOf("shoutcastv1") ? (l.shoutcastVersion = 1, l.isShoutcast_bl = !0) : -1 != l.isShoutcast_bl.toLowerCase().indexOf("shoutcastv2") ? (l.shoutcastVersion = 2, l.isShoutcast_bl = !0) : l.isShoutcast_bl = !1), l.isIcecast_bl = FWDMSPUtils.getAttributeValue(o, "data-type"), l.isIcecast_bl && (-1 != l.isIcecast_bl.toLowerCase().indexOf("icecast") ? l.isIcecast_bl = !0 : l.isIcecast_bl = !1), l.source = FWDMSPUtils.getAttributeValue(o, "data-path"), -1 != l.source.indexOf("encrypt:") && (l.source = atob(l.source.substr(8))), -1 != l.source.indexOf("youtube.")) {
								var r = l.source.match(/^.*(youtu\.be\/|v\/|u\/\w\/|embed\/|watch\?v=|\&v=)([^#\&\?]*).*/);
								l.source = r[2]
							} else if (-1 == l.source.lastIndexOf("google.") && !l.isShoutcast_bl && !l.isIcecast_bl) {
								var a = encodeURI(l.source.substr(0, l.source.lastIndexOf("/") + 1)),
									d = l.source.substr(l.source.lastIndexOf("/") + 1);
								d = -1 != d.indexOf(";.mp3") || FWDMSPUtils.isURLEncoded(d) ? l.source.substr(l.source.lastIndexOf("/") + 1) : encodeURIComponent(l.source.substr(l.source.lastIndexOf("/") + 1)), l.source = a + d
							}
							if (l.source.indexOf(".soundcloud.") != -1) {
								l.source = l.source + '/stream?client_id=' + 'dce5652caa1b66331903493735ddd64d'
							};
							(l.isShoutcast_bl || l.isIcecast_bl) && "/" != l.source.substr(l.source.length - 1) && (l.source += "/"), FWDMSPUtils.hasAttribute(o, "data-thumbpath") ? l.thumbPath = FWDMSPUtils.getAttributeValue(o, "data-thumbpath") : l.thumbPath = void 0, FWDMSPUtils.hasAttribute(o, "data-track-id") ? l.buy = FWDMSPUtils.getAttributeValue(o, "data-track-id") : l.buy = void 0, FWDMSPUtils.hasAttribute(o, "data-add") ? l.can_add = FWDMSPUtils.getAttributeValue(o, "data-add") : l.can_add = void 0, l.title = "not defined!";
							try {
								var u = "";
								self.showTracksNumbers_bl ? (n < 9 && (u = "0"), u = u + (n + 1) + ". ", l.title = u + FWDMSPUtils.getChildren(o)[0].innerHTML) : l.title = FWDMSPUtils.getChildren(o)[0].innerHTML
							} catch (e) {}
							try {
								l.titleText = FWDMSPUtils.getChildren(o)[0].textContent || FWDMSPUtils.getChildren(o)[0].innerText
							} catch (e) {}
							FWDMSPUtils.hasAttribute(o, "data-duration") && (l.duration = FWDMSPUtils.getAttributeValue(o, "data-duration")),
								FWDMSPUtils.hasAttribute(o, "data-use-a-to-b") && (l.atb = FWDMSPUtils.getAttributeValue(o, "data-use-a-to-b")), l.isPrivate = FWDMSPUtils.getAttributeValue(o, "data-is-private"), "yes" == l.isPrivate ? l.isPrivate = !0 : l.isPrivate = !1, l.startAtTime = FWDMSPUtils.getAttributeValue(o, "data-start-at-time"), "00:00:00" != l.startAtTime && FWDMSPUtils.checkTime(l.startAtTime) || (l.startAtTime = void 0), l.stopAtTime = FWDMSPUtils.getAttributeValue(o, "data-stop-at-time"), "00:00:00" != l.stopAtTime && FWDMSPUtils.checkTime(l.stopAtTime) || (l.stopAtTime = void 0), self.playlist_ar[n] = l
						}
						clearTimeout(self.dispatchPlaylistLoadCompleteWidthDelayId_to), self.dispatchPlaylistLoadCompleteWidthDelayId_to = setTimeout(function() {
							self.dispatchEvent(FWDMSPAudioData.PLAYLIST_LOAD_COMPLETE)
						}, 50), self.isDataLoaded_bl = !0
					}
				}, this.closeXHR = function() {
					self.closeJsonPLoader();
					try {
						document.documentElement.removeChild(self.scs_el), self.scs_el = null
					} catch (e) {}
					if (null != self.xhr) {
						try {
							self.xhr.abort()
						} catch (e) {}
						self.xhr.onreadystatechange = null, self.xhr.onerror = null, self.xhr = null
					}
					self.countID3 = 2e3
				}, this.closeData = function() {
					self.closeXHR(), self.closeJsonPLoader(),
						clearTimeout(self.loadImageId_to),
						clearTimeout(self.showLoadPlaylistErrorId_to),
						clearTimeout(self.dispatchPlaylistLoadCompleteWidthDelayId_to), clearTimeout(self.loadImageId_to),
						clearTimeout(self.loadPreloaderId_to),
						self.image_img && (self.image_img.onload = null, self.image_img.onerror = null)
				}, self.init()
		};
		FWDMSPAudioData.setPrototype = function() {
				FWDMSPAudioData.prototype = new FWDMSPEventDispatcher
			},
		FWDMSPAudioData.prototype = null,
		FWDMSPAudioData.RADIO_TRACK_UPDATE = "shoutcastTitleUpdate",
		FWDMSPAudioData.RADIO_TRACK_READY = "radioTrackReady",
		FWDMSPAudioData.UPDATE_IMAGE = "updateImage",
		FWDMSPAudioData.SOUNDCLOUD_TRACK_READY = "soundcloudTrackReady",
		FWDMSPAudioData.PRELOADER_LOAD_DONE = "onPreloaderLoadDone",
		FWDMSPAudioData.LOAD_DONE = "onLoadDone",
		FWDMSPAudioData.LOAD_ERROR = "onLoadError",
		FWDMSPAudioData.IMAGE_LOADED = "onImageLoaded",
		FWDMSPAudioData.SKIN_LOAD_COMPLETE = "onSkinLoadComplete",
		FWDMSPAudioData.SKIN_PROGRESS = "onSkinProgress",
		FWDMSPAudioData.IMAGES_PROGRESS = "onImagesPogress",
		FWDMSPAudioData.PLAYLIST_LOAD_COMPLETE = "onPlaylistLoadComplete",
		window.FWDMSPAudioData = FWDMSPAudioData
	}(window),

	function(o) {
		var i = function(e) {
			var l = this;
			i.prototype;
			this.audio_el = null, this.sourcePath_str = null, this.lastPercentPlayed = 0, this.volume = e, this.curDuration = 0, this.countNormalMp3Errors = 0,
				this.countShoutCastErrors = 0, this.maxShoutCastCountErrors = 5, this.maxNormalCountErrors = 1, this.testShoutCastId_to, this.preload_bl = !1, this.allowScrubing_bl = !1, this.hasError_bl = !0,
				this.isPlaying_bl = !1, this.isStopped_bl = !0,
				this.hasPlayedOnce_bl = !1, this.isStartEventDispatched_bl = !1, this.isSafeToBeControlled_bl = !1, this.isShoutcast_bl = !1, this.isNormalMp3_bl = !1,
				this.init = function() {
					l.setupAudio(), l.setHeight(0)
				}, this.setupAudio = function() {
					null == l.audio_el && (l.audio_el = document.createElement("audio"), l.screen.appendChild(l.audio_el), l.audio_el.controls = !1, l.audio_el.preload = "auto", l.audio_el.volume = l.volume),
						l.audio_el.addEventListener("error", l.errorHandler), l.audio_el.addEventListener("canplay", l.safeToBeControlled), l.audio_el.addEventListener("canplaythrough", l.safeToBeControlled), l.audio_el.addEventListener("progress", l.updateProgress), l.audio_el.addEventListener("timeupdate", l.updateAudio), l.audio_el.addEventListener("pause", l.pauseHandler), l.audio_el.addEventListener("play", l.playHandler),
						l.audio_el.addEventListener("ended", l.endedHandler)
				}, this.destroyAudio = function() {
					l.audio_el && (l.audio_el.removeEventListener("error", l.errorHandler), l.audio_el.removeEventListener("canplay", l.safeToBeControlled), l.audio_el.removeEventListener("canplaythrough", l.safeToBeControlled), l.audio_el.removeEventListener("progress", l.updateProgress), l.audio_el.removeEventListener("timeupdate", l.updateAudio), l.audio_el.removeEventListener("pause", l.pauseHandler), l.audio_el.removeEventListener("play", l.playHandler), l.audio_el.removeEventListener("ended", l.endedHandler), l.audio_el.src = "", l.audio_el.load())
				}, this.errorHandler = function(e) {
					if (null != l.sourcePath_str && null != l.sourcePath_str) {
						if (l.isNormalMp3_bl && l.countNormalMp3Errors <= l.maxNormalCountErrors) return l.stop(), l.testShoutCastId_to = setTimeout(l.play, 200), void l.countNormalMp3Errors++;
						if (l.isShoutcast_bl && l.countShoutCastErrors <= l.maxShoutCastCountErrors && 0 == l.audio_el.networkState) return l.testShoutCastId_to = setTimeout(l.play, 200), void l.countShoutCastErrors++;
						var t;
						l.hasError_bl = !0, l.stop(), t = 0 == l.audio_el.networkState ? "error 'self.audio_el.networkState = 1'" : 1 == l.audio_el.networkState ? "error 'self.audio_el.networkState = 1'" : 2 == l.audio_el.networkState ? "'self.audio_el.networkState = 2'" : 3 == l.audio_el.networkState ? "source not found <font color='#FF0000'>" + l.sourcePath_str + "</font>" : e, o.console && o.console.log(l.audio_el.networkState), l.dispatchEvent(i.ERROR, {
							text: t
						})
					}
				}, this.setSource = function(e) {
					l.sourcePath_str = e, clearTimeout(l.testShoutCastId_to), -1 != l.sourcePath_str.indexOf(";") ? (l.isShoutcast_bl = !0, l.countShoutCastErrors = 0) : l.isShoutcast_bl = !1, -1 == l.sourcePath_str.indexOf(";") ? (l.isNormalMp3_bl = !0, l.countNormalMp3Errors = 0) : l.isNormalMp3_bl = !1, l.lastPercentPlayed = 0, l.audio_el && l.stop(!0)
				}, this.play = function(e) {
					if (l.isStopped_bl) l.isPlaying_bl = !1, l.hasError_bl = !1, l.allowScrubing_bl = !1, l.isStopped_bl = !1, l.setupAudio(), l.audio_el.src = l.sourcePath_str, l.play();
					else if (!l.audio_el.ended || e) try {
						l.isPlaying_bl = !0, l.hasPlayedOnce_bl = !0, l.audio_el.play(), FWDMSPUtils.isIE && l.dispatchEvent(i.PLAY)
					}
					catch (e) {}
				}, this.pause = function() {
					if (null != l && null != l.audio_el && !l.audio_el.ended) try {
						l.audio_el.pause(), l.isPlaying_bl = !1, FWDMSPUtils.isIE && l.dispatchEvent(i.PAUSE)
					}
					catch (e) {}
				}, this.pauseHandler = function() {
					l.allowScrubing_bl || l.dispatchEvent(i.PAUSE)
				}, this.playHandler = function() {
					l.allowScrubing_bl || (l.isStartEventDispatched_bl || (l.dispatchEvent(i.START), l.isStartEventDispatched_bl = !0), l.dispatchEvent(i.PLAY))
				}, this.endedHandler = function() {
					l.dispatchEvent(i.PLAY_COMPLETE)
				}, this.getDuration = function() {
					return l.formatTime(l.audio_el.duration)
				}, this.getCurrentTime = function() {
					return l.formatTime(l.audio_el.currentTime)
				}, this.stop = function(e) {
					l.dispatchEvent(i.UPDATE_TIME, {
						curTime: "00:00",
						totalTime: "00:00",
						seconds: 0
					}), (null != l && null != l.audio_el && !l.isStopped_bl || e) && (l.isPlaying_bl = !1, l.isStopped_bl = !0, l.hasPlayedOnce_bl = !0, l.isSafeToBeControlled_bl = !1, l.isStartEventDispatched_bl = !1, clearTimeout(l.testShoutCastId_to), l.audio_el.pause(), l.destroyAudio(), l.dispatchEvent(i.STOP), l.dispatchEvent(i.LOAD_PROGRESS, {
						percent: 0
					}))
				}, this.togglePlayPause = function() {
					null != l && l.isSafeToBeControlled_bl && (l.isPlaying_bl ? l.pause() : l.play())
				}, this.safeToBeControlled = function() {
					l.isSafeToBeControlled_bl || (l.hasHours_bl = 0 < Math.floor(l.audio_el.duration / 3600), l.isPlaying_bl = !0, l.isSafeToBeControlled_bl = !0, l.dispatchEvent(i.SAFE_TO_SCRUBB), l.dispatchEvent(i.SAFE_TO_UPDATE_VOLUME))
				}, this.updateProgress = function() {
					var e = 0;
					0 < l.audio_el.buffered.length && (e = l.audio_el.buffered.end(l.audio_el.buffered.length - 1).toFixed(1) / l.audio_el.duration.toFixed(1), !isNaN(e) && e || (e = 0)), 1 == e && l.audio_el.removeEventListener("progress", l.updateProgress), l.dispatchEvent(i.LOAD_PROGRESS, {
						percent: e
					})
				}, this.updateAudio = function() {
					var e;
					l.allowScrubing_bl || (e = l.audio_el.currentTime / l.audio_el.duration, l.dispatchEvent(i.UPDATE, {
						percent: e
					}));
					var t = l.formatTime(l.audio_el.duration),
						o = l.formatTime(l.audio_el.currentTime);
					isNaN(l.audio_el.duration) ? l.dispatchEvent(i.UPDATE_TIME, {
						curTime: "00:00",
						totalTime: "00:00",
						seconds: 0,
						totalTimeInSeconds: 0
					}) : l.dispatchEvent(i.UPDATE_TIME, {
						curTime: o,
						totalTime: t,
						seconds: Math.round(l.audio_el.currentTime),
						totalTimeInSeconds: l.audio_el.duration
					}), l.lastPercentPlayed = e, l.curDuration = o
				}, this.startToScrub = function() {
					l.allowScrubing_bl = !0
				}, this.stopToScrub = function() {
					l.allowScrubing_bl = !1
				}, this.scrubbAtTime = function(e) {
					l.audio_el.currentTime = e;
					var t = FWDMSPUtils.formatTime(l.audio_el.duration),
						o = FWDMSPUtils.formatTime(l.audio_el.currentTime);
					l.dispatchEvent(i.UPDATE_TIME, {
						curTime: o,
						totalTime: t,
						seconds: e
					})
				}, this.scrub = function(e, t) {
					if (null != l.audio_el && l.audio_el.duration) {
						t && l.startToScrub();
						try {
							l.audio_el.currentTime = l.audio_el.duration * e;
							var o = l.formatTime(l.audio_el.duration),
								s = l.formatTime(l.audio_el.currentTime);
							l.dispatchEvent(i.UPDATE_TIME, {
								curTime: s,
								totalTime: o
							})
						} catch (t) {}
					}
				}, this.replay = function() {
					l.scrub(0), l.play()
				}, this.setVolume = function(e) {
					null != e && (l.volume = e), l.audio_el && (l.audio_el.volume = l.volume)
				}, this.formatTime = function(e) {
					var t = Math.floor(e / 3600),
						o = e % 3600,
						s = Math.floor(o / 60),
						i = o % 60,
						n = Math.ceil(i);
					return s = 10 <= s ? s : "0" + s, n = 10 <= n ? n : "0" + n, isNaN(n) ? "00:00" : l.hasHours_bl ? t + ":" + s + ":" + n : s + ":" + n
				}, this.setPlaybackRate = function(e) {
					l.audio_el && (l.audio_el.defaultPlaybackRate = e, l.audio_el.playbackRate = e)
				}, this.init()
		};
		i.setPrototype = function() {
			i.prototype = new FWDMSPDisplayObject("div")
		}, i.ERROR = "error", i.UPDATE = "update", i.UPDATE = "update", i.UPDATE_TIME = "updateTime", i.SAFE_TO_SCRUBB = "safeToControll", i.SAFE_TO_UPDATE_VOLUME = "safeToUpdateVolume", i.LOAD_PROGRESS = "loadProgress", i.START = "start", i.PLAY = "play", i.PAUSE = "pause", i.STOP = "stop", i.PLAY_COMPLETE = "playComplete", o.FWDMSPAudioScreen = i
	}(window),
	function() {
		var t = function(o, e) {
			var p = this;
			t.prototype;
			this.image_img, this.catThumbBk_img = o.catThumbBk_img, this.catNextN_img = o.catNextN_img, this.catPrevN_img = o.catPrevN_img,
				this.catCloseN_img = o.catCloseN_img, this.mainHolder_do = null, this.closeButton_do = null,
				this.nextButton_do = null, this.prevButton_do = null, this.thumbs_ar = [], this.categories_ar = o.cats_ar, this.catBkPath_str = o.catBkPath_str,
				this.id = 0, this.mouseX = 0, this.mouseY = 0,
				this.dif = 0, this.tempId = p.id, this.stageWidth = 0, this.stageHeight = 0, this.thumbW = 0, this.thumbH = 0, this.buttonsMargins = o.buttonsMargins, this.thumbnailMaxWidth = o.thumbnailMaxWidth, this.thumbnailMaxHeight = o.thumbnailMaxHeight, this.spacerH = o.horizontalSpaceBetweenThumbnails,
				this.spacerV = o.verticalSpaceBetweenThumbnails,
				this.dl, this.howManyThumbsToDisplayH = 0, this.howManyThumbsToDisplayV = 0, this.categoriesOffsetTotalWidth = 2 * p.catNextN_img.width + 30, this.categoriesOffsetTotalHeight = p.catNextN_img.height + 30, this.totalThumbnails = p.categories_ar.length, this.delayRate = .06, this.countLoadedThumbs = 0, this.hideCompleteId_to,
				this.showCompleteId_to, this.loadThumbnailsId_to,
				this.preventMouseWheelNavigId_to, this.showSearchInput_bl = o.showPlaylistsSearchInput_bl, this.inputBackgroundColor_str = o.inputBackgroundColor_str, this.inputColor_str = o.searchInputColor_str, this.preventMouseWheelNavig_bl = !1, this.areThumbnailsCreated_bl = !1, this.areThumbnailsLoaded_bl = !1, this.isShowed_bl = !1, this.isOnDOM_bl = !1, this.isMobile_bl = FWDMSPUtils.isMobile, this.hasPointerEvent_bl = FWDMSPUtils.hasPointerEvent, p.init = function() {
					-1 != o.skinPath_str.indexOf("hex_white") ? p.selectedButtonsColor_str = "#FFFFFF" : p.selectedButtonsColor_str = o.selectedButtonsColor_str, p.getStyle().zIndex = 16777271, p.getStyle().msTouchAction = "none", p.getStyle().webkitTapHighlightColor = "rgba(0, 0, 0, 0)", p.getStyle().width = "100%", p.mainHolder_do = new FWDMSPDisplayObject("div"), p.mainHolder_do.getStyle().background = "url('" + p.catBkPath_str + "')", p.mainHolder_do.setY(-3e3), p.addChild(p.mainHolder_do), p.setupButtons(), p.setupDisable(), p.isMobile_bl && (p.setupMobileMove(), FWDMSPUtils.isChrome && (FWDMSPUtils.isIEAndLessThen9 ? document.getElementsByTagName("body")[0].appendChild(p.screen) : document.documentElement.appendChild(p.screen))), (!p.isMobile_bl || p.isMobile_bl && p.hasPointerEvent_bl) && p.setSelectable(!1), window.addEventListener ? (p.screen.addEventListener("mousewheel", p.mouseWheelDumyHandler), p.screen.addEventListener("DOMMouseScroll", p.mouseWheelDumyHandler)) : document.attachEvent && p.screen.attachEvent("onmousewheel", p.mouseWheelDumyHandler), p.showSearchInput_bl && p.setupInput()
				}, this.mouseWheelDumyHandler = function(e) {
					var t;
					if (FWDAnimation.isTweening(p.mainHolder_do)) return e.preventDefault && e.preventDefault(), !1;
					for (var o = 0; o < p.totalThumbnails; o++)
						if (t = p.thumbs_ar[o], FWDAnimation.isTweening(t)) return e.preventDefault && e.preventDefault(), !1;
					var s = e.detail || e.wheelDelta;
					if (e.wheelDelta && (s *= -1), FWDMSPUtils.isOpera && (s *= -1), 0 < s) p.nextButtonOnMouseUpHandler();
					else if (s < 0) {
						if (p.leftId <= 0) return;
						p.prevButtonOnMouseUpHandler()
					}
					if (!e.preventDefault) return !1;
					e.preventDefault()
				}, p.resizeAndPosition = function(e) {
					if (p.isShowed_bl || e) {
						var t = FWDMSPUtils.getScrollOffsets(),
							o = FWDMSPUtils.getViewportSize();
						p.stageWidth = o.w, p.stageHeight = o.h,
							FWDAnimation.killTweensOf(p.mainHolder_do), p.mainHolder_do.setX(0), p.mainHolder_do.setWidth(p.stageWidth), p.mainHolder_do.setHeight(p.stageHeight), p.setX(t.x), p.setY(t.y), p.setHeight(p.stageHeight), p.isMobile_bl && p.setWidth(p.stageWidth), p.positionButtons(), p.tempId = p.id, p.resizeAndPositionThumbnails(), p.disableEnableNextAndPrevButtons(), p.input_do && (p.input_do.setX(p.stageWidth - p.input_do.getWidth() - p.buttonsMargins), p.input_do.setY(p.stageHeight - p.input_do.getHeight() - p.buttonsMargins), p.inputArrow_do.setX(p.input_do.x + p.input_do.getWidth() - 20), p.inputArrow_do.setY(p.input_do.y + p.input_do.getHeight() / 2 - p.inputArrow_do.getHeight() / 2 - 1))
					}
				}, p.onScrollHandler = function() {
					var e = FWDMSPUtils.getScrollOffsets();
					p.setX(e.x), p.setY(e.y)
				}, this.setupInput = function() {
					p.input_do = new FWDMSPDisplayObject("input"),
						p.input_do.screen.maxLength = 20, p.input_do.getStyle().textAlign = "left", p.input_do.getStyle().outline = "none", p.input_do.getStyle().boxShadow = "none", p.input_do.getStyle().fontSmoothing = "antialiased", p.input_do.getStyle().webkitFontSmoothing = "antialiased", p.input_do.getStyle().textRendering = "optimizeLegibility", p.input_do.getStyle().fontFamily = "Arial", p.input_do.getStyle().fontSize = "12px", p.input_do.getStyle().padding = "6px",
						FWDMSPUtils.isIEAndLessThen9 || (p.input_do.getStyle().paddingRight = "-6px"), p.input_do.getStyle().paddingTop = "2px", p.input_do.getStyle().paddingBottom = "3px", p.input_do.getStyle().backgroundColor = p.inputBackgroundColor_str, p.input_do.getStyle().color = p.inputColor_str, p.input_do.getStyle().borderRadius = "6px",
						p.input_do.screen.value = "search", p.input_do.setHeight(20), p.input_do.setX(18), p.noSearchFound_do = new FWDMSPDisplayObject("div"), p.noSearchFound_do.setX(0), p.noSearchFound_do.getStyle().textAlign = "center", p.noSearchFound_do.getStyle().width = "100%", p.noSearchFound_do.getStyle().fontSmoothing = "antialiased",
						p.noSearchFound_do.getStyle().webkitFontSmoothing = "antialiased", p.noSearchFound_do.getStyle().textRendering = "optimizeLegibility", p.noSearchFound_do.getStyle().fontFamily = "Arial", p.noSearchFound_do.getStyle().fontSize = "12px", p.noSearchFound_do.getStyle().color = p.inputColor_str, p.noSearchFound_do.setInnerHTML("NOTHING FOUND!"), p.noSearchFound_do.setVisible(!1), p.addChild(p.noSearchFound_do);
					var e = new Image;
					e.src = o.inputArrowPath_str, p.inputArrow_do = new FWDMSPDisplayObject("img"), p.inputArrow_do.setScreen(e),
						p.inputArrow_do.setWidth(14), p.inputArrow_do.setHeight(12), p.hasPointerEvent_bl ? p.input_do.screen.addEventListener("pointerdown", p.inputFocusInHandler) : p.input_do.screen.addEventListener && (p.input_do.screen.addEventListener("mousedown", p.inputFocusInHandler), p.input_do.screen.addEventListener("touchstart", p.inputFocusInHandler)), p.input_do.screen.addEventListener("keyup", p.keyUpHandler), p.mainHolder_do.addChild(p.input_do), p.mainHolder_do.addChild(p.inputArrow_do)
				}, this.inputFocusInHandler = function() {
					p.hasInputFocus_bl || (p.hasInputFocus_bl = !0, "search" == p.input_do.screen.value && (p.input_do.screen.value = ""), p.input_do.screen.focus(), setTimeout(function() {
						p.hasPointerEvent_bl ? window.addEventListener("pointerdown", p.inputFocusOutHandler) : window.addEventListener && (window.addEventListener("mousedown", p.inputFocusOutHandler), window.addEventListener("touchstart", p.inputFocusOutHandler))
					}, 50))
				}, this.inputFocusOutHandler = function(e) {
					if (p.hasInputFocus_bl) {
						var t = FWDMSPUtils.getViewportMouseCoordinates(e);
						return FWDMSPUtils.hitTest(p.input_do.screen, t.screenX, t.screenY) ? void 0 : (p.hasInputFocus_bl = !1, void("" == p.input_do.screen.value && (p.input_do.screen.value = "search", p.hasPointerEvent_bl ? window.removeEventListener("pointerdown", p.inputFocusOutHandler) : window.removeEventListener && (window.removeEventListener("mousedown", p.inputFocusOutHandler), window.removeEventListener("touchstart", p.inputFocusOutHandler)))))
					}
				}, this.keyUpHandler = function(e) {
					e.stopPropagation && e.stopPropagation(), p.prevInputValue_str != p.input_do.screen.value && (clearTimeout(p.keyPressedId_to), p.keyPressed_bl = !0, clearTimeout(p.rsId_to), p.rsId_to = setTimeout(function() {
						p.resizeAndPositionThumbnails(!0), p.disableEnableNextAndPrevButtons()
					}, 400)), p.prevInputValue_str = p.input_do.screen.value, p.keyPressedId_to = setTimeout(function() {
						p.keyPressed_bl = !1
					}, 450)
				}, this.showNothingFound = function() {
					p.isShowNothingFound_bl || (p.isShowNothingFound_bl = !0, p.noSearchFound_do.setVisible(!0), p.noSearchFound_do.setY(parseInt((p.stageHeight - p.noSearchFound_do.getHeight()) / 2)), p.noSearchFound_do.setAlpha(0), FWDAnimation.to(p.noSearchFound_do, .1, {
						alpha: 1,
						yoyo: !0,
						repeat: 4
					}))
				}, this.hideNothingFound = function() {
					p.isShowNothingFound_bl && (p.isShowNothingFound_bl = !1, FWDAnimation.killTweensOf(p.noSearchFound_do), p.noSearchFound_do.setVisible(!1))
				}, this.setupDisable = function() {
					p.disable_do = new FWDMSPDisplayObject("div"),
						FWDMSPUtils.isIE && (p.disable_do.setBkColor("#FFFFFF"), p.disable_do.setAlpha(.01)), p.addChild(p.disable_do)
				}, this.showDisable = function() {
					p.disable_do.w != p.stageWidth && (p.disable_do.setWidth(p.stageWidth), p.disable_do.setHeight(p.stageHeight))
				}, this.hideDisable = function() {
					0 != p.disable_do.w && (p.disable_do.setWidth(0), p.disable_do.setHeight(0))
				}, this.setupButtons = function() {
					FWDMSPSimpleButton.setPrototype(), p.closeButton_do = new FWDMSPSimpleButton(p.catCloseN_img, o.catCloseSPath_str, void 0, !0, o.useHEXColorsForSkin_bl, o.normalButtonsColor_str, p.selectedButtonsColor_str), p.closeButton_do.addListener(FWDMSPSimpleButton.MOUSE_UP, p.closeButtonOnMouseUpHandler),
						FWDMSPSimpleButton.setPrototype(), p.nextButton_do = new FWDMSPSimpleButton(p.catNextN_img, o.catNextSPath_str, void 0, !0, o.useHEXColorsForSkin_bl, o.normalButtonsColor_str, p.selectedButtonsColor_str), p.nextButton_do.addListener(FWDMSPSimpleButton.MOUSE_UP, p.nextButtonOnMouseUpHandler),
						FWDMSPSimpleButton.setPrototype(), p.prevButton_do = new FWDMSPSimpleButton(p.catPrevN_img, o.catPrevSPath_str, void 0, !0, o.useHEXColorsForSkin_bl, o.normalButtonsColor_str, p.selectedButtonsColor_str), p.prevButton_do.addListener(FWDMSPSimpleButton.MOUSE_UP, p.prevButtonOnMouseUpHandler)
				}, this.closeButtonOnMouseUpHandler = function() {
					p.hide()
				}, this.nextButtonOnMouseUpHandler = function() {
					var e = p.howManyThumbsToDisplayH * p.howManyThumbsToDisplayV;
					p.tempId += e, p.tempId > p.totalThumbnails - 1 && (p.tempId = p.totalThumbnails - 1);
					var t = Math.floor(p.tempId / e);
					p.tempId = t * e, p.resizeAndPositionThumbnails(!0, "next"),
						p.disableEnableNextAndPrevButtons(!1, !0)
				}, this.prevButtonOnMouseUpHandler = function() {
					var e = p.howManyThumbsToDisplayH * p.howManyThumbsToDisplayV;
					p.tempId -= e, p.tempId < 0 && (p.tempId = 0);
					var t = Math.floor(p.tempId / e);
					p.tempId = t * e, p.resizeAndPositionThumbnails(!0, "prev"),
						p.disableEnableNextAndPrevButtons(!0, !1)
				}, this.positionButtons = function() {
					p.closeButton_do.setX(p.stageWidth - p.closeButton_do.w - p.buttonsMargins),
						p.closeButton_do.setY(p.buttonsMargins), p.nextButton_do.setX(p.stageWidth - p.nextButton_do.w - p.buttonsMargins),
						p.nextButton_do.setY(parseInt((p.stageHeight - p.nextButton_do.h) / 2)), p.prevButton_do.setX(p.buttonsMargins), p.prevButton_do.setY(parseInt((p.stageHeight - p.prevButton_do.h) / 2))
				}, this.disableEnableNextAndPrevButtons = function(e, t) {
					var o = p.howManyThumbsToDisplayH * p.howManyThumbsToDisplayV,
						s = Math.floor(p.tempId / o),
						i = Math.ceil(p.totalThumbnails / o) - 1;
					p.howManyThumbsToDisplayH, p.howManyThumbsToDisplayH;
					o >= p.totalThumbnails ? (p.nextButton_do.disable(), p.prevButton_do.disable(), p.nextButton_do.setDisabledState(), p.prevButton_do.setDisabledState()) : 0 == s ? (p.nextButton_do.enable(), p.prevButton_do.disable(), p.nextButton_do.setEnabledState(), p.prevButton_do.setDisabledState()) : (s == i ? (p.nextButton_do.disable(), p.prevButton_do.enable(), p.nextButton_do.setDisabledState()) : (p.nextButton_do.enable(), p.prevButton_do.enable(), p.nextButton_do.setEnabledState()), p.prevButton_do.setEnabledState()),
						e || p.prevButton_do.setNormalState(),
						t || p.nextButton_do.setNormalState()
				}, this.setupMobileMove = function() {
					p.hasPointerEvent_bl ? p.screen.addEventListener("pointerdown", p.mobileDownHandler) : p.screen.addEventListener("touchstart", p.mobileDownHandler)
				}, this.mobileDownHandler = function(e) {
					if (!e.touches || 1 == e.touches.length) {
						var t = FWDMSPUtils.getViewportMouseCoordinates(e);
						p.mouseX = t.screenX, p.mouseY = t.screenY, p.hasPointerEvent_bl ? (window.addEventListener("pointerup", p.mobileUpHandler), window.addEventListener("pointermove", p.mobileMoveHandler)) : (window.addEventListener("touchend", p.mobileUpHandler), window.addEventListener("touchmove", p.mobileMoveHandler))
					}
				}, this.mobileMoveHandler = function(e) {
					if (e.preventDefault && e.preventDefault(), !e.touches || 1 == e.touches.length) {
						p.showDisable();
						var t = FWDMSPUtils.getViewportMouseCoordinates(e);
						p.dif = p.mouseX - t.screenX, p.mouseX = t.screenX, p.mouseY = t.screenY
					}
				}, this.mobileUpHandler = function(e) {
					p.hideDisable(), 10 < p.dif ? p.nextButtonOnMouseUpHandler() : p.dif < -10 && p.prevButtonOnMouseUpHandler(), p.dif = 0, p.hasPointerEvent_bl ? (window.removeEventListener("pointerup", p.mobileUpHandler), window.removeEventListener("pointermove", p.mobileMoveHandler)) : (window.removeEventListener("touchend", p.mobileUpHandler), window.removeEventListener("touchmove", p.mobileMoveHandler))
				}, this.setupThumbnails = function() {
					if (!p.areThumbnailsCreated_bl) {
						var e;
						p.areThumbnailsCreated_bl = !0;
						for (var t = 0; t < p.totalThumbnails; t++) FWDMSPCategoriesThumb.setPrototype(),
							(e = new FWDMSPCategoriesThumb(p, t, o.catThumbBkPath_str, o.catThumbBkTextPath_str, o.thumbnailSelectedType_str, p.categories_ar[t].htmlContent, p.categories_ar[t].htmlText_str)).addListener(FWDMSPCategoriesThumb.MOUSE_UP, p.thumbnailOnMouseUpHandler), p.thumbs_ar[t] = e, p.mainHolder_do.addChild(e);
						p.mainHolder_do.addChild(p.closeButton_do), p.mainHolder_do.addChild(p.nextButton_do), p.mainHolder_do.addChild(p.prevButton_do)
					}
				}, this.thumbnailOnMouseUpHandler = function(e) {
					p.id = e.id, p.disableOrEnableThumbnails(), p.hide()
				}, this.resizeAndPositionThumbnails = function(e, t) {
					if (p.areThumbnailsCreated_bl) {
						var o, s, i, n, l, r, a, d, u, c = [].concat(p.thumbs_ar);
						if (p.isSearched_bl = !1, p.input_do && (inputValue = p.input_do.screen.value.toLowerCase(), "search" != inputValue))
							for (var h = 0; h < c.length; h++) - 1 == (o = c[h]).htmlText_str.toLowerCase().indexOf(inputValue.toLowerCase()) && (FWDAnimation.killTweensOf(o), o.hide(), c.splice(h, 1), h--);
						p.totalThumbnails = c.length, p.totalThumbnails != p.thumbs_ar.length && (p.isSearched_bl = !0),
							0 == p.totalThumbnails ? p.showNothingFound() : p.hideNothingFound(), this.remainWidthSpace = this.stageWidth - n;
						var _ = p.stageWidth - p.categoriesOffsetTotalWidth,
							f = p.stageHeight - p.categoriesOffsetTotalHeight;
						p.howManyThumbsToDisplayH = Math.ceil((_ - p.spacerH) / (p.thumbnailMaxWidth + p.spacerH)), p.thumbW = Math.floor((_ - p.spacerH * (p.howManyThumbsToDisplayH - 1)) / p.howManyThumbsToDisplayH), p.thumbW > p.thumbnailMaxWidth && (p.howManyThumbsToDisplayH += 1, p.thumbW = Math.floor((_ - p.spacerH * (p.howManyThumbsToDisplayH - 1)) / p.howManyThumbsToDisplayH)), p.thumbH = Math.floor(p.thumbW / p.thumbnailMaxWidth * p.thumbnailMaxHeight), p.howManyThumbsToDisplayV = Math.floor(f / (p.thumbH + p.spacerV)), p.howManyThumbsToDisplayV < 1 && (p.howManyThumbsToDisplayV = 1), n = Math.min(p.howManyThumbsToDisplayH, p.totalThumbnails) * (p.thumbW + p.spacerH) - p.spacerH, l = Math.min(Math.ceil(p.totalThumbnails / p.howManyThumbsToDisplayH), p.howManyThumbsToDisplayV) * (p.thumbH + p.spacerV) - p.spacerV,
							r = p.howManyThumbsToDisplayH > p.totalThumbnails ? 0 : _ - n, p.howManyThumbsToDisplayH > p.totalThumbnails && (p.howManyThumbsToDisplayH = p.totalThumbnails), u = p.howManyThumbsToDisplayH * p.howManyThumbsToDisplayV, s = Math.floor(p.tempId / u), p.isSearched_bl && (s = 0), d = p.howManyThumbsToDisplayH * s,
							firstId = s * u, (a = firstId + u) > p.totalThumbnails && (a = p.totalThumbnails);
						for (h = 0; h < p.totalThumbnails; h++)(o = c[h]).finalW = p.thumbW, h % p.howManyThumbsToDisplayH == p.howManyThumbsToDisplayH - 1 && (o.finalW += r), o.finalH = p.thumbH, o.finalX = h % p.howManyThumbsToDisplayH * (p.thumbW + p.spacerH), o.finalX += Math.floor(h / u) * p.howManyThumbsToDisplayH * (p.thumbW + p.spacerH), o.finalX += (p.stageWidth - n) / 2, o.finalX = Math.floor(o.finalX - d * (p.thumbW + p.spacerH)), o.finalY = h % u, o.finalY = Math.floor(o.finalY / p.howManyThumbsToDisplayH) * (p.thumbH + p.spacerV), o.finalY += (f - l) / 2, o.finalY += p.categoriesOffsetTotalHeight / 2, o.finalY = Math.floor(o.finalY), s < (i = Math.floor(h / u)) ? o.finalX += 150 : i < s && (o.finalX -= 150), e ? h >= firstId && h < a ? (dl = "next" == t ? h % u * p.delayRate + .1 : (u - h % u) * p.delayRate + .1, p.keyPressed_bl && (dl = 0), o.resizeAndPosition(!0, dl)) : o.resizeAndPosition(!0, 0) : o.resizeAndPosition(), o.show();
						p.howManyThumbsToDisplayH * p.howManyThumbsToDisplayV >= p.totalThumbnails ? (p.nextButton_do.setVisible(!1), p.prevButton_do.setVisible(!1)) : (p.nextButton_do.setVisible(!0), p.prevButton_do.setVisible(!0))
					}
				}, this.loadImages = function() {
					p.countLoadedThumbs > p.totalThumbnails - 1 || (p.image_img && (p.image_img.onload = null, p.image_img.onerror = null), p.image_img = new Image, p.image_img.onerror = p.onImageLoadError, p.image_img.onload = p.onImageLoadComplete, p.image_img.src = p.categories_ar[p.countLoadedThumbs].thumbnailPath)
				}, this.onImageLoadError = function(e) {}, this.onImageLoadComplete = function(e) {
					p.thumbs_ar[p.countLoadedThumbs].setImage(p.image_img), p.countLoadedThumbs++, p.loadWithDelayId_to = setTimeout(p.loadImages, 40)
				}, this.disableOrEnableThumbnails = function() {
					for (var e, t = 0; t < p.totalThumbnails; t++) e = p.thumbs_ar[t], t == p.id ? e.disable() : e.enable()
				}, this.show = function(e) {
					p.isShowed_bl || (p.isShowed_bl = !0, p.isOnDOM_bl = !0, p.id = e, FWDMSPUtils.isChrome && p.isMobile_bl ? p.setVisible(!0) : FWDMSPUtils.isIEAndLessThen9 ? document.getElementsByTagName("body")[0].appendChild(p.screen) : document.documentElement.appendChild(p.screen), window.addEventListener ? window.addEventListener("scroll", p.onScrollHandler) : window.attachEvent && window.attachEvent("onscroll", p.onScrollHandler), p.setupThumbnails(), p.resizeAndPosition(!0), p.showDisable(), p.disableOrEnableThumbnails(), clearTimeout(p.hideCompleteId_to), clearTimeout(p.showCompleteId_to), p.mainHolder_do.setY(-p.stageHeight), p.isMobile_bl ? (p.showCompleteId_to = setTimeout(p.showCompleteHandler, 1200), FWDAnimation.to(p.mainHolder_do, .8, {
						y: 0,
						delay: .4,
						ease: Expo.easeInOut
					})) : (p.showCompleteId_to = setTimeout(p.showCompleteHandler, 800), FWDAnimation.to(p.mainHolder_do, .8, {
						y: 0,
						ease: Expo.easeInOut
					})))
				}, this.showCompleteHandler = function() {
					p.mainHolder_do.setY(0), p.hideDisable(),
						FWDMSPUtils.isIphone, p.resizeAndPosition(!0),
						p.areThumbnailsLoaded_bl || (p.loadImages(), p.areThumbnailsLoaded_bl = !0)
				}, this.hide = function() {
					p.isShowed_bl && (p.isShowed_bl = !1, FWDMSPUtils.isIphone, clearTimeout(p.hideCompleteId_to), clearTimeout(p.showCompleteId_to), p.showDisable(), p.hideCompleteId_to = setTimeout(p.hideCompleteHandler, 800), FWDAnimation.killTweensOf(p.mainHolder_do), FWDAnimation.to(p.mainHolder_do, .8, {
						y: -p.stageHeight,
						ease: Expo.easeInOut
					}), window.addEventListener ? window.removeEventListener("scroll", p.onScrollHandler) : window.detachEvent && window.detachEvent("onscroll", p.onScrollHandler), p.resizeAndPosition())
				}, this.hideCompleteHandler = function() {
					FWDMSPUtils.isChrome && p.isMobile_bl ? p.setVisible(!1) : FWDMSPUtils.isIEAndLessThen9 ? document.getElementsByTagName("body")[0].removeChild(p.screen) : document.documentElement.removeChild(p.screen), p.isOnDOM_bl = !1, p.dispatchEvent(t.HIDE_COMPLETE)
				}, this.updateHEXColors = function(e, t) {
					-1 != o.skinPath_str.indexOf("hex_white") ? p.selectedColor_str = "#FFFFFF" : p.selectedColor_str = t, p.closeButton_do.updateHEXColors(e, p.selectedColor_str),
						p.nextButton_do.updateHEXColors(e, p.selectedColor_str), p.prevButton_do.updateHEXColors(e, p.selectedColor_str)
				}, this.init()
		};
		t.setPrototype = function() {
				t.prototype = new FWDMSPDisplayObject("div")
			}, t.HIDE_COMPLETE = "hideComplete", t.prototype = null,
			window.FWDMSPCategories = t
	}(),
	function(e) {
		var a = function(t, e, o, s, i, n, l) {
			var r = this;
			a.prototype;
			this.backgroundImagePath_str = o, this.catThumbTextBkPath_str = s, this.canvas_el = null, this.htmlContent = n, this.simpleText_do = null, this.effectImage_do = null, this.imageHolder_do = null, this.normalImage_do = null, this.effectImage_do = null, this.dumy_do = null, this.htmlText_str = l, this.thumbnailSelectedType_str = i, this.id = e, this.imageOriginalW, this.imageOriginalH, this.finalX,
				this.finalY, this.finalW, this.finalH, this.imageFinalX, this.imageFinalY, this.imageFinalW,
				this.imageFinalH, this.dispatchShowWithDelayId_to,
				this.isShowed_bl = !1, this.hasImage_bl = !1, this.isSelected_bl = !1, this.isDisabled_bl = !1, this.hasCanvas_bl = FWDMSP.hasCanvas, this.isMobile_bl = FWDMSPUtils.isMobile, this.hasPointerEvent_bl = FWDMSPUtils.hasPointerEvent,
				this.init = function() {
					r.getStyle().background = "url('" + r.backgroundImagePath_str + "')", r.setupMainContainers(), r.setupDescription(), r.setupDumy()
				}, this.setupMainContainers = function() {
					r.imageHolder_do = new FWDMSPDisplayObject("div"), r.addChild(r.imageHolder_do)
				}, this.setupDumy = function() {
					r.dumy_do = new FWDMSPDisplayObject("div"),
						FWDMSPUtils.isIE && (r.dumy_do.setBkColor("#FFFFFF"), r.dumy_do.setAlpha(0)), r.addChild(r.dumy_do)
				},
				this.setupDescription = function() {
					r.simpleText_do = new FWDMSPDisplayObject("div"),
					r.simpleText_do.getStyle().background = "url('" + r.catThumbTextBkPath_str + "')",
					FWDMSPUtils.isFirefox && (r.simpleText_do.hasTransform3d_bl = !1,
						                        r.simpleText_do.hasTransform2d_bl = !1),
				  r.simpleText_do.getStyle().width = "100%",
				  r.simpleText_do.getStyle().fontFamily = "Arial",
				  r.simpleText_do.getStyle().fontSize = "12px",
				  r.simpleText_do.getStyle().textAlign = "left",
				  r.simpleText_do.getStyle().color = "#FFFFFF",
				  r.simpleText_do.getStyle().fontSmoothing = "antialiased",
				  r.simpleText_do.getStyle().webkitFontSmoothing = "antialiased",
				  r.simpleText_do.getStyle().textRendering = "optimizeLegibility",
				  r.simpleText_do.setInnerHTML(r.htmlContent),
				  r.addChild(r.simpleText_do)
				}, this.positionDescription = function() {
					r.simpleText_do.setY(parseInt(r.finalH - r.simpleText_do.getHeight()))
				}, this.setupBlackAndWhiteImage = function(e) {
					if (r.hasCanvas_bl && "opacity" != r.thumbnailSelectedType_str) {
						var t = document.createElement("canvas"),
							o = t.getContext("2d");
						t.width = r.imageOriginalW, t.height = r.imageOriginalH, o.drawImage(e, 0, 0);
						var s = o.getImageData(0, 0, t.width, t.height),
							i = s.data;
						if ("threshold" == r.thumbnailSelectedType_str)
							for (var n = 0; n < i.length; n += 4) {
								var l = 150 <= .2126 * i[n] + .7152 * i[n + 1] + .0722 * i[n + 2] ? 255 : 0;
								i[n] = i[n + 1] = i[n + 2] = l
							}
						else if ("blackAndWhite" == r.thumbnailSelectedType_str)
							for (n = 0; n < i.length; n += 4) {
								l = .2126 * i[n] + .7152 * i[n + 1] + .0722 * i[n + 2];
								i[n] = i[n + 1] = i[n + 2] = l
							}
						o.putImageData(s, 0, 0, 0, 0, s.width, s.height), r.effectImage_do = new FWDMSPDisplayObject("canvas"), r.effectImage_do.screen = t,
							r.effectImage_do.setAlpha(.9), r.effectImage_do.setMainProperties()
					}
				}, this.setImage = function(e) {
					r.normalImage_do = new FWDMSPDisplayObject("img"), r.normalImage_do.setScreen(e),
						r.imageOriginalW = r.normalImage_do.w, r.imageOriginalH = r.normalImage_do.h, r.setButtonMode(!0), r.setupBlackAndWhiteImage(e), r.resizeImage(), r.imageHolder_do.setX(parseInt(r.finalW / 2)), r.imageHolder_do.setY(parseInt(r.finalH / 2)), r.imageHolder_do.setWidth(0), r.imageHolder_do.setHeight(0), r.normalImage_do.setX(-parseInt(r.normalImage_do.w / 2)), r.normalImage_do.setY(-parseInt(r.normalImage_do.h / 2)), r.normalImage_do.setAlpha(0), r.effectImage_do && (r.effectImage_do.setX(-parseInt(r.normalImage_do.w / 2)), r.effectImage_do.setY(-parseInt(r.normalImage_do.h / 2)), r.effectImage_do.setAlpha(.01)),
						FWDAnimation.to(r.imageHolder_do, .8, {
							x: 0,
							y: 0,
							w: r.finalW,
							h: r.finalH,
							ease: Expo.easeInOut
						}), FWDAnimation.to(r.normalImage_do, .8, {
							alpha: 1,
							x: r.imageFinalX,
							y: r.imageFinalY,
							ease: Expo.easeInOut
						}), r.effectImage_do && FWDAnimation.to(r.effectImage_do, .8, {
							x: r.imageFinalX,
							y: r.imageFinalY,
							ease: Expo.easeInOut
						}), r.isMobile_bl ? r.hasPointerEvent_bl ? (r.screen.addEventListener("pointerup", r.onMouseUp), r.screen.addEventListener("pointerover", r.onMouseOver), r.screen.addEventListener("pointerout", r.onMouseOut)) : r.screen.addEventListener("mouseup", r.onMouseUp) : r.screen.addEventListener ? (r.screen.addEventListener("mouseover", r.onMouseOver), r.screen.addEventListener("mouseout", r.onMouseOut), r.screen.addEventListener("mouseup", r.onMouseUp)) : r.screen.attachEvent && (r.screen.attachEvent("onmouseover", r.onMouseOver), r.screen.attachEvent("onmouseout", r.onMouseOut), r.screen.attachEvent("onmouseup", r.onMouseUp)), this.imageHolder_do.addChild(r.normalImage_do), r.effectImage_do && r.imageHolder_do.addChild(r.effectImage_do),
						this.hasImage_bl = !0, r.id == t.id && r.disable()
				}, r.onMouseOver = function(e, t) {
					r.isDisabled_bl || e.pointerType && e.pointerType != e.MSPOINTER_TYPE_MOUSE || r.setSelectedState(!0)
				}, r.onMouseOut = function(e) {
					r.isDisabled_bl || e.pointerType && e.pointerType != e.MSPOINTER_TYPE_MOUSE || r.setNormalState(!0)
				}, r.onMouseUp = function(e) {
					r.isDisabled_bl || 2 == e.button || (e.preventDefault && e.preventDefault(), r.dispatchEvent(a.MOUSE_UP, {
						id: r.id
					}))
				}, this.resizeAndPosition = function(e, t) {
					FWDAnimation.killTweensOf(r), FWDAnimation.killTweensOf(r.imageHolder_do), e ? FWDAnimation.to(r, .8, {
							x: r.finalX,
							y: r.finalY,
							delay: t,
							ease: Expo.easeInOut
						}) : (r.setX(r.finalX), r.setY(r.finalY)),
						r.setWidth(r.finalW), r.setHeight(r.finalH), r.imageHolder_do.setX(0), r.imageHolder_do.setY(0), r.imageHolder_do.setWidth(r.finalW), r.imageHolder_do.setHeight(r.finalH), r.dumy_do.setWidth(r.finalW), r.dumy_do.setHeight(r.finalH), r.resizeImage(), r.positionDescription()
				}, this.resizeImage = function(e) {
					if (r.normalImage_do) {
						FWDAnimation.killTweensOf(r.normalImage_do);
						var t, o = r.finalW / r.imageOriginalW,
							s = r.finalH / r.imageOriginalH;
						t = s <= o ? o : s, r.imageFinalW = Math.ceil(t * r.imageOriginalW), r.imageFinalH = Math.ceil(t * r.imageOriginalH), r.imageFinalX = Math.round((r.finalW - r.imageFinalW) / 2), r.imageFinalY = Math.round((r.finalH - r.imageFinalH) / 2), r.effectImage_do && (FWDAnimation.killTweensOf(r.effectImage_do), r.effectImage_do.setX(r.imageFinalX), r.effectImage_do.setY(r.imageFinalY), r.effectImage_do.setWidth(r.imageFinalW), r.effectImage_do.setHeight(r.imageFinalH), r.isDisabled_bl && r.setSelectedState(!1, !0)), r.normalImage_do.setX(r.imageFinalX), r.normalImage_do.setY(r.imageFinalY), r.normalImage_do.setWidth(r.imageFinalW), r.normalImage_do.setHeight(r.imageFinalH), r.isDisabled_bl ? r.normalImage_do.setAlpha(.3) : r.normalImage_do.setAlpha(1)
					}
				}, this.setNormalState = function(e) {
					r.isSelected_bl && (r.isSelected_bl = !1, "threshold" == r.thumbnailSelectedType_str || "blackAndWhite" == r.thumbnailSelectedType_str ? e ? FWDAnimation.to(r.effectImage_do, 1, {
						alpha: .01,
						ease: Quart.easeOut
					}) : r.effectImage_do.setAlpha(.01) : "opacity" == r.thumbnailSelectedType_str && (e ? FWDAnimation.to(r.normalImage_do, 1, {
						alpha: 1,
						ease: Quart.easeOut
					}) : r.normalImage_do.setAlpha(1)))
				}, this.setSelectedState = function(e, t) {
					r.isSelected_bl && !t || (r.isSelected_bl = !0, "threshold" == r.thumbnailSelectedType_str || "blackAndWhite" == r.thumbnailSelectedType_str ? e ? FWDAnimation.to(r.effectImage_do, 1, {
						alpha: 1,
						ease: Expo.easeOut
					}) : r.effectImage_do.setAlpha(1) : "opacity" == r.thumbnailSelectedType_str && (e ? FWDAnimation.to(r.normalImage_do, 1, {
						alpha: .3,
						ease: Expo.easeOut
					}) : r.normalImage_do.setAlpha(.3)))
				}, this.show = function() {
					FWDAnimation.to(r, .8, {
						scale: 1,
						ease: Expo.easeInOut
					})
				}, this.hide = function() {
					FWDAnimation.to(r, .8, {
						scale: 0,
						ease: Expo.easeInOut
					})
				}, this.enable = function() {
					r.hasImage_bl && (r.isDisabled_bl = !1, r.setButtonMode(!0), r.setNormalState(!0))
				}, this.disable = function() {
					r.hasImage_bl && (r.isDisabled_bl = !0, r.setButtonMode(!1), r.setSelectedState(!0))
				}, this.init()
		};
		a.setPrototype = function() {
			a.prototype = new FWDMSPTransformDisplayObject("div")
		}, a.MOUSE_UP = "onMouseUp", a.prototype = null, e.FWDMSPCategoriesThumb = a
	}(window),
	function(r) {
		var t = function(i, n) {
			var l = this,
				e = t.prototype;
			this.categories_ar = n.categories_ar,
			this.buttons_ar = [],
			this.mainHolder_do = null,
			this.selector_do = null,
			this.mainButtonsHolder_do = null,
			this.buttonsHolder_do = null,
			this.arrowW = n.arrowW,
			this.arrowH = n.arrowH,
			l.useHEXColorsForSkin_bl = i.data.useHEXColorsForSkin_bl,
			l.normalButtonsColor_str = i.data.normalButtonsColor_str,
			l.selectedButtonsColor_str = i.data.selectedButtonsColor_str,
			this.arrowN_str = n.arrowN_str,
			this.arrowS_str = n.arrowS_str,
			this.bk1_str = n.bk1_str,
			this.bk2_str = n.bk2_str,
			this.selectorLabel_str = n.selectorLabel,
			this.selectorBkColorN_str = n.selectorBackgroundNormalColor,
			this.selectorBkColorS_str = n.selectorBackgroundSelectedColor,
			this.selectorTextColorN_str = n.selectorTextNormalColor,
			this.selectorTextColorS_str = n.selectorTextSelectedColor,
			this.itemBkColorN_str = n.buttonBackgroundNormalColor,
			this.itemBkColorS_str = n.buttonBackgroundSelectedColor,
			this.itemTextColorN_str = n.buttonTextNormalColor,
			this.itemTextColorS_str = n.buttonTextSelectedColor,
			this.scrollBarHandlerFinalY = 0,
			this.finalX,
			this.finalY,
			this.totalButtons = l.categories_ar.length,
			this.curId = n.startAtPlaylist,
			this.buttonsHolderWidth = 0,
			this.buttonsHolderHeight = 0,
			this.totalWidth = i.stageWidth,
			this.buttonHeight = n.buttonHeight,
				this.totalButtonsHeight = 0,
				this.sapaceBetweenButtons = 0,
				this.thumbnailsFinalY = 0,
				this.vy = 0,
				this.vy2 = 0,
				this.friction = .9,
				this.hideMenuTimeOutId_to,
				this.getMaxWidthResizeAndPositionId_to,
				this.isShowed_bl = !1,
				this.addMouseWheelSupport_bl = i.data.addScrollBarMouseWheelSupport_bl,
				this.scollbarSpeedSensitivity = .5,
				this.isOpened_bl = !1,
				this.hasPointerEvent_bl = FWDMSPUtils.hasPointerEvent,
				this.isMobile_bl = FWDMSPUtils.isMobile,
				this.init = function() {
					l.setOverflow("visible"),
					l.setupMainContainers(),
					l.setupScrollLogic(),
					l.getMaxWidthResizeAndPosition(),
					l.setupSeparator(),
					l.mainButtonsHolder_do.setVisible(!1),
					l.bk_do.setVisible(!1)
				},
				this.setupSeparator = function() {
					l.separator_do = new FWDMSPDisplayObject("div"),
					l.separator_do.hasTransform3d_bl = !1,
					l.separator_do.hasTransform2d_bl = !1,
					l.separator_do.getStyle().background = "url('" + i.playlistSeparator_img.src + "')",
					l.separator_do.setHeight(i.playlistSeparator_img.height),
					l.separator_do.setY(l.buttonHeight),
					l.addChild(l.separator_do)
				},
				this.setupMainContainers = function() {
					var e;
					if (l.mainHolder_do = new FWDMSPDisplayObject("div"),
					    l.mainHolder_do.setOverflow("visible"),
							l.addChild(l.mainHolder_do),
							l.bk_do = new FWDMSPDisplayObject("div"),
							l.bk_do.setY(l.buttonHeight),
							l.bk_do.setBkColor(i.playlistBackgroundColor_str),
							l.bk_do.setAlpha(0),
							l.mainHolder_do.addChild(l.bk_do),
							l.mainButtonsHolder_do = new FWDMSPDisplayObject("div"),
							l.mainButtonsHolder_do.setY(l.buttonHeight),
							l.mainHolder_do.addChild(l.mainButtonsHolder_do),
							i.expandPlaylistBackground_bl) {
								l.dummyBk_do = new FWDMSPDisplayObject("img");
								var t = new Image;
								t.src = i.controllerBkPath_str,
								l.dummyBk_do.setScreen(t),
								l.dummyBk_do.getStyle().backgroundColor = "#000000"
					}
					else l.dummyBk_do = new FWDMSPDisplayObject("div"),
					     l.dummyBk_do.getStyle().background = "url('" + i.controllerBkPath_str + "')";
							 l.dummyBk_do.setHeight(l.buttonHeight),
							 l.mainHolder_do.addChild(l.dummyBk_do),
							 l.buttonsHolder_do = new FWDMSPDisplayObject("div"),
							 l.mainButtonsHolder_do.addChild(l.buttonsHolder_do);
					var o = l.selectorLabel_str;
					"default" == l.selectorLabel_str && (o = l.categories_ar[l.curId]),
					FWDMSPComboBoxSelector.setPrototype(),
					l.selector_do = new FWDMSPComboBoxSelector(11, 6, n.arrowN_str,
						                                         n.arrowS_str, o,
																										 l.selectorBkColorN_str,
																										 l.selectorBkColorS_str,
																										 l.selectorTextColorN_str,
																										 l.selectorTextColorS_str,
																										 l.buttonHeight,
																										 l.useHEXColorsForSkin_bl,
																										 l.normalButtonsColor_str,
																										 l.selectedButtonsColor_str),
					l.mainHolder_do.addChild(l.selector_do),
					l.selector_do.setNormalState(!1),
					l.selector_do.addListener(FWDMSPComboBoxSelector.MOUSE_DOWN, l.openMenuHandler);
					for (var s = 0; s < l.totalButtons; s++)
					   FWDMSPComboBoxButton.setPrototype(),
						 e = new FWDMSPComboBoxButton(l, l.categories_ar[s],
							                            l.bk1_str,
																					l.bk2_str,
																					l.itemBkColorN_str,
																					l.itemBkColorS_str,
																					l.itemTextColorN_str,
																					l.itemTextColorS_str, s,
																					l.buttonHeight),
						(l.buttons_ar[s] = e).addListener(FWDMSPComboBoxButton.MOUSE_DOWN,
							                                l.buttonOnMouseDownHandler),
																							l.buttonsHolder_do.addChild(e)
				},
				this.buttonOnMouseDownHandler = function(e) {
					l.curId = e.target.id,
					clearTimeout(l.hideMenuTimeOutId_to),
					l.hide(!1),
					l.selector_do.enable(),
					l.isMobile_bl ? l.hasPointerEvent_bl
					                 ? r.removeEventListener("MSPointerDown", l.checkOpenedMenu)
													 : r.removeEventListener("touchstart", l.checkOpenedMenu)
												: r.addEventListener
												   ? (r.removeEventListener("mousedown", l.checkOpenedMenu),
													    r.removeEventListener("mousemove", l.checkOpenedMenu))
													 : document.attachEvent && document.detachEvent("onmousemove", l.checkOpenedMenu),
													   i.data.showPlaylistsSelectBoxNumbers_bl ? l.selector_do.setText(l.buttons_ar[l.curId].label1_str.substr(4))
														                                         : l.selector_do.setText(l.buttons_ar[l.curId].label1_str),
													 l.isButtonCliecked_bl = !0,
						l.dispatchEvent(t.BUTTON_PRESSED, {
							id: l.curId
						})
				},
				this.openMenuHandler = function(e) {
					FWDAnimation.isTweening(l.mainButtonsHolder_do) || (l.isShowed_bl ? l.checkOpenedMenu(e.e, !0)
					                                                                  : (l.selector_do.disable(),
																																						   l.show(!0),
																																							 l.startToCheckOpenedMenu(),
																																							 l.dispatchEvent(t.OPEN)))
				},
				this.setButtonsStateBasedOnId = function(e) {
					l.curId = e;
					for (var t = 0; t < l.totalButtons; t++)
					   button_do = l.buttons_ar[t],
						 t == l.curId ? button_do.disable() : button_do.enable();
						i.data.showPlaylistsSelectBoxNumbers_bl ? l.selector_do.setText(l.buttons_ar[l.curId].label1_str.substr(4))
						                                        : l.selector_do.setText(l.buttons_ar[l.curId].label1_str),
																										  l.scrHandler_do ? (l.updateScrollBarSizeActiveAndDeactivate(), l.updateScrollBarHandlerAndContent(!1, !0))
																											                : l.thumbnailsFinalY = 0
				},
				this.setValue = function(e) {
					l.curId = e,
					l.setButtonsStateBasedOnId()
				},
				this.startToCheckOpenedMenu = function(e) {
					l.isMobile_bl ? l.hasPointerEvent_bl ? r.addEventListener("MSPointerDown", l.checkOpenedMenu) : r.addEventListener("touchstart", l.checkOpenedMenu) : r.addEventListener ? r.addEventListener("mousedown", l.checkOpenedMenu) : document.attachEvent && document.attachEvent("onmousemove", l.checkOpenedMenu)
				},
				this.checkOpenedMenu = function(e, t) {
					e.preventDefault && e.preventDefault();
					var o = FWDMSPUtils.getViewportMouseCoordinates(e),
						s = 1e3;
					"mousedown" == e.type && (s = 0),
					!FWDMSPUtils.hitTest(l.screen, o.screenX, o.screenY) && !FWDMSPUtils.hitTest(l.mainButtonsHolder_do.screen, o.screenX, o.screenY) || t ? (l.isMobile_bl
						                                                                                                                                          ? (l.hide(!0),
																																																																											   l.selector_do.enable())
																																																																											: (clearTimeout(l.hideMenuTimeOutId_to),
																																																																											  l.hideMenuTimeOutId_to = setTimeout(function() {
																																																																													l.hide(!0), l.selector_do.enable()
																																																																												}, s)),
																																																																										l.isMobile_bl ? l.hasPointerEvent_bl
																																																																										                ? r.removeEventListener("MSPointerDown", l.checkOpenedMenu)
																																																																																		: r.removeEventListener("touchstart", l.checkOpenedMenu)
																																																																																	: r.addEventListener
																																																																																	  ? (r.removeEventListener("mousemove", l.checkOpenedMenu),
																																																																																		   r.removeEventListener("mousedown", l.checkOpenedMenu))
																																																																																		: document.attachEvent && document.detachEvent("onmousemove", l.checkOpenedMenu))
																																																																	               : clearTimeout(l.hideMenuTimeOutId_to)
				},
				l.getMaxWidthResizeAndPosition = function() {
					for (var e, t = l.totalButtonsHeight = 0;
						   t < l.totalButtons; t++)
						 (e = l.buttons_ar[t]).setY(1 + t * (e.totalHeight + l.sapaceBetweenButtons)),
						 l.allowToScrollAndScrollBarIsActive_bl && !l.isMobile_bl ? l.totalWidth = i.stageWidth - 6
						 																													: l.totalWidth = i.stageWidth,
																																			  e.totalWidth = l.totalWidth,
																																				e.setWidth(l.totalWidth),
																																				e.centerText();
					l.totalButtonsHeight = e.getY() + e.totalHeight - l.sapaceBetweenButtons,
					l.dummyBk_do.setWidth(l.totalWidth + 6),
					l.setWidth(l.totalWidth),
					l.setHeight(l.buttonHeight),
					l.selector_do.totalWidth = l.totalWidth + 6,
					l.selector_do.setWidth(l.totalWidth + 6),
					l.selector_do.centerText(),
					l.buttonsHolder_do.setWidth(l.totalWidth),
					l.buttonsHolder_do.setHeight(l.totalButtonsHeight)
				},
				this.position = function() {
					FWDMSPUtils.isAndroid ? (l.setX(Math.floor(l.finalX)),
					                         l.setY(Math.floor(l.finalY - 1)),
																	 setTimeout(l.poscombo - box, 100))
																: (l.poscombo, box())
				},
				this.resizeAndPosition = function() {
					l.stageWidth = i.stageWidth,
					l.stageHeight = i.stageHeight,
					l.bk_do.setWidth(l.stageWidth),
					l.bk_do.setHeight(l.stageHeight),
					l.mainButtonsHolder_do.setWidth(l.stageWidth),
					l.mainButtonsHolder_do.setHeight(l.stageHeight),
					l.totalButtonsHeight > l.mainButtonsHolder_do.h ? l.allowToScrollAndScrollBarIsActive_bl = !0
					                                                : l.allowToScrollAndScrollBarIsActive_bl = !1,
					!l.allowToScrollAndScrollBarIsActive_bl && l.scrMainHolder_do ? l.scrMainHolder_do.setVisible(!1)
					                                                              : l.allowToScrollAndScrollBarIsActive_bl && l.scrMainHolder_do && l.isShowed_bl && l.scrMainHolder_do.setVisible(!0),
																																				  l.separator_do.setWidth(l.stageWidth),
																																					l.scrHandler_do && l.updateScrollBarSizeActiveAndDeactivate(),
																																					this.getMaxWidthResizeAndPosition(),
																																					l.updateScrollBarHandlerAndContent()
				},
				this.hide = function(e, t) {
					(l.isShowed_bl || t) && (FWDAnimation.killTweensOf(this),
					                         l.isShowed_bl = !1,
																	 FWDAnimation.killTweensOf(l.mainButtonsHolder_do),
																	 FWDAnimation.killTweensOf(l.bk_do), e ? (FWDAnimation.to(l.mainButtonsHolder_do, .8, {
																		 																					y: -l.totalButtonsHeight,
																																							ease: Expo.easeInOut,
																																							onComplete: l.hideComplete
																																						}), FWDAnimation.to(l.bk_do, .8, {
																																							alpha: 0
																																						}))
																																					: (l.bk_do.setVisible(!1),
																																					   l.mainButtonsHolder_do.setY(l.buttonHeight - l.totalButtonsHeight),
																																						 l.bk_do.setAlpha(0),
																																						 l.setHeight(l.buttonHeight),
																																						 l.hideComplete()))
				},
				this.hideComplete = function() {
					l.mainButtonsHolder_do.setVisible(!1),
					l.bk_do.setVisible(!1)
				},
				this.show = function(e, t) {
					l.isShowed_bl && !t || (FWDAnimation.killTweensOf(this),
					                        l.mainButtonsHolder_do.setY(-l.totalButtonsHeight),
																	l.isShowed_bl = !0,
																	l.mainButtonsHolder_do.setVisible(!0),
																	l.bk_do.setVisible(!0),
																	l.resizeAndPosition(),
																	FWDAnimation.killTweensOf(l.mainButtonsHolder_do),
																	FWDAnimation.killTweensOf(l.bk_do),
																	l.scrMainHolder_do && l.allowToScrollAndScrollBarIsActive_bl && l.scrMainHolder_do.setVisible(!0),
																	e ? (FWDAnimation.to(l.bk_do, .8, {
																			alpha: 1
																			}), FWDAnimation.to(l.mainButtonsHolder_do, .8, {
																				y: l.buttonHeight,
																				ease: Expo.easeInOut
																			}))
																		: (l.bk_do.setAlpha(1), l.mainButtonsHolder_do.setY(l.buttonHeight)))
				},
				this.setupScrollLogic = function() {
					l.isMobile_bl ? l.setupMobileScrollbar() : (l.setupScrollbar(), l.addMouseWheelSupport_bl && l.addMouseWheelSupport())
				}, this.setupMobileScrollbar = function() {
					l.hasPointerEvent_bl ? l.mainButtonsHolder_do.screen.addEventListener("pointerdown", l.scrollBarTouchStartHandler) : l.mainButtonsHolder_do.screen.addEventListener("touchstart", l.scrollBarTouchStartHandler), l.mainButtonsHolder_do.screen.addEventListener("mousedown", l.scrollBarTouchStartHandler), l.updateMobileScrollBarId_int = setInterval(l.updateMobileScrollBar, 16)
				}, this.scrollBarTouchStartHandler = function(e) {
					e.preventDefault && e.preventDefault(), l.isScrollingOnMove_bl = !1, FWDAnimation.killTweensOf(l.buttonsHolder_do);
					var t = FWDMSPUtils.getViewportMouseCoordinates(e);
					l.isDragging_bl = !0, l.lastPresedY = t.screenY, l.checkLastPresedY = t.screenY,
						l.hasPointerEvent_bl ? (r.addEventListener("pointerup", l.scrollBarTouchEndHandler), r.addEventListener("pointermove", l.scrollBarTouchMoveHandler)) : (r.addEventListener("touchend", l.scrollBarTouchEndHandler), r.addEventListener("touchmove", l.scrollBarTouchMoveHandler)), r.addEventListener("mouseup", l.scrollBarTouchEndHandler), r.addEventListener("mousemove", l.scrollBarTouchMoveHandler),
						clearInterval(l.updateMoveMobileScrollbarId_int), l.updateMoveMobileScrollbarId_int = setInterval(l.updateMoveMobileScrollbar, 20)
				}, this.scrollBarTouchMoveHandler = function(e) {
					if (e.preventDefault && e.preventDefault(), e.stopImmediatePropagation(), !(l.totalButtonsHeight < l.mainButtonsHolder_do.h)) {
						i.showDisable();
						var t = FWDMSPUtils.getViewportMouseCoordinates(e);
						(t.screenY >= l.checkLastPresedY + 6 || t.screenY <= l.checkLastPresedY - 6) && (l.isScrollingOnMove_bl = !0);
						var o = t.screenY - l.lastPresedY;
						l.thumbnailsFinalY += o, l.thumbnailsFinalY = Math.round(l.thumbnailsFinalY), l.lastPresedY = t.screenY, l.vy = 2 * o
					}
				}, this.scrollBarTouchEndHandler = function(e) {
					l.isDragging_bl = !1, clearInterval(l.updateMoveMobileScrollbarId_int),
						clearTimeout(l.disableOnMoveId_to), l.disableOnMoveId_to = setTimeout(function() {
							i.hideDisable()
						}, 100), l.hasPointerEvent_bl ? (r.removeEventListener("pointerup", l.scrollBarTouchEndHandler), r.removeEventListener("pointermove", l.scrollBarTouchMoveHandler)) : (r.removeEventListener("touchend", l.scrollBarTouchEndHandler), r.removeEventListener("touchmove", l.scrollBarTouchMoveHandler)), r.removeEventListener("mousemove", l.scrollBarTouchMoveHandler)
				}, this.updateMoveMobileScrollbar = function() {
					l.buttonsHolder_do.setY(l.thumbnailsFinalY)
				}, this.updateMobileScrollBar = function(e) {
					l.isDragging_bl || (l.totalButtonsHeight < l.mainButtonsHolder_do.h && (l.thumbnailsFinalY = .01), l.vy *= l.friction, l.thumbnailsFinalY += l.vy, 0 < l.thumbnailsFinalY ? (l.vy2 = .3 * (0 - l.thumbnailsFinalY), l.vy *= l.friction, l.thumbnailsFinalY += l.vy2) : l.thumbnailsFinalY < l.mainButtonsHolder_do.h - l.totalButtonsHeight && (l.vy2 = .3 * (l.mainButtonsHolder_do.h - l.totalButtonsHeight - l.thumbnailsFinalY), l.vy *= l.friction, l.thumbnailsFinalY += l.vy2), l.buttonsHolder_do.setY(Math.round(l.thumbnailsFinalY)))
				}, this.setupScrollbar = function() {
					l.scrMainHolder_do = new FWDMSPDisplayObject("div"), l.scrMainHolder_do.setVisible(!1), l.scrMainHolder_do.setWidth(i.scrWidth), l.scrTrack_do = new FWDMSPDisplayObject("div"), l.scrTrack_do.setWidth(i.scrWidth);
					var e = new Image;
					e.src = i.playlistScrBkTop_img.src, l.scrTrackTop_do = new FWDMSPDisplayObject("img"), l.scrTrackTop_do.setWidth(i.scrTrackTop_do.w), l.scrTrackTop_do.setHeight(i.scrTrackTop_do.h), l.scrTrackTop_do.setScreen(e), l.scrTrackMiddle_do = new FWDMSPDisplayObject("div"),
						l.scrTrackMiddle_do.getStyle().background = "url('" + i.data.scrBkMiddlePath_str + "')", l.scrTrackMiddle_do.setWidth(i.scrWidth), l.scrTrackMiddle_do.setY(l.scrTrackTop_do.h);
					var t = new Image;
					t.src = i.data.scrBkBottomPath_str, l.scrTrackBottom_do = new FWDMSPDisplayObject("img"), l.scrTrackBottom_do.setScreen(t), l.scrTrackBottom_do.setWidth(l.scrTrackTop_do.w), l.scrTrackBottom_do.setHeight(l.scrTrackTop_do.h), l.scrHandler_do = new FWDMSPDisplayObject("div"), l.scrHandler_do.setWidth(i.scrWidth), l.playlistScrDragTop_img = new Image, l.playlistScrDragTop_img.src = i.data.scrDragBottomPath_str, l.playlistScrDragTop_img.width = i.playlistScrDragTop_img.width, l.playlistScrDragTop_img.height = i.playlistScrDragTop_img.height, l.scrHandlerTop_do = new FWDMSPDisplayObject("img"), l.useHEXColorsForSkin_bl ? (l.scrHandlerTop_do = new FWDMSPDisplayObject("div"), l.scrHandlerTop_do.setWidth(l.playlistScrDragTop_img.width), l.scrHandlerTop_do.setHeight(l.playlistScrDragTop_img.height), l.mainScrubberDragTop_canvas = FWDMSPUtils.getCanvasWithModifiedColor(l.playlistScrDragTop_img, l.normalButtonsColor_str).canvas, l.scrHandlerTop_do.screen.appendChild(l.mainScrubberDragTop_canvas)) : (l.scrHandlerTop_do = new FWDMSPDisplayObject("img"), l.scrHandlerTop_do.setScreen(l.playlistScrDragTop_img)), l.scrHandlerMiddle_do = new FWDMSPDisplayObject("div"), l.middleImage = new Image, l.middleImage.src = i.data.scrDragMiddlePath_str, l.useHEXColorsForSkin_bl ? l.middleImage.onload = function() {
							l.scrubberDragMiddle_canvas = FWDMSPUtils.getCanvasWithModifiedColor(l.middleImage, l.normalButtonsColor_str, !0),
								l.scrubberDragImage_img = l.scrubberDragMiddle_canvas.image,
								l.scrHandlerMiddle_do.getStyle().background = "url('" + l.scrubberDragImage_img.src + "') repeat-y"
						} : l.scrHandlerMiddle_do.getStyle().background = "url('" + i.data.scrDragMiddlePath_str + "')", l.scrHandlerMiddle_do.setWidth(i.scrWidth),
						l.scrHandlerMiddle_do.setY(l.scrHandlerTop_do.h), l.scrHandlerBottom_do = new FWDMSPDisplayObject("div"), l.bottomImage = new Image,
						l.bottomImage.src = i.data.scrDragMiddlePath_str, l.useHEXColorsForSkin_bl ? l.bottomImage.onload = function() {
							l.scrubberDragBottom_canvas = FWDMSPUtils.getCanvasWithModifiedColor(l.bottomImage, l.normalButtonsColor_str, !0),
								l.scrubberDragBottomImage_img = l.scrubberDragBottom_canvas.image,
								l.scrHandlerBottom_do.getStyle().background = "url('" + l.scrubberDragBottomImage_img.src + "') repeat-y"
						} : l.scrHandlerBottom_do.getStyle().background = "url('" + i.playlistScrDragTop_img.src + "')", l.scrHandlerBottom_do.setWidth(i.scrWidth),
						l.scrHandlerBottom_do.setY(l.scrHandlerTop_do.h), console.log(),
						console.log(), l.scrHandlerBottom_do.setWidth(l.scrHandlerTop_do.w), l.scrHandlerBottom_do.setHeight(l.scrHandlerTop_do.h), l.scrHandler_do.setButtonMode(!0), l.playlistScrLines_img = new Image, l.playlistScrLines_img.src = i.playlistScrLines_img.src, l.playlistScrLines_img.width = i.playlistScrLines_img.width, l.playlistScrLines_img.height = i.playlistScrLines_img.height, l.useHEXColorsForSkin_bl ? (l.scrHandlerLinesN_do = new FWDMSPDisplayObject("div"), l.scrHandlerLinesN_do.setWidth(l.playlistScrLines_img.width), l.scrHandlerLinesN_do.setHeight(l.playlistScrLines_img.height), l.mainhandlerN_canvas = FWDMSPUtils.getCanvasWithModifiedColor(l.playlistScrLines_img, l.normalButtonsColor_str).canvas, l.scrHandlerLinesN_do.screen.appendChild(l.mainhandlerN_canvas)) : (l.scrHandlerLinesN_do = new FWDMSPDisplayObject("img"), l.scrHandlerLinesN_do.setScreen(l.playlistScrLines_img)), l.scrHandlerLinesS_img = new Image, l.scrHandlerLinesS_img.src = i.data.scrLinesSPath_str, l.useHEXColorsForSkin_bl ? (l.scrHandlerLinesS_do = new FWDMSPDisplayObject("div"), l.scrHandlerLinesS_img.onload = function() {
							l.scrHandlerLinesS_do.setWidth(l.scrHandlerLinesN_do.w), l.scrHandlerLinesS_do.setHeight(l.scrHandlerLinesN_do.h), l.scrubberLines_s_canvas = FWDMSPUtils.getCanvasWithModifiedColor(l.scrHandlerLinesS_img, l.selectedButtonsColor_str, !0), l.scrubbelinesSImage_img = l.scrubberLines_s_canvas.image,
								l.scrHandlerLinesS_do.getStyle().background = "url('" + l.scrubbelinesSImage_img.src + "') repeat-y"
						}) : (l.scrHandlerLinesS_do = new FWDMSPDisplayObject("img"), l.scrHandlerLinesS_do.setScreen(l.scrHandlerLinesS_img), l.scrHandlerLinesS_do.setWidth(l.scrHandlerLinesN_do.w), l.scrHandlerLinesS_do.setHeight(l.scrHandlerLinesN_do.h)), l.scrHandlerLinesS_do.setAlpha(0), l.scrHandlerLines_do = new FWDMSPDisplayObject("div"), l.scrHandlerLines_do.setWidth(l.scrHandlerLinesN_do.w), l.scrHandlerLines_do.setHeight(l.scrHandlerLinesN_do.h), l.scrHandlerLines_do.setButtonMode(!0), l.scrTrack_do.addChild(l.scrTrackTop_do), l.scrTrack_do.addChild(l.scrTrackMiddle_do), l.scrTrack_do.addChild(l.scrTrackBottom_do), l.scrHandler_do.addChild(l.scrHandlerTop_do), l.scrHandler_do.addChild(l.scrHandlerMiddle_do), l.scrHandler_do.addChild(l.scrHandlerBottom_do), l.scrHandlerLines_do.addChild(l.scrHandlerLinesN_do), l.scrHandlerLines_do.addChild(l.scrHandlerLinesS_do), l.scrMainHolder_do.addChild(l.scrTrack_do),
						l.scrMainHolder_do.addChild(l.scrHandler_do), l.scrMainHolder_do.addChild(l.scrHandlerLines_do), l.mainButtonsHolder_do.addChild(l.scrMainHolder_do), l.scrHandler_do.screen.addEventListener ? (l.scrHandler_do.screen.addEventListener("mouseover", l.scrollBarHandlerOnMouseOver), l.scrHandler_do.screen.addEventListener("mouseout", l.scrollBarHandlerOnMouseOut), l.scrHandler_do.screen.addEventListener("mousedown", l.scrollBarHandlerOnMouseDown), l.scrHandlerLines_do.screen.addEventListener("mouseover", l.scrollBarHandlerOnMouseOver), l.scrHandlerLines_do.screen.addEventListener("mouseout", l.scrollBarHandlerOnMouseOut), l.scrHandlerLines_do.screen.addEventListener("mousedown", l.scrollBarHandlerOnMouseDown)) : l.scrHandler_do.screen.attachEvent && (l.scrHandler_do.screen.attachEvent("onmouseover", l.scrollBarHandlerOnMouseOver), l.scrHandler_do.screen.attachEvent("onmouseout", l.scrollBarHandlerOnMouseOut), l.scrHandler_do.screen.attachEvent("onmousedown", l.scrollBarHandlerOnMouseDown), l.scrHandlerLines_do.screen.attachEvent("onmouseover", l.scrollBarHandlerOnMouseOver), l.scrHandlerLines_do.screen.attachEvent("onmouseout", l.scrollBarHandlerOnMouseOut), l.scrHandlerLines_do.screen.attachEvent("onmousedown", l.scrollBarHandlerOnMouseDown))
				}, this.scrollBarHandlerOnMouseOver = function(e) {
					l.allowToScrollAndScrollBarIsActive_bl && (FWDAnimation.killTweensOf(l.scrHandlerLinesN_do), FWDAnimation.killTweensOf(l.scrHandlerLinesS_do), FWDAnimation.to(l.scrHandlerLinesN_do, .8, {
						alpha: 0,
						ease: Expo.easeOut
					}), FWDAnimation.to(l.scrHandlerLinesS_do, .8, {
						alpha: 1,
						ease: Expo.easeOut
					}))
				}, this.scrollBarHandlerOnMouseOut = function(e) {
					!l.isDragging_bl && l.allowToScrollAndScrollBarIsActive_bl && (FWDAnimation.killTweensOf(l.scrHandlerLinesN_do), FWDAnimation.killTweensOf(l.scrHandlerLinesS_do), FWDAnimation.to(l.scrHandlerLinesN_do, .8, {
						alpha: 1,
						ease: Expo.easeOut
					}), FWDAnimation.to(l.scrHandlerLinesS_do, .8, {
						alpha: 0,
						ease: Expo.easeOut
					}))
				}, this.scrollBarHandlerOnMouseDown = function(e) {
					if (l.allowToScrollAndScrollBarIsActive_bl) {
						var t = FWDMSPUtils.getViewportMouseCoordinates(e);
						l.isDragging_bl = !0, l.yPositionOnPress = l.scrHandler_do.y, l.lastPresedY = t.screenY, FWDAnimation.killTweensOf(l.scrHandler_do),
							i.showDisable(),
							r.addEventListener ? (r.addEventListener("mousemove", l.scrollBarHandlerMoveHandler), r.addEventListener("mouseup", l.scrollBarHandlerEndHandler)) : document.attachEvent && (document.attachEvent("onmousemove", l.scrollBarHandlerMoveHandler), document.attachEvent("onmouseup", l.scrollBarHandlerEndHandler))
					}
				}, this.scrollBarHandlerMoveHandler = function(e) {
					e.preventDefault && e.preventDefault();
					var t = FWDMSPUtils.getViewportMouseCoordinates(e),
						o = l.scrollBarHandlerFinalY + parseInt((l.scrHandler_do.h - l.scrHandlerLines_do.h) / 2);
					l.scrollBarHandlerFinalY = Math.round(l.yPositionOnPress + t.screenY - l.lastPresedY), l.scrollBarHandlerFinalY >= l.scrTrack_do.h - l.scrHandler_do.h ? l.scrollBarHandlerFinalY = l.scrTrack_do.h - l.scrHandler_do.h : l.scrollBarHandlerFinalY <= 0 && (l.scrollBarHandlerFinalY = 0), l.scrHandler_do.setY(l.scrollBarHandlerFinalY), FWDAnimation.killTweensOf(l.scrHandler_do),
						FWDAnimation.to(l.scrHandlerLines_do, .8, {
							y: o,
							ease: Quart.easeOut
						}), l.updateScrollBarHandlerAndContent(!0)
				}, l.scrollBarHandlerEndHandler = function(e) {
					var t = FWDMSPUtils.getViewportMouseCoordinates(e);
					l.isDragging_bl = !1, FWDMSPUtils.hitTest(l.scrHandler_do.screen, t.screenX, t.screenY) || (FWDAnimation.killTweensOf(l.scrHandlerLinesN_do), FWDAnimation.killTweensOf(l.scrHandlerLinesS_do), FWDAnimation.to(l.scrHandlerLinesN_do, .8, {
							alpha: 1,
							ease: Expo.easeOut
						}), FWDAnimation.to(l.scrHandlerLinesS_do, .8, {
							alpha: 0,
							ease: Expo.easeOut
						})), i.hideDisable(), FWDAnimation.killTweensOf(l.scrHandler_do),
						FWDAnimation.to(l.scrHandler_do, .4, {
							y: l.scrollBarHandlerFinalY,
							ease: Quart.easeOut
						}), r.removeEventListener ? (r.removeEventListener("mousemove", l.scrollBarHandlerMoveHandler), r.removeEventListener("mouseup", l.scrollBarHandlerEndHandler)) : document.detachEvent && (document.detachEvent("onmousemove", l.scrollBarHandlerMoveHandler), document.detachEvent("onmouseup", l.scrollBarHandlerEndHandler))
				}, this.updateScrollBarSizeActiveAndDeactivate = function() {
					l.disableForAWhileAfterThumbClick_bl || (l.allowToScrollAndScrollBarIsActive_bl ? (l.allowToScrollAndScrollBarIsActive_bl = !0, l.scrMainHolder_do.setX(l.stageWidth - l.scrMainHolder_do.w), l.scrMainHolder_do.setHeight(l.mainButtonsHolder_do.h), l.scrTrack_do.setHeight(l.scrMainHolder_do.h), l.scrTrackMiddle_do.setHeight(l.scrTrack_do.h - 2 * l.scrTrackTop_do.h), l.scrTrackBottom_do.setY(l.scrTrackMiddle_do.y + l.scrTrackMiddle_do.h), l.scrMainHolder_do.setAlpha(1), l.scrHandler_do.setButtonMode(!0), l.scrHandlerLines_do.setButtonMode(!0)) : (l.allowToScrollAndScrollBarIsActive_bl = !1, l.scrMainHolder_do.setX(l.stageWidth - l.scrMainHolder_do.w), l.scrMainHolder_do.setHeight(l.mainButtonsHolder_do.h), l.scrTrack_do.setHeight(l.scrMainHolder_do.h), l.scrTrackMiddle_do.setHeight(l.scrTrack_do.h - 2 * l.scrTrackTop_do.h), l.scrTrackBottom_do.setY(l.scrTrackMiddle_do.y + l.scrTrackMiddle_do.h), l.scrMainHolder_do.setAlpha(.5), l.scrHandler_do.setY(0), l.scrHandler_do.setButtonMode(!1), l.scrHandlerLines_do.setButtonMode(!1)), l.scrHandler_do.setHeight(Math.max(120, Math.round(Math.min(1, l.scrMainHolder_do.h / l.totalButtonsHeight) * l.scrMainHolder_do.h))), l.scrHandlerMiddle_do.setHeight(l.scrHandler_do.h - 2 * l.scrHandlerTop_do.h), FWDAnimation.killTweensOf(l.scrHandlerLines_do), l.scrHandlerLines_do.setY(l.scrollBarHandlerFinalY + parseInt((l.scrHandler_do.h - l.scrHandlerLines_do.h) / 2)), l.scrHandlerBottom_do.setY(l.scrHandler_do.h - l.scrHandlerBottom_do.h - 1))
				}, this.addMouseWheelSupport = function() {
					l.screen.addEventListener ? (l.screen.addEventListener("DOMMouseScroll", l.mouseWheelHandler), l.screen.addEventListener("mousewheel", l.mouseWheelHandler)) : l.screen.attachEvent && l.screen.attachEvent("onmousewheel", l.mouseWheelHandler)
				}, l.mouseWheelHandler = function(e) {
					if (e.preventDefault && e.preventDefault(), l.disableMouseWheel_bl || l.isDragging_bl) return !1;
					var t = e.detail || e.wheelDelta;
					e.wheelDelta && (t *= -1), 0 < t ? l.scrollBarHandlerFinalY += Math.round(160 * l.scollbarSpeedSensitivity * (l.mainButtonsHolder_do.h / l.totalButtonsHeight)) : t < 0 && (l.scrollBarHandlerFinalY -= Math.round(160 * l.scollbarSpeedSensitivity * (l.mainButtonsHolder_do.h / l.totalButtonsHeight))), l.scrollBarHandlerFinalY >= l.scrTrack_do.h - l.scrHandler_do.h ? l.scrollBarHandlerFinalY = l.scrTrack_do.h - l.scrHandler_do.h : l.scrollBarHandlerFinalY <= 0 && (l.scrollBarHandlerFinalY = 0);
					var o = l.scrollBarHandlerFinalY + parseInt((l.scrHandler_do.h - l.scrHandlerLines_do.h) / 2);
					if (FWDAnimation.killTweensOf(l.scrHandler_do), FWDAnimation.killTweensOf(l.scrHandlerLines_do), FWDAnimation.to(l.scrHandlerLines_do, .8, {
							y: o,
							ease: Quart.easeOut
						}), FWDAnimation.to(l.scrHandler_do, .5, {
							y: l.scrollBarHandlerFinalY,
							ease: Quart.easeOut
						}), l.isDragging_bl = !0, l.updateScrollBarHandlerAndContent(!0), l.isDragging_bl = !1, !e.preventDefault) return !1;
					e.preventDefault()
				}, this.updateScrollBarHandlerAndContent = function(e, t) {
					if (!l.disableForAWhileAfterThumbClick_bl && (l.allowToScrollAndScrollBarIsActive_bl || t)) {
						var o = 0;
						l.isDragging_bl && !l.isMobile_bl ? ("Infinity" == (o = l.scrollBarHandlerFinalY / (l.scrMainHolder_do.h - l.scrHandler_do.h)) ? o = 0 : 1 <= o && (scrollPercent = 1), l.thumbnailsFinalY = -1 * Math.round(o * (l.totalButtonsHeight - l.mainButtonsHolder_do.h))) : (o = l.curId / (l.totalButtons - 1), l.thumbnailsFinalY = Math.min(0, -1 * Math.round(o * (l.totalButtonsHeight - l.mainButtonsHolder_do.h))), l.scrMainHolder_do && (l.scrollBarHandlerFinalY = Math.round((l.scrMainHolder_do.h - l.scrHandler_do.h) * o), l.scrollBarHandlerFinalY < 0 ? l.scrollBarHandlerFinalY = 0 : l.scrollBarHandlerFinalY > l.scrMainHolder_do.h - l.scrHandler_do.h - 1 && (l.scrollBarHandlerFinalY = l.scrMainHolder_do.h - l.scrHandler_do.h - 1), FWDAnimation.killTweensOf(l.scrHandler_do), FWDAnimation.killTweensOf(l.scrHandlerLines_do), e ? (FWDAnimation.to(l.scrHandler_do, .4, {
							y: l.scrollBarHandlerFinalY,
							ease: Quart.easeOut
						}), FWDAnimation.to(l.scrHandlerLines_do, .8, {
							y: l.scrollBarHandlerFinalY + parseInt((l.scrHandler_do.h - l.scrHandlerLinesN_do.h) / 2),
							ease: Quart.easeOut
						})) : (l.scrHandler_do.setY(l.scrollBarHandlerFinalY), l.scrHandlerLines_do.setY(l.scrollBarHandlerFinalY + parseInt((l.scrHandler_do.h - l.scrHandlerLinesN_do.h) / 2))))), l.lastThumbnailFinalY != l.thumbnailsFinalY && (FWDAnimation.killTweensOf(l.buttonsHolder_do), e ? FWDAnimation.to(l.buttonsHolder_do, .5, {
							y: l.thumbnailsFinalY,
							ease: Quart.easeOut
						}) : l.buttonsHolder_do.setY(l.thumbnailsFinalY)), l.lastThumbnailFinalY = l.thumbnailsFinalY
					}
				}, this.init(), this.destroy = function() {
					l.isMobile_bl ? (r.removeEventListener("MSPointerDown", l.checkOpenedMenu), r.removeEventListener("touchstart", l.checkOpenedMenu)) : r.removeEventListener ? r.removeEventListener("mousemove", l.checkOpenedMenu) : document.detachEvent && document.detachEvent("onmousemove", l.checkOpenedMenu),
						clearTimeout(l.hideMenuTimeOutId_to),
						clearTimeout(l.getMaxWidthResizeAndPositionId_to),
						FWDAnimation.killTweensOf(l), FWDAnimation.killTweensOf(l.mainHolder_do),
						FWDAnimation.killTweensOf(l.buttonsHolder_do), FWDAnimation.killTweensOf(l.mainButtonsHolder_do), l.mainHolder_do.destroy(), l.selector_do.destroy(), l.mainButtonsHolder_do.destroy(), l.buttonsHolder_do.destroy(),
						l.categories_ar = null, l.buttons_ar = null, l.mainHolder_do = null, l.selector_do = null, l.mainButtonsHolder_do = null, l.buttonsHolder_do = null, l.upArrowN_img = null, l.upArrowS_img = null, n = i = null,
						l.setInnerHTML(""), e.destroy(), e = l = null, t.prototype = null
				}
		};
		t.setPrototype = function() {
			t.prototype = new FWDMSPDisplayObject("div")
		},
		t.OPEN = "open",
		t.HIDE_COMPLETE = "infoWindowHideComplete",
		t.BUTTON_PRESSED = "buttonPressed",
		t.prototype = null,
		r.FWDMSPComboBox = t
	}(window),

	function() {
		var h = function(t, e, o, s, i, n, l, r, a, d) {
			var u = this,
				c = h.prototype;
			this.bk_sdo = null,
			this.text_sdo = null,
			this.dumy_sdo = null,
			this.label1_str = e,
			this.backgroundNormalColor_str = i,
			this.backgroundSelectedColor_str = n,
			this.textNormalColor_str = l,
			this.textSelectedColor_str = r,
			this.bk1_str = o,
			this.bk2_str = s,
			this.totalWidth = 400,
			this.totalHeight = d,
			this.id = a,
			this.hasPointerEvent_bl = FWDMSPUtils.hasPointerEvent,
			this.isMobile_bl = FWDMSPUtils.isMobile,
			this.isDisabled_bl = !1,
			u.init = function() {
					u.setButtonMode(!0),
					u.setupMainContainers(),
					u.setWidth(u.totalWidth),
					u.setHeight(u.totalHeight),
					u.setNormalState()
				},
				u.setupMainContainers = function() {
					u.bk_sdo = new FWDMSPDisplayObject("div"),
					u.bk_sdo.setBkColor(u.backgroundNormalColor_str),
					u.id % 2 == 0 ? u.bk_sdo.getStyle().background = "url('" + u.bk1_str + "')" : (u.bk_sdo.getStyle().background = "url('" + u.bk2_str + "')", u.type = 2),
					u.addChild(u.bk_sdo),
					u.text_sdo = new FWDMSPDisplayObject("div"),
					u.text_sdo.getStyle().whiteSpace = "nowrap",
					u.text_sdo.setOverflow("visible"),
					u.text_sdo.setDisplay("inline-block"),
					u.text_sdo.getStyle().fontFamily = "Arial",
					u.text_sdo.getStyle().fontSize = "13px",
					u.text_sdo.getStyle().padding = "6px",
					u.text_sdo.getStyle().fontWeight = "100",
					u.text_sdo.getStyle().color = u.normalColor_str,
					u.text_sdo.getStyle().fontSmoothing = "antialiased",
					u.text_sdo.getStyle().webkitFontSmoothing = "antialiased",
					u.text_sdo.getStyle().textRendering = "optimizeLegibility",
					FWDMSPUtils.isIEAndLessThen9 ? u.text_sdo.screen.innerText = u.label1_str : u.text_sdo.setInnerHTML(u.label1_str),
					u.addChild(u.text_sdo),
					u.dumy_sdo = new FWDMSPDisplayObject("div"),
					FWDMSPUtils.isIE && (u.dumy_sdo.setBkColor("#FF0000"), u.dumy_sdo.setAlpha(0)),
					u.addChild(u.dumy_sdo),
					u.isMobile_bl ? u.hasPointerEvent_bl
					              ? (u.screen.addEventListener("MSPointerOver", u.onMouseOver),
												   u.screen.addEventListener("MSPointerOut", u.onMouseOut),
													 u.screen.addEventListener("MSPointerDown", u.onMouseDown),
													 u.screen.addEventListener("MSPointerUp", u.onClick))
												: u.screen.addEventListener("touchend", u.onMouseDown)
												: u.screen.addEventListener
												? (u.screen.addEventListener("mouseover", u.onMouseOver),
													u.screen.addEventListener("mouseout", u.onMouseOut),
													u.screen.addEventListener("click", u.onMouseDown),
													u.screen.addEventListener("click", u.onClick))
												: u.screen.attachEvent && (u.screen.attachEvent("onmouseover", u.onMouseOver),
																									u.screen.attachEvent("onmouseout", u.onMouseOut),
																									u.screen.attachEvent("onmousedown", u.onMouseDown),
																									u.screen.attachEvent("onclick", u.onClick))
				},
				u.onMouseOver = function(e) {
					u.isDisabled_bl || e.pointerType && e.pointerType != e.MSPOINTER_TYPE_MOUSE || (FWDAnimation.killTweensOf(u.text_sdo), u.setSelectedState(!0), u.dispatchEvent(h.MOUSE_OVER))
				},
				u.onMouseOut = function(e) {
					u.isDisabled_bl || e.pointerType && e.pointerType != e.MSPOINTER_TYPE_MOUSE || (FWDAnimation.killTweensOf(u.text_sdo), u.setNormalState(!0), u.dispatchEvent(h.MOUSE_OUT))
				},
				u.onClick = function(e) {
					u.isDisabled_bl || (e.preventDefault && e.preventDefault(), u.dispatchEvent(h.CLICK))
				},
				u.onMouseDown = function(e) {
					u.isDisabled_bl || t.isScrollingOnMove_bl || (e.preventDefault && e.preventDefault(), u.dispatchEvent(h.MOUSE_DOWN, {
						e: e
					}))
				},
				this.setSelectedState = function(e) {
					e ? FWDAnimation.to(u.text_sdo.screen, .6, {
						css: {
							color: u.textSelectedColor_str
						},
						ease: Quart.easeOut
					}) : u.text_sdo.getStyle().color = u.textSelectedColor_str
				},
				this.setNormalState = function(e) {
					e ? FWDAnimation.to(u.text_sdo.screen, .6, {
						css: {
							color: u.textNormalColor_str
						},
						ease: Quart.easeOut
					}) : u.text_sdo.getStyle().color = u.textNormalColor_str
				},
				u.centerText = function() {
					u.dumy_sdo.setWidth(u.totalWidth),
					u.dumy_sdo.setHeight(u.totalHeight),
					u.bk_sdo.setWidth(u.totalWidth),
					u.bk_sdo.setHeight(u.totalHeight),
					u.text_sdo.setX(4),
					u.text_sdo.setY(Math.round((u.totalHeight - u.text_sdo.getHeight()) / 2))
				},
				u.getMaxTextWidth = function() {
					return u.text_sdo.getWidth()
				},
				this.disable = function() {
					u.isDisabled_bl = !0,
					u.setButtonMode(!1),
					u.setSelectedState(!0)
				},
				this.enable = function() {
					u.isDisabled_bl = !1,
					u.setNormalState(!0),
					u.setButtonMode(!0)
				},
				u.destroy = function() {
					u.isMobile_bl ? u.hasPointerEvent_bl
					              ? (u.screen.removeEventListener("MSPointerOver", u.onMouseOver),
												   u.screen.removeEventListener("MSPointerOut", u.onMouseOut),
													 u.screen.removeEventListener("MSPointerDown", u.onMouseDown),
													 u.screen.removeEventListener("MSPointerUp", u.onClick))
												: u.screen.removeEventListener("touchstart", u.onMouseDown)
												: u.screen.removeEventListener
												? (u.screen.removeEventListener("mouseover", u.onMouseOver),
													u.screen.removeEventListener("mouseout", u.onMouseOut),
													u.screen.removeEventListener("mousedown", u.onMouseDown),
													u.screen.removeEventListener("click", u.onClick))
												: u.screen.detachEvent && (u.screen.detachEvent("onmouseover", u.onMouseOver),
					u.screen.detachEvent("onmouseout", u.onMouseOut),
					u.screen.detachEvent("onmousedown", u.onMouseDown),
					u.screen.detachEvent("onclick", u.onClick)),
					FWDAnimation.killTweensOf(u.text_sdo.screen),
					FWDAnimation.killTweensOf(u.bk_sdo.screen),
					u.text_sdo.destroy(),
					u.bk_sdo.destroy(),
					u.dumy_sdo.destroy(),
					u.bk_sdo = null,
					u.text_sdo = null,
					u.dumy_sdo = null,
					u.label1_str = null,
					u.normalColor_str = null,
					u.textSelectedColor_str = null,
					u.disabledColor_str = null,
					u.setInnerHTML(""),
					c.destroy(),
					c = u = null,
					h.prototype = null
				},
				u.init()
		};
		h.setPrototype = function() {
			h.prototype = new FWDMSPDisplayObject("div")
		},
		h.FIRST_BUTTON_CLICK = "onFirstClick",
		h.SECOND_BUTTON_CLICK = "secondButtonOnClick",
	  h.MOUSE_OVER = "onMouseOver",
		h.MOUSE_OUT = "onMouseOut",
		h.MOUSE_DOWN = "onMouseDown",
		h.CLICK = "onClick",
		h.prototype = null,
		window.FWDMSPComboBoxButton = h
	}(window),

	function() {
		var p = function(e, t, o, s, i, n, l, r, a, d, u, c, h) {
			var _ = this,
			f = p.prototype;
			this.arrow_do = null,
			this.arrowN_sdo = null,
			this.arrowS_sdo = null,
			this.arrowN_str = o,
			this.arrowS_str = s,
			this.label1_str = i,
			this.backgroundNormalColor_str = n,
			this.backgroundSelectedColor_str = l,
			this.textNormalColor_str = r,
			this.textSelectedColor_str = a,
			_.useHEXColorsForSkin_bl = u,
			_.normalButtonsColor_str = c,
			_.selectedButtonsColor_str = h,
			this.totalWidth = 400,
			this.totalHeight = d,
			this.arrowWidth = e,
			this.arrowHeight = t,
			this.bk_sdo = null,
			this.text_sdo = null,
			this.dumy_sdo = null,
			this.hasPointerEvent_bl = FWDMSPUtils.hasPointerEvent,
			this.isMobile_bl = FWDMSPUtils.isMobile,
			this.isDisabled_bl = !1,
			_.init = function() {
					_.setButtonMode(!0),
					_.setupMainContainers(),
					_.setWidth(_.totalWidth),
					_.setHeight(_.totalHeight)
				},
				_.setupMainContainers = function() {
					_.bk_sdo = new FWDMSPDisplayObject("div"),
					_.bk_sdo.getStyle().backgroundColor = _.backgroundNormalColor_str,
					_.addChild(_.bk_sdo),
					_.text_sdo = new FWDMSPDisplayObject("div"),
					_.text_sdo.getStyle().whiteSpace = "nowrap",
					_.text_sdo.setOverflow("visible"),
					_.text_sdo.setDisplay("inline-block"),
					_.text_sdo.getStyle().fontFamily = "Arial",
					_.text_sdo.getStyle().fontSize = "13px",
					_.text_sdo.getStyle().fontWeight = "100",
					_.text_sdo.getStyle().padding = "6px",
					_.text_sdo.getStyle().color = _.normalColor_str,
					_.text_sdo.getStyle().fontSmoothing = "antialiased",
					_.text_sdo.getStyle().webkitFontSmoothing = "antialiased",
					_.text_sdo.getStyle().textRendering = "optimizeLegibility",
					FWDMSPUtils.isIEAndLessThen9 ? _.text_sdo.screen.innerText = _.label1_str : _.text_sdo.setInnerHTML(_.label1_str),
					_.addChild(_.text_sdo),
					_.arrow_do = new FWDMSPDisplayObject("div"),
					_.arrow_do.setOverflow("visible"),
					_.useHEXColorsForSkin_bl ? (_.arrowN_img = new Image,
						                          _.arrowN_img.src = _.arrowN_str,
																			_.arrowS_img = new Image,
																			_.arrowS_img.src =
																			_.arrowS_str,
																			_.arrowN_sdo = new FWDMSPDisplayObject("div"),
																			_.arrowS_sdo = new FWDMSPDisplayObject("div"),
																			_.arrowN_img.onload = function() {
																				_.arrowN_sdo.setWidth(_.arrowN_img.width),
																				_.arrowN_sdo.setHeight(_.arrowN_img.height),
																				_.scrubberLines_n_canvas = FWDMSPUtils.getCanvasWithModifiedColor(_.arrowN_img, _.normalButtonsColor_str, !0),
																				_.scrubbelinesNImage_img = _.scrubberLines_n_canvas.image,
																				_.arrowN_sdo.getStyle().background = "url('" + _.scrubbelinesNImage_img.src + "') repeat-y",
																				_.arrowS_sdo.setWidth(_.arrowS_img.width),
																				_.arrowS_sdo.setHeight(_.arrowS_img.height),
																				_.scrubberLines_s_canvas = FWDMSPUtils.getCanvasWithModifiedColor(_.arrowS_img, _.selectedButtonsColor_str, !0),
																				_.scrubbelinesSImage_img = _.scrubberLines_s_canvas.image,
																				_.arrowS_sdo.getStyle().background = "url('" + _.scrubbelinesSImage_img.src + "') repeat-y"
																			})
																		: (_.arrowN_sdo = new FWDMSPDisplayObject("div"),
																		   _.arrowN_sdo.screen.style.backgroundImage = "url(" + _.arrowN_str + ")",
																			 _.arrowS_sdo = new FWDMSPDisplayObject("div"),
																			 _.arrowS_sdo.screen.style.backgroundImage = "url(" + _.arrowS_str + ")"),
					_.arrowS_sdo.setAlpha(0),
					_.arrow_do.addChild(_.arrowN_sdo),
					_.arrow_do.addChild(_.arrowS_sdo),
					_.addChild(_.arrow_do),
					_.arrowN_sdo.setWidth(_.arrowWidth),
					_.arrowN_sdo.setHeight(_.arrowHeight),
					_.arrowS_sdo.setWidth(_.arrowWidth),
					_.arrowS_sdo.setHeight(_.arrowHeight),
					_.dumy_sdo = new FWDMSPDisplayObject("div"),
					FWDMSPUtils.isIE && (_.dumy_sdo.setBkColor("#FF0000"), _.dumy_sdo.setAlpha(0)),
					_.addChild(_.dumy_sdo),
					_.isMobile_bl ? _.hasPointerEvent_bl
					              ? (_.screen.addEventListener("MSPointerOver", _.onMouseOver),
											    _.screen.addEventListener("MSPointerOut", _.onMouseOut),
													_.screen.addEventListener("MSPointerDown", _.onMouseDown),
													_.screen.addEventListener("MSPointerUp", _.onClick))
											 : _.screen.addEventListener("touchend", _.onMouseDown)
											 : _.screen.addEventListener
											 ? (_.screen.addEventListener("mouseover", _.onMouseOver),
											 		_.screen.addEventListener("mouseout", _.onMouseOut),
													_.screen.addEventListener("mousedown", _.onMouseDown),
													_.screen.addEventListener("click", _.onClick))
											 : _.screen.attachEvent && (_.screen.attachEvent("onmouseover", _.onMouseOver),
											 _.screen.attachEvent("onmouseout", _.onMouseOut),
											 _.screen.attachEvent("onmousedown", _.onMouseDown),
											 _.screen.attachEvent("onclick", _.onClick))
				},
				_.onMouseOver = function(e) {
					_.isDisabled_bl || e.pointerType && e.pointerType != e.MSPOINTER_TYPE_MOUSE || (FWDAnimation.killTweensOf(_.text_sdo), _.setSelectedState(!0, 0), _.dispatchEvent(p.MOUSE_OVER))
				},
				_.onMouseOut = function(e) {
					_.isDisabled_bl || e.pointerType && e.pointerType != e.MSPOINTER_TYPE_MOUSE || (FWDAnimation.killTweensOf(_.text_sdo), _.setNormalState(!0, !0), _.dispatchEvent(p.MOUSE_OUT))
				},
				_.onClick = function(e) {
					_.isDeveleper_bl ? window.open("", "_blank") : _.isDisabled_bl ||
					                                                               (e.preventDefault && e.preventDefault(), _.dispatchEvent(p.CLICK))
				},
				_.onMouseDown = function(e) {
					e.preventDefault && e.preventDefault(),
					_.dispatchEvent(p.MOUSE_DOWN, {
						e: e
					})
				},
				this.setSelectedState = function(e, t) {
					FWDAnimation.killTweensOf(_.bk_sdo),
					FWDAnimation.killTweensOf(_.text_sdo),
					FWDAnimation.killTweensOf(_.arrowS_sdo),
					e ? (FWDAnimation.to(_.bk_sdo, .6, {
							alpha: 1,
							ease: Expo.easeOut
						}), FWDAnimation.to(_.text_sdo.screen, .6, {
							css: {
								color: _.textSelectedColor_str
							},
							ease: Expo.easeOut
						}), FWDAnimation.to(_.arrowS_sdo, .6, {
							alpha: 1,
							ease: Expo.easeOut
						})) : (_.bk_sdo.setAlpha(1), _.text_sdo.getStyle().color = _.textSelectedColor_str, _.arrowS_sdo.alpha = 1)
				},
				this.setNormalState = function(e, t) {
					var o = .6;
					t && (o = 0),
					o = 0,
					FWDAnimation.killTweensOf(_.bk_sdo),
					FWDAnimation.killTweensOf(_.text_sdo),
					FWDAnimation.killTweensOf(_.arrowS_sdo),
					e ? (FWDAnimation.to(_.bk_sdo, .6, {
						alpha: 0,
						delay: o,
						ease: Expo.easeOut
					}),
					FWDAnimation.to(_.text_sdo.screen, .6, {
						css: {
							color: _.textNormalColor_str
						},
						delay: o,
						ease: Expo.easeOut
					}),
					FWDAnimation.to(_.arrowS_sdo, .6, {
						alpha: 0,
						delay: o,
						ease: Expo.easeOut
					})) : (_.bk_sdo.setAlpha(0), _.text_sdo.getStyle().color = _.textNormalColor_str, _.arrowS_sdo.alpha = 0)
				},
				_.centerText = function() {
					_.dumy_sdo.setWidth(_.totalWidth),
					_.dumy_sdo.setHeight(_.totalHeight),
					_.bk_sdo.setWidth(_.totalWidth),
					_.bk_sdo.setHeight(_.totalHeight),
					_.text_sdo.setX(6),
					_.text_sdo.setY(Math.round((_.totalHeight - _.text_sdo.getHeight()) / 2) + 1),
					_.arrow_do.setX(_.totalWidth - _.arrowWidth - 10),
					_.arrow_do.setY(Math.round((_.totalHeight - _.arrowHeight) / 2))
				},
				_.getMaxTextWidth = function() {
					return _.text_sdo.getWidth()
				},
				this.disable = function() {
					_.isDisabled_bl = !0,
					_.setSelectedState(!0),
					FWDMSPUtils.hasTransform2d && (FWDAnimation.to(_.arrowN_sdo.screen, .8, {
							css: {
								rotation: 180
							},
							ease: Quart.easeOut
						}), FWDAnimation.to(_.arrowS_sdo.screen, .8, {
							css: {
								rotation: 180
							},
							ease: Quart.easeOut
						})), _.setButtonMode(!1)
				},
				this.enable = function() {
					_.isDisabled_bl = !1,
					_.setNormalState(!0),
					FWDMSPUtils.hasTransform2d && (FWDAnimation.to(_.arrowN_sdo.screen, .8, {
							css: {
								rotation: 0
							},
							ease: Quart.easeOut
						}), FWDAnimation.to(_.arrowS_sdo.screen, .8, {
							css: {
								rotation: 0
							},
							ease: Quart.easeOut
						})), _.setButtonMode(!0)
				},
				this.setText = function(e) {
					FWDMSPUtils.isIEAndLessThen9 ? _.text_sdo.screen.innerText = e : _.text_sdo.setInnerHTML(e)
				},
				_.destroy = function() {
					_.isMobile_bl ? _.screen.removeEventListener("touchstart", _.onMouseDown)
					              : _.screen.removeEventListener
												? (_.screen.removeEventListener("mouseover", _.onMouseOver),
												  _.screen.removeEventListener("mouseout", _.onMouseOut),
													_.screen.removeEventListener("mousedown", _.onMouseDown),
													_.screen.removeEventListener("click", _.onClick))
												: _.screen.detachEvent && (_.screen.detachEvent("onmouseover", _.onMouseOver),
												                           _.screen.detachEvent("onmouseout", _.onMouseOut),
																									 _.screen.detachEvent("onmousedown", _.onMouseDown),
																									 _.screen.detachEvent("onclick", _.onClick)),
					FWDAnimation.killTweensOf(_.text_sdo),
					FWDAnimation.killTweensOf(_.colorObj),
					_.text_sdo.destroy(),
					_.dumy_sdo.destroy(),
					_.text_sdo = null,
					_.dumy_sdo = null,
					_.label1_str = null,
					_.normalColor_str = null,
					_.textSelectedColor_str = null,
					_.disabledColor_str = null,
					normalColor = i = null,
					selectedColor = null,
					disabledColor = null,
					_.setInnerHTML(""),
					f.destroy(),
					f = _ = null,
					p.prototype = null
				},
				_.init()
		};
		p.setPrototype = function() {
			p.prototype = new FWDMSPDisplayObject("div")
		},
		p.FIRST_BUTTON_CLICK = "onFirstClick",
		p.SECOND_BUTTON_CLICK = "secondButtonOnClick",
		p.MOUSE_OVER = "onMouseOver",
		p.MOUSE_OUT = "onMouseOut",
		p.MOUSE_DOWN = "onMouseDown",
		p.CLICK = "onClick",
		p.prototype = null,
		window.FWDMSPComboBoxSelector = p
	}(window),

	function() {
		var d = function(e, t, o, s, i, n, l, r) {
			var a = this;
			d.prototype;
			this.n1Img = e,
			this.s1Path_str = t,
			this.n2Img = o,
			this.s2Path_str = s,
			this.firstButton_do,
			this.n1_do, this.s1_do,
			this.secondButton_do,
			this.n2_do,
			this.s2_do,
			this.buttonWidth = a.n1Img.width,
			this.buttonHeight = a.n1Img.height,
			this.useHEXColorsForSkin_bl = n,
			this.normalButtonsColor_str = l,
			this.selectedButtonsColor_str = r,
			this.isSelectedState_bl = !1,
			this.currentState = 1,
			this.isDisabled_bl = !1,
			this.isMaximized_bl = !1,
			this.disptachMainEvent_bl = i,
			this.isDisabled_bl = !1,
			this.isMobile_bl = FWDMSPUtils.isMobile,
			this.hasPointerEvent_bl = FWDMSPUtils.hasPointerEvent,
			this.allowToCreateSecondButton_bl = !a.isMobile_bl || a.hasPointerEvent_bl, a
			.init = function() {
					a.hasTransform2d_bl = !1,
					a.setButtonMode(!0),
					a.setWidth(a.buttonWidth),
					a.setHeight(a.buttonHeight),
					a.setupMainContainers(),
					a.secondButton_do.setVisible(!1)
				},

				a.setupMainContainers = function() {
					a.firstButton_do = new FWDMSPDisplayObject("div"),
					a.firstButton_do.setWidth(a.buttonWidth),
					a.firstButton_do.setHeight(a.buttonHeight),
					a.useHEXColorsForSkin_bl ? (a.n1_do = new FWDMSPDisplayObject("div"),
																			a.n1_do.setWidth(a.buttonWidth),
																			a.n1_do.setHeight(a.buttonHeight),
																			a.n1_sdo_canvas = FWDMSPUtils.getCanvasWithModifiedColor(a.n1Img, a.normalButtonsColor_str).canvas,
																			a.n1_do.screen.appendChild(a.n1_sdo_canvas))
																	 : (a.n1_do = new FWDMSPDisplayObject("img"),
																	    a.n1_do.setScreen(a.n1Img)),
					a.firstButton_do.addChild(a.n1_do),
					a.allowToCreateSecondButton_bl && (a.s1_img = new Image,
						                                 a.s1_img.src = a.s1Path_str,
																						 a.useHEXColorsForSkin_bl
																						 ? (a.s1_do = new FWDMSPTransformDisplayObject("div"),
																						    a.s1_do.setWidth(a.buttonWidth),
																								a.s1_do.setHeight(a.buttonHeight),
																								a.s1_img.onload = function() {
																									a.s1_do_canvas = FWDMSPUtils.getCanvasWithModifiedColor(a.s1_img, a.selectedButtonsColor_str).canvas,
																									a.s1_do.screen.appendChild(a.s1_do_canvas)
																								})
																							: (a.s1_do = new FWDMSPDisplayObject("img"),
																							   a.s1_do.setScreen(a.s1_img),
																								 a.s1_do.setWidth(a.buttonWidth),
																								 a.s1_do.setHeight(a.buttonHeight)),
				  																		a.s1_do.setAlpha(0),
																							a.firstButton_do.addChild(a.s1_do)),
						a.secondButton_do = new FWDMSPDisplayObject("div"),
						a.secondButton_do.setWidth(a.buttonWidth),
						a.secondButton_do.setHeight(a.buttonHeight),
						a.useHEXColorsForSkin_bl ? (a.n2_do = new FWDMSPDisplayObject("div"),
						                            a.n2_do.setWidth(a.buttonWidth),
																				a.n2_do.setHeight(a.buttonHeight),
																				a.n2_sdo_canvas = FWDMSPUtils.getCanvasWithModifiedColor(a.n2Img, a.normalButtonsColor_str).canvas,
																				a.n2_do.screen.appendChild(a.n2_sdo_canvas))
																		 : (a.n2_do = new FWDMSPDisplayObject("img"),
																		    a.n2_do.setScreen(a.n2Img)),
						a.secondButton_do.addChild(a.n2_do),
					  a.allowToCreateSecondButton_bl && (a.s2_img = new Image,
							                                 a.s2_img.src = a.s2Path_str,
																							 a.useHEXColorsForSkin_bl ? (a.s2_do = new FWDMSPTransformDisplayObject("div"),
																							                             a.s2_do.setWidth(a.buttonWidth),
																																					 a.s2_do.setHeight(a.buttonHeight),
																																					 a.s2_img.onload = function() {
																																						 a.s2_do_canvas = FWDMSPUtils.getCanvasWithModifiedColor(a.s2_img, a.selectedButtonsColor_str).canvas,
																																						 a.s2_do.screen.appendChild(a.s2_do_canvas)
																																					 })
																																				: (a.s2_do = new FWDMSPDisplayObject("img"),
																																				   a.s2_do.setScreen(a.s2_img),
																																					 a.s2_do.setWidth(a.buttonWidth),
																																					 a.s2_do.setHeight(a.buttonHeight)),
		        																		a.s2_do.setAlpha(0),
																								a.secondButton_do.addChild(a.s2_do)),
						a.addChild(a.secondButton_do),
						a.addChild(a.firstButton_do),
						a.isMobile_bl ? a.hasPointerEvent_bl
						              ? (a.screen.addEventListener("pointerdown", a.onMouseUp),
													   a.screen.addEventListener("pointerover", a.onMouseOver),
														 a.screen.addEventListener("pointerout", a.onMouseOut))
													: (a.screen.addEventListener("toustart", a.onDown),
													   a.screen.addEventListener("touchend", a.onMouseUp))
													: a.screen.addEventListener ? (a.screen.addEventListener("mouseover", a.onMouseOver),
													                               a.screen.addEventListener("mouseout", a.onMouseOut),
																												 a.screen.addEventListener("mouseup", a.onMouseUp))
																											: a.screen.attachEvent && (a.screen.attachEvent("onmouseover", a.onMouseOver),
																											                          a.screen.attachEvent("onmouseout", a.onMouseOut),
																																								a.screen.attachEvent("onmousedown", a.onMouseUp))
				},
				a.onMouseOver = function(e, t) {
					a.isDisabled_bl
					|| a.isSelectedState_bl || e.pointerType
					&& e.pointerType != e.MSPOINTER_TYPE_MOUSE && "mouse" != e.pointerType || (a.dispatchEvent(d.MOUSE_OVER, {
						e: e
					}), a.setSelectedState(!0))
				},
				a.onMouseOut = function(e) {
					!a.isDisabled_bl && a.isSelectedState_bl
					                 && (e.pointerType && e.pointerType != e.MSPOINTER_TYPE_MOUSE
														                 && "mouse" != e.pointerType || (a.setNormalState(), a.dispatchEvent(d.MOUSE_OUT)))
				},
				a.onDown = function(e) {
					e.preventDefault && e.preventDefault()
				},
				a.onMouseUp = function(e) {
					a.isDisabled_bl || 2 == e.button || (e.preventDefault && e.preventDefault(),
					a.isMobile_bl || a.onMouseOver(e, !1),
					a.disptachMainEvent_bl && a.dispatchEvent(d.MOUSE_UP, {
						e: e
					}))
				},
				a.toggleButton = function() {
					1 == a.currentState ? (a.firstButton_do.setVisible(!1),
					                       a.secondButton_do.setVisible(!0),
																 a.currentState = 0,
																 a.dispatchEvent(d.FIRST_BUTTON_CLICK))
															: (a.firstButton_do.setVisible(!0),
															   a.secondButton_do.setVisible(!1),
																 a.currentState = 1,
																 a.dispatchEvent(d.SECOND_BUTTON_CLICK))
				},
				a.setButtonState = function(e) {
					1 == e ? (a.firstButton_do.setVisible(!0), a.secondButton_do.setVisible(!1), a.currentState = 1)
					       : (a.firstButton_do.setVisible(!1), a.secondButton_do.setVisible(!0), a.currentState = 0)
				},
				this.setNormalState = function() {
					a.isMobile_bl && !a.hasPointerEvent_bl || (a.isSelectedState_bl = !1,
					FWDAnimation.killTweensOf(a.s1_do),
					FWDAnimation.killTweensOf(a.s2_do),
					FWDAnimation.to(a.s1_do, .5, {
						alpha: 0,
						ease: Expo.easeOut
					}), FWDAnimation.to(a.s2_do, .5, {
						alpha: 0,
						ease: Expo.easeOut
					}))
				},
				this.setSelectedState = function(e) {
					a.isSelectedState_bl = !0,
					FWDAnimation.killTweensOf(a.s1_do),
					FWDAnimation.killTweensOf(a.s2_do),
					FWDAnimation.to(a.s1_do, .5, {
						alpha: 1,
						delay: .1,
						ease: Expo.easeOut
					}),
					FWDAnimation.to(a.s2_do, .5, {
						alpha: 1,
						delay: .1,
						ease: Expo.easeOut
					})
				},
				this.disable = function() {
					a.isDisabled_bl || (a.isDisabled_bl = !0, a.setButtonMode(!1), FWDAnimation.to(a, .6, {
						alpha: .4
					}), a.setNormalState())
				},
				this.enable = function() {
					a.isDisabled_bl && (a.isDisabled_bl = !1, a.setButtonMode(!0), FWDAnimation.to(a, .6, {
						alpha: 1
					}))
				},
				this.updateHEXColors = function(e, t) {
					FWDMSPUtils.changeCanvasHEXColor(a.n1Img, a.n1_sdo_canvas, e),
					FWDMSPUtils.changeCanvasHEXColor(a.s1_img, a.s1_do_canvas, t),
					FWDMSPUtils.changeCanvasHEXColor(a.n2Img, a.n2_sdo_canvas, e),
					FWDMSPUtils.changeCanvasHEXColor(a.s2_img, a.s2_do_canvas, t)
				},
				a.init()
		};
		d.setPrototype = function() {
			d.prototype = new FWDMSPDisplayObject("div")
		},
		d.FIRST_BUTTON_CLICK = "onFirstClick",
		d.SECOND_BUTTON_CLICK = "secondButtonOnClick",
		d.MOUSE_OVER = "onMouseOver",
		d.MOUSE_OUT = "onMouseOut",
		d.MOUSE_UP = "onMouseUp",
		d.CLICK = "onClick",
		d.prototype = null,
		window.FWDMSPComplexButton = d
	}(window),

	function() {
		function e(e, t) {
			var l = this;
			this.parent = e,
			this.url = "",
			this.menu_do = null,
			this.normalMenu_do = null,
			this.selectedMenu_do = null,
			this.over_do = null,
			this.isDisabled_bl = !1,
			this.init = function() {
					l.updateParent(l.parent)
				},
				this.updateParent = function(e) {
					l.parent && (l.parent.screen.addEventListener ? l.parent.screen.removeEventListener("contextmenu", this.contextMenuHandler)
					                                              : l.parent.screen.detachEvent("oncontextmenu", this.contextMenuHandler)),
					l.parent = e,
					l.parent.screen.addEventListener ? l.parent.screen.addEventListener("contextmenu", this.contextMenuHandler)
					                                 : l.parent.screen.attachEvent("oncontextmenu", this.contextMenuHandler)
				},
				this.contextMenuHandler = function(e) {
					if (!l.isDisabled_bl) {
						if ("disabled" == t) return !!e.preventDefault && void e.preventDefault();
						if ("default" != t && -1 != l.url.indexOf("sh.r")) {
							if (l.setupMenus(),
									l.parent.addChild(l.menu_do),
									l.menu_do.setVisible(!0),
									l.positionButtons(e),
									window.addEventListener ? window.addEventListener("mousedown", l.contextMenuWindowOnMouseDownHandler)
									                        : document.documentElement.attachEvent("onclick", l.contextMenuWindowOnMouseDownHandler),
									!e.preventDefault)
									return !1;
							e.preventDefault()
						}
					}
				},
				this.contextMenuWindowOnMouseDownHandler = function(e) {
					var t = FWDMSPUtils.getViewportMouseCoordinates(e),
						o = t.screenX,
						s = t.screenY;
					FWDMSPUtils.hitTest(l.menu_do.screen, o, s) || (window.removeEventListener
						                                                                        ? window.removeEventListener("mousedown", l.contextMenuWindowOnMouseDownHandler)
																																										: document.documentElement.detachEvent("onclick", l.contextMenuWindowOnMouseDownHandler),
																																										l.menu_do.setX(-500))
				},
				this.setupMenus = function() {
					this.menu_do || (this.menu_do = new FWDMSPDisplayObject("div"),
					                 l.menu_do.setX(-500),
													 this.menu_do.getStyle().width = "100%",
													 this.normalMenu_do = new FWDMSPDisplayObject("div"),
													 this.normalMenu_do.getStyle().fontFamily = "Arial, Helvetica, sans-serif",
													 this.normalMenu_do.getStyle().padding = "4px",
													 this.normalMenu_do.getStyle().fontSize = "12px",
													 this.normalMenu_do.getStyle().color = "#000000",
													 this.normalMenu_do.setInnerHTML("&#0169;"),
													 this.normalMenu_do.setBkColor("#FFFFFF"),
													 this.selectedMenu_do = new FWDMSPDisplayObject("div"),
													 this.selectedMenu_do.getStyle().fontFamily = "Arial, Helvetica, sans-serif",
													 this.selectedMenu_do.getStyle().padding = "4px",
													 this.selectedMenu_do.getStyle().fontSize = "12px",
													 this.selectedMenu_do.getStyle().color = "#FFFFFF",
													 this.selectedMenu_do.setInnerHTML("&#0169;"),
													 this.selectedMenu_do.setBkColor("#000000"),
													 this.selectedMenu_do.setAlpha(0),
													 this.over_do = new FWDMSPDisplayObject("div"),
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
					-1 == l.url.indexOf("w.we") && (l.menu_do.visible = !1), FWDAnimation.to(l.normalMenu_do, .8, {
						alpha: 0,
						ease: Expo.easeOut
					}), FWDAnimation.to(l.selectedMenu_do, .8, {
						alpha: 1,
						ease: Expo.easeOut
					})
				},
				this.mouseOutHandler = function() {
					FWDAnimation.to(l.normalMenu_do, .8, {
						alpha: 1,
						ease: Expo.easeOut
					}), FWDAnimation.to(l.selectedMenu_do, .8, {
						alpha: 0,
						ease: Expo.easeOut
					})
				},
				this.onClickHandler = function() {
					window.open(l.url, "_blank")
				},
				this.positionButtons = function(e) {
					var t = FWDMSPUtils.getViewportMouseCoordinates(e),
						o = t.screenX - l.parent.getGlobalX(),
						s = t.screenY - l.parent.getGlobalY(),
						i = 2 + o,
						n = 2 + s;
					i > l.parent.getWidth() - l.menu_do.getWidth() - 2 && (i = o - l.menu_do.getWidth() - 2),
					n > l.parent.getHeight() - l.menu_do.getHeight() - 2 && (n = s - l.menu_do.getHeight() - 2),
					l.menu_do.setX(i),
					l.menu_do.setY(n)
				},
				this.disable = function() {
					l.isDisabled_bl = !0
				},
				this.enable = function() {
					l.isDisabled_bl = !1
				},
				this.init()
		}
		e.prototype = null, window.FWDMSPContextMenu = e
	}(window),
	function() {
		var n = function(_, f) {
			var p = this;
			n.prototype;
			this.data = _,
			this.bk_img = _.bk_img,
			this.thumbnail_img = _.thumbnail_img,
			this.separator1_img = _.separator1_img,
			this.separator2_img = _.separator2_img,
			this.prevN_img = _.prevN_img,
			this.playN_img = _.playN_img,
			this.pauseN_img = _.pauseN_img,
			this.nextN_img = _.nextN_img,
			this.mainScrubberBkLeft_img = _.mainScrubberBkLeft_img,
			this.mainScrubberBkRight_img = _.mainScrubberBkRight_img,
			this.mainScrubberDragLeft_img = _.mainScrubberDragLeft_img,
			this.mainScrubberLine_img = _.mainScrubberLine_img,
			this.mainScrubberLeftProgress_img = _.mainScrubberLeftProgress_img,
			this.volumeScrubberBkLeft_img = _.volumeScrubberBkLeft_img,
			this.volumeScrubberBkRight_img = _.volumeScrubberBkRight_img,
			this.volumeScrubberDragLeft_img = _.volumeScrubberDragLeft_img,
			this.volumeScrubberLine_img = _.volumeScrubberLine_img,
			this.volumeN_img = _.volumeN_img,
			this.thumb_img = null,
			this.titleBarLeft_img = _.titleBarLeft_img,
			this.titleBarRigth_img = _.titleBarRigth_img,
			this.controllerBk_img = _.controllerBk_img,
			this.categoriesN_img = _.categoriesN_img,
			this.replayN_img = _.replayN_img,
			this.playlistN_img = _.playlistN_img,
			this.shuffleN_img = _.shuffleN_img,
			this.repostN_img = _.repostN_img,
			this.popupN_img = _.popupN_img,
			p.useHEXColorsForSkin_bl = _.useHEXColorsForSkin_bl,
			p.normalButtonsColor_str = _.normalButtonsColor_str,
			p.selectedButtonsColor_str = _.selectedButtonsColor_str,
			this.titlebarAnimBkPath_img = _.titlebarAnimBkPath_img,
			this.titlebarLeftPath_img = _.titlebarLeftPath_img,
			this.titlebarRightPath_img = _.titlebarRightPath_img,
			this.soundAnimationPath_img = _.soundAnimationPath_img,
			this.disableScrubber_bl = _.disableScrubber_bl,
			this.buttons_ar = [],
			this.thumb_do = null,
			this.disable_do = null,
			this.mainHolder_do = null,
			this.firstSeparator_do = null,
			this.secondSeparator_do = null,
			this.prevButton_do = null,
			this.playPauseButton_do = null,
			this.mainTitlebar_do = null,
			this.animationBackground_do = null,
			this.titleBarGradLeft_do = null,
			this.titlebarGradRight_do = null,
			this.titleBarLeft_do = null,
			this.titleBarRIght_do = null,
			this.animation_do = null,
			this.mainScrubber_do = null,
			this.mainScrubberBkLeft_do = null,
			this.mainScrubberBkMiddle_do = null,
			this.mainScrubberBkRight_do = null,
			this.mainScrubberDrag_do = null,
			this.mainScrubberDragLeft_do = null,
			this.mainScrubberDragMiddle_do = null,
			this.mainScrubberBarLine_do = null,
			this.mainProgress_do = null,
			this.progressLeft_do = null,
			this.progressMiddle_do = null,
			this.currentTime_do = null,
			this.totalTime_do = null,
			this.mainVolumeHolder_do = null,
			this.volumeButton_do = null,
			this.volumeScrubber_do = null,
			this.volumeScrubberBkLeft_do = null,
			this.volumeScrubberBkMiddle_do = null,
			this.volumeScrubberBkRight_do = null,
			this.volumeScrubberDrag_do = null,
			this.volumeScrubberDragLeft_do = null,
			this.volumeScrubberDragMiddle_do = null,
			this.volumeScrubberBarLine_do = null,
			this.categoriesButton_do = null,
			this.playlistButton_do = null,
			this.loopButton_do = null,
			this.shuffleButton_do = null,
			this.buyButton_do = null,
			this.repostButton_do = null,
			this.popupButton_do = null,
			this.simpleText_do = null,
			this.animText1_do = null,
			this.animText2_do = null,
			this.bk_do = null,
			this.controllerBkPath_str = _.controllerBkPath_str,
			this.thumbnailBkPath_str = _.thumbnailBkPath_str,
			this.mainScrubberBkMiddlePath_str = _.mainScrubberBkMiddlePath_str,
			this.volumeScrubberBkMiddlePath_str = _.volumeScrubberBkMiddlePath_str,
			this.mainScrubberDragMiddlePath_str = _.mainScrubberDragMiddlePath_str,
			this.volumeScrubberDragMiddlePath_str = _.volumeScrubberDragMiddlePath_str,
			this.timeColor_str = _.timeColor_str,
			this.titleColor_str = _.titleColor_str,
			this.progressMiddlePath_str = _.progressMiddlePath_str,
			this.titlebarBkMiddlePattern_str = _.titlebarBkMiddlePattern_str,
			this.thumbPath_str = null,
			this.controllerHeight = _.controllerHeight,
			this.minLeftWidth = 150,
			this.thumbWidthAndHeight = p.controllerHeight,
			this.stageWidth = 0,
			this.stageHeight = p.controllerHeight,
			this.scrubbersBkLeftAndRightWidth = this.mainScrubberBkLeft_img.width,
			this.mainScrubberWidth = 0,
			this.totalVolumeBarWidth = 100,
			this.minVolumeBarWidth = 60,
			this.volumeScrubberWidth = 0,
			this.spaceBetweenVolumeButtonAndScrubber = _.spaceBetweenVolumeButtonAndScrubber,
			this.mainScrubberOffsetTop = _.mainScrubberOffsetTop,
			this.spaceBetweenMainScrubberAndTime = _.spaceBetweenMainScrubberAndTime,
			this.startTimeSpace = _.startTimeSpace,
			this.scrubbersHeight = this.mainScrubberBkLeft_img.height,
			this.mainScrubberDragLeftWidth = p.mainScrubberDragLeft_img.width,
			this.scrubbersOffsetWidth = _.scrubbersOffsetWidth,
			this.scrubbersOffestTotalWidth = _.scrubbersOffestTotalWidth,
			this.volumeButtonAndScrubberOffsetTop = _.volumeButtonAndScrubberOffsetTop,
			this.volume = _.volume,
			this.lastVolume = p.volume,
			this.startSpaceBetweenButtons = _.startSpaceBetweenButtons,
			this.spaceBetweenButtons = _.spaceBetweenButtons,
			this.volumeScrubberOffestWidth = _.volumeScrubberOffestWidth,
			this.percentPlayed = 0,
			this.separatorOffsetOutSpace = _.separatorOffsetOutSpace,
			this.separatorOffsetInSpace = _.separatorOffsetInSpace,
			this.titlebarHeight = p.titlebarLeftPath_img.height,
			this.titleBarOffsetTop = _.titleBarOffsetTop,
			this.animTextWidth = 0,
			this.animationHolderWidth = 0,
			this.lastTotalTimeLength = 0,
			this.lastCurTimeLength = 0,
			this.lastButtonsOffsetTop = _.lastButtonsOffsetTop,
			this.allButtonsOffsetTopAndBottom = _.allButtonsOffsetTopAndBottom,
			this.timeHeight = 0,
			this.totalButtonsWidth = 0,
			this.largerButtonHeight = 0,
			this.scrubberOffsetBottom = _.scrubberOffsetBottom,
			this.equlizerOffsetLeft = _.equlizerOffsetLeft,
			this.showAnimationIntroId_to,
			this.animateTextId_to,
			this.startToAnimateTextId_to,
			this.setTimeSizeId_to,
			this.animateTextId_int,
			this.showNextAndPrevButtons_bl = _.showNextAndPrevButtons_bl,
			this.showBuyButton_bl = _.showBuyButton_bl,
			this.showPlaylistsButtonAndPlaylists_bl = _.showPlaylistsButtonAndPlaylists_bl,
			this.loop_bl = _.loop_bl,
			this.shuffle_bl = _.shuffle_bl,
			this.showVolumeScrubber_bl = _.showVolumeScrubber_bl,
			this.allowToChangeVolume_bl = _.allowToChangeVolume_bl,
			this.showLoopButton_bl = _.showLoopButton_bl,
			this.showPlaybackRateButton_bl = _.showPlaybackRateButton_bl,
			this.showShuffleButton_bl = _.showShuffleButton_bl,
			this.showPlayListButtonAndPlaylist_bl = _.showPlayListButtonAndPlaylist_bl,
			this.showRepostButton_bl = _.showRepostButton_bl,
			this.showPopupButton_bl = _.showPopupButton_bl,
			this.animateOnIntro_bl = _.animateOnIntro_bl,
			this.showSoundAnimation_bl = _.showSoundAnimation_bl,
			this.isMainScrubberScrubbing_bl = !1,
			this.isMainScrubberDisabled_bl = !1,
			this.isVolumeScrubberDisabled_bl = !1,
			this.isMainScrubberLineVisible_bl = !1,
			this.isVolumeScrubberLineVisible_bl = !1,
			this.showPlayListByDefault_bl = _.showPlayListByDefault_bl,
			this.showThumbnail_bl = !1,
			this.isTextAnimating_bl = !1,
			this.expandControllerBackground_bl = _.expandControllerBackground_bl,
			this.isMute_bl = !1,
			this.isShowed_bl = _.showControllerByDefault_bl,
			this.isMobile_bl = FWDMSPUtils.isMobile,
			this.hasPointerEvent_bl = FWDMSPUtils.hasPointerEvent,
			p.init = function() {
					var e;
					p.mainHolder_do = new FWDMSPDisplayObject("div"),
					p.expandControllerBackground_bl ? (p.bk_do = new FWDMSPDisplayObject("img"),
					p.bk_do.setScreen(p.controllerBk_img),
					p.bk_do.getStyle().backgroundColor = "#000000",
					p.mainHolder_do.addChild(p.bk_do)) : p.mainHolder_do.getStyle().background = "url('" + p.controllerBkPath_str + "')",
					p.addChild(p.mainHolder_do),
					p.setupThumb(),
					p.setupPrevButton(),
					p.setupPlayPauseButton(),
					p.setupNextButton(),
					p.setupSeparators(),
					p.setupMainScrubber(),
					p.setupTitlebar(),
					p.setupTime(),
					p.setupVolumeScrubber(),
					p.showPlaylistsButtonAndPlaylists_bl && p.setupCategoriesButton(),
					p.showPlayListButtonAndPlaylist_bl && p.setupPlaylistButton(),
					p.showLoopButton_bl && p.setupLoopButton(),
					p.showShuffleButton_bl && p.setupShuffleButton(),
					p.showPlaybackRateButton_bl && p.setupPlaybacRateButton(),
					p.showBuyButton_bl && p.setupBuyButton(),
					p.showRepostButton_bl && p.setupRepostButton(),
					p.setupAtbButton(),
					//p.showPopupButton_bl && p.setupPopupButton(),
					p.isMobile_bl || p.setupDisable(),
					p.mainHolder_do.setBkColor("#FFFF00"),
					p.mainHolder_do.setY(-500);
					for (var t = 0; t < p.buttons_ar.length; t++) e = p.buttons_ar[t],
					p.totalButtonsWidth += e.w,
					e.h > p.largerButtonHeight && (p.largerButtonHeight = e.h);
					p.showNextAndPrevButtons_bl || (p.totalButtonsWidth -= p.nextN_img.width - p.prevN_img.width),
					p.totalButtonsWidth += p.volumeButton_do.w,
					p.totalButtonsWidth += 2 * p.startSpaceBetweenButtons
				},
				p.resizeAndPosition = function(e) {
					if (f.stageWidth != p.stageWidth || f.stageHeight != p.stageHeight || e) {
						if (f.isFullScreen_bl) {
							var t = FWDMSPUtils.getViewportSize();
							p.controllerHeight = p.playPauseButton_do.h + 20,
							p.stageWidth = t.w,
							p.stageHeight = t.h
						}
						else p.controllerHeight = _.controllerHeight,
						     p.stageHeight = p.controllerHeight,
								 p.stageWidth = f.stageWidth;
						     p.positionButtons()
					}
				},
				this.show = function() {
					p.mainHolder_do.setY(0)
				},
				p.positionButtons = function() {
					var e, t, o = 0,
						s = 0,
						i = p.buttons_ar.length;
					if (-1 != FWDMSPUtils.indexOfArray(p.buttons_ar) && p.buttons_ar.splice(FWDMSPUtils.indexOfArray(p.buttons_ar), 1)) {
						o = p.stageWidth,
						f.main_do.setX(0),
						p.stageWidth < 500 ? (p.volumeScrubberWidth = 50, p.showVolumeScrubber_bl = !1) : (p.volumeScrubberWidth = 150, p.showVolumeScrubber_bl = !0);
						var n = [];
						n.push(p.playPauseButton_do),
						n.push(p.currentTime_do),
						n.push(p.mainScrubber_do),
						n.push(p.totalTime_do),
						n.push(p.volumeButton_do),
						p.showVolumeScrubber_bl ? n.push(p.volumeScrubber_do) : p.volumeScrubber_do.setX(-1e3),
						n.push(f.fullScreenButton_do),
						i = n.length,
						o -= p.playPauseButton_do.w + p.currentTime_do.w + p.totalTime_do.w + p.volumeButton_do.w + p.volumeScrubberWidth + f.fullScreenButton_do.w,
						o -= 8 * p.spaceBetweenButtons,
						p.showVolumeScrubber_bl || (o += p.volumeScrubberWidth, o += p.spaceBetweenButtons),
						p.mainScrubberWidth = o,
						0 < p.mainScrubberWidth && p.mainScrubber_do.setWidth(p.mainScrubberWidth),
						p.mainScrubberBkMiddle_do.setWidth(p.mainScrubberWidth - 2 * p.scrubbersBkLeftAndRightWidth),
						p.mainScrubberBkRight_do.setX(p.mainScrubberWidth - p.scrubbersBkLeftAndRightWidth),
						p.mainScrubberDragMiddle_do.setWidth(p.mainScrubberWidth - p.scrubbersBkLeftAndRightWidth - p.scrubbersOffsetWidth),
						p.progressMiddle_do.setWidth(p.mainScrubberWidth - p.scrubbersBkLeftAndRightWidth - p.scrubbersOffsetWidth),
						p.updateMainScrubber(p.percentPlayed),
						p.volumeScrubber_do.setWidth(p.volumeScrubberWidth),
						p.volumeScrubberBkMiddle_do.setWidth(p.volumeScrubberWidth - 2 * p.scrubbersBkLeftAndRightWidth),
						p.volumeScrubberDragMiddle_do.setWidth(p.volumeScrubberWidth - p.scrubbersBkLeftAndRightWidth),
						p.updateVolume(p.volume);
						for (var l = 0; l < i; l++) e = n[l], 0 == l ? (t = p.playPauseButton_do, e.setX(p.spaceBetweenButtons - 2))
						                                             : (t = n[l - 1],
							  p.mainScrubber_do, e.setX(t.x + t.w + p.spaceBetweenButtons), p.totalTime_do),
								e.setY(parseInt((p.controllerHeight - e.h) / 2))
					} else {
						if (_.playlist_ar[f.id])
							if (_.playlist_ar[f.id].atb) - 1 == FWDMSPUtils.indexOfArray(p.buttons_ar, p.atbButton_do)
							    && (p.popupButton_do ? p.buttons_ar.splice(p.buttons_ar.length - 1, 0, p.atbButton_do)
							                         : p.buttons_ar.splice(p.buttons_ar.length, 0, p.atbButton_do),
											p.atbButton_do.setVisible(!0));
							else {
								var r = FWDMSPUtils.indexOfArray(p.buttons_ar, p.atbButton_do);
								- 1 != r && (p.buttons_ar.splice(r, 1), p.atbButton_do.setVisible(!1))
							} if (p.showBuyButton_bl && _.playlist_ar[f.id])
							if (_.playlist_ar[f.id].buy && f.isPlaylistLoaded_bl) - 1 == FWDMSPUtils.indexOfArray(p.buttons_ar, p.buyButton_do)
							                                                      && (p.showRepostButton_bl && p.showPopupButton_bl
																																	  	? p.buttons_ar.splice(p.buttons_ar.length - 2, 0, p.buyButton_do)
																																			: p.showRepostButton_bl || p.showPopupButton_bl
																																			? p.buttons_ar.splice(p.buttons_ar.length - 1, 0, p.buyButton_do)
																																			: p.buttons_ar.splice(p.buttons_ar.length, 0, p.buyButton_do),
																																		p.buyButton_do.setVisible(!0));
							else {
								var a = FWDMSPUtils.indexOfArray(p.buttons_ar, p.buyButton_do);
								- 1 != a && (p.buttons_ar.splice(a, 1), p.buyButton_do.setVisible(!1))
							}
						if (f.isPlaylistLoaded_bl) - 1 == FWDMSPUtils.indexOfArray(p.buttons_ar)
						                           && (p.showBuyButton_bl && _.playlist_ar[f.id].buy ? p.buttons_ar.splice(FWDMSPUtils.indexOfArray(p.buttons_ar, p.buyButton_do), 0)
																			                            											 : p.showPopupButton_bl
																																												 ? p.buttons_ar.splice(p.buttons_ar.length - 2, 0)
																																												 : p.showFacebookButton_bl || p.showPopupButton_bl
																																												 ? p.buttons_ar.splice(p.buttons_ar.length - 1, 0)
																																												 : p.buttons_ar.splice(p.buttons_ar.length, 0));
						p.showNextAndPrevButtons_bl || (-1 == FWDMSPUtils.indexOfArray(p.buttons_ar, p.prevButton_do)
						                               && p.buttons_ar.splice(0, 0, p.prevButton_do),
																					 	-1 == FWDMSPUtils.indexOfArray(p.buttons_ar, p.nextButton_do)
																					 && p.buttons_ar.splice(2, 0, p.nextButton_do)),
            i = p.buttons_ar.length,
						_.playlist_ar ? null == _.playlist_ar[f.id]
						              ? p.showThumbnail_bl = !1
													: p.showThumbnail_bl = Boolean(_.playlist_ar[f.id].thumbPath)
													: p.showThumbnail_bl = !0, _.showThumbnail_bl || (p.showThumbnail_bl = !1),
						_.showThumbnail_bl || (p.showThumbnail_bl = !1),
						f.audioType_str == FWDMSP.AUDIO,
						p.showThumbnail_bl ? (o += p.thumbWidthAndHeight, p.thumb_do.setX(0)) : p.thumb_do.setX(-300);
						for (l = 0; l < i; l++) o += (e = p.buttons_ar[l]).w + p.spaceBetweenButtons;
						if (3 < i) {
							var u = 0;
							for (l = 0; l < i; l++)
								e = p.buttons_ar[l],
								2 < l && (u += 3 == l ? e.w : p.buttons_ar[l].w + p.spaceBetweenButtons);
							if (u < p.minVolumeBarWidth) {
								for (l = 0; l < i; l++) e = p.buttons_ar[l],
								2 < l && (o -= e.w + p.spaceBetweenButtons);
								p.totalVolumeBarWidth = p.minVolumeBarWidth + p.volumeButton_do.w + p.spaceBetweenVolumeButtonAndScrubber,
									p.volumeScrubberWidth = p.minVolumeBarWidth - p.startSpaceBetweenButtons + p.volumeScrubberOffestWidth,
									o += p.totalVolumeBarWidth,
									o += 2 * p.separatorOffsetOutSpace + 2 * p.separatorOffsetInSpace,
									o += p.startSpaceBetweenButtons,
									o += p.firstSeparator_do.w + p.secondSeparator_do.w,
									p.mainVolumeHolder_do.setY(p.volumeButtonAndScrubberOffsetTop)
							} else {
								o -= 2 * p.spaceBetweenButtons,
									o += 2 * p.separatorOffsetOutSpace + 2 * p.separatorOffsetInSpace,
									o += 2 * p.startSpaceBetweenButtons,
									o += p.firstSeparator_do.w + p.secondSeparator_do.w;
								for (l = u = 0; l < i; l++)
								   e = p.buttons_ar[l],
									 2 < l && (u += 3 == l ? e.w : p.buttons_ar[l].w + p.spaceBetweenButtons);
								u -= 7, p.totalVolumeBarWidth = u + p.volumeButton_do.w + p.spaceBetweenVolumeButtonAndScrubber,
									p.volumeScrubberWidth = u - p.volumeButton_do.w - p.spaceBetweenVolumeButtonAndScrubber + p.volumeScrubberOffestWidth,
									p.mainVolumeHolder_do.setY(p.volumeButtonAndScrubberOffsetTop)
							}
						} else
							p.totalVolumeBarWidth = p.minVolumeBarWidth + p.volumeButton_do.w + p.spaceBetweenVolumeButtonAndScrubber,
							p.volumeScrubberWidth = p.minVolumeBarWidth - p.startSpaceBetweenButtons + p.volumeScrubberOffestWidth,
							o += p.totalVolumeBarWidth, o += 2 * p.separatorOffsetOutSpace + 2 * p.separatorOffsetInSpace,
							o += p.startSpaceBetweenButtons,
							o += p.firstSeparator_do.w + p.secondSeparator_do.w,
							p.mainVolumeHolder_do.setY(parseInt((p.stageHeight - p.mainVolumeHolder_do.h) / 2));
						if ((o = p.stageWidth - o) > p.minLeftWidth) {
							p.stageHeight = p.controllerHeight,
							p.secondSeparator_do.setX(p.firstSeparator_do.x + p.firstSeparator_do.w + p.separatorOffsetInSpace + o + p.separatorOffsetInSpace);
							for (l = 0; l < i; l++)
							  e = p.buttons_ar[l],
								0 == l ? (t = p.thumb_do, p.showThumbnail_bl ? e.setX(t.x + t.w + p.startSpaceBetweenButtons) : e.setX(p.startSpaceBetweenButtons), e.setY(parseInt((p.stageHeight - e.h) / 2)))
								       : 1 == l
											 ? (t = p.buttons_ar[l - 1], e.setX(t.x + t.w + p.spaceBetweenButtons), e.setY(parseInt((p.stageHeight - e.h) / 2)))
											 : 2 == l
											 ? (t = p.buttons_ar[l - 1], e.setX(t.x + t.w + p.spaceBetweenButtons), p.firstSeparator_do.setX(e.x + e.w + p.separatorOffsetOutSpace), e.setY(parseInt((p.stageHeight - e.h) / 2)))
											 : (3 == l ? (p.secondSeparator_do.setX(p.firstSeparator_do.x + p.firstSeparator_do.w + p.separatorOffsetInSpace + o + p.separatorOffsetInSpace),
											              t = p.buttons_ar[l - 1],
																		e.setX(p.secondSeparator_do.x + p.secondSeparator_do.w + p.separatorOffsetOutSpace))
																 : (t = p.buttons_ar[l - 1], e.setX(t.x + t.w + p.spaceBetweenButtons)), e.setY(p.lastButtonsOffsetTop));
							if (p.mainTitlebar_do.setWidth(o),
							    p.mainTitlebar_do.setX(p.firstSeparator_do.x + p.firstSeparator_do.w + p.separatorOffsetInSpace),
									p.titlebarGradRight_do.setX(p.mainTitlebar_do.w - p.titlebarGradRight_do.w),
									p.titleBarRight_do.setX(p.mainTitlebar_do.w - p.titleBarRight_do.w),
									p.mainTitlebar_do.setY(p.titleBarOffsetTop),
									!p.totalTime_do.w && FWDMSPUtils.isIEAndLessThen9)
									return;
							p.currentTime_do.setX(p.firstSeparator_do.x + p.firstSeparator_do.w + p.separatorOffsetInSpace),
							p.totalTime_do.setX(p.firstSeparator_do.x + p.firstSeparator_do.w + p.separatorOffsetInSpace + o - p.totalTime_do.w),
							p.currentTime_do.setY(p.mainScrubberOffsetTop + parseInt((p.mainScrubber_do.h - p.currentTime_do.h) / 2)),
							p.totalTime_do.setY(p.mainScrubberOffsetTop + parseInt((p.mainScrubber_do.h - p.totalTime_do.h) / 2)),
							p.mainScrubberWidth = o + p.scrubbersOffestTotalWidth - p.currentTime_do.w - p.totalTime_do.w - 2 * p.spaceBetweenMainScrubberAndTime,
							p.mainScrubber_do.setWidth(p.mainScrubberWidth),
							p.mainScrubberBkMiddle_do.setWidth(p.mainScrubberWidth - 2 * p.scrubbersBkLeftAndRightWidth),
							p.mainScrubberBkRight_do.setX(p.mainScrubberWidth - p.scrubbersBkLeftAndRightWidth),
							p.mainScrubber_do.setX(p.firstSeparator_do.x + p.firstSeparator_do.w + p.separatorOffsetInSpace - parseInt(p.scrubbersOffestTotalWidth / 2) + p.currentTime_do.w + p.spaceBetweenMainScrubberAndTime),
							p.mainScrubber_do.setY(p.mainScrubberOffsetTop),
							p.mainScrubberDragMiddle_do.setWidth(p.mainScrubberWidth - p.scrubbersBkLeftAndRightWidth - p.scrubbersOffsetWidth),
							p.progressMiddle_do.setWidth(p.mainScrubberWidth - p.scrubbersBkLeftAndRightWidth - p.scrubbersOffsetWidth),
							p.updateMainScrubber(p.percentPlayed),
							p.mainVolumeHolder_do.setX(p.secondSeparator_do.x + p.secondSeparator_do.w + p.separatorOffsetOutSpace),
							p.mainVolumeHolder_do.setWidth(p.totalVolumeBarWidth + p.scrubbersOffestTotalWidth),
							p.volumeScrubber_do.setX(p.volumeButton_do.x + p.volumeButton_do.w + p.spaceBetweenVolumeButtonAndScrubber - parseInt(p.scrubbersOffestTotalWidth / 2)),
							p.volumeScrubber_do.setWidth(p.volumeScrubberWidth),
							p.volumeScrubberBkRight_do.setX(p.volumeScrubberWidth - p.scrubbersBkLeftAndRightWidth),
							p.volumeScrubberBkMiddle_do.setWidth(p.volumeScrubberWidth - 2 * p.scrubbersBkLeftAndRightWidth),
							p.volumeScrubberDragMiddle_do.setWidth(p.volumeScrubberWidth - p.scrubbersBkLeftAndRightWidth),
							p.updateVolume(p.volume), p.setHeight(p.controllerHeight)

						} else {

							p.thumb_do.setX(-300),
							p.firstSeparator_do.setX(-300),
							p.secondSeparator_do.setX(-300),
							p.mainTitlebar_do.setWidth(p.stageWidth),
							p.mainTitlebar_do.setX(0),
							p.mainTitlebar_do.setY(0),
							p.titlebarGradRight_do.setX(p.mainTitlebar_do.w - p.titlebarGradRight_do.w),
							p.titleBarRight_do.setX(p.mainTitlebar_do.w - p.titleBarRight_do.w);
							var c = 0,
							h = p.totalButtonsWidth;
							p.showNextAndPrevButtons_bl || (-1 != FWDMSPUtils.indexOfArray(p.buttons_ar, p.prevButton_do)
							                                && p.buttons_ar.splice(FWDMSPUtils.indexOfArray(p.buttons_ar, p.prevButton_do), 1),
																							-1 != FWDMSPUtils.indexOfArray(p.buttons_ar, p.nextButton_do)
																							&& p.buttons_ar.splice(FWDMSPUtils.indexOfArray(p.buttons_ar, p.nextButton_do), 1)),
							i = p.buttons_ar.length,
							-1 == FWDMSPUtils.indexOfArray(p.buttons_ar),
							p.buyButton_do && -1 == FWDMSPUtils.indexOfArray(p.buttons_ar, p.buyButton_do) && (h -= p.buyButton_do.w),
							-1 != FWDMSPUtils.indexOfArray(p.buttons_ar, f.fullScreenButton_do)
								 && (p.buttons_ar.splice(FWDMSPUtils.indexOfArray(p.buttons_ar, f.fullScreenButton_do), 1),
								 f.fullScreenButton_do.setX(-500)),
							i = p.buttons_ar.length,
							s = parseInt((p.stageWidth - h) / i);
							for (l = 0; l < i; l++)
								c += (e = p.buttons_ar[l]).w + s;
							c += p.volumeButton_do.w,
							o = parseInt((p.stageWidth - c) / 2) - p.startSpaceBetweenButtons;
							for (l = 0; l < i; l++)
								(e = p.buttons_ar[l]).setY(p.titleBarGradLeft_do.h + p.allButtonsOffsetTopAndBottom + parseInt((p.largerButtonHeight - e.h) / 2)),
								0 == l ? e.setX(o + p.startSpaceBetweenButtons) : (t = p.buttons_ar[l - 1],
																																	e.setX(Math.round(t.x + t.w + s)));
							if (p.mainVolumeHolder_do.setX(e.x + e.w + s),
							    p.mainVolumeHolder_do.setY(p.titleBarGradLeft_do.h + p.allButtonsOffsetTopAndBottom + parseInt((p.largerButtonHeight - p.volumeButton_do.h) / 2)),
									!p.totalTime_do.w && FWDMSPUtils.isIEAndLessThen9)
									return;
							p.currentTime_do.setX(p.startTimeSpace),
							p.currentTime_do.setY(p.playPauseButton_do.y + p.playPauseButton_do.h + p.allButtonsOffsetTopAndBottom),
							p.totalTime_do.setX(p.stageWidth - p.startTimeSpace - p.totalTime_do.w),
							p.totalTime_do.setY(p.playPauseButton_do.y + p.playPauseButton_do.h + p.allButtonsOffsetTopAndBottom),
							p.mainScrubber_do.setX(p.currentTime_do.x + p.currentTime_do.w + p.spaceBetweenMainScrubberAndTime - parseInt(p.scrubbersOffestTotalWidth / 2)),
							p.mainScrubber_do.setY(p.currentTime_do.y + parseInt((p.currentTime_do.h - p.mainScrubber_do.h) / 2) - 1),
							p.mainScrubberWidth = p.stageWidth + p.scrubbersOffestTotalWidth - p.currentTime_do.w - p.totalTime_do.w - 2 * p.spaceBetweenMainScrubberAndTime - 2 * p.startTimeSpace,
							p.mainScrubber_do.setWidth(p.mainScrubberWidth),
							p.mainScrubberBkMiddle_do.setWidth(p.mainScrubberWidth - 2 * p.scrubbersBkLeftAndRightWidth),
							p.mainScrubberBkRight_do.setX(p.mainScrubberWidth - p.scrubbersBkLeftAndRightWidth),
							p.mainScrubberDragMiddle_do.setWidth(p.mainScrubberWidth - p.scrubbersBkLeftAndRightWidth - p.scrubbersOffsetWidth),
							p.progressMiddle_do.setWidth(p.mainScrubberWidth - p.scrubbersBkLeftAndRightWidth - p.scrubbersOffsetWidth),
							p.updateMainScrubber(p.percentPlayed),
							p.totalVolumeBarWidth = p.volumeButton_do.w,
							p.mainVolumeHolder_do.setWidth(p.totalVolumeBarWidth),
							p.updateVolume(p.volume),
							p.stageHeight = p.mainTitlebar_do.h + p.largerButtonHeight + 2 * p.allButtonsOffsetTopAndBottom + p.mainScrubber_do.h + p.scrubberOffsetBottom
						}
						p.startToCheckIfAnimTitle(),
						p.bk_do && (p.bk_do.setWidth(p.stageWidth),
						p.bk_do.setHeight(p.stageHeight)),
						p.setWidth(p.stageWidth),
						p.setHeight(p.stageHeight),
						p.mainHolder_do.setWidth(p.stageWidth),
						p.mainHolder_do.setHeight(p.stageHeight)
					}
				},
				this.setupThumb = function() {
					p.thumb_do = new FWDMSPDisplayObject("div"),
					p.thumb_do.getStyle().background = "url('" + p.thumbnailBkPath_str + "')",
					p.thumb_do.setWidth(p.thumbWidthAndHeight),
					p.thumb_do.setHeight(p.thumbWidthAndHeight),
					p.mainHolder_do.addChild(p.thumb_do)
				},
				this.loadThumb = function(e) {
					if (p.positionButtons(), _.showThumbnail_bl) return e ? void(p.thumbPath_str != e && (p.thumbPath_str = e,
							 																																									p.thumb_img && (p.thumb_img.onload = null,
								               																																	p.thumb_img.onerror = null,
															 																																	p.thumb_img = null),
															 																																	p.thumbPath_str
															 																																	&& (p.thumb_img = new Image,
																   																																p.thumb_img.onload = p.thumbImageLoadComplete,
																	 																																p.thumb_img.onerror = p.thumbImageLoadError,
																	 																																p.thumb_img.src = p.thumbPath_str)))
																																: (p.cleanThumbnails(!0), void(p.thumbPath_str = "none"))
				},
				this.thumbImageLoadError = function() {
					p.cleanThumbnails(!0)
				},
				this.thumbImageLoadComplete = function() {
					var e = new FWDMSPDisplayObject("img");
					e.setScreen(p.thumb_img);
					var t = p.thumb_img.width,
						o = p.thumb_img.height,
						s = p.thumbWidthAndHeight / t,
						i = p.thumbWidthAndHeight / o,
						n = 0;
					s <= i ? n = s : i <= s && (n = i),
					e.setWidth(parseInt(t * n)),
					e.setHeight(parseInt(o * n)),
					e.setX(parseInt((p.thumbWidthAndHeight - t * n) / 2)),
					e.setY(parseInt((p.thumbWidthAndHeight - o * n) / 2)),
					e.setAlpha(0);
					for (var l = 0; l < p.thumb_do.getNumChildren(); l++)
					child = p.thumb_do.getChildAt(l),
					FWDAnimation.killTweensOf(child);
					FWDAnimation.to(e, .8, {
						alpha: 1,
						delay: .2,
						ease: Expo.easeOut,
						onComplete: p.cleanThumbnails
					}),
					p.thumb_do.addChild(e)
				},
				this.cleanThumbnails = function(e) {
					for (var t, o = e ? 0 : 1; p.thumb_do.getNumChildren() > o;) t = p.thumb_do.getChildAt(0), FWDAnimation.killTweensOf(t), p.thumb_do.removeChild(t), t.destroy()
				},
				this.setupDisable = function() {
					p.disable_do = new FWDMSPDisplayObject("div"),
						FWDMSPUtils.isIE && (p.disable_do.setBkColor("#FFFFFF"),
						p.disable_do.setAlpha(0))
				},
				this.setupAtbButton = function() {
					FWDMSPSimpleButton.setPrototype(),
					p.atbButton_do = new FWDMSPSimpleButton(_.atbNPath_img,
																								  _.atbSPath_str,
																									void 0,
																									!0,
																									p.useHEXColorsForSkin_bl,
																									p.normalButtonsColor_str,
																									p.selectedButtonsColor_str),
				  p.atbButton_do.addListener(FWDMSPSimpleButton.MOUSE_UP, p.atbButtonMouseUpHandler),
					p.atbButton_do.setX(-5e3), p.atbButton_do.setY(parseInt((p.stageHeight - p.atbButton_do.h) / 2)),
					p.mainHolder_do.addChild(p.atbButton_do)
				},
				this.atbButtonMouseUpHandler = function() {
					p.dispatchEvent(n.SHOW_ATOB)
				},
				this.disableAtbButton = function() {
					p.atbButton_do && p.atbButton_do.disable()
				},
				this.enableAtbButton = function() {
					p.atbButton_do && p.atbButton_do.enable()
				},
				this.setupPlaybacRateButton = function() {
					FWDMSPSimpleButton.setPrototype(),
					p.playbackRateButton_do = new FWDMSPSimpleButton(_.playbackRateNormal_img,
						                                               _.playbackRateSelectedPath_str,
																													 null,
																													 !0,
																													 _.useHEXColorsForSkin_bl,
																													 _.normalButtonsColor_str,
																													 _.selectedButtonsColor_str),
					p.playbackRateButton_do.addListener(FWDMSPSimpleButton.MOUSE_UP, p.playbacRateButtonOnMouseUpHandler),
					p.buttons_ar.push(p.playbackRateButton_do),
					p.mainHolder_do.addChild(p.playbackRateButton_do)
				},
				this.playbacRateButtonOnMouseUpHandler = function() {
					p.dispatchEvent(n.SHOW_PLAYBACKRATE)
				},
				this.setupPrevButton = function() {
					FWDMSPSimpleButton.setPrototype(),
					p.prevButton_do = new FWDMSPSimpleButton(p.prevN_img,
						                                       _.prevSPath_str,
																									 null,
																									 !0,
																									 _.useHEXColorsForSkin_bl,
																									 _.normalButtonsColor_str,
																									 _.selectedButtonsColor_str),
				   p.prevButton_do.addListener(FWDMSPSimpleButton.MOUSE_UP, p.prevButtonOnMouseUpHandler),
					 p.buttons_ar.push(p.prevButton_do),
					 p.mainHolder_do.addChild(p.prevButton_do),
					p.showNextAndPrevButtons_bl || this.prevButton_do.setWidth(0)
				},
				this.prevButtonOnMouseUpHandler = function() {
					p.dispatchEvent(n.PLAY_PREV)
				},
				this.setupPlayPauseButton = function() {
					FWDMSPComplexButton.setPrototype(),
					p.playPauseButton_do = new FWDMSPComplexButton(p.playN_img,
						                                             _.playSPath_str,
																												 p.pauseN_img,
																												 _.pauseSPath_str,
																												 !0,
																												 _.useHEXColorsForSkin_bl,
																												 _.normalButtonsColor_str,
																												 _.selectedButtonsColor_str),
				  p.buttons_ar.push(p.playPauseButton_do),
					p.playPauseButton_do.addListener(FWDMSPComplexButton.MOUSE_UP, p.playButtonMouseUpHandler),
					p.mainHolder_do.addChild(p.playPauseButton_do)
				},
				this.showPlayButton = function() {
					p.playPauseButton_do && p.playPauseButton_do.setButtonState(1)
				},
				this.showPauseButton = function() {
					p.playPauseButton_do && p.playPauseButton_do.setButtonState(0)
				},
				this.playButtonMouseUpHandler = function() {
					0 == p.playPauseButton_do.currentState ? p.dispatchEvent(n.PAUSE) : p.dispatchEvent(n.PLAY)
				},
				this.setupNextButton = function() {
					FWDMSPSimpleButton.setPrototype(),
					p.nextButton_do = new FWDMSPSimpleButton(p.nextN_img,
						                                       _.nextSPath_str,
																									 null,
																									 !0,
																									 _.useHEXColorsForSkin_bl,
																									 _.normalButtonsColor_str,
																									 _.selectedButtonsColor_str),
			    p.nextButton_do.addListener(FWDMSPSimpleButton.MOUSE_UP, p.nextButtonOnMouseUpHandler),
					p.nextButton_do.setY(parseInt((p.stageHeight - p.nextButton_do.h) / 2)),
					p.buttons_ar.push(p.nextButton_do),
					p.mainHolder_do.addChild(p.nextButton_do),
					p.showNextAndPrevButtons_bl || this.nextButton_do.setWidth(0)
				},
				this.nextButtonOnMouseUpHandler = function() {
					p.dispatchEvent(n.PLAY_NEXT)
				},
				this.setupSeparators = function() {
					p.firstSeparator_do = new FWDMSPDisplayObject("img"),
					p.firstSeparator_do.setScreen(p.separator1_img),
					p.secondSeparator_do = new FWDMSPDisplayObject("img"),
					p.secondSeparator_do.setScreen(p.separator2_img),
					p.firstSeparator_do.setX(-10),
					p.secondSeparator_do.setX(-10),
					p.firstSeparator_do.setY(parseInt((p.stageHeight - p.firstSeparator_do.h) / 2)),
					p.secondSeparator_do.setY(parseInt((p.stageHeight - p.secondSeparator_do.h) / 2)),
					p.mainHolder_do.addChild(p.firstSeparator_do),
					p.mainHolder_do.addChild(p.secondSeparator_do)
				},
				this.setupTitlebar = function() {
					p.mainTitlebar_do = new FWDMSPDisplayObject("div"),
					p.mainTitlebar_do.getStyle().background = "url('" + p.titlebarBkMiddlePattern_str + "')",
					p.mainTitlebar_do.setHeight(p.titlebarHeight),
					p.titleBarLeft_do = new FWDMSPDisplayObject("img"),
					p.titleBarLeft_do.setScreen(p.titleBarLeft_img),
					p.titleBarRight_do = new FWDMSPDisplayObject("img"),
					p.titleBarRight_do.setScreen(p.titleBarRigth_img),
					p.simpleText_do = new FWDMSPDisplayObject("div"),
					p.simpleText_do.setOverflow("visible"),
					p.simpleText_do.hasTransform3d_bl = !1,
					p.simpleText_do.hasTransform2d_bl = !1,
					p.simpleText_do.getStyle().fontFamily = "Arial",
					p.simpleText_do.getStyle().fontSize = "12px",
					p.simpleText_do.getStyle().whiteSpace = "nowrap",
					p.simpleText_do.getStyle().textAlign = "left",
					p.simpleText_do.getStyle().color = p.titleColor_str,
					p.simpleText_do.getStyle().fontSmoothing = "antialiased",
					p.simpleText_do.getStyle().webkitFontSmoothing = "antialiased",
					p.simpleText_do.getStyle().textRendering = "optimizeLegibility",
					p.animText1_do = new FWDMSPDisplayObject("div"),
					p.animText1_do.setOverflow("visible"),
					p.animText1_do.hasTransform3d_bl = !1,
					p.animText1_do.hasTransform2d_bl = !1,
					p.animText1_do.getStyle().fontFamily = "Arial",
					p.animText1_do.getStyle().fontSize = "12px",
					p.animText1_do.getStyle().whiteSpace = "nowrap",
					p.animText1_do.getStyle().textAlign = "left",
					p.animText1_do.getStyle().color = p.titleColor_str,
					p.animText1_do.getStyle().fontSmoothing = "antialiased",
					p.animText1_do.getStyle().webkitFontSmoothing = "antialiased",
					p.animText1_do.getStyle().textRendering = "optimizeLegibility",
					p.animText2_do = new FWDMSPDisplayObject("div"),
					p.animText2_do.setOverflow("visible"),
					p.animText2_do.hasTransform3d_bl = !1,
					p.animText2_do.hasTransform2d_bl = !1,
					p.animText2_do.getStyle().fontFamily = "Arial",
					p.animText2_do.getStyle().fontSize = "12px",
					p.animText2_do.getStyle().whiteSpace = "nowrap",
					p.animText2_do.getStyle().textAlign = "left",
					p.animText2_do.getStyle().color = p.titleColor_str,
					p.animText2_do.getStyle().fontSmoothing = "antialiased",
					p.animText2_do.getStyle().webkitFontSmoothing = "antialiased",
					p.animText2_do.getStyle().textRendering = "optimizeLegibility",
					p.titleBarGradLeft_do = new FWDMSPDisplayObject("img"),
					p.titleBarGradLeft_do.setScreen(p.titlebarLeftPath_img),
					p.titleBarGradLeft_do.setX(-50),
					p.titlebarGradRight_do = new FWDMSPDisplayObject("img"),
					p.titlebarGradRight_do.setScreen(p.titlebarRightPath_img),
					p.showSoundAnimation_bl ? (p.animationBackground_do = new FWDMSPDisplayObject("img"),
					                           p.animationBackground_do.setScreen(p.titlebarAnimBkPath_img),
																		 p.animationHolderWidth = p.animationBackground_do.w,
																		 p.simpleText_do.setX(p.animationBackground_do.w + 5),
																		 FWDMSPPreloader.setPrototype(),
																		 p.animation_do = new FWDMSPPreloader(_.animationPath_str, 29, 22, 31, 80, !0),
																		 p.animation_do.setX(p.equlizerOffsetLeft),
																		 p.animation_do.setY(0),
																		 p.animation_do.show(!0),
																		 p.animation_do.stop())
																	: p.simpleText_do.setX(5),
					p.positionTitleId_to = setTimeout(function e() {
							if (null == p) return;
							clearTimeout(p.positionTitleId_to);
							0 == p.simpleText_do.getHeight() ? p.positionTitleId_to = setTimeout(e, 200) : (p.simpleText_do.setY(parseInt((p.mainTitlebar_do.h - p.simpleText_do.getHeight()) / 2) + 1),
							p.animText1_do.setY(parseInt((p.mainTitlebar_do.h - p.simpleText_do.getHeight()) / 2) + 1),
							p.animText2_do.setY(parseInt((p.mainTitlebar_do.h - p.simpleText_do.getHeight()) / 2) + 1))
						}, 1e3),
					p.mainTitlebar_do.addChild(p.titleBarLeft_do),
					p.mainTitlebar_do.addChild(p.titleBarRight_do),
					p.mainTitlebar_do.addChild(p.simpleText_do),
					p.mainTitlebar_do.addChild(p.animText1_do),
					p.mainTitlebar_do.addChild(p.animText2_do),
					p.showSoundAnimation_bl && (p.mainTitlebar_do.addChild(p.animationBackground_do),
					                            p.mainTitlebar_do.addChild(p.animation_do)),
				  p.mainTitlebar_do.addChild(p.titleBarGradLeft_do),
					p.mainTitlebar_do.addChild(p.titlebarGradRight_do),
					p.mainHolder_do.addChild(p.mainTitlebar_do)
				},
				this.setTitle = function(e) {
					p.simpleText_do.setInnerHTML(e),
					p.animText1_do.setInnerHTML(e + "***"),
					p.animText2_do.setInnerHTML(e + "***"),
					p.animText1_do.setX(-1e3),
					p.animText2_do.setX(-1e3),
					p.startToCheckIfAnimTitle(!0)
				},
				this.startToCheckIfAnimTitle = function(e) {
					e && p.stopToAnimateText(),
					clearTimeout(p.animateTextId_to),
					clearTimeout(p.startToAnimateTextId_to),
					p.animateTextId_to = setTimeout(p.checkIfAnimTitle, 10)
				},
				this.checkIfAnimTitle = function() {
					var e = p.mainTitlebar_do.w - 5 - p.titlebarGradRight_do.w;
					if (e -= p.animationHolderWidth, p.simpleText_do.getWidth() > e) {
						if (p.isTextAnimating_bl) return;
						p.showSoundAnimation_bl ? p.titleBarGradLeft_do.setX(p.animationHolderWidth) : p.titleBarGradLeft_do.setX(0),
						p.titlebarGradRight_do.setY(0),
						clearTimeout(p.startToAnimateTextId_to),
						p.startToAnimateTextId_to = setTimeout(p.startToAnimateText, 300)
					} else p.titleBarGradLeft_do.setX(-50), p.titlebarGradRight_do.setY(-50), p.stopToAnimateText()
				},
				this.startToAnimateText = function() {
					p.isTextAnimating_bl || (p.isTextAnimating_bl = !0,
						                       p.animTextWidth = p.animText1_do.getWidth(),
																	 p.simpleText_do.setX(-1e3),
																	 p.animText1_do.setX(p.animationHolderWidth + 5),
																	 p.animText2_do.setX(p.animationHolderWidth + p.animTextWidth + 10),
																	 clearInterval(p.animateTextId_int),
																	 p.animateTextId_int = setInterval(p.animateText, 40))
				},
				this.stopToAnimateText = function() {
					p.isTextAnimating_bl && (p.isTextAnimating_bl = !1,
						                       p.simpleText_do.setX(p.animationHolderWidth + 5),
																	 p.animText1_do.setX(-1e3),
																	 p.animText2_do.setX(-1e3), clearInterval(p.animateTextId_int))
				},
				this.animateText = function() {
					p.animText1_do.setX(p.animText1_do.x - 1),
					p.animText2_do.setX(p.animText2_do.x - 1),
					p.animText1_do.x < -(p.animTextWidth - p.animationHolderWidth) && p.animText1_do.setX(p.animText2_do.x + p.animTextWidth + 5),
					p.animText2_do.x < -(p.animTextWidth - p.animationHolderWidth) && p.animText2_do.setX(p.animText1_do.x + p.animTextWidth + 5)
				},
				this.stopEqulizer = function() {
					p.animation_do && p.animation_do.stop()
				},
				this.startEqulizer = function() {
					p.animation_do && p.animation_do.start()
				},
				this.setupMainScrubber = function() {
					p.mainScrubber_do = new FWDMSPDisplayObject("div"),
					p.mainScrubber_do.setY(parseInt((p.stageHeight - p.scrubbersHeight) / 2)),
					p.mainScrubber_do.setHeight(p.scrubbersHeight),
					p.mainScrubberBkLeft_do = new FWDMSPDisplayObject("img"),
					p.mainScrubberBkLeft_do.setScreen(p.mainScrubberBkLeft_img),
					p.mainScrubberBkRight_do = new FWDMSPDisplayObject("img"),
					p.mainScrubberBkRight_do.setScreen(p.mainScrubberBkRight_img);
					var e = new Image;
					e.src = p.mainScrubberBkMiddlePath_str,
					p.mainScrubberBkMiddle_do = new FWDMSPDisplayObject("div"),
					p.mainScrubberBkMiddle_do.getStyle().background = "url('" + p.mainScrubberBkMiddlePath_str + "')",
					p.mainScrubberBkMiddle_do.setHeight(p.scrubbersHeight),
					p.mainScrubberBkMiddle_do.setX(p.scrubbersBkLeftAndRightWidth),
					p.mainProgress_do = new FWDMSPDisplayObject("div"),
					p.mainProgress_do.setHeight(p.scrubbersHeight),
					p.progressLeft_do = new FWDMSPDisplayObject("img"),
					p.progressLeft_do.setScreen(p.mainScrubberLeftProgress_img),
					(e = new Image).src = p.progressMiddlePath_str,
					p.progressMiddle_do = new FWDMSPDisplayObject("div"),
					p.progressMiddle_do.getStyle().background = "url('" + p.progressMiddlePath_str + "')",
					p.progressMiddle_do.setHeight(p.scrubbersHeight),
					p.progressMiddle_do.setX(p.mainScrubberDragLeftWidth),
					p.mainScrubberDrag_do = new FWDMSPDisplayObject("div"),
					p.mainScrubberDrag_do.setHeight(p.scrubbersHeight),
					p.useHEXColorsForSkin_bl ? (p.mainScrubberDragLeft_do = new FWDMSPDisplayObject("div"),
					                            p.mainScrubberDragLeft_do.setWidth(p.mainScrubberDragLeft_img.width),
																			p.mainScrubberDragLeft_do.setHeight(p.mainScrubberDragLeft_img.height),
																			p.mainScrubberDragLeft_canvas = FWDMSPUtils.getCanvasWithModifiedColor(p.mainScrubberDragLeft_img, p.normalButtonsColor_str).canvas,
																			p.mainScrubberDragLeft_do.screen.appendChild(p.mainScrubberDragLeft_canvas))
																	 : (p.mainScrubberDragLeft_do = new FWDMSPDisplayObject("img"),
																	    p.mainScrubberDragLeft_do.setScreen(p.mainScrubberDragLeft_img)),
					p.mainScrubberMiddleImage = new Image,
					p.mainScrubberMiddleImage.src = p.mainScrubberDragMiddlePath_str,
					p.volumeScrubberDragMiddle_do = new FWDMSPDisplayObject("div"),
					p.useHEXColorsForSkin_bl ? (p.mainScrubberDragMiddle_do = new FWDMSPDisplayObject("div"),
					                            p.mainScrubberMiddleImage.onload = function() {
							                        var e = FWDMSPUtils.getCanvasWithModifiedColor(p.mainScrubberMiddleImage, p.normalButtonsColor_str, !0);
																			p.mainSCrubberMiddleCanvas = e.canvas,
																			p.mainSCrubberDragMiddleImageBackground = e.image,
																			p.mainScrubberDragMiddle_do.getStyle().background = "url('" + p.mainSCrubberDragMiddleImageBackground.src + "') repeat-x",
																			setTimeout(function() {
																				p.volumeScrubberDragMiddle_do.getStyle().background = "url('" + p.mainSCrubberDragMiddleImageBackground.src + "') repeat-x"
																			}, 50)
																			})
																		: (p.mainScrubberDragMiddle_do = new FWDMSPDisplayObject("div"),
																		  p.mainScrubberDragMiddle_do.getStyle().background = "url('" + p.mainScrubberDragMiddlePath_str + "') repeat-x"),
																			p.mainScrubberDragMiddle_do.setHeight(p.scrubbersHeight),
																			p.mainScrubberDragMiddle_do.setX(p.mainScrubberDragLeftWidth),
																			p.mainScrubberBarLine_do = new FWDMSPDisplayObject("img"),
																			p.mainScrubberBarLine_do.setScreen(p.mainScrubberLine_img),
																			p.mainScrubberBarLine_do.setAlpha(0),
																			p.mainScrubberBarLine_do.hasTransform3d_bl = !1,
																			p.mainScrubberBarLine_do.hasTransform2d_bl = !1,
																			p.mainScrubber_do.addChild(p.mainScrubberBkLeft_do),
																			p.mainScrubber_do.addChild(p.mainScrubberBkMiddle_do),
																			p.mainScrubber_do.addChild(p.mainScrubberBkRight_do),
																			p.mainScrubberDrag_do.addChild(p.mainScrubberDragLeft_do),
																			p.mainScrubberDrag_do.addChild(p.mainScrubberDragMiddle_do),
																			p.mainProgress_do.addChild(p.progressLeft_do),
																			p.mainProgress_do.addChild(p.progressMiddle_do),
																			p.mainScrubber_do.addChild(p.mainProgress_do),
																			p.mainScrubber_do.addChild(p.mainScrubberDrag_do),
																			p.mainScrubber_do.addChild(p.mainScrubberBarLine_do),
																			p.mainHolder_do.addChild(p.mainScrubber_do),
																			p.disableScrubber_bl
																		|| (p.hasPointerEvent_bl
																				? (p.mainScrubber_do.screen.addEventListener("pointerover", p.mainScrubberOnOverHandler),
																		   		p.mainScrubber_do.screen.addEventListener("pointerout", p.mainScrubberOnOutHandler),
																			 		p.mainScrubber_do.screen.addEventListener("pointerdown", p.mainScrubberOnDownHandler))
																				: p.screen.addEventListener && (p.isMobile_bl || (p.mainScrubber_do.screen.addEventListener("mouseover", p.mainScrubberOnOverHandler),
																		                                                  p.mainScrubber_do.screen.addEventListener("mouseout", p.mainScrubberOnOutHandler),
																																											p.mainScrubber_do.screen.addEventListener("mousedown", p.mainScrubberOnDownHandler)),
																																	  p.mainScrubber_do.screen.addEventListener("touchstart", p.mainScrubberOnDownHandler))),
						p.disableMainScrubber()
				},
				this.mainScrubberOnOverHandler = function(e) {
					if (!p.isMainScrubberDisabled_bl) {
						0 != f.totalDuration, !p.isMobile_bl && p.ttm && window.addEventListener("mousemove", p.mainScrubberWMouseMove);
						var t = FWDMSPUtils.getViewportMouseCoordinates(e).screenX - p.mainScrubber_do.getGlobalX();
						t < 0 ? t = 0 : t > p.mainScrubberWidth - p.scrubbersOffsetWidth && (t = p.mainScrubberWidth - p.scrubbersOffsetWidth);
						var o = t / p.mainScrubberWidth;
					}
				},
				p.mainScrubberWMouseMove = function(e) {
					var t = FWDMSPUtils.getViewportMouseCoordinates(e);
					p.vcX = t.screenX,
					p.vcY = t.screenY,
					FWDMSPUtils.hitTest(p.mainScrubber_do.screen, p.vcX, p.vcY) || p.isMainScrubberScrubbing_bl || (window.removeEventListener("mousemove", p.mainScrubberWMouseMove), p.ttm.hide());
					var o = FWDMSPUtils.getViewportMouseCoordinates(e).screenX - p.mainScrubber_do.getGlobalX();
					o < 0 ? o = 0 : o > p.mainScrubberWidth - p.scrubbersOffsetWidth && (o = p.mainScrubberWidth - p.scrubbersOffsetWidth);
					var s = o / p.mainScrubberWidth;
				},
				this.mainScrubberOnOutHandler = function(e) {
					p.isMainScrubberDisabled_bl || p.isMainScrubberScrubbing_bl || p.ttm && p.ttm.hide()
				},
				this.mainScrubberOnDownHandler = function(e) {
					if (!p.isMainScrubberDisabled_bl) {
						e.preventDefault && e.preventDefault(), p.isMainScrubberScrubbing_bl = !0;
						var t = FWDMSPUtils.getViewportMouseCoordinates(e).screenX - p.mainScrubber_do.getGlobalX();
						t < 0 ? t = 0 : t > p.mainScrubberWidth - p.scrubbersOffsetWidth && (t = p.mainScrubberWidth - p.scrubbersOffsetWidth);
						var o = t / p.mainScrubberWidth;
						!FWDMSP.hasHTML5Audio && t >= p.mainProgress_do.w && (t = p.mainProgress_do.w);
						var s = t / p.mainScrubberWidth;
						p.disable_do && p.addChild(p.disable_do),
							p.updateMainScrubber(o), p.dispatchEvent(n.START_TO_SCRUB), p.dispatchEvent(n.SCRUB_PLAYLIST_ITEM, {
								percent: s
							}), p.dispatchEvent(n.SCRUB, {
								percent: o
							}), p.hasPointerEvent_bl ? (window.addEventListener("pointermove", p.mainScrubberMoveHandler), window.addEventListener("pointerup", p.mainScrubberEndHandler)) : (window.addEventListener("mousemove", p.mainScrubberMoveHandler), window.addEventListener("mouseup", p.mainScrubberEndHandler), window.addEventListener("touchmove", p.mainScrubberMoveHandler), window.addEventListener("touchend", p.mainScrubberEndHandler))
					}
				},
				this.mainScrubberMoveHandler = function(e) {
					e.preventDefault && e.preventDefault();
					var t = FWDMSPUtils.getViewportMouseCoordinates(e);
					p.vcX = t.screenX,
					p.vcY = t.screenY,
					FWDMSPUtils.hitTest(p.mainScrubber_do.screen, p.vcX, p.vcY) || p.isMainScrubberScrubbing_bl
					                                                            || (window.removeEventListener("mousemove", p.mainScrubberWMouseMove), p.ttm.hide());
					var o = FWDMSPUtils.getViewportMouseCoordinates(e).screenX - p.mainScrubber_do.getGlobalX();
					FWDMSPUtils.hitTest(p.mainScrubber_do.screen, p.vcX, p.vcY) || p.isMainScrubberScrubbing_bl
					                                                            || (window.removeEventListener("mousemove", p.mainScrubberWMouseMove), p.ttm.hide()),
					o < 0 ? o = 0 : o > p.mainScrubberWidth - p.scrubbersOffsetWidth && (o = p.mainScrubberWidth - p.scrubbersOffsetWidth);
					var s = o / p.mainScrubberWidth;
					!FWDMSP.hasHTML5Audio && o >= p.mainProgress_do.w && (o = p.mainProgress_do.w);
					var i = o / p.mainScrubberWidth;
					p.updateMainScrubber(s),
					p.dispatchEvent(n.SCRUB_PLAYLIST_ITEM, {
						percent: i
					}), p.dispatchEvent(n.SCRUB, {
						percent: s
					})
				},
				this.mainScrubberEndHandler = function(e) {
					if (p.disable_do && p.contains(p.disable_do) && p.removeChild(p.disable_do), p.isMainScrubberScrubbing_bl = !1, e) {
						var t = FWDMSPUtils.getViewportMouseCoordinates(e);
						FWDMSPUtils.hitTest(p.mainScrubber_do.screen, t.screenX, t.screenY) || p.ttm && p.ttm.hide()
					}
					p.dispatchEvent(n.STOP_TO_SCRUB), p.hasPointerEvent_bl ? (window.removeEventListener("pointermove", p.mainScrubberMoveHandler), window.removeEventListener("pointerup", p.mainScrubberEndHandler)) : (window.removeEventListener("mousemove", p.mainScrubberMoveHandler), window.removeEventListener("mouseup", p.mainScrubberEndHandler), window.removeEventListener("touchmove", p.mainScrubberMoveHandler), window.removeEventListener("touchend", p.mainScrubberEndHandler))
				},
				this.disableMainScrubber = function() {
					p.mainScrubber_do && (p.isMainScrubberDisabled_bl = !0, p.mainScrubber_do.setButtonMode(!1), p.updateMainScrubber(0), p.updatePreloaderBar(0), p.mainScrubberEndHandler(), p.disableAtbButton())
				},
				this.enableMainScrubber = function() {
					p.mainScrubber_do && (p.isMainScrubberDisabled_bl = !1, p.disableScrubber_bl || p.mainScrubber_do.setButtonMode(!0), p.enableAtbButton())
				},
				this.updateMainScrubber = function(e) {
					if (p.mainScrubber_do && !isNaN(e)) {
						var t = parseInt(e * p.mainScrubberWidth);
						p.percentPlayed = e,
						!FWDMSP.hasHTML5Audio && t >= p.mainProgress_do.w && (t = p.mainProgress_do.w), t < 1 && p.isMainScrubberLineVisible_bl ? (p.isMainScrubberLineVisible_bl = !1, FWDAnimation.to(p.mainScrubberBarLine_do, .5, {
								alpha: 0
							})) : 2 < t && !p.isMainScrubberLineVisible_bl && (p.isMainScrubberLineVisible_bl = !0, FWDAnimation.to(p.mainScrubberBarLine_do, .5, {
								alpha: 1
							})), p.mainScrubberDrag_do.setWidth(t), t > p.mainScrubberWidth - p.scrubbersOffsetWidth && (t = p.mainScrubberWidth - p.scrubbersOffsetWidth),
							FWDAnimation.to(p.mainScrubberBarLine_do, .8, {
								x: t,
								ease: Expo.easeOut
							})
					}
				},
				this.updatePreloaderBar = function(e) {
					if (p.mainProgress_do) {
						var t = parseInt(e * p.mainScrubberWidth);
						1 == e ? p.mainProgress_do.setY(-30) : 0 != p.mainProgress_do.y && 1 != e && p.mainProgress_do.setY(0), t > p.mainScrubberWidth - p.scrubbersOffsetWidth && (t = p.mainScrubberWidth - p.scrubbersOffsetWidth), t < 0 && (t = 0), p.mainProgress_do.setWidth(t)
					}
				},
				this.setupTime = function() {
					p.currentTime_do = new FWDMSPDisplayObject("div"),
					p.currentTime_do.hasTransform3d_bl = !1,
					p.currentTime_do.hasTransform2d_bl = !1,
					p.currentTime_do.getStyle().fontFamily = "Arial",
					p.currentTime_do.getStyle().fontSize = "12px",
					p.currentTime_do.getStyle().whiteSpace = "nowrap",
					p.currentTime_do.getStyle().textAlign = "left",
					p.currentTime_do.getStyle().color = p.timeColor_str,
					p.currentTime_do.getStyle().fontSmoothing = "antialiased",
					p.currentTime_do.getStyle().webkitFontSmoothing = "antialiased",
					p.currentTime_do.getStyle().textRendering = "optimizeLegibility",
					p.currentTime_do.setInnerHTML("00"),
					p.mainHolder_do.addChild(p.currentTime_do),
					p.totalTime_do = new FWDMSPDisplayObject("div"),
					p.totalTime_do.hasTransform3d_bl = !1,
					p.totalTime_do.hasTransform2d_bl = !1,
					p.totalTime_do.getStyle().fontFamily = "Arial",
					p.totalTime_do.getStyle().fontSize = "12px",
					p.totalTime_do.getStyle().whiteSpace = "nowrap",
					p.totalTime_do.getStyle().textAlign = "right",
					p.totalTime_do.getStyle().color = p.timeColor_str,
					p.totalTime_do.getStyle().fontSmoothing = "antialiased",
					p.totalTime_do.getStyle().webkitFontSmoothing = "antialiased",
					p.totalTime_do.getStyle().textRendering = "optimizeLegibility",
					p.mainHolder_do.addChild(p.totalTime_do),
					p.updateTime(),
						setTimeout(function() {
							null != p && (p.timeHeight = p.currentTime_do.getHeight(),
							              p.currentTime_do.h = p.timeHeight,
														p.totalTime_do.h = p.timeHeight,
														p.stageWidth = f.stageWidth,
														p.positionButtons())
						}, 100)
				},
				this.updateTime = function(e, t) {
					if (p.currentTime_do && t && ("00:00" == t && (t = e), p.currentTime_do.setInnerHTML(e), p.totalTime_do.setInnerHTML(t), e.length != p.lastTotalTimeLength || t.length != p.lastCurTimeLength)) {
						var o = p.currentTime_do.offsetWidth,
							s = p.totalTime_do.offsetWidth;
						p.currentTime_do.w = o,
						p.totalTime_do.w = s, p.positionButtons(),
						setTimeout(function() {
							p.currentTime_do.w = p.currentTime_do.getWidth(),
							p.totalTime_do.w = p.totalTime_do.getWidth(),
							p.positionButtons()
						}, 50),
						p.lastCurTimeLength = e.length,
						p.lastTotalTimeLength = t.length
					}
				},
				this.setupVolumeScrubber = function() {
					p.mainVolumeHolder_do = new FWDMSPDisplayObject("div"),
					p.mainVolumeHolder_do.setHeight(p.volumeN_img.height),
					p.mainHolder_do.addChild(p.mainVolumeHolder_do),
					FWDMSPSimpleButton.setPrototype(),
					p.volumeButton_do = new FWDMSPSimpleButton(p.volumeN_img,
						                                         _.volumeSPath_str,
																										 _.volumeDPath_str,
																										 !0,
																										 _.useHEXColorsForSkin_bl,
																										 _.normalButtonsColor_str,
																										 _.selectedButtonsColor_str),
					p.volumeButton_do.addListener(FWDMSPSimpleButton.MOUSE_UP, p.volumeButtonOnMouseUpHandler),
					p.allowToChangeVolume_bl || p.volumeButton_do.disable(),
					p.volumeScrubber_do = new FWDMSPDisplayObject("div"),
					p.volumeScrubber_do.setHeight(p.scrubbersHeight),
					p.volumeScrubber_do.setX(p.volumeButton_do.w),
					p.volumeScrubber_do.setY(parseInt((p.volumeButton_do.h - p.scrubbersHeight) / 2)),
					p.volumeScrubberBkLeft_do = new FWDMSPDisplayObject("img");
					var e = new Image;
					e.src = p.mainScrubberBkLeft_do.screen.src,
					p.volumeScrubberBkLeft_do.setScreen(e),
					p.volumeScrubberBkLeft_do.setWidth(p.mainScrubberBkLeft_do.w),
					p.volumeScrubberBkLeft_do.setHeight(p.mainScrubberBkLeft_do.h),
					p.volumeScrubberBkRight_do = new FWDMSPDisplayObject("img");
					var t = new Image;
					t.src = p.mainScrubberBkRight_do.screen.src,
					p.volumeScrubberBkRight_do.setScreen(t),
					p.volumeScrubberBkRight_do.setWidth(p.mainScrubberBkRight_do.w),
					p.volumeScrubberBkRight_do.setHeight(p.mainScrubberBkRight_do.h),
					(new Image).src = p.volumeScrubberBkMiddlePath_str,
					p.volumeScrubberBkMiddle_do = new FWDMSPDisplayObject("div"),
					p.volumeScrubberBkMiddle_do.getStyle().background = "url('" + p.volumeScrubberBkMiddlePath_str + "')",
					p.volumeScrubberBkMiddle_do.setHeight(p.scrubbersHeight),
					p.volumeScrubberBkMiddle_do.setX(p.scrubbersBkLeftAndRightWidth),
					p.volumeScrubberDrag_do = new FWDMSPDisplayObject("div"),
					p.volumeScrubberDrag_do.setHeight(p.scrubbersHeight),
					p.useHEXColorsForSkin_bl ? (p.volumeScrubberDragLeft_do = new FWDMSPDisplayObject("div"),
					                            p.volumeScrubberDragLeft_do.setWidth(p.volumeScrubberDragLeft_img.width),
																			p.volumeScrubberDragLeft_do.setHeight(p.volumeScrubberDragLeft_img.height),
																			p.volumeScrubberDragLeft_canvas = FWDMSPUtils.getCanvasWithModifiedColor(p.volumeScrubberDragLeft_img, p.normalButtonsColor_str).canvas,
																			p.volumeScrubberDragLeft_do.screen.appendChild(p.volumeScrubberDragLeft_canvas))
																	 : (p.volumeScrubberDragLeft_do = new FWDMSPDisplayObject("img"),
																	    p.volumeScrubberDragLeft_do.setScreen(p.volumeScrubberDragLeft_img)),
					p.useHEXColorsForSkin_bl || (p.volumeScrubberDragMiddle_do = new FWDMSPDisplayObject("div"),
					                             p.volumeScrubberDragMiddle_do.getStyle().background = "url('" + p.volumeScrubberDragMiddlePath_str + "') repeat-x"),
																			 p.volumeScrubberDragMiddle_do.setHeight(p.scrubbersHeight),
																			 p.volumeScrubberDragMiddle_do.setX(p.mainScrubberDragLeftWidth),
																			 p.volumeScrubberBarLine_do = new FWDMSPDisplayObject("img");
					var o = new Image;
					o.src = p.mainScrubberBarLine_do.screen.src,
					p.volumeScrubberBarLine_do.setScreen(o),
					p.volumeScrubberBarLine_do.setWidth(p.mainScrubberBarLine_do.w),
					p.volumeScrubberBarLine_do.setHeight(p.mainScrubberBarLine_do.h),
					p.volumeScrubberBarLine_do.setAlpha(0),
					p.volumeScrubberBarLine_do.hasTransform3d_bl = !1,
					p.volumeScrubberBarLine_do.hasTransform2d_bl = !1,
					p.volumeScrubber_do.addChild(p.volumeScrubberBkLeft_do),
					p.volumeScrubber_do.addChild(p.volumeScrubberBkMiddle_do),
					p.volumeScrubber_do.addChild(p.volumeScrubberBkRight_do),
					p.volumeScrubber_do.addChild(p.volumeScrubberBarLine_do),
					p.volumeScrubberDrag_do.addChild(p.volumeScrubberDragLeft_do),
					p.volumeScrubberDrag_do.addChild(p.volumeScrubberDragMiddle_do),
					p.volumeScrubber_do.addChild(p.volumeScrubberDrag_do),
					p.volumeScrubber_do.addChild(p.volumeScrubberBarLine_do),
					p.mainVolumeHolder_do.addChild(p.volumeButton_do),
					p.mainVolumeHolder_do.addChild(p.volumeScrubber_do),
					p.allowToChangeVolume_bl && (p.hasPointerEvent_bl ? (p.volumeScrubber_do.screen.addEventListener("pointerover",
					                                                     p.volumeScrubberOnOverHandler),
																															 p.volumeScrubber_do.screen.addEventListener("pointerout", p.volumeScrubberOnOutHandler),
																															 p.volumeScrubber_do.screen.addEventListener("pointerdown", p.volumeScrubberOnDownHandler))
																														: (p.isMobile_bl || (p.volumeScrubber_do.screen.addEventListener("mouseover", p.volumeScrubberOnOverHandler),
																														                     p.volumeScrubber_do.screen.addEventListener("mouseout", p.volumeScrubberOnOutHandler),
																																								 p.volumeScrubber_do.screen.addEventListener("mousedown", p.volumeScrubberOnDownHandler)),
																												      p.volumeScrubber_do.screen.addEventListener("touchstart", p.volumeScrubberOnDownHandler))),
					p.enableVolumeScrubber(),
					p.updateVolumeScrubber(p.volume)
				},
				this.volumeButtonOnMouseUpHandler = function() {
					var e = p.lastVolume;
					p.isMute_bl ? (e = p.lastVolume, p.isMute_bl = !1) : (e = 1e-6, p.isMute_bl = !0), p.updateVolume(e)
				},
				this.volumeScrubberOnOverHandler = function(e) {
					p.isVolumeScrubberDisabled_bl
				},
				this.volumeScrubberOnOutHandler = function(e) {
					p.isVolumeScrubberDisabled_bl || p.isVolumeScrubberScrubbing_bl || p.ttm2 && p.ttm2.hide()
				},
				this.volumeScrubberOnDownHandler = function(e) {
					if (!p.isVolumeScrubberDisabled_bl) {
						e.preventDefault && e.preventDefault();
						var t = FWDMSPUtils.getViewportMouseCoordinates(e).screenX - p.volumeScrubber_do.getGlobalX();
						t < 0 ? t = 0 : t > p.volumeScrubberWidth - p.scrubbersOffsetWidth && (t = p.volumeScrubberWidth - p.scrubbersOffsetWidth);
						var o = t / p.volumeScrubberWidth;
						p.disable_do && p.addChild(p.disable_do),
						p.lastVolume = o,
						p.isVolumeScrubberScrubbing_bl = !0,
						p.updateVolume(o),
						p.dispatchEvent(n.VOLUME_START_TO_SCRUB),
						p.isMobile_bl ? p.hasPointerEvent_bl
						              ? (window.addEventListener("pointermove", p.volumeScrubberMoveHandler),
													  window.addEventListener("pointerup", p.volumeScrubberEndHandler))
													: (window.addEventListener("touchmove", p.volumeScrubberMoveHandler),
													   window.addEventListener("touchend", p.volumeScrubberEndHandler))
													: (window.addEventListener("mousemove", p.volumeScrubberMoveHandler),
													   window.addEventListener("mouseup", p.volumeScrubberEndHandler))
					}
				},
				this.volumeScrubberMoveHandler = function(e) {
					if (!p.isVolumeScrubberDisabled_bl) {
						e.preventDefault && e.preventDefault();
						var t = FWDMSPUtils.getViewportMouseCoordinates(e).screenX - p.volumeScrubber_do.getGlobalX();
						t < 0 ? t = 0 : t > p.volumeScrubberWidth - p.scrubbersOffsetWidth && (t = p.volumeScrubberWidth - p.scrubbersOffsetWidth);
						var o = t / p.volumeScrubberWidth;
						.98 <= o && (o = 1), p.lastVolume = o, p.updateVolume(o)
					}
				},
				this.volumeScrubberEndHandler = function(e) {
					if (p.dispatchEvent(n.VOLUME_STOP_TO_SCRUB),
							p.isVolumeScrubberScrubbing_bl = !1,
							p.disable_do && p.contains(p.disable_do) && p.removeChild(p.disable_do), e) {
						var t = FWDMSPUtils.getViewportMouseCoordinates(e);
						FWDMSPUtils.hitTest(p.volumeScrubber_do.screen, t.screenX, t.screenY) || p.ttm2 && p.ttm2.hide()
					}
					p.isMobile_bl ? p.hasPointerEvent_bl ? (window.removeEventListener("pointermove", p.volumeScrubberMoveHandler), window.removeEventListener("pointerup", p.volumeScrubberEndHandler)) : (window.removeEventListener("touchmove", p.volumeScrubberMoveHandler), window.removeEventListener("touchend", p.volumeScrubberEndHandler)) : window.removeEventListener ? (window.removeEventListener("mousemove", p.volumeScrubberMoveHandler), window.removeEventListener("mouseup", p.volumeScrubberEndHandler)) : document.detachEvent && (document.detachEvent("onmousemove", p.volumeScrubberMoveHandler), document.detachEvent("onmouseup", p.volumeScrubberEndHandler))
				},
				this.disableVolumeScrubber = function() {
					p.isVolumeScrubberDisabled_bl = !0,
					p.volumeScrubber_do.setButtonMode(!1),
					p.volumeScrubberEndHandler()
				},
				this.enableVolumeScrubber = function() {
					p.isVolumeScrubberDisabled_bl = !1,
					p.volumeScrubber_do.setButtonMode(!0)
				},
				this.updateVolumeScrubber = function(e) {
					var t = parseInt(e * p.volumeScrubberWidth);
					p.volume = e,
					p.volumeScrubberDrag_do.setWidth(t),
					t < 1 && p.isVolumeScrubberLineVisible_bl ? (p.isVolumeScrubberLineVisible_bl = !1,
						                                           FWDAnimation.to(p.volumeScrubberBarLine_do, .5, {
																												 alpha: 0
																											 }))
																										: 1 < t && !p.isVolumeScrubberLineVisible_bl
				        && (p.isVolumeScrubberLineVisible_bl = !0,
								FWDAnimation.to(p.volumeScrubberBarLine_do, .5, {
									alpha: 1
								})),
					t > p.volumeScrubberWidth - p.scrubbersOffsetWidth && (t = p.volumeScrubberWidth - p.scrubbersOffsetWidth),
					FWDAnimation.to(p.volumeScrubberBarLine_do, .8, {
						x: t,
						ease: Expo.easeOut
					})
				},
				this.updateVolume = function(e, t) {
					p.volume = e,
					p.volume <= 1e-6 ? (p.isMute_bl = !0, p.volume = 1e-6) : 1 <= p.volume
					                 ? (p.isMute_bl = !1, p.volume = 1) : p.isMute_bl = !1,
						1e-6 == p.volume ? p.volumeButton_do && p.volumeButton_do.setDisabledState() : p.volumeButton_do && p.volumeButton_do.setEnabledState(),
						p.volumeScrubberBarLine_do && p.updateVolumeScrubber(p.volume),
						t || p.dispatchEvent(n.CHANGE_VOLUME, {
							percent: p.volume
						})
				},
				this.setupPlaylistButton = function() {
					FWDMSPSimpleButton.setPrototype(),
					p.playlistButton_do = new FWDMSPSimpleButton(p.playlistN_img,
						                                           _.playlistSPath_str,
																											 null,
																											 !0,
																											 _.useHEXColorsForSkin_bl,
																											 _.normalButtonsColor_str,
																											 _.selectedButtonsColor_str),
					p.playlistButton_do.addListener(FWDMSPSimpleButton.MOUSE_UP, p.playlistButtonOnMouseUpHandler),
					p.playlistButton_do.setY(parseInt((p.stageHeight - p.playlistButton_do.h) / 2)),
					p.buttons_ar.push(p.playlistButton_do),
					p.mainHolder_do.addChild(p.playlistButton_do),
					p.showPlayListByDefault_bl && p.setPlaylistButtonState("selected")
				},
				this.playlistButtonOnMouseUpHandler = function() {
					p.playlistButton_do.isSelectedFinal_bl ? p.dispatchEvent(n.HIDE_PLAYLIST) : p.dispatchEvent(n.SHOW_PLAYLIST)
				},
				this.setPlaylistButtonState = function(e) {
					p.playlistButton_do && ("selected" == e ? p.playlistButton_do.setSelected() : "unselected" == e && p.playlistButton_do.setUnselected())
				},
				this.setupCategoriesButton = function() {
					FWDMSPSimpleButton.setPrototype(),
					p.categoriesButton_do = new FWDMSPSimpleButton(p.categoriesN_img,
						                                             _.categoriesSPath_str,
																												 null,
																												 !0,
																												 _.useHEXColorsForSkin_bl,
																												 _.normalButtonsColor_str,
																												 _.selectedButtonsColor_str),
					p.categoriesButton_do.addListener(FWDMSPSimpleButton.MOUSE_UP, p.categoriesButtonOnMouseUpHandler),
					p.categoriesButton_do.setY(parseInt((p.stageHeight - p.categoriesButton_do.h) / 2)),
					p.buttons_ar.push(p.categoriesButton_do),
					p.mainHolder_do.addChild(p.categoriesButton_do)
				},
				this.categoriesButtonOnMouseUpHandler = function() {
					p.dispatchEvent(n.SHOW_CATEGORIES)
				},
				this.setCategoriesButtonState = function(e) {
					p.categoriesButton_do && ("selected" == e ? p.categoriesButton_do.setSelected() : "unselected" == e && p.categoriesButton_do.setUnselected())
				},
				this.setupLoopButton = function() {
					FWDMSPSimpleButton.setPrototype(),
					p.loopButton_do = new FWDMSPSimpleButton(p.replayN_img,
						                                       _.replaySPath_str,
																									 null,
																									 !0,
																									 _.useHEXColorsForSkin_bl,
																									 _.normalButtonsColor_str,
																									 _.selectedButtonsColor_str),
					p.loopButton_do.addListener(FWDMSPSimpleButton.MOUSE_UP, p.loopButtonOnMouseUpHandler),
					p.loopButton_do.setY(parseInt((p.stageHeight - p.loopButton_do.h) / 2)),
					p.buttons_ar.push(p.loopButton_do),
					p.mainHolder_do.addChild(p.loopButton_do),
					p.loop_bl && p.setLoopStateButton("selected")
				},
				this.loopButtonOnMouseUpHandler = function() {
					p.loopButton_do.isSelectedFinal_bl ? p.dispatchEvent(n.DISABLE_LOOP) : p.dispatchEvent(n.ENABLE_LOOP)
				},
				this.setLoopStateButton = function(e) {
					p.loopButton_do && ("selected" == e ? p.loopButton_do.setSelected() : "unselected" == e && p.loopButton_do.setUnselected())
				},
				this.setupDownloadButton = function() {
					FWDMSPSimpleButton.setPrototype(),
					p.downloadButton_do = new FWDMSPSimpleButton(p.downloaderN_img,
						                                           _.downloaderSPath_str,
																											 null,
																											 !0,
																											 _.useHEXColorsForSkin_bl,
																											 _.normalButtonsColor_str,
																											 _.selectedButtonsColor_str),
					p.downloadButton_do.addListener(FWDMSPSimpleButton.MOUSE_UP, p.downloadButtonOnMouseUpHandler),
					p.downloadButton_do.setY(parseInt((p.stageHeight - p.downloadButton_do.h) / 2)),
					p.buttons_ar.push(p.downloadButton_do),
					p.mainHolder_do.addChild(p.downloadButton_do)
				},
				this.downloadButtonOnMouseUpHandler = function() {
					p.dispatchEvent(n.DOWNLOAD_MP3)
				},
				this.setupBuyButton = function() {
					FWDMSPSimpleButton.setPrototype(),
					p.buyButton_do = new FWDMSPSimpleButton(_.buyN_img,
						                                      _.buySPath_str,
																									null,
																									!0,
																									_.useHEXColorsForSkin_bl,
																									_.normalButtonsColor_str,
																									_.selectedButtonsColor_str),
																									p.buyButton_do.addListener(FWDMSPSimpleButton.MOUSE_UP, p.buyButtonOnMouseUpHandler),
					p.buttons_ar.push(p.buyButton_do),
					p.mainHolder_do.addChild(p.buyButton_do)
				},
				this.buyButtonOnMouseUpHandler = function() {
					p.dispatchEvent(n.BUY)
				},
				this.setupShuffleButton = function() {
					FWDMSPSimpleButton.setPrototype(),
					p.shuffleButton_do = new FWDMSPSimpleButton(p.shuffleN_img,
						                                          _.shuffleSPath_str,
																											null,
																											!0,
																											_.useHEXColorsForSkin_bl,
																											_.normalButtonsColor_str,
																											_.selectedButtonsColor_str),
				  p.shuffleButton_do.addListener(FWDMSPSimpleButton.MOUSE_UP,
						                             p.shuffleButtonOnMouseUpHandler),
					p.shuffleButton_do.setY(parseInt((p.stageHeight - p.shuffleButton_do.h) / 2)),
					p.buttons_ar.push(p.shuffleButton_do),
					p.mainHolder_do.addChild(p.shuffleButton_do),
					!p.loop_bl && p.shuffle_bl && p.setShuffleButtonState("selected")
				},
				this.shuffleButtonOnMouseUpHandler = function() {
					p.shuffleButton_do.isSelectedFinal_bl ? p.dispatchEvent(n.DISABLE_SHUFFLE) : p.dispatchEvent(n.ENABLE_SHUFFLE)
				},
				this.setShuffleButtonState = function(e) {
					p.shuffleButton_do && ("selected" == e ? p.shuffleButton_do.setSelected() : "unselected" == e && p.shuffleButton_do.setUnselected())
				},
				this.setupRepostButton = function() {
					FWDMSPSimpleButton.setPrototype(),
					p.repostButton_do = new FWDMSPSimpleButton(p.repostN_img,
						                                        _.repostSPath_str,
																										null,
																										!0,
																										_.useHEXColorsForSkin_bl,
																										_.normalButtonsColor_str,
																										_.selectedButtonsColor_str),
					p.repostButton_do.addListener(FWDMSPSimpleButton.MOUSE_UP,
						                           p.repostButtonOnMouseUpHandler),
																			 p.repostButton_do.setY(parseInt((p.stageHeight - p.repostButton_do.h) / 2)),
																			 p.buttons_ar.push(p.repostButton_do),
																			 p.mainHolder_do.addChild(p.repostButton_do)
				},
				this.repostButtonOnMouseUpHandler = function() {
					p.dispatchEvent(n.REPOST)
				},
				this.setupPopupButton = function() {
					FWDMSPSimpleButton.setPrototype(),
					p.popupButton_do = new FWDMSPSimpleButton(p.popupN_img,
						                                        _.popupSPath_str,
																										null,
																										!0,
																										_.useHEXColorsForSkin_bl,
																										_.normalButtonsColor_str,
																										_.selectedButtonsColor_str),
		      p.popupButton_do.addListener(FWDMSPSimpleButton.MOUSE_UP,
						                           p.popupButtonOnMouseUpHandler),
																			 p.popupButton_do.setY(parseInt((p.stageHeight - p.popupButton_do.h) / 2)),
																			 p.buttons_ar.push(p.popupButton_do),
																			 p.mainHolder_do.addChild(p.popupButton_do)
				},
				this.disableControllerWhileLoadingPlaylist = function() {
					p.prevButton_do.disable(),
					p.playPauseButton_do.disable(),
					p.nextButton_do.disable(),
					p.downloadButton_do && p.downloadButton_do.disable(),
					p.buyButton_do && p.buyButton_do.disable(),
					p.playlistButton_do && p.playlistButton_do.disable(!0),
					p.repostButton_do && p.repostButton_do.disable(),
					p.updateTime("...", "..."),
					p.setTitle("...")
				},
				this.enableControllerWhileLoadingPlaylist = function() {
					p.prevButton_do.enable(),
					p.playPauseButton_do.enable(),
					p.nextButton_do.enable(),
					p.downloadButton_do && p.downloadButton_do.enable(),
					p.buyButton_do && p.buyButton_do.enable(),
					p.playlistButton_do && p.playlistButton_do.enable(),
					p.repostButton_do && p.repostButton_do.enable()
				},
				p.updateHEXColors = function(e, t) {
					p.normalColor_str = e,
					p.selectedColor_str = t,
					FWDMSPUtils.changeCanvasHEXColor(p.mainScrubberDragLeft_img, p.mainScrubberDragLeft_canvas, e);
					try {
						FWDMSPUtils.changeCanvasHEXColor(p.volumeScrubberDragBottom_img, p.volumeScrubberDragBottom_canvas, e)
					} catch (e) {}
					var o = FWDMSPUtils.changeCanvasHEXColor(p.mainScrubberMiddleImage, p.mainSCrubberMiddleCanvas, e, !0);
					p.mainScrubberDragMiddle_do.getStyle().background = "url('" + o.src + "') repeat-x";
					try {
						FWDMSPUtils.changeCanvasHEXColor(p.volumeScrubberDragLeft_img, p.volumeScrubberDragLeft_canvas, e),
						p.volumeScrubberDragMiddle_do.getStyle().background = "url('" + o.src + "') repeat-x"
					} catch (e) {}
					if (p.playPauseButton_do.updateHEXColors(e, t),
					    p.volumeButton_do && p.volumeButton_do.updateHEXColors(e, t),
							p.playlistButton_do && p.playlistButton_do.updateHEXColors(e, t),
							p.downloadButton_do && p.downloadButton_do.updateHEXColors(e, t),
							p.infoButton_do && p.infoButton_do.updateHEXColors(e, t),
							p.categoriesButton_do && p.categoriesButton_do.updateHEXColors(e, t),
							p.nextButton_do && p.nextButton_do.updateHEXColors(e, t),
							p.repostButton_do && p.repostButton_do.updateHEXColors(e, t),
							p.prevButton_do && p.prevButton_do.updateHEXColors(e, t),
							f.fullScreenButton_do && f.fullScreenButton_do.updateHEXColors(e, t),
							p.loopButton_do && p.loopButton_do.updateHEXColors(e, t),
							p.shuffleButton_do && p.shuffleButton_do.updateHEXColors(e, t),
							p.buyButton_do && p.buyButton_do.updateHEXColors(e, t),
							p.popupButton_do && p.popupButton_do.updateHEXColors(e, t),
							p.playbackRateButton_do && p.playbackRateButton_do.updateHEXColors(e, t),
							p.currentTime_do && (p.currentTime_do.getStyle().color = e),
							p.totalTime_do && (p.totalTime_do.getStyle().color = e),
							p.ytbButtons_ar)
						for (var s = 0; s < p.totalYtbButtons; s++) {
							var i = p.ytbButtons_ar[s];
							i.normalColor_str = e,
							i.selectedColor_str = t,
							i.isSelected_bl ? i.isSelected_bl || i.setSelectedState() : i.setNormalState()
						}
				},
				this.init()
		};
		n.setPrototype = function() {
				n.prototype = new FWDMSPDisplayObject("div")
			},
		n.SHOW_ATOB = "showAtob",
		n.REPOST = "repostShare",
		n.SHOW_PLAYBACKRATE = "showPlaybackRate",
		n.PLAY_NEXT = "playNext",
		n.PLAY_PREV = "playPrev",
		n.PLAY = "play",
		n.PAUSE = "pause",
		n.POPUP = "popup",
		n.VOLUME_START_TO_SCRUB = "volumeStartToScrub",
		n.VOLUME_STOP_TO_SCRUB = "volumeStopToScrub",
		n.START_TO_SCRUB = "startToScrub",
		n.SCRUB = "scrub",
		n.SCRUB_PLAYLIST_ITEM = "scrubPlaylistItem",
		n.STOP_TO_SCRUB = "stopToScrub",
		n.CHANGE_VOLUME = "changeVolume",
		n.SHOW_CATEGORIES = "showCategories",
		n.SHOW_PLAYLIST = "showPlaylist",
		n.HIDE_PLAYLIST = "hidePlaylist",
		n.ENABLE_LOOP = "enableLoop",
		n.DISABLE_LOOP = "disableLoop",
		n.ENABLE_SHUFFLE = "enableShuffle",
		n.DISABLE_SHUFFLE = "disableShuffle",
		n.DOWNLOAD_MP3 = "downloadMp3",
		n.BUY = "buy",
		n.prototype = null,
		window.FWDMSPController = n
	}(),
	window.FWDMSPDisplayObject = function(e, t, o, s) {
		var i = this;
		i.listeners = {
				events_ar: []
			},
			i.type = e,
			this.children_ar = [],
			this.style,
			this.screen,
			this.transform,
			this.position = t || "absolute",
			this.overflow = o || "hidden",
			this.display = s || "inline-block",
			this.visible = !0,
			this.buttonMode,
			this.x = 0,
			this.y = 0,
			this.w = 0,
			this.h = 0, this.rect,
			this.alpha = 1,
			this.innerHTML = "",
			this.opacityType = "",
			this.isHtml5_bl = !1,
			this.hasTransform3d_bl = FWDMSPUtils.hasTransform3d,
			this.hasTransform2d_bl = FWDMSPUtils.hasTransform2d,
			(FWDMSPUtils.isIE || FWDMSPUtils.isIE11 && !FWDMSPUtils.isMobile) && (i.hasTransform3d_bl = !1,
			i.hasTransform2d_bl = !1),
			this.hasBeenSetSelectable_bl = !1,
			i.init = function() {
				i.setScreen()
			},
			i.getTransform = function() {
				for (var e, t = ["transform", "msTransform", "WebkitTransform", "MozTransform", "OTransform"]; e = t.shift();)
					if (void 0 !== i.screen.style[e]) return e;
				return !1
			},
			i.getOpacityType = function() {
				return void 0 !== i.screen.style.opacity ? "opacity" : "filter"
			},
			i.setScreen = function(e) {
				"img" == i.type && e ? i.screen = e : i.screen = document.createElement(i.type), i.setMainProperties()
			},
			i.setMainProperties = function() {
				i.transform = i.getTransform(),
				i.setPosition(i.position),
				i.setOverflow(i.overflow),
				i.opacityType = i.getOpacityType(),
				"opacity" == i.opacityType && (i.isHtml5_bl = !0),
				"filter" == i.opacityType && (i.screen.style.filter = "inherit"),
				i.screen.style.left = "0px",
				i.screen.style.top = "0px",
				i.screen.style.margin = "0px",
				i.screen.style.padding = "0px",
				i.screen.style.maxWidth = "none",
				i.screen.style.maxHeight = "none",
				i.screen.style.border = "none",
				i.screen.style.lineHeight = "1",
				i.screen.style.backgroundColor = "transparent",
				i.screen.style.MozImageRendering = "optimizeSpeed",
				i.screen.style.WebkitImageRendering = "optimizeSpeed",
				"img" == e && (i.setWidth(i.screen.width), i.setHeight(i.screen.height))
			},
			i.setSelectable = function(e) {
				e || (i.screen.style.userSelect = "none",
				      i.screen.style.MozUserSelect = "none",
							i.screen.style.webkitUserSelect = "none",
							i.screen.style.khtmlUserSelect = "none",
							i.screen.style.oUserSelect = "none",
							i.screen.style.msUserSelect = "none",
							i.screen.msUserSelect = "none",
							i.screen.ondragstart = function(e) {
					return !1
				},
				i.screen.onselectstart = function() {
					return !1
				},
				i.screen.ontouchstart = function() {
					return !1
				},
				i.screen.style.webkitTouchCallout = "none",
				i.hasBeenSetSelectable_bl = !0)
			},
			i.getScreen = function() {
				return i.screen
			},
			i.setVisible = function(e) {
				i.visible = e,
				1 == i.visible ? i.screen.style.visibility = "visible" : i.screen.style.visibility = "hidden"
			},
			i.getVisible = function() {
				return i.visible
			},
			i.setResizableSizeAfterParent = function() {
				i.screen.style.width = "100%",
				i.screen.style.height = "100%"
			},
			i.getStyle = function() {
				return i.screen.style
			},
			i.setOverflow = function(e) {
				i.overflow = e,
				i.screen.style.overflow = i.overflow
			},
			i.setPosition = function(e) {
				i.position = e,
				i.screen.style.position = i.position
			},
			i.setDisplay = function(e) {
				i.display = e,
				i.screen.style.display = i.display
			},
			i.setButtonMode = function(e) {
				i.buttonMode = e,
				1 == i.buttonMode ? i.screen.style.cursor = "pointer" : i.screen.style.cursor = "default"
			},
			i.setBkColor = function(e) {
				i.screen.style.backgroundColor = e
			},
			i.setInnerHTML = function(e) {
				i.innerHTML = e,
				i.screen.innerHTML = i.innerHTML
			},
			i.getInnerHTML = function() {
				return i.innerHTML
			},
			i.getRect = function() {
				return i.screen.getBoundingClientRect()
			},
			i.setAlpha = function(e) {
				i.alpha = e,
				"opacity" == i.opacityType ? i.screen.style.opacity = i.alpha
				                           : "filter" == i.opacityType && (i.screen.style.filter = "alpha(opacity=" + 100 * i.alpha + ")",
																	                                 i.screen.style.filter = "progid:DXImageTransform.Microsoft.Alpha(Opacity=" + Math.round(100 * i.alpha) + ")")
			},
			i.getAlpha = function() {
				return i.alpha
			},
			i.getRect = function() {
				return i.screen.getBoundingClientRect()
			},
			i.getGlobalX = function() {
				return i.getRect().left
			},
			i.getGlobalY = function() {
				return i.getRect().top
			},
			i.setX = function(e) {
				i.x = e,
				i.hasTransform3d_bl ? i.screen.style[i.transform] = "translate3d(" + i.x + "px," + i.y + "px,0)"
				                    : i.hasTransform2d_bl
														? i.screen.style[i.transform] = "translate(" + i.x + "px," + i.y + "px)"
														: i.screen.style.left = i.x + "px"
			},
			i.getX = function() {
				return i.x
			},
			i.setY = function(e) {
				i.y = e,
				i.hasTransform3d_bl ? i.screen.style[i.transform] = "translate3d(" + i.x + "px," + i.y + "px,0)"
				                    : i.hasTransform2d_bl
														? i.screen.style[i.transform] = "translate(" + i.x + "px," + i.y + "px)"
														: i.screen.style.top = i.y + "px"
			},
			i.getY = function() {
				return i.y
			},
			i.setWidth = function(e) {
				i.w = e,
				"img" == i.type && (i.screen.width = i.w),
				i.screen.style.width = i.w + "px"
			},
			i.getWidth = function() {
				return "div" == i.type || "input" == i.type ? 0 != i.screen.offsetWidth
				                                            ? i.screen.offsetWidth
																										: i.w
																										: "img" == i.type
																										? 0 != i.screen.offsetWidth
																										? i.screen.offsetWidth
																										: 0
																										!= i.screen.width
																										? i.screen.width
																										: i._w
																										: "canvas" == i.type
																										? 0 != i.screen.offsetWidth
																										? i.screen.offsetWidth
																										: i.w
																										: void 0
			},
			i.setHeight = function(e) {
				i.h = e,
				"img" == i.type && (i.screen.height = i.h),
				i.screen.style.height = i.h + "px"
			},
			i.getHeight = function() {
				return "div" == i.type || "input" == i.type ? 0 != i.screen.offsetHeight
				                                            ? i.screen.offsetHeight
																										: i.h
																										: "img" == i.type
																										? 0 != i.screen.offsetHeight
																										? i.screen.offsetHeight
																										: 0 != i.screen.height
																										? i.screen.height
																										: i.h
																										: "canvas" == i.type
																										? 0 != i.screen.offsetHeight
																										? i.screen.offsetHeight
																										: i.h
																										: void 0
			},
			i.addChild = function(e) {
				i.contains(e) && i.children_ar.splice(FWDMSPUtils.indexOfArray(i.children_ar, e), 1),
				i.children_ar.push(e),
				i.screen.appendChild(e.screen)
			},
			i.removeChild = function(e) {
				if (!i.contains(e)) throw Error("##removeChild()## Child dose't exist, it can't be removed!");
				i.children_ar.splice(FWDMSPUtils.indexOfArray(i.children_ar, e), 1), i.screen.removeChild(e.screen)
			},
			i.contains = function(e) {
				return -1 != FWDMSPUtils.indexOfArray(i.children_ar, e)
			},
			i.addChildAt = function(e, t) {
				if (0 == i.getNumChildren()) i.children_ar.push(e), i.screen.appendChild(e.screen);
				else if (1 == t) i.screen.insertBefore(e.screen, i.children_ar[0].screen), i.screen.insertBefore(i.children_ar[0].screen, e.screen),
					i.contains(e) ? i.children_ar.splice(FWDMSPUtils.indexOfArray(i.children_ar, e), 1, e) : i.children_ar.splice(FWDMSPUtils.indexOfArray(i.children_ar, e), 0, e);
				else {
					if (t < 0 || t > i.getNumChildren() - 1) throw Error("##getChildAt()## Index out of bounds!");
					i.screen.insertBefore(e.screen, i.children_ar[t].screen), i.contains(e) ? i.children_ar.splice(FWDMSPUtils.indexOfArray(i.children_ar, e), 1, e) : i.children_ar.splice(FWDMSPUtils.indexOfArray(i.children_ar, e), 0, e)
				}
			},
			i.getChildAt = function(e) {
				if (e < 0 || e > i.getNumChildren() - 1) throw Error("##getChildAt()## Index out of bounds!");
				if (0 == i.getNumChildren()) throw Errror("##getChildAt## Child dose not exist!");
				return i.children_ar[e]
			},
			i.removeChildAtZero = function() {
				i.screen.removeChild(i.children_ar[0].screen), i.children_ar.shift()
			},
			i.getNumChildren = function() {
				return i.children_ar.length
			},
			i.addListener = function(e, t) {
				if (null == e) throw Error("type is required.");
				if ("object" == typeof e) throw Error("type must be of type String.");
				if ("function" != typeof t) throw Error("listener must be of type Function.");
				var o = {};
				o.type = e, o.listener = t, (o.target = this).listeners.events_ar.push(o)
			},
			i.dispatchEvent = function(e, t) {
				if (null != this.listeners) {
					if (null == e) throw Error("type is required.");
					if ("object" == typeof e) throw Error("type must be of type String.");
					for (var o = 0, s = this.listeners.events_ar.length; o < s; o++)
						if (this.listeners.events_ar[o].target === this && this.listeners.events_ar[o].type === e) {
							if (t)
								for (var i in t) this.listeners.events_ar[o][i] = t[i];
							this.listeners.events_ar[o].listener.call(this, this.listeners.events_ar[o])
						}
				}
			},
			i.removeListener = function(e, t) {
				if (null == e) throw Error("type is required.");
				if ("object" == typeof e) throw Error("type must be of type String.");
				if ("function" != typeof t) throw Error("listener must be of type Function." + e);
				for (var o = 0, s = this.listeners.events_ar.length; o < s; o++)
					if (this.listeners.events_ar[o].target === this && this.listeners.events_ar[o].type === e && this.listeners.events_ar[o].listener === t) {
						this.listeners.events_ar.splice(o, 1);
						break
					}
			},
			i.disposeImage = function() {
				"img" == i.type && (i.screen.src = null)
			},
			i.destroy = function() {
				i.hasBeenSetSelectable_bl && (i.screen.ondragstart = null,
					                            i.screen.onselectstart = null,
																			i.screen.ontouchstart = null),
        i.screen.removeAttribute("style"),
				i.listeners = [],
				i.listeners = null,
				i.children_ar = [],
				i.children_ar = null,
				i.style = null,
				i.screen = null,
				i.transform = null,
				i.position = null,
				i.overflow = null,
				i.display = null,
				i.visible = null,
				i.buttonMode = null,
				i.x = null,
				i.y = null,
				i.w = null,
				i.h = null,
				i.rect = null,
				i.alpha = null,
				i.innerHTML = null,
				i.opacityType = null,
				i.isHtml5_bl = null,
				i.hasTransform3d_bl = null,
				i.hasTransform2d_bl = null,
				i = null
			},
			i.init()
	},
	window, window.FWDMSPEventDispatcher = function() {
		this.listeners = {
			events_ar: []
		},
		this.addListener = function(e, t) {
			if (null == e) throw Error("type is required.");
			if ("object" == typeof e) throw Error("type must be of type String.");
			if ("function" != typeof t) throw Error("listener must be of type Function.");
			var o = {};
			o.type = e, o.listener = t, (o.target = this).listeners.events_ar.push(o)
		},
		this.dispatchEvent = function(e, t) {
			if (null != this.listeners) {
				if (null == e) throw Error("type is required.");
				if ("object" == typeof e) throw Error("type must be of type String.");
				for (var o = 0, s = this.listeners.events_ar.length; o < s; o++)
					if (this.listeners.events_ar[o].target === this && this.listeners.events_ar[o].type === e) {
						if (t)
							for (var i in t) this.listeners.events_ar[o][i] = t[i];
						this.listeners.events_ar[o].listener.call(this, this.listeners.events_ar[o])
					}
			}
		},
		this.removeListener = function(e, t) {
			if (null == e) throw Error("type is required.");
			if ("object" == typeof e) throw Error("type must be of type String.");
			if ("function" != typeof t) throw Error("listener must be of type Function." + e);
			for (var o = 0, s = this.listeners.events_ar.length; o < s; o++)
				if (this.listeners.events_ar[o].target === this && this.listeners.events_ar[o].type === e && this.listeners.events_ar[o].listener === t) {
					this.listeners.events_ar.splice(o, 1);
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
	function(n) {
		var l = function(e, t, o) {
			var s = this,
				i = l.prototype;
			this.screenToTest = e,
			this.screenToTest2 = t,
			this.hideDelay = o,
			this.globalX = 0,
			this.globalY = 0,
			this.currentTime,
			this.checkIntervalId_int,
			this.hideCompleteId_to,
			this.hasInitialTestEvents_bl = !1,
			this.addSecondTestEvents_bl = !1,
			this.dispatchOnceShow_bl = !0,
			this.dispatchOnceHide_bl = !1,
			this.isStopped_bl = !0,
			this.isMobile_bl = FWDMSPUtils.isMobile,
			this.hasPointerEvent_bl = FWDMSPUtils.hasPointerEvent,
			s.init = function() {},
			s.start = function() {
			s.currentTime = (new Date).getTime(),
			clearInterval(s.checkIntervalId_int),
			s.checkIntervalId_int = setInterval(s.update, 100),
			s.addMouseOrTouchCheck(),
			s.isStopped_bl = !1
		},
				s.stop = function() {
					clearInterval(s.checkIntervalId_int),
					s.isStopped_bl = !0,
					s.removeMouseOrTouchCheck(),
					s.removeMouseOrTouchCheck2()
				},
				s.addMouseOrTouchCheck = function() {
					s.hasInitialTestEvents_bl || (s.hasInitialTestEvents_bl = !0,
						                            s.isMobile_bl ? s.hasPointerEvent_bl
																				              ? (s.screenToTest.screen.addEventListener("pointerdown", s.onMouseOrTouchUpdate),
																				                s.screenToTest.screen.addEventListener("MSPointerMove", s.onMouseOrTouchUpdate))
																											: s.screenToTest.screen.addEventListener("touchstart", s.onMouseOrTouchUpdate)
																											: n.addEventListener ? n.addEventListener("mousemove", s.onMouseOrTouchUpdate)
																											                     : document.attachEvent && document.attachEvent("onmousemove", s.onMouseOrTouchUpdate))
				},
				s.removeMouseOrTouchCheck = function() {
					s.hasInitialTestEvents_bl && (s.hasInitialTestEvents_bl = !1,
						                            s.isMobile_bl ? s.hasPointerEvent_bl
																				              ? (s.screenToTest.screen.removeEventListener("pointerdown", s.onMouseOrTouchUpdate),
																											   s.screenToTest.screen.removeEventListener("MSPointerMove",
																												 s.onMouseOrTouchUpdate))
																											: s.screenToTest.screen.removeEventListener("touchstart", s.onMouseOrTouchUpdate)
																											: n.removeEventListener ? n.removeEventListener("mousemove", s.onMouseOrTouchUpdate)
																											: document.detachEvent && document.detachEvent("onmousemove", s.onMouseOrTouchUpdate))
				},
				s.addMouseOrTouchCheck2 = function() {
					s.addSecondTestEvents_bl || (s.addSecondTestEvents_bl = !0,
						                           s.screenToTest.screen.addEventListener ? s.screenToTest.screen.addEventListener("mousemove", s.secondTestMoveDummy)
																			                                        : s.screenToTest.screen.attachEvent && s.screenToTest.screen.attachEvent("onmousemove", s.secondTestMoveDummy))
				},
				s.removeMouseOrTouchCheck2 = function() {
					s.addSecondTestEvents_bl && (s.addSecondTestEvents_bl = !1,
						                           s.screenToTest.screen.removeEventListener
																			 ? s.screenToTest.screen.removeEventListener("mousemove", s.secondTestMoveDummy)
																			 : s.screenToTest.screen.detachEvent && s.screenToTest.screen.detachEvent("onmousemove", s.secondTestMoveDummy))
				},
				this.secondTestMoveDummy = function() {
					s.removeMouseOrTouchCheck2(),
					s.addMouseOrTouchCheck()
				},
				s.onMouseOrTouchUpdate = function(e) {
					var t = FWDMSPUtils.getViewportMouseCoordinates(e);
					s.globalX != t.screenX && s.globalY != t.screenY && (s.currentTime = (new Date).getTime()),
					s.globalX = t.screenX,
					s.globalY = t.screenY,
					s.isMobile_bl || FWDMSPUtils.hitTest(s.screenToTest.screen, s.globalX, s.globalY)
					              || (s.removeMouseOrTouchCheck(), s.addMouseOrTouchCheck2())
				},
				s.update = function(e) {
					(new Date).getTime() > s.currentTime + s.hideDelay ? s.dispatchOnceShow_bl && (s.dispatchOnceHide_bl = !0, s.dispatchOnceShow_bl = !1, s.dispatchEvent(l.HIDE), clearTimeout(s.hideCompleteId_to), s.hideCompleteId_to = setTimeout(function() {
						s.dispatchEvent(l.HIDE_COMPLETE)
					}, 1e3)) : s.dispatchOnceHide_bl && (clearTimeout(s.hideCompleteId_to), s.dispatchOnceHide_bl = !1, s.dispatchOnceShow_bl = !0, s.dispatchEvent(l.SHOW))
				},
				s.reset = function() {
					clearTimeout(s.hideCompleteId_to),
					s.currentTime = (new Date).getTime(),
					s.dispatchEvent(l.SHOW)
				},
				s.destroy = function() {
					s.removeMouseOrTouchCheck(),
					clearInterval(s.checkIntervalId_int),
					s.screenToTest = null,
					e = null,
					s.init = null,
					s.start = null,
					s.stop = null,
					s.addMouseOrTouchCheck = null,
					s.removeMouseOrTouchCheck = null,
					s.onMouseOrTouchUpdate = null,
					s.update = null,
					s.reset = null,
					s.destroy = null,
					i.destroy(),
					s = i = null,
					l.prototype = null
				},
				s.init()
		};
		l.HIDE = "hide",
		l.SHOW = "show",
		l.HIDE_COMPLETE = "hideComplete",
		l.setPrototype = function() {
			l.prototype = new FWDMSPEventDispatcher
		},
		n.FWDMSPHider = l
	}(window),
	function(e) {
		var t = function(i, e) {
			var n = this;
			t.prototype;
			this.bk_do = null,
			this.textHolder_do = null,
			this.warningIconPath_str = e,
			this.show_to = null,
			this.isShowed_bl = !1,
			this.isShowedOnce_bl = !1,
			this.allowToRemove_bl = !0,
			this.init = function() {
			n.setResizableSizeAfterParent(),
			n.bk_do = new FWDMSPDisplayObject("div"),
			n.bk_do.setAlpha(.6), n.bk_do.setBkColor("#000000"),
			n.addChild(n.bk_do),
			n.textHolder_do = new FWDMSPDisplayObject("div"),
			FWDMSPUtils.isIEAndLessThen9 || (n.textHolder_do.getStyle().font = "Arial"),
			n.textHolder_do.getStyle().wordWrap = "break-word",
			n.textHolder_do.getStyle().padding = "10px",
			n.textHolder_do.getStyle().paddingLeft = "42px",
			n.textHolder_do.getStyle().lineHeight = "18px",
			n.textHolder_do.getStyle().color = "#000000",
			n.textHolder_do.setBkColor("#EEEEEE");
			var e = new Image;
			e.src = this.warningIconPath_str,
			this.img_do = new FWDMSPDisplayObject("img"),
			this.img_do.setScreen(e),
			this.img_do.setWidth(28),
			this.img_do.setHeight(28),
			n.addChild(n.textHolder_do),
			n.addChild(n.img_do)
				},
				this.showText = function(e) {
					n.isShowedOnce_bl || (n.screen.addEventListener ? n.screen.addEventListener("click", n.closeWindow)
					                                                : n.screen.attachEvent && n.screen.attachEvent("onclick", n.closeWindow),
					n.isShowedOnce_bl = !0),
					n.setVisible(!1),
					n.textHolder_do.getStyle().paddingBottom = "10px",
					n.textHolder_do.setInnerHTML(e),
					clearTimeout(n.show_to),
					n.show_to = setTimeout(n.show, 60),
					setTimeout(function() {
						n.positionAndResize()
					}, 10)
				},
				this.show = function() {
					var e = Math.min(640, i.stageWidth - 120);
					n.isShowed_bl = !0, n.textHolder_do.setWidth(e), setTimeout(function() {
						n.setVisible(!0),
						n.positionAndResize()
					}, 100)
				},
				this.positionAndResize = function() {
					var e = n.textHolder_do.getWidth(),
						t = n.textHolder_do.getHeight(),
						o = parseInt((i.stageWidth - e) / 2),
						s = 0;
					i.playlist_do && i.playlist_do.isShowed_bl ? s = parseInt((Math.max(i.main_do.h, i.maxHeight) - t) / 2) : i.controller_do && (s = parseInt((Math.max(i.controller_do.h, i.maxHeight) - t) / 2)),
					n.bk_do.setWidth(i.stageWidth),
					n.bk_do.setHeight(Math.max(i.main_do.h, i.maxHeight)),
					n.textHolder_do.setX(o),
					n.textHolder_do.setY(s),
					n.img_do.setX(o + 6),
					n.img_do.setY(s + parseInt((n.textHolder_do.getHeight() - n.img_do.h) / 2))
				},
				this.closeWindow = function() {
					if (n.allowToRemove_bl) {
						n.isShowed_bl = !1, clearTimeout(n.show_to);
						try {
							i.main_do.removeChild(n)
						} catch (e) {}
					}
				}, this.init()
		};
		t.setPrototype = function() {
			t.prototype = new FWDMSPDisplayObject("div", "relative")
		}, t.prototype = null, e.FWDMSPInfo = t
	}(window),
	function() {
		var i = function(e, t, o) {
			var s = this;
			this.animation_img = e.openerAnimation_img,
			t == FWDMSP.POSITION_TOP ? (this.openN_img = e.openTopN_img, this.openSPath_str = e.openTopSPath_str)
			                         : (this.openN_img = e.openBottomN_img, this.openSPath_str = e.openBottomSPath_str),
			this.openerPauseN_img = e.openerPauseN_img,
			this.openerPlayN_img = e.openerPlayN_img,
			this.closeN_img = e.closeN_img,
			s.useHEXColorsForSkin_bl = e.useHEXColorsForSkin_bl,
			s.normalButtonsColor_str = e.normalButtonsColor_str,
			s.selectedButtonsColor_str = e.selectedButtonsColor_str,
			this.openerPauseS_str = e.openerPauseS_str,
			this.openerPlaySPath_str = e.openerPlayS_str,
			this.closeSPath_str = e.closeSPath_str,
			this.animationPath_str = e.animationPath_str,
			this.totalWidth = s.openN_img.width,
			this.totalHeight = s.openN_img.height,
			this.mainHolder_do = null,
			this.dumy_do = null,
			this.openN_do = null,
			this.openS_do = null,
			this.closeN_do = null,
			this.closeS_do = null,
			this.animation_do = null,
			this.playPauseButton_do = null,
			this.position_str = t,
			this.alignment_str = e.openerAlignment_str,
			this.openerEqulizerOffsetLeft = e.openerEqulizerOffsetLeft,
			this.openerEqulizerOffsetTop = e.openerEqulizerOffsetTop,
			this.showFirstTime_bl = !0,
			this.playerIsShowed_bl = o,
			this.showOpenerPlayPauseButton_bl = e.showOpenerPlayPauseButton_bl,
			this.isMobile_bl = FWDMSPUtils.isMobile,
			this.hasPointerEvent_bl = FWDMSPUtils.hasPointerEvent,
			this.init = function() {
					-1 != e.skinPath_str.indexOf("hex_white") ? s.selectedButtonsColor_str = "#FFFFFF" : s.selectedButtonsColor_str = e.selectedButtonsColor_str,
					s.hasTransform3d_bl = !1,
					s.hasTransform2d_bl = !1,
					s.getStyle().msTouchAction = "none",
					s.getStyle().webkitTapHighlightColor = "rgba(0, 0, 0, 0)",
					s.setupStuff(),
					s.showOpenerPlayPauseButton_bl && s.setupPlayPauseButton(),
					s.playerIsShowed_bl && s.showCloseButton(),
					s.hide(),
					s.showOpenerPlayPauseButton_bl ? s.setWidth(s.totalWidth + s.openerPauseN_img.width + 1) : s.setWidth(s.totalWidth),
					s.setHeight(s.totalHeight)
				},
				this.setupStuff = function(e) {
					s.mainHolder_do = new FWDMSPDisplayObject("div"),
					s.mainHolder_do.hasTransform3d_bl = !1,
					s.mainHolder_do.hasTransform2d_bl = !1,
					s.showOpenerPlayPauseButton_bl ? s.mainHolder_do.setWidth(s.totalWidth + s.openerPauseN_img.width + 1) : s.mainHolder_do.setWidth(s.totalWidth),
					s.mainHolder_do.setHeight(s.totalHeight),
					s.useHEXColorsForSkin_bl ? (s.openN_do = new FWDMSPDisplayObject("div"),
					                            s.openN_canvas = FWDMSPUtils.getCanvasWithModifiedColor(s.openN_img, s.normalButtonsColor_str).canvas,
																			s.openN_do.screen.appendChild(s.openN_canvas))
																	 : (s.openN_do = new FWDMSPDisplayObject("img"),
																	    s.openN_do.setScreen(s.openN_img)),
					s.openN_do.setWidth(s.openN_img.width),
					s.openN_do.setHeight(s.openN_img.height),
					s.openS_img = new Image,
					s.openS_img.src = s.openSPath_str,
					s.useHEXColorsForSkin_bl ? (s.openS_do = new FWDMSPDisplayObject("div"),
					                            s.openS_img.onload = function() {
																																				s.openS_canvas = FWDMSPUtils.getCanvasWithModifiedColor(s.openS_img, s.selectedButtonsColor_str).canvas,
																																				s.openS_do.setWidth(s.openS_img.width),
																																				s.openS_do.setHeight(s.openS_img.height),
																																				s.openS_do.screen.appendChild(s.openS_canvas)
																																			})
																   : (s.openS_do = new FWDMSPDisplayObject("img"),
																			s.openS_do.setScreen(s.openS_img)),
						s.openS_do.setWidth(s.openN_do.w),
						s.openS_do.setHeight(s.openN_do.h),
						s.openS_do.setAlpha(0),
						s.useHEXColorsForSkin_bl ? (s.closeN_do = new FWDMSPDisplayObject("div"),
						                            s.closeN_canvas = FWDMSPUtils.getCanvasWithModifiedColor(s.closeN_img, s.normalButtonsColor_str).canvas,
																				s.closeN_do.screen.appendChild(s.closeN_canvas))
																		 : (s.closeN_do = new FWDMSPDisplayObject("img"),
																		    s.closeN_do.setScreen(s.closeN_img)),
						s.closeN_do.setWidth(s.closeN_img.width),
						s.closeN_do.setHeight(s.closeN_img.height),
						s.closeN_do.hasTransform3d_bl = !1,
						s.closeN_do.hasTransform2d_bl = !1,
						s.closeS_img = new Image,
						s.closeS_img.src = s.closeSPath_str,
						s.useHEXColorsForSkin_bl ? (s.closeS_do = new FWDMSPDisplayObject("div"),
						                            s.closeS_img.onload = function() {
																																					s.closeS_canvas = FWDMSPUtils.getCanvasWithModifiedColor(s.closeS_img, s.selectedButtonsColor_str).canvas,
																																					s.closeS_do.setWidth(s.closeS_img.width),
																																					s.closeS_do.setHeight(s.closeS_img.height),
																																					s.closeS_do.screen.appendChild(s.closeS_canvas)
																																				 })
																			: (s.closeS_do = new FWDMSPDisplayObject("img"),
																			   s.closeS_do.setScreen(s.closeS_img)),
						s.closeS_do.setWidth(s.closeS_img.width),
						s.closeS_do.setHeight(s.closeS_img.height),
						s.closeS_do.setAlpha(0),
						s.closeS_do.hasTransform3d_bl = !1,
						s.closeS_do.hasTransform2d_bl = !1,
						FWDMSPPreloader.setPrototype(),
						s.animation_do = new FWDMSPPreloader(s.animationPath_str, 29, 22, 31, 80, !0),
						s.animation_do.setY(s.openerEqulizerOffsetTop),
						s.animation_do.show(!1),
						s.animation_do.stop(),
						s.dumy_do = new FWDMSPDisplayObject("div"),
						s.dumy_do.setWidth(s.totalWidth),
						s.dumy_do.setHeight(s.totalHeight),
						s.dumy_do.getStyle().zIndex = 2,
						s.dumy_do.hasTransform3d_bl = !1,
						s.dumy_do.hasTransform2d_bl = !1,
						s.dumy_do.setButtonMode(!0),
						(FWDMSPUtils.isIE || FWDMSPUtils.isAndroid) && (s.dumy_do.setBkColor("#FF0000"), s.dumy_do.setAlpha(.01)),
						s.isMobile_bl ? s.hasPointerEvent_bl
						              ? (s.dumy_do.screen.addEventListener("pointerdown", s.onMouseUp),
						                 s.dumy_do.screen.addEventListener("pointerover", s.onMouseOver),
														 s.dumy_do.screen.addEventListener("pointerout", s.onMouseOut))
													:  s.dumy_do.screen.addEventListener("touchstart", s.onMouseUp)
													:  s.dumy_do.screen.addEventListener ? (s.dumy_do.screen.addEventListener("mouseover", s.onMouseOver),
													                                        s.dumy_do.screen.addEventListener("mouseout", s.onMouseOut),
																																	s.dumy_do.screen.addEventListener("mousedown", s.onMouseUp))
																															 : s.dumy_do.screen.attachEvent && (s.dumy_do.screen.attachEvent("onmouseover", s.onMouseOver),
																															                                    s.dumy_do.screen.attachEvent("onmouseout", s.onMouseOut),
																																																	s.dumy_do.screen.attachEvent("onmousedown", s.onMouseUp)),
						s.mainHolder_do.addChild(s.openN_do),
						s.mainHolder_do.addChild(s.openS_do),
						s.mainHolder_do.addChild(s.closeN_do),
						s.mainHolder_do.addChild(s.closeS_do),
						s.mainHolder_do.addChild(s.animation_do),
						s.mainHolder_do.addChild(s.dumy_do),
						s.addChild(s.mainHolder_do)
				},
				this.onMouseOver = function(e, t) {
					e.pointerType && e.pointerType != e.MSPOINTER_TYPE_MOUSE || s.setSelectedState(!0)
				},
				this.onMouseOut = function(e) {
					e.pointerType && e.pointerType != e.MSPOINTER_TYPE_MOUSE || s.setNormalState()
				},
				this.onMouseUp = function(e) {
					e.preventDefault && e.preventDefault(),
					s.playerIsShowed_bl ? (s.playerIsShowed_bl = !1, s.dispatchEvent(i.HIDE)) : (s.playerIsShowed_bl = !0, s.dispatchEvent(i.SHOW))
				},
				this.setupPlayPauseButton = function() {
					FWDMSPComplexButton.setPrototype(),
					s.playPauseButton_do = new FWDMSPComplexButton(s.openerPlayN_img,
						                                             s.openerPlaySPath_str,
																												 s.openerPauseN_img,
																												 s.openerPauseS_str,
																												 !0,
																												 s.useHEXColorsForSkin_bl,
																												 s.normalButtonsColor_str,
																												 s.selectedButtonsColor_str),
					s.playPauseButton_do.addListener(FWDMSPComplexButton.MOUSE_UP,
						                               s.playButtonMouseUpHandler),
		      s.addChild(s.playPauseButton_do)
				},
				this.showPlayButton = function() {
					s.playPauseButton_do && s.playPauseButton_do.setButtonState(1),
					s.animation_do.stop()
				},
				this.showPauseButton = function() {
					s.playPauseButton_do && s.playPauseButton_do.setButtonState(0),
					s.animation_do.start(0)
				},
				this.playButtonMouseUpHandler = function() {
					0 == s.playPauseButton_do.currentState ? s.dispatchEvent(FWDMSPController.PAUSE) : s.dispatchEvent(FWDMSPController.PLAY)
				},
				this.setNormalState = function() {
					s.isMobile_bl && !s.hasPointerEvent_bl || (FWDAnimation.killTweensOf(s.openS_do),
					                                           FWDAnimation.killTweensOf(s.closeS_do),
																										 FWDAnimation.to(s.openS_do, .5, {
																											 	alpha: 0,
																											 	ease: Expo.easeOut
																										 	}), FWDAnimation.to(s.closeS_do, .5, {
																												alpha: 0,
																												ease: Expo.easeOut
																											}))
				}, this.setSelectedState = function(e) {
					FWDAnimation.killTweensOf(s.openS_do),
						FWDAnimation.killTweensOf(s.closeS_do),
						FWDAnimation.to(s.openS_do, .5, {
							alpha: 1,
							ease: Expo.easeOut
						}), FWDAnimation.to(s.closeS_do, .5, {
							alpha: 1,
							ease: Expo.easeOut
						})
				}, this.showOpenButton = function() {
					s.playerIsShowed_bl = !1,
					s.closeN_do.setX(150),
					s.closeS_do.setX(150),
					s.playPauseButton_do ? "right" == s.alignment_str ? (s.playPauseButton_do.setX(0),
					                                                     s.openN_do.setX(s.playPauseButton_do.w + 1),
																															 s.openS_do.setX(s.playPauseButton_do.w + 1),
																															 s.dumy_do.setX(s.playPauseButton_do.w + 1),
																															 s.dumy_do.setWidth(s.totalWidth),
																															 s.animation_do.setX(s.playPauseButton_do.w + 1 + s.openerEqulizerOffsetLeft))
																														: (s.playPauseButton_do.setX(s.openN_do.w + 1),
																														   s.openN_do.setX(0),
																															 s.openS_do.setX(0),
																															 s.dumy_do.setX(0),
																															 s.dumy_do.setWidth(s.totalWidth),
																															 s.animation_do.setX(s.openerEqulizerOffsetLeft))
																														: (s.openN_do.setX(0),
																														   s.openS_do.setX(0),
																															 s.dumy_do.setX(0),
																															 s.dumy_do.setWidth(s.totalWidth),
																															 s.animation_do.setX(s.openerEqulizerOffsetLeft))
					s.animation_do.setVisible(!0)
				}, this.showCloseButton = function() {
					s.playerIsShowed_bl = !0, s.openN_do.setX(150), s.openS_do.setX(150), s.dumy_do.setWidth(s.closeN_do.w), "right" == s.alignment_str ? s.playPauseButton_do ? (s.closeN_do.setX(s.totalWidth + 1), s.closeS_do.setX(s.totalWidth + 1), s.dumy_do.setX(s.totalWidth + 1)) : (s.closeN_do.setX(s.totalWidth - s.closeN_do.w), s.closeS_do.setX(s.totalWidth - s.closeN_do.w), s.dumy_do.setX(s.totalWidth - s.closeN_do.w)) : (s.closeN_do.setX(0), s.closeS_do.setX(0), s.dumy_do.setX(0)), s.playPauseButton_do && s.playPauseButton_do.setX(150), s.animation_do.setX(150), s.animation_do.setVisible(!1)
				}, this.hide = function() {
					s.mainHolder_do.setX(150)
				}, this.show = function() {
					s.mainHolder_do.setX(0)
				}, s.updateHEXColors = function(e, t) {
					s.normalColor_str = e,
					s.selectedColor_str = t,
					s.playPauseButton_do.updateHEXColors(e, t),
					FWDMSPUtils.changeCanvasHEXColor(s.openN_img, s.openN_canvas, e),
					FWDMSPUtils.changeCanvasHEXColor(s.closeN_img, s.closeN_canvas, e),
					FWDMSPUtils.changeCanvasHEXColor(s.openS_img, s.openS_canvas, t),
					FWDMSPUtils.changeCanvasHEXColor(s.closeS_img, s.closeS_canvas, t)
				},
				this.init()
		};
		i.setPrototype = function() {
				i.prototype = new FWDMSPDisplayObject("div")
			}, i.SHOW = "show", i.HIDE = "hise", i.prototype = null,
			window.FWDMSPOpener = i
	}(window),
	function(e) {
		var s = function(e, t) {
			var o = this;
			s.prototype;
			this.xhr = null,
      this.bk_do = null,
			this.mainHolder_do = null,
      this.closeButton_do = null,
      this.secondaryLabelsColor_str = e.secondaryLabelsColor_str,
			this.inputColor_str = e.inputColor_str,
      this.mainLabelsColor_str = e.mainLabelsColor_str,
      this.inputBackgroundColor_str = e.inputBackgroundColor_str,
      this.borderColor_str = e.borderColor_str,
      this.maxTextWidth = 0,
      this.totalWidth = 0,
      this.stageWidth = 0,
      this.stageHeight = 0,
      this.buttonWidth = 28,
      this.buttonHeight = 19,
      this.embedWindowCloseButtonMargins = 0,
      this.finalEmbedPath_str = null,
      this.isShowed_bl = !1,
			this.isMobile_bl = FWDMSPUtils.isMobile,
      this.init = function() {
          o.mainHolder_do = new FWDMSPDisplayObject("div"),
          o.mainHolder_do.hasTransform3d_bl = !1,
          o.mainHolder_do.hasTransform2d_bl = !1,
          o.bk_do = new FWDMSPDisplayObject("div"),
          o.bk_do.getStyle().width = "100%",
          o.bk_do.getStyle().height = "100%",
          o.bk_do.setAlpha(.9),
          o.bk_do.getStyle().background = "url('" + o.backgrondPath_str + "')",
					FWDMSPSimpleSizeButton.setPrototype(),
          FWDMSPSimpleButton.setPrototype(),
          o.closeButton_do = new FWDMSPSimpleButton(
            e.embedWindowClosePathS_str,
            void 0,
            !0,
            e.useHEXColorsForSkin_bl,
            e.normalButtonsColor_str,
            e.selectedButtonsColor_str
          ),
          o.closeButton_do.addListener(FWDMSPSimpleButton.MOUSE_UP, o.closeButtonOnMouseUpHandler),
          o.addChild(o.mainHolder_do),
          o.mainHolder_do.addChild(o.bk_do),
          o.mainHolder_do.addChild(o.closeButton_do)
				}, this.closeButtonOnMouseUpHandler = function() {
					o.isShowed_bl && o.hide()
				}, this.positionAndResize = function() {
					o.stageWidth = t.stageWidth,
          o.stageHeight = t.stageHeight,
          o.maxTextWidth = Math.min(o.stageWidth - 150, 300),
          o.totalWidth = o.maxTextWidth + o.buttonWidth,
          o.positionFinal(),
          o.closeButton_do.setX(o.stageWidth - o.closeButton_do.w - o.embedWindowCloseButtonMargins),
          o.closeButton_do.setY(o.embedWindowCloseButtonMargins),
					finalY = t.playlist_do && t.position_str == FWDMSP.POSITION_TOP ? t.playlist_do.h : o.embedWindowCloseButtonMargins,
          o.setY(finalY),
					o.setWidth(o.stageWidth),
          o.setHeight(o.stageHeight),
          o.mainHolder_do.setWidth(o.stageWidth),
          o.mainHolder_do.setHeight(o.stageHeight)
				}, this.updateHEXColors = function(e, t) {
          o.closeButton_do.updateHEXColors(e, t)
				}, this.showInfo = function(e, t) {
					o.infoText_do.setInnerHTML(e),
          o.infoText_do.setWidth(o.buttonWidth),
          o.infoText_do.setHeight(o.buttonHeight - 4),
					o.infoText_do.setAlpha(0),
          o.infoText_do.getStyle().color = t ? "#FF0000" : o.mainLabelsColor_str,
          FWDAnimation.killTweensOf(o.infoText_do),
          FWDAnimation.to(o.infoText_do, .16, {
							alpha: 1,
							yoyo: !0,
							repeat: 7
						})
				}, this.show = function(e) {
					o.isShowed_bl || (o.isShowed_bl = !0,
          t.main_do.addChild(o),
          o.positionAndResize(),
          (!FWDMSPUtils.isMobile || FWDMSPUtils.isMobile && FWDMSPUtils.hasPointerEvent) && t.main_do.setSelectable(!0), clearTimeout(o.hideCompleteId_to),
          clearTimeout(o.showCompleteId_to),
          o.mainHolder_do.setY(-o.stageHeight),
          o.showCompleteId_to = setTimeout(o.showCompleteHandler, 900),
          setTimeout(function() {
						FWDAnimation.to(o.mainHolder_do, .8, {
							y: 0,
							delay: .1,
							ease: Expo.easeInOut
						})
					}, 100))
				},
        this.showCompleteHandler = function() {},
        this.hide = function() {},
        this.hideCompleteHandler = function() {
					t.main_do.removeChild(o), o.dispatchEvent(s.HIDE_COMPLETE)
				},
        this.init()
		};
		s.setPrototype = function() {
			s.prototype = new FWDMSPDisplayObject("div")
		}, s.ERROR = "error", s.CORRECT = "correct", s.HIDE_COMPLETE = "hideComplete", s.prototype = null
	}(window),
	function(n) {
		var l = function(s, o) {
			var i = this;
			l.prototype;
			this.embedColoseN_img = s.embedColoseN_img,
      this.bk_do = null,
      this.mainHolder_do = null,
      this.closeButton_do = null,
      this.buttons_ar = [],
			this.embedWindowCloseButtonMargins = 0,
      this.scrubbersHeight = s.mainScrubberBkLeft_img.height,
      this.scrubberBkMiddlePath_str = s.mainScrubberBkMiddlePath_str,
      this.scrubbersBkLeftAndRightWidth = s.mainScrubberBkLeft_img.width,
      this.useHEXColorsForSkin_bl = s.useHEXColorsForSkin_bl,
      this.normalButtonsColor_str = s.normalButtonsColor_str,
      this.selectedButtonsColor_str = s.selectedButtonsColor_str,
			this.mainScrubberDragMiddlePath_str = s.mainScrubberDragMiddlePath_str,
      this.scrubberDragLeftWidth = s.mainScrubberDragLeft_img.width,
      this.playbackRateWindowTextColor_str = s.playbackRateWindowTextColor_str,
      this.defaultPlaybackRate = s.defaultPlaybackRate,
      this.totalWidth = 0,
      this.stageWidth = 0,
      this.stageHeight = 0,
      this.minMarginXSpace = 20,
      this.hSpace = 20,
      this.minHSpace = 10,
      this.vSpace = 15,
      this.minValue = .5,
      this.maxValue = 3,
      this.pointerWidth = 7,
      this.pointerHeight = 4,
      this.percent = 0,
			this.isScrubbing_bl = !1,
      this.isShowed_bl = !1,
      this.isMobile_bl = FWDMSPUtils.isMobile,
      this.init = function() {
          i.mainHolder_do = new FWDMSPDisplayObject("div"),
          i.mainHolder_do.hasTransform3d_bl = !1,
          i.mainHolder_do.hasTransform2d_bl = !1,
          i.bk_do = new FWDMSPDisplayObject("div"),
          i.bk_do.getStyle().width = "100%",
          i.bk_do.getStyle().height = "100%",
          i.bk_do.setAlpha(.9),
          i.bk_do.getStyle().background = "url('" + i.embedWindowBackground_str + "')",
					FWDMSPSimpleButton.setPrototype(),
          i.closeButton_do = new FWDMSPSimpleButton(
                                                    s.playbackRateWindowClooseN_img,
                                                    s.playbackRateClosePathS_str,
                                                    void 0, !0,
                                                    s.useHEXColorsForSkin_bl,
                                                    s.normalButtonsColor_str,
                                                    s.selectedButtonsColor_str
                                                  ),
          i.closeButton_do.addListener(FWDMSPSimpleButton.MOUSE_UP, i.closeButtonOnMouseUpHandler),
          i.addChild(i.mainHolder_do),
          i.mainHolder_do.addChild(i.bk_do),
          i.mainHolder_do.addChild(i.closeButton_do),
					this.setupScrubber()
				}, this.closeButtonOnMouseUpHandler = function() {
					i.isShowed_bl && i.hide(!0)
				}, this.positionAndResize = function() {
					i.stageWidth = o.stageWidth,
          i.stageHeight = o.stageHeight;
					var e = i.stageWidth - i.closeButton_do.w - i.embedWindowCloseButtonMargins,
					t = 0;
					t = o.playlist_do && o.position_str == FWDMSP.POSITION_TOP ? o.playlist_do.h : i.embedWindowCloseButtonMargins,
          i.closeButton_do.setX(e),
          i.closeButton_do.setY(0),
          i.setY(t),
          i.setWidth(i.stageWidth),
          i.setHeight(i.stageHeight),
          i.mainHolder_do.setWidth(i.stageWidth),
          i.mainHolder_do.setHeight(i.stageHeight),
          i.positionScruber(),
          i.updateScrubber(i.percent)
				}, this.setupScrubber = function() {
					i.scrubber_do = new FWDMSPDisplayObject("div"),
          i.scrubber_do.setHeight(i.scrubbersHeight), i.scrubber_do.setButtonMode(!0),
          i.scrubberBkLeft_do = new FWDMSPDisplayObject("img");
					var e = new Image;
					e.src = s.mainScrubberBkLeft_img.src,
          i.scrubberBkLeft_do.setScreen(e),
          i.scrubberBkLeft_do.setWidth(s.mainScrubberBkLeft_img.wideth),
          i.scrubberBkLeft_do.setHeight(s.mainScrubberBkLeft_img.height),
          i.scrubberBkRight_do = new FWDMSPDisplayObject("img");
					var t = new Image;
					t.src = s.mainScrubberBkRight_img.src,
          i.scrubberBkRight_do.setScreen(t),
          i.scrubberBkRight_do.setWidth(s.mainScrubberBkRight_img.width),
          i.scrubberBkRight_do.setHeight(s.mainScrubberBkRight_img.height),
          (new Image).src = i.scrubberBkMiddlePath_str,
          i.scrubberBkMiddle_do = new FWDMSPDisplayObject("div"),
          i.scrubberBkMiddle_do.getStyle().background = "url('" + i.scrubberBkMiddlePath_str + "')",
          i.scrubberBkMiddle_do.setHeight(i.scrubbersHeight),
          i.scrubberBkMiddle_do.setX(i.scrubbersBkLeftAndRightWidth),
          i.scrubberDrag_do = new FWDMSPDisplayObject("div"),
          i.scrubberDrag_do.setHeight(i.scrubbersHeight),
          i.useHEXColorsForSkin_bl ? (i.scrubberDragLeft_do = new FWDMSPDisplayObject("div"),
          i.scrubberDragLeft_do.setWidth(s.mainScrubberDragLeft_img.width),
          i.scrubberDragLeft_do.setHeight(s.mainScrubberDragLeft_img.height),
          i.scrubberDragLeft_canvas = FWDMSPUtils.getCanvasWithModifiedColor(s.mainScrubberDragLeft_img,i.normalButtonsColor_str).canvas,
          i.scrubberDragLeft_do.screen.appendChild(i.scrubberDragLeft_canvas)) : (i.mainScrubberDragLeft_img = new Image,
                                                                                  i.mainScrubberDragLeft_img.src = s.mainScrubberDragLeft_img.src,
                                                                                  i.mainScrubberDragLeft_img.width = s.mainScrubberDragLeft_img.width,
                                                                                  i.mainScrubberDragLeft_img.height = s.mainScrubberDragLeft_img.height,
                                                                                  i.scrubberDragLeft_do = new FWDMSPDisplayObject("img"),
                                                                                  i.scrubberDragLeft_do.setScreen(i.mainScrubberDragLeft_img)
                                                                                ),
          i.mainScrubberMiddleImage = new Image,
          i.mainScrubberMiddleImage.src = s.mainScrubberDragMiddlePath_str,
          i.useHEXColorsForSkin_bl ? (i.mainScrubberDragMiddle_do = new FWDMSPDisplayObject("div"),
                                      i.mainScrubberMiddleImage.onload = function() {
						                                                                         i.mainScrubberDragMiddle_canvas = FWDMSPUtils.getCanvasWithModifiedColor(i.mainScrubberMiddleImage, i.normalButtonsColor_str, !0),
                                                                                     i.mainSCrubberMiddleCanvas = i.mainScrubberDragMiddle_canvas.canvas,
                                                                                     i.mainSCrubberDragMiddleImageBackground = i.mainScrubberDragMiddle_canvas.image,
                                                                                     i.mainScrubberDragMiddle_do.getStyle().background = "url('" + i.mainSCrubberDragMiddleImageBackground.src + "') repeat-x"
                                                                                   }
                                      )
                                      :(i.mainScrubberDragMiddle_do = new FWDMSPDisplayObject("div"),
                                        i.mainScrubberDragMiddle_do.getStyle().background = "url('" + i.mainScrubberDragMiddlePath_str + "') repeat-x"
                                      ),
          i.mainScrubberDragMiddle_do.setHeight(i.scrubbersHeight),
          i.mainScrubberDragMiddle_do.setX(i.scrubberDragLeftWidth),
          i.scrubberBarLine_do = new FWDMSPDisplayObject("img");
					var o = new Image;
					o.src = s.mainScrubberLine_img.src,
          i.scrubberBarLine_do.setScreen(o),
          i.scrubberBarLine_do.setWidth(s.mainScrubberLine_img.width),
          i.scrubberBarLine_do.setHeight(s.mainScrubberLine_img.height),
          i.scrubberBarLine_do.setAlpha(0),
          i.scrubberBarLine_do.hasTransform3d_bl = !1,
          i.scrubberBarLine_do.hasTransform2d_bl = !1,
          i.minTime_do = new FWDMSPDisplayObject("div"),
          i.minTime_do.hasTransform3d_bl = !1,
          i.minTime_do.hasTransform2d_bl = !1,
          i.minTime_do.getStyle().fontFamily = "Arial",
          i.minTime_do.getStyle().fontSize = "12px",
          i.minTime_do.getStyle().whiteSpace = "nowrap",
          i.minTime_do.getStyle().textAlign = "left",
					i.minTime_do.getStyle().color = i.playbackRateWindowTextColor_str,
          i.minTime_do.getStyle().fontSmoothing = "antialiased",
          i.minTime_do.getStyle().webkitFontSmoothing = "antialiased",
          i.minTime_do.getStyle().textRendering = "optimizeLegibility",
          i.minTime_do.setInnerHTML("0.5"),
          i.mainHolder_do.addChild(i.minTime_do),
          i.maxTime_do = new FWDMSPDisplayObject("div"),
          i.maxTime_do.hasTransform3d_bl = !1,
          i.maxTime_do.hasTransform2d_bl = !1,
          i.maxTime_do.getStyle().fontFamily = "Arial",
          i.maxTime_do.getStyle().fontSize = "12px",
          i.maxTime_do.getStyle().whiteSpace = "nowrap",
          i.maxTime_do.getStyle().textAlign = "left",
					i.maxTime_do.getStyle().color = i.playbackRateWindowTextColor_str,
          i.maxTime_do.getStyle().fontSmoothing = "antialiased",
          i.maxTime_do.getStyle().webkitFontSmoothing = "antialiased",
          i.maxTime_do.getStyle().textRendering = "optimizeLegibility",
          i.maxTime_do.setInnerHTML("3.0"),
          i.mainHolder_do.addChild(i.maxTime_do),
          i.scrubber_do.addChild(i.scrubberBkLeft_do),
          i.scrubber_do.addChild(i.scrubberBkMiddle_do),
          i.scrubber_do.addChild(i.scrubberBkRight_do),
          i.scrubber_do.addChild(i.scrubberBarLine_do),
          i.scrubberDrag_do.addChild(i.scrubberDragLeft_do),
          i.scrubberDrag_do.addChild(i.mainScrubberDragMiddle_do),
          i.scrubber_do.addChild(i.scrubberDrag_do),
					i.scrubber_do.addChild(i.scrubberBarLine_do),
          i.mainHolder_do.addChild(i.scrubber_do),
          i.isMobile_bl ? i.hasPointerEvent_bl ? (i.scrubber_do.screen.addEventListener("pointerover", i.mainScrubberOnOverHandler),
                                                  i.scrubber_do.screen.addEventListener("pointerout", i.mainScrubberOnOutHandler),
                                                  i.scrubber_do.screen.addEventListener("pointerdown", i.mainScrubberOnDownHandler)
                                                 )
                                                 : i.scrubber_do.screen.addEventListener("touchstart", i.mainScrubberOnDownHandler)
                                                 : i.screen.addEventListener ? (i.scrubber_do.screen.addEventListener("mouseover", i.mainScrubberOnOverHandler),
                                                                                i.scrubber_do.screen.addEventListener("mouseout", i.mainScrubberOnOutHandler),
                                                                                i.scrubber_do.screen.addEventListener("mousedown", i.mainScrubberOnDownHandler)
                                                                              )
                                                 : i.screen.attachEvent && (i.scrubber_do.screen.attachEvent("onmouseover", i.mainScrubberOnOverHandler),
                                                                            i.scrubber_do.screen.attachEvent("onmouseout", i.mainScrubberOnOutHandler),
                                                                            i.scrubber_do.screen.attachEvent("onmousedown", i.mainScrubberOnDownHandler)
                                                                           )
				},
        this.mainScrubberOnOverHandler = function(e) {},
        this.mainScrubberOnOutHandler = function(e) {},
        this.mainScrubberOnDownHandler = function(e) {
					e.preventDefault && e.preventDefault(),
          i.isScrubbing_bl = !0;
					var t = FWDMSPUtils.getViewportMouseCoordinates(e).screenX - i.scrubber_do.getGlobalX();
					t < 0 ? t = 0 : t > i.scruberWidth - i.scrubbersOffsetWidth && (t = i.scruberWidth - i.scrubbersOffsetWidth);
					var o = t / i.scruberWidth,
						s = t / i.scruberWidth;
					i.disable_do && i.addChild(i.disable_do),
          i.updateScrubber(o),
          i.dispatchEvent(FWDMSPController.START_TO_SCRUB),
          i.dispatchEvent(FWDMSPController.SCRUB_PLAYLIST_ITEM, {
						              percent: s}
                          ),
          i.dispatchEvent(FWDMSPController.SCRUB, {
						              percent: o
					                 }),
          i.isMobile_bl ? i.hasPointerEvent_bl ? (n.addEventListener("pointermove", i.mainScrubberMoveHandler), n.addEventListener("pointerup", i.mainScrubberEndHandler))
          : (n.addEventListener("touchmove", i.mainScrubberMoveHandler), n.addEventListener("touchend", i.mainScrubberEndHandler))
          : n.addEventListener ? (n.addEventListener("mousemove", i.mainScrubberMoveHandler), n.addEventListener("mouseup", i.mainScrubberEndHandler))
          : document.attachEvent && (document.attachEvent("onmousemove", i.mainScrubberMoveHandler), document.attachEvent("onmouseup", i.mainScrubberEndHandler))
				},
         this.mainScrubberMoveHandler = function(e) {
					e.preventDefault && e.preventDefault();
					var t = FWDMSPUtils.getViewportMouseCoordinates(e).screenX - i.scrubber_do.getGlobalX();
					t < 0 ? t = 0 : t > i.scruberWidth - i.scrubbersOffsetWidth && (t = i.scruberWidth - i.scrubbersOffsetWidth);
					var o = t / i.scruberWidth,
						s = t / i.scruberWidth;
					i.updateScrubber(o),
          i.dispatchEvent(FWDMSPController.SCRUB_PLAYLIST_ITEM, {
						percent: s
					}), i.dispatchEvent(FWDMSPController.SCRUB, {
						percent: o
					})
				},
        this.mainScrubberEndHandler = function(e) {
					i.isScrubbing_bl = !1,
          i.disable_do && i.contains(i.disable_do) && i.removeChild(i.disable_do),
          i.updateScrubber(),
          i.dispatchEvent(FWDMSPController.STOP_TO_SCRUB),
          i.isMobile_bl ? i.hasPointerEvent_bl ? (n.removeEventListener("pointermove", i.mainScrubberMoveHandler), n.removeEventListener("pointerup", i.mainScrubberEndHandler))
          : (n.removeEventListener("touchmove", i.mainScrubberMoveHandler), n.removeEventListener("touchend", i.mainScrubberEndHandler)) : n.removeEventListener ? (n.removeEventListener("mousemove", i.mainScrubberMoveHandler), n.removeEventListener("mouseup", i.mainScrubberEndHandler))
          : document.detachEvent && (document.detachEvent("onmousemove", i.mainScrubberMoveHandler), document.detachEvent("onmouseup", i.mainScrubberEndHandler))
				},
        this.updateScrubber = function(e) {
					(i.percent = e) < 0 ? e = 0 : 1 < e && (e = 1);
					var t = parseInt(e * i.scruberWidth);
					i.isScrubbing_bl ? i.defaultPlaybackRate = Number(i.minValue + (i.maxValue - i.minValue) * t / i.scruberWidth).toFixed(1)
          : t = (i.defaultPlaybackRate - i.minValue) / (i.maxValue - i.minValue) * i.scruberWidth, t < 1 && i.isMainScrubberLineVisible_bl ? (i.isMainScrubberLineVisible_bl = !1, FWDAnimation.to(i.scrubberBarLine_do, .5, {
						alpha: 0
					})) : 2 < t && !i.isMainScrubberLineVisible_bl && (i.isMainScrubberLineVisible_bl = !0, FWDAnimation.to(i.scrubberBarLine_do, .5, {
						alpha: 1
					})), i.scrubberDrag_do.setWidth(t), t > i.scruberWidth - i.scrubbersOffsetWidth && (t = i.scruberWidth - i.scrubbersOffsetWidth), FWDAnimation.to(i.scrubberBarLine_do, .8, {
						x: t,
						ease: Expo.easeOut
					}), i.dispatchEvent(l.SET_PLAYBACK_RATE, {
						rate: i.defaultPlaybackRate
					})
				},
        this.positionScruber = function() {
					i.scruberWidth = Math.min(600, i.stageWidth - 100),
          i.scrubber_do.setWidth(i.scruberWidth),
          i.scrubber_do.setX(Math.round((i.stageWidth - i.scruberWidth) / 2)),
          i.scrubber_do.setY(Math.round((i.stageHeight - i.scrubbersHeight) / 2)),
          i.scrubberBkMiddle_do.setWidth(i.scruberWidth - 2 * i.scrubbersBkLeftAndRightWidth),
          i.scrubberBkRight_do.setX(i.scruberWidth - i.scrubbersBkLeftAndRightWidth),
          i.mainScrubberDragMiddle_do.setWidth(i.scruberWidth - i.scrubbersBkLeftAndRightWidth),
          i.minTime_do.setX(i.scrubber_do.x - 26),
          i.minTime_do.setY(i.scrubber_do.y + 4),
          i.maxTime_do.setX(i.scrubber_do.x + i.scrubber_do.w + 8),
          i.maxTime_do.setY(i.scrubber_do.y + 4)
				},
        this.show = function(e) {
					i.isShowed_bl || (i.isShowed_bl = !0, o.main_do.addChild(i), (!FWDMSPUtils.isMobile || FWDMSPUtils.isMobile && FWDMSPUtils.hasPointerEvent) && o.main_do.setSelectable(!0), i.positionAndResize(), clearTimeout(i.hideCompleteId_to), clearTimeout(i.showCompleteId_to), i.mainHolder_do.setY(-i.stageHeight), i.positionScruber(), setTimeout(function() {
						i.updateScrubber(i.percent)
					}, 200), i.showCompleteId_to = setTimeout(i.showCompleteHandler, 900), setTimeout(function() {
						FWDAnimation.to(i.mainHolder_do, .8, {
							y: 0,
							delay: .1,
							ease: Expo.easeInOut
						})
					}, 100))
				},
        this.showCompleteHandler = function() {}, this.hide = function(e) {
					i.isShowed_bl && (i.isShowed_bl = !1, o.customContextMenu_do && o.customContextMenu_do.enable(), clearTimeout(i.hideCompleteId_to), clearTimeout(i.showCompleteId_to), (!FWDMSPUtils.isMobile || FWDMSPUtils.isMobile && FWDMSPUtils.hasPointerEvent) && o.main_do.setSelectable(!1), i.hideCompleteId_to = setTimeout(i.hideCompleteHandler, 800), FWDAnimation.killTweensOf(i.mainHolder_do), e ? FWDAnimation.to(i.mainHolder_do, .8, {
						y: -i.stageHeight,
						ease: Expo.easeInOut
					}) : i.hideCompleteHandler())
				},
        this.hideCompleteHandler = function() {
					o.main_do.contains(i) && o.main_do.removeChild(i), i.dispatchEvent(l.HIDE_COMPLETE)
				},
        this.updateHEXColors = function(e, t) {
					-1 != s.skinPath_str.indexOf("hex_white") ? i.selectedColor_str = "#FFFFFF" : i.selectedColor_str = t, i.closeButton_do.updateHEXColors(e, i.selectedColor_str),
						FWDMSPUtils.changeCanvasHEXColor(i.mainScrubberDragLeft_img, i.scrubberDragLeft_canvas, e);
					var o = FWDMSPUtils.changeCanvasHEXColor(i.mainScrubberMiddleImage, i.mainSCrubberMiddleCanvas, e, !0);
					i.mainScrubberDragMiddle_do.getStyle().background = "url('" + o.src + "') repeat-x"
				},
        this.init()
		};
		l.setPrototype = function() {
			l.prototype = new FWDMSPDisplayObject("div")
		},
    l.HIDE_COMPLETE = "hideComplete",
    l.SET_PLAYBACK_RATE = "setPlaybackRate",
    l.prototype = null,
    n.FWDMSPPlaybackRateWindow = l
	}(window),

	function() {
		var n = function(p, m) {
			var b = this;
			b.data = p;
			n.prototype;
			this.playlist_ar = null,
      this.items_ar = null,
      this.playlistItemBk1_img = p.playlistItemBk1_img,
      this.playlistItemBk2_img = p.playlistItemBk2_img,
      this.playlistSeparator_img = p.playlistSeparator_img,
			this.playlistScrBkTop_img = p.playlistScrBkTop_img,
      this.playlistScrBkMiddle_img = p.playlistScrBkMiddle_img,
      this.playlistScrBkBottom_img = p.playlistScrBkBottom_img,
      this.playlistScrDragTop_img = p.playlistScrDragTop_img,
      this.playlistScrDragMiddle_img = p.playlistScrDragMiddle_img,
      this.playlistScrDragBottom_img = p.playlistScrDragBottom_img,
      this.playlistPlayButtonN_img = p.playlistPlayButtonN_img,
      this.playlistScrLines_img = p.playlistScrLines_img,
			this.playlistScrLinesOver_img = p.playlistScrLinesOver_img,
      this.playlistDownloadButtonN_img = p.playlistDownloadButtonN_img,
      this.playlistBuyButtonN_img = p.playlistBuyButtonN_img,
      this.disable_do = null,
			this.separator_do = null,
      this.itemsHolder_do = null,
      this.curItem_do = null,
      this.scrMainHolder_do = null,
      this.scrTrack_do = null,
			this.scrTrackTop_do = null,
      this.scrTrackMiddle_do = null,
      this.scrTrackBottom_do = null,
      this.scrHandler_do = null,
      this.scrHandlerTop_do = null,
      this.scrHandlerMiddle_do = null,
      this.scrHandlerBottom_do = null,
      this.scrHandlerLines_do = null,
      this.scrHandlerLinesN_do = null,
			this.scrHandlerLinesS_do = null,
      this.playlistPlayButtonN_str = p.playlistPlayButtonN_str,
      this.playlistPlayButtonS_str = p.playlistPlayButtonS_str,
      this.playlistPauseButtonN_str = p.playlistPauseButtonN_str,
      this.playlistPauseButtonS_str = p.playlistPauseButtonS_str,
      this.controllerBkPath_str = p.controllerBkPath_str,
			this.playlistBackgroundColor_str = p.playlistBackgroundColor_str,
      this.searchInputColor_str = p.searchInputColor_str,
			b.useHEXColorsForSkin_bl = p.useHEXColorsForSkin_bl,
      b.normalButtonsColor_str = p.normalButtonsColor_str,
      b.selectedButtonsColor_str = p.selectedButtonsColor_str,
      this.countTrack = 0,
			this.inputSearchTextOffsetTop = p.inputSearchTextOffsetTop,
      this.inputSearchOffsetLeft = p.inputSearchOffsetLeft,
			this.startSpaceBetweenButtons = p.startSpaceBetweenButtons,
      this.spaceBetweenButtons = p.spaceBetweenButtons,
      15 < this.spaceBetweenButtons && (this.spaceBetweenButtons = 10),
      this.searchBarHeight = p.searchBarHeight,
      this.countID3 = 0,
      this.id = 0,
      this.stageWidth = 0,
			this.stageHeight = 0,
      this.itemsTotalHeight = 0,
			this.scrollbarOffestWidth = p.scrollbarOffestWidth,
      this.scrWidth = b.playlistScrBkTop_img.width,
      this.trackTitleOffsetLeft = p.trackTitleOffsetLeft,
			this.downloadButtonOffsetRight = p.downloadButtonOffsetRight,
      this.itemHeight = b.playlistItemBk1_img.height,
      this.playPuaseIconWidth = b.playlistPlayButtonN_img.width,
      this.playPuaseIconHeight = b.playlistPlayButtonN_img.height,
      this.nrOfVisiblePlaylistItems = p.nrOfVisiblePlaylistItems,
      this.durationOffsetRight = p.durationOffsetRight,
			this.totalPlayListItems = 0,
      this.visibleNrOfItems = 0,
      this.yPositionOnPress = 0,
      this.lastPresedY = 0,
      this.lastListY = 0,
      this.playListFinalY = 0,
			this.scrollBarHandlerFinalY = 0,
      this.scrollBarHandlerFinalY = 0,
      this.vy = 0,
      this.vy2 = 0,
      this.friction = .9,
      this.comboboxHeight = 31,
			this.updateMobileScrollBarId_int,
      this.updateMoveMobileScrollbarId_int,
      this.disableOnMoveId_to,
      this.updateMobileScrollbarOnPlaylistLoadId_to,
      this.usePlaylistsSelectBox_bl = p.usePlaylistsSelectBox_bl,
      this.allowToTweenPlaylistItems_bl = !1,
      this.expandPlaylistBackground_bl = p.expandControllerBackground_bl,
      this.isSortedNumerical_bl = !0,
      this.showSortButtons_bl = p.showSortButtons_bl,
      this.showSearchBar_bl = p.showSearchBar_bl,
      this.showPlaylistItemBuyButton_bl = p.showPlaylistItemBuyButton_bl,
      this.addScrollBarMouseWheelSupport_bl = p.addScrollBarMouseWheelSupport_bl,
			this.allowToScrollAndScrollBarIsActive_bl = !1,
      this.isDragging_bl = !1,
      this.showPlaylistItemPlayButton_bl = p.showPlaylistItemPlayButton_bl,
      this.showPlaylistItemDownloadButton_bl = p.showPlaylistItemDownloadButton_bl,
      this.isShowed_bl = p.showPlayListByDefault_bl,
      this.isShowedFirstTime_bl = !1,
      this.animateOnIntro_bl = p.animateOnIntro_bl,
      this.isListCreated_bl = !1,
      this.isMobile_bl = FWDMSPUtils.isMobile,
      this.hasPointerEvent_bl = FWDMSPUtils.hasPointerEvent,
      b.init = function() {
					if (b.hasTransform3d_bl = !1,
              b.hasTransform2d_bl = !1,
              b.mainHolder_do = new FWDMSPDisplayObject("div"),
              b.mainHolder_do.hasTransform3d_bl = !1,
              b.mainHolder_do.hasTransform2d_bl = !1,
              b.itemsHolder_do = new FWDMSPDisplayObject("div"),
              b.itemsHolder_do.setOverflow("visible"),
              b.itemsHolder_do.setY(0),
              b.setupSeparator(),
              b.itemsHolder_do.setY(0),
              b.mainHolder_do.addChild(b.itemsHolder_do),
              b.addChild(b.mainHolder_do),
              b.isMobile_bl ? (b.setupMobileScrollbar(), b.hasPointerEvent_bl && b.setupDisable())
              : (b.setupDisable(), b.setupScrollbar(), b.addScrollBarMouseWheelSupport_bl && b.addMouseWheelSupport()),
              b.usePlaylistsSelectBox_bl && b.setupcomboBox(),
              b.showSearchBar_bl)
            {
						if (b.searchBar_do = new FWDMSPDisplayObject("div"), b.searchBar_do.setOverflow("visible"), b.expandPlaylistBackground_bl) {
							b.controllerBk_do = new FWDMSPDisplayObject("img");
							var e = new Image;
							e.src = b.controllerBkPath_str,
              b.controllerBk_do.setScreen(e)
						}
            else b.controllerBk_do = new FWDMSPDisplayObject("div"),
            b.controllerBk_do.getStyle().background = "url('" + b.controllerBkPath_str + "')";
						b.controllerBk_do.getStyle().width = "100%",
            b.searchSeparator_do = new FWDMSPDisplayObject("div"),
            b.searchSeparator_do.hasTransform3d_bl = !1,
            b.searchSeparator_do.hasTransform2d_bl = !1,
            b.searchSeparator_do.getStyle().background = "url('" + b.playlistSeparator_img.src + "')",
            b.searchSeparator_do.setHeight(b.playlistSeparator_img.height),
            b.searchBar_do.setHeight(b.searchBarHeight + b.searchSeparator_do.h),
            b.controllerBk_do.setHeight(b.searchBar_do.h + 1),
            b.searchBar_do.addChild(b.controllerBk_do),
            b.searchBar_do.addChild(b.searchSeparator_do),
            b.setupInput(),
            b.showSortButtons_bl && (b.setupButtons()),
            b.mainHolder_do.addChild(b.searchBar_do)
					}
					b.addChild(b.separator_do),
          b.mainHolder_do.setWidth(500),
          b.mainHolder_do.setHeight(500)
				},
        b.disableSearchBar = function() {
					b.isSearchBarDisabled_bl || (b.isSearchBarDisabled_bl = !0, b.input_do.screen.value = "Search will be available when all tracks data is loaded!", b.input_do.screen.disabled = !0, b.sortNButton_do && (b.sortNButton_do.disable(), b.sortAButton_do.disable(), b.ascDscButton_do.disable()))
				},
        b.enableSearchBar = function() {
					b.isSearchBarDisabled_bl && (b.isSearchBarDisabled_bl = !1, b.input_do.screen.value = "Search for track", b.input_do.screen.disabled = !1, b.sortNButton_do && (b.sortNButton_do.enable(), b.sortAButton_do.enable(), b.ascDscButton_do.enable()))
				},
        b.resizeAndPosition = function(e) {
					(m.stageWidth != b.stageWidth || m.stageHeight != b.stageHeight || e) && b.isListCreated_bl && (b.stageWidth = m.stageWidth, b.stageWidth = m.stageWidth, b.comboBox_do && b.comboBox_do.resizeAndPosition(), b.positionList(), b.searchBar_do && b.positionSearchBar(), b.scrMainHolder_do && b.allowToScrollAndScrollBarIsActive_bl && b.scrMainHolder_do.setX(b.stageWidth - b.scrWidth))
				},
        b.positionList = function(e) {
					if (b.isListCreated_bl) {
						var t, o = 0;
						if (b.usePlaylistsSelectBox_bl && (o = b.comboboxHeight),
                b.copy_ar = [].concat(b.items_ar),
                b.isSearched_bl = !1,
                b.input_do && (inputValue = b.input_do.screen.value, "Search for track" != inputValue && !b.isSearchBarDisabled_bl)
              )
              {
							inputValue = b.input_do.screen.value.toLowerCase();
							for (var s = 0; s < b.copy_ar.length; s++) - 1 == (t = b.copy_ar[s]).titleText_str.toLowerCase().indexOf(inputValue.toLowerCase()) && (FWDAnimation.killTweensOf(t),
              1 != t.alpha && t.setAlpha(1),
              t.setX(-t.w),
              b.copy_ar.splice(s, 1), s--)
						}
						for (s = 0; s < b.copy_ar.length; s++)(t = b.copy_ar[s]).changeSource(s % 2);
						var i = b.copy_ar.length;
						b.totalSearchedItems = i,
            b.itemsTotalHeight = i * b.itemHeight,
						b.visibleNrOfItems >= i ? b.allowToScrollAndScrollBarIsActive_bl = !1 : b.allowToScrollAndScrollBarIsActive_bl = !0;
						for (s = 0; s < i; s++) t = b.copy_ar[s],
						b.allowToTweenPlaylistItems_bl && t.x < 0 && !b.isMobile_bl ? FWDAnimation.isTweening(t) || FWDAnimation.to(t, .8, {
								x: 0,
								ease: Expo.easeInOut
							}) : (FWDAnimation.killTweensOf(t), t.setX(0)), t.setY(b.itemHeight * s), b.allowToScrollAndScrollBarIsActive_bl && b.scrMainHolder_do ? t.resize(b.stageWidth - b.scrollbarOffestWidth, b.itemHeight) : t.resize(b.stageWidth, b.itemHeight), 1 != t.alpha && t.setAlpha(1);
						b.allowToScrollAndScrollBarIsActive_bl && b.scrMainHolder_do ? b.itemsHolder_do.setWidth(b.stageWidth - b.scrollbarOffestWidth) : b.itemsHolder_do.setWidth(b.stageWidth), b.input_do && (0 == i ? b.showNothingFound() : b.hideNothingFound()), b.scrHandler_do && b.updateScrollBarSizeActiveAndDeactivate(),
							b.separator_do.setWidth(b.stageWidth),
							b.mainHolder_do.setWidth(b.stageWidth), b.mainHolder_do.setY(o),
							b.mainHolder_do.setHeight(b.stageHeight + o), b.setWidth(b.stageWidth), b.setHeight(b.stageHeight + o)
					}
				}, this.setupcomboBox = function() {
						b.labels_ar = [];
						for (var e = 0; e < p.cats_ar.length; e++) {
								b.labels_ar[e] = p.cats_ar[e];
								var t = "";
								p.showPlaylistsSelectBoxNumbers_bl ? (e < 9 && (t = "0"), t = t + (e + 1) + ". ", b.labels_ar[e] = t + p.cats_ar[e]) : b.labels_ar[e] = p.cats_ar[e]
						}
						var o = {
								categories_ar: b.labels_ar,
								selectorLabel: b.labels_ar[0],
								bk1_str: p.comboboxBk1_str,
								bk2_str: p.comboboxBk2_str,
								selectorBackgroundNormalColor: p.mainSelectorBackgroundSelectedColor,
								selectorTextNormalColor: p.mainSelectorTextNormalColor,
								selectorTextSelectedColor: p.mainSelectorTextSelectedColor,
								buttonBackgroundNormalColor: p.mainButtonBackgroundNormalColor,
								buttonBackgroundSelectedColor: p.mainButtonBackgroundSelectedColor,
								buttonTextNormalColor: p.mainButtonTextNormalColor,
								buttonTextSelectedColor: p.mainButtonTextSelectedColor,
								buttonHeight: b.comboboxHeight,
								arrowN_str: p.arrowN_str,
								arrowS_str: p.arrowS_str,
								arrowW: 11,
								arrowH: 6
						};
						FWDMSPComboBox.setPrototype(), b.comboBox_do = new FWDMSPComboBox(b, o), b.comboBox_do.addListener(FWDMSPComboBox.BUTTON_PRESSED, b.changePlaylistOnClick), b.addChild(b.comboBox_do)
				}, this.changePlaylistOnClick = function(e) {
					b.dispatchEvent(n.CHANGE_PLAYLIST, {
						id: e.id
					})
				}, this.updatePlaylist = function(e) {
					if (!b.isListCreated_bl) {
						b.playlist_ar = e, b.isShowedFirstTime_bl = !0, b.stageHeight = 0, b.isListCreated_bl = !0, b.input_do && (b.input_do.screen.value = "Search for track"), b.allowToScrollAndScrollBarIsActive_bl = !1, b.countID3, b.countTrack = 0, b.visibleNrOfItems = b.nrOfVisiblePlaylistItems, b.totalPlayListItems = b.playlist_ar.length, b.nrOfVisiblePlaylistItems > b.totalPlayListItems && (b.visibleNrOfItems = b.totalPlayListItems), b.nrOfVisiblePlaylistItems > b.totalPlayListItems && (b.nrOfVisiblePlaylistItems = b.totalPlayListItems), b.stageHeight = b.visibleNrOfItems * b.itemHeight + b.separator_do.h, b.searchBar_do && (b.stageHeight += b.separator_do.h + b.searchBarHeight), b.itemsTotalHeight = b.totalPlayListItems * b.itemHeight, b.mainHolder_do.setY(-b.stageHeight), b.itemsHolder_do.setY(0), b.sortNButton_do && (b.disableSortNButton(), b.ascDscButton_do.setButtonState(1), b.srotAscending_bl = !0), b.showSearchBar_bl && b.enableSearchBar(), b.createPlayList(),
							b.loadId3();
						var t = b.items_ar.length;
						clearTimeout(b.updateMobileScrollbarOnPlaylistLoadId_to), b.updateMobileScrollbarOnPlaylistLoadId_to = setTimeout(b.updateScrollBarHandlerAndContent, 900), clearTimeout(b.showAnimationIntroId_to), b.showAnimationIntroId_to = setTimeout(function() {
							for (var e = 0; e < t; e++) b.items_ar[e].setTextSizes();
							b.isListCreated_bl = !0, b.visibleNrOfItems >= b.totalPlayListItems ? b.allowToScrollAndScrollBarIsActive_bl = !1 : b.allowToScrollAndScrollBarIsActive_bl = !0, b.scrHandler_do && b.updateScrollBarSizeActiveAndDeactivate(),
								b.scrMainHolder_do && b.allowToScrollAndScrollBarIsActive_bl && b.scrMainHolder_do.setX(b.stageWidth - b.scrWidth),
								m.position_str == FWDMSP.POSITION_TOP ? (b.mainHolder_do.setY(0), b.usePlaylistsSelectBox_bl ? b.separator_do.setY(b.stageHeight - b.separator_do.h + b.comboboxHeight) : b.separator_do.setY(b.stageHeight - b.separator_do.h)) : (b.mainHolder_do.setY(b.separator_do.h), b.separator_do.setY(0)), b.positionList(), b.allowToTweenPlaylistItems_bl = !0
						}, 100)
					}
				}, this.destroyPlaylist = function() {
					if (b.isListCreated_bl) {
						var e, t = b.items_ar.length;
						b.isListCreated_bl = !1, b.allowToTweenPlaylistItems_bl = !1,
							clearTimeout(b.showAnimationIntroId_to);
						for (var o = 0; o < t; o++) e = b.items_ar[o], b.itemsHolder_do.removeChild(e), e.destroy();
						b.items_ar = null, b.stageHeight = 0, b.setHeight(b.stageHeight)
					}
				}, this.createPlayList = function() {
					var e, t;
					b.itemsHolder_do.setHeight(b.totalPlayListItems * b.itemHeight), b.mainHolder_do.setBkColor(b.playlistBackgroundColor_str), b.items_ar = [];
					for (var o = 0; o < b.totalPlayListItems; o++) {
						t = null == b.playlist_ar[o].duration ? void 0 : FWDMSP.formatTotalTime(b.playlist_ar[o].duration);
						var s = b.playlist_ar[o].downloadable;
						b.showPlaylistItemDownloadButton_bl || (s = !1);
						var i = Boolean(b.playlist_ar[o].buy);
						b.showPlaylistItemBuyButton_bl || (i = !1), FWDMSPPlaylistItem.setPrototype(),
							(e = new FWDMSPPlaylistItem(b.playlist_ar[o].title, b.playlist_ar[o].titleText, b.playlistDownloadButtonN_img, p.playlistDownloadButtonS_str, b.playlistBuyButtonN_img, p.playlistBuyButtonS_str, p.playlistItemGrad1_img, p.playlistItemGrad2_img, p.playlistItemProgress1_img, p.playlistItemProgress2_img, p.playlistPlayButtonN_img, p.playlistItemBk1_img.src, p.playlistItemBk2_img.src, b.playlistPlayButtonN_str, b.playlistPlayButtonS_str, b.playlistPauseButtonN_str, b.playlistPauseButtonS_str, p.trackTitleNormalColor_str, p.trackTitleSelected_str, p.trackDurationColor_str, o, p.playPauseButtonOffsetLeftAndRight, b.trackTitleOffsetLeft, b.durationOffsetRight, b.downloadButtonOffsetRight, b.showPlaylistItemPlayButton_bl, s, i, t, b.useHEXColorsForSkin_bl, b.normalButtonsColor_str, b.selectedButtonsColor_str, b)).addListener(FWDMSPPlaylistItem.MOUSE_UP, b.itemOnUpHandler), e.addListener(FWDMSPPlaylistItem.DOWNLOAD, b.downloadHandler), e.addListener(FWDMSPPlaylistItem.BUY, b.buyHandler), b.items_ar[o] = e, b.itemsHolder_do.addChild(e)
					}
				},
				this.addTrack = function(e, t, o, s, i, n, l) {
					var r;
					b.isSortedNumerical_bl = !0, b.srotAscending_bl = !0, b.ascDscButton_do && b.ascDscButton_do.setButtonState(1), b.disableSortNButton(),
						b.sortList();
					var a, d = 0;
					b.addAtThePlaylistEnd_bl = !1, b.addAtThePlaylistBeggingin_bl = !1, a = i ? (b.addAtThePlaylistBeggingin_bl = !0, 0) : (b.addAtThePlaylistEnd_bl = !0, b.totalPlayListItems + 1),
						clearTimeout(b.resetItemsAddOrderId_to), b.resetItemsAddOrderId_to = setTimeout(function() {
							b.addAtThePlaylistEnd_bl = !1, b.addAtThePlaylistBeggingin_bl = !1
						}, 100);
					var u = Boolean(l);
					b.showPlaylistItemBuyButton_bl || (u = !1),
						r = t = p.showTracksNumbers_bl ? (a < 9 && (d = "0" + (a + 1)), d + ". " + t) : t, FWDMSPPlaylistItem.setPrototype(); 
					var c = new FWDMSPPlaylistItem(t, r, b.playlistDownloadButtonN_img, p.playlistDownloadButtonS_str, b.playlistBuyButtonN_img, p.playlistBuyButtonS_str, p.playlistItemGrad1_img, p.playlistItemGrad2_img, p.playlistItemProgress1_img, p.playlistItemProgress2_img, p.playlistPlayButtonN_img, p.playlistItemBk1_img.src, p.playlistItemBk2_img.src, b.playlistPlayButtonN_str, b.playlistPlayButtonS_str, b.playlistPauseButtonN_str, b.playlistPauseButtonS_str, p.trackTitleNormalColor_str, p.trackTitleSelected_str, p.trackDurationColor_str, a, p.playPauseButtonOffsetLeftAndRight, b.trackTitleOffsetLeft, b.durationOffsetRight, b.downloadButtonOffsetRight, b.showPlaylistItemPlayButton_bl, n, u, s, b.useHEXColorsForSkin_bl, b.normalButtonsColor_str, b.selectedButtonsColor_str, b),
						h = {};
					h.title = t, h.titleText = t, h.source = e, h.duration = s, h.thumbPath = o, h.downloadable = n, h.buy = l, u && (h.buy = 100), b.playlist_ar.splice(a, 0, h), b.items_ar.splice(a, 0, c), b.itemsHolder_do.addChild(c), b.totalPlayListItems = b.playlist_ar.length,
						m.totalAudio = b.totalPlayListItems;
					for (var _ = 0; _ < b.totalPlayListItems; _++) {
						var f = b.items_ar[_];
						f.id = f.sortId = _, t = (t = b.playlist_ar[_].title).substr(t.indexOf(".") + 1), t = p.showTracksNumbers_bl ? (d = _ < 9 ? "0" + (_ + 1) : _ + 1) + ". " + t : t, f.title_str = t, f.updateTitle(), f.setTextSizes(!0)
					}
					setTimeout(function() {
							c && (c.setTextSizes(!0), b.allowToScrollAndScrollBarIsActive_bl && b.scrMainHolder_do ? c.resize(b.stageWidth - b.scrollbarOffestWidth, b.itemHeight) : c.resize(b.stageWidth, b.itemHeight), FWDAnimation.to(c, .1, {
								alpha: 1,
								ease: Expo.easeOut,
								overwrite: !1
							}), FWDAnimation.to(c, .1, {
								alpha: .5,
								delay: .1,
								ease: Expo.easeOut,
								overwrite: !1
							}), FWDAnimation.to(c, .1, {
								alpha: 1,
								delay: .2,
								ease: Expo.easeOut,
								overwrite: !1
							}), FWDAnimation.to(c, .1, {
								alpha: .5,
								delay: .3,
								ease: Expo.easeOut,
								overwrite: !1
							}), FWDAnimation.to(c, .1, {
								alpha: 1,
								delay: .4,
								ease: Expo.easeOut,
								overwrite: !1
							}))
						}, 50), c.addListener(FWDMSPPlaylistItem.MOUSE_UP, b.itemOnUpHandler), c.addListener(FWDMSPPlaylistItem.DOWNLOAD, b.downloadHandler), c.addListener(FWDMSPPlaylistItem.BUY, b.buyHandler),
						b.positionList(), b.updateScrollBarHandlerAndContent(!0, !0),
						c.setAlpha(0)
				}, this.itemOnUpHandler = function(e) {
					b.dispatchEvent(FWDMSPPlaylistItem.MOUSE_UP, {
						id: e.id
					})
				}, this.downloadHandler = function(e) {
					b.dispatchEvent(FWDMSPPlaylistItem.DOWNLOAD, {
						id: e.id
					})
				}, this.buyHandler = function(e) {
					b.dispatchEvent(FWDMSPPlaylistItem.BUY, {
						id: e.id
					})
				}, this.loadId3 = function() {
					clearTimeout(b.populateNextItemId_to);
					for (var e = 0; e < b.totalPlayListItems; e++)
						if ("..." != b.playlist_ar[e].title) return void(b.countID3 = 2001);
					b.showSearchBar_bl && b.disableSearchBar(), b.countID3 = 0, b.loadID3AndPopulate()
				}, this.loadID3AndPopulate = function() {
					if (b.items_ar)
						if (b.playlist_ar[b.countID3]) {
							var t = "",
								o = b.items_ar[b.countID3],
								s = b.playlist_ar[b.countID3].source + "?rand=" + parseInt(99999999 * Math.random()),
								i = b.playlist_ar[b.countID3];
							ID3.loadTags(s, function() {
								if (b.countID3 > b.playlist_ar.length || 2001 == b.countID3) clearTimeout(b.populateNextItemId_to);
								else {
									var e = ID3.getAllTags(s);
									e.artist && (i.titleText_str = e.artist + " - " + e.title, p.showTracksNumbers_bl ? (b.countTrack < 9 && (t = "0"), t = t + (b.countTrack + 1) + ". ", i.title = t + i.titleText_str) : i.title = i.titleText_str, b.countTrack++), o.title_str = i.title, o.titleText_str = i.titleText_str, b.countID3 == b.id && b.dispatchEvent(n.UPDATE_TRACK_TITLE_if_FOLDER, {
											title: o.title_str
										}), o.updateTitle(),
										setTimeout(function() {
											o && (o.setTextSizes(!0), b.allowToScrollAndScrollBarIsActive_bl && b.scrMainHolder_do ? o.resize(b.stageWidth - b.scrollbarOffestWidth, b.itemHeight) : o.resize(b.stageWidth, b.itemHeight))
										}, 50), b.countID3++, b.populateNextItemId_to = setTimeout(b.loadID3AndPopulate, 150)
								}
							})
						}
					else b.showSearchBar_bl && b.enableSearchBar()
				}, this.activateItems = function(e, t) {
					var o;
					if (b.id = e, b.items_ar) {
						for (var s = 0; s < b.totalPlayListItems; s++)
							if ((o = b.items_ar[s]).id == b.id) {
								b.sortId = o.sortId;
								break
							} b.curItem_do = b.items_ar[b.sortId],
							b.id = b.curItem_do.id;
						for (s = 0; s < b.totalPlayListItems; s++) o = b.items_ar[s], s == b.sortId ? o.setActive() : o.setInActive();
						t || b.updateScrollBarHandlerAndContent(!0)
					}
				}, this.setCurItemPlayState = function() {
					b.curItem_do && b.curItem_do.showPlayButton()
				}, this.setCurItemPauseState = function() {
					b.curItem_do && b.curItem_do.showPauseButton()
				}, this.updateCurItemProgress = function(e) {
					b.curItem_do && b.curItem_do.updateProgressPercent(e)
				}, this.setupInput = function() {
					b.titlebarHeight = p.titlebarLeftPath_img.height,
          b.mainSearchInput_do = new FWDMSPDisplayObject("div"),
          b.mainSearchInput_do.getStyle().background = "url('" + p.titlebarBkMiddlePattern_str + "')",
          b.mainSearchInput_do.setHeight(b.titlebarHeight);
					var e = new Image;
					e.src = p.titleBarLeft_img.src,
          b.titleBarLeft_do = new FWDMSPDisplayObject("img"),
          b.titleBarLeft_do.setScreen(e),
          b.titleBarLeft_do.setWidth(p.titleBarLeft_img.width),
          b.titleBarLeft_do.setHeight(p.titleBarLeft_img.height);
					var t = new Image;
					t.src = p.titleBarRigth_img.src,
          b.titleBarRight_do = new FWDMSPDisplayObject("img"),
          b.titleBarRight_do.setScreen(t),
          b.titleBarRight_do.setWidth(p.titleBarRigth_img.width),
          b.titleBarRight_do.setHeight(p.titleBarRigth_img.height),
          b.input_do = new FWDMSPDisplayObject("input"),
          b.input_do.screen.maxLength = 20,
          b.input_do.getStyle().textAlign = "left",
          b.input_do.getStyle().outline = "none",
          b.input_do.getStyle().boxShadow = "none",
          b.input_do.getStyle().fontSmoothing = "antialiased",
          b.input_do.getStyle().webkitFontSmoothing = "antialiased",
          b.input_do.getStyle().textRendering = "optimizeLegibility",
          b.input_do.getStyle().fontFamily = "Arial",
					b.input_do.getStyle().fontSize = "12px",
          b.input_do.getStyle().padding = "6px",
					FWDMSPUtils.isIEAndLessThen9 || (b.input_do.getStyle().paddingRight = "-6px"),
          b.input_do.getStyle().paddingTop = "2px",
          b.input_do.getStyle().paddingBottom = "3px",
          b.input_do.getStyle().color = b.searchInputColor_str,
          b.input_do.screen.value = "Search for track",
          b.noSearchFound_do = new FWDMSPDisplayObject("div"),
          b.noSearchFound_do.setX(0), b.noSearchFound_do.getStyle().textAlign = "center",
          b.noSearchFound_do.getStyle().width = "100%",
          b.noSearchFound_do.getStyle().fontSmoothing = "antialiased",
					b.noSearchFound_do.getStyle().webkitFontSmoothing = "antialiased",
          b.noSearchFound_do.getStyle().textRendering = "optimizeLegibility",
          b.noSearchFound_do.getStyle().fontFamily = "Arial",
          b.noSearchFound_do.getStyle().fontSize = "12px",
          b.noSearchFound_do.getStyle().color = b.searchInputColor_str,
          b.noSearchFound_do.setInnerHTML("NOTHING FOUND!"),
          b.noSearchFound_do.setVisible(!1),
          b.mainHolder_do.addChild(b.noSearchFound_do),
          b.input_do.screen.addEventListener ? (b.input_do.screen.addEventListener("focus", b.inputFocusInHandler), b.input_do.screen.addEventListener("blur", b.inputFocusOutHandler), b.input_do.screen.addEventListener("keyup", b.keyUpHandler))
          : b.input_do.screen.attachEvent && (b.input_do.screen.attachEvent("onfocus", b.inputFocusInHandler), b.input_do.screen.attachEvent("onblur", b.inputFocusOutHandler), b.input_do.screen.attachEvent("onkeyup", b.keyUpHandler)),
          b.inputArrow_img = new Image,
          b.inputArrow_img.src = p.inputArrowPath_str,
          b.useHEXColorsForSkin_bl ? (b.inputArrow_do = new FWDMSPDisplayObject("div"),
          b.inputArrow_img.onload = function() {
							b.mainScrubberDragLeft_canvas = FWDMSPUtils.getCanvasWithModifiedColor(b.inputArrow_img, b.normalButtonsColor_str).canvas,
              b.inputArrow_do.setWidth(b.inputArrow_img.width),
              b.inputArrow_do.setHeight(b.inputArrow_img.height),
              b.inputArrow_do.screen.appendChild(b.mainScrubberDragLeft_canvas)
						}) : (b.inputArrow_do = new FWDMSPDisplayObject("img"), b.inputArrow_do.setScreen(b.inputArrow_img), b.inputArrow_do.setWidth(14), b.inputArrow_do.setHeight(12)),
            setTimeout(function() {
							b.input_do.setY(parseInt((b.titlebarHeight - b.input_do.getHeight()) / 2) + b.inputSearchTextOffsetTop)
						}, 50),
            b.mainSearchInput_do.addChild(b.titleBarLeft_do),
            b.mainSearchInput_do.addChild(b.titleBarRight_do),
            b.mainSearchInput_do.addChild(b.input_do),
						b.searchBar_do.addChild(b.inputArrow_do),
						b.searchBar_do.addChild(b.mainSearchInput_do)
				},
        this.inputFocusInHandler = function() {
					b.hasInputFocus_bl || (b.hasInputFocus_bl = !0, FWDMSP.isSearchedFocused_bl = !0, b.isSearchBarDisabled_bl ? b.input_do.screen.value : "Search for track" == b.input_do.screen.value && (b.input_do.screen.value = ""))
				}, this.inputFocusOutHandler = function(e) {
					if (b.hasInputFocus_bl) {
						FWDMSP.isSearchedFocused_bl = !1;
						var t = FWDMSPUtils.getViewportMouseCoordinates(e);
						return FWDMSPUtils.hitTest(b.input_do.screen, t.screenX, t.screenY) ? void 0 : (b.hasInputFocus_bl = !1, void("" == b.input_do.screen.value && (b.input_do.screen.value = "Search for track")))
					}
				}, this.keyUpHandler = function(e) {
					e.stopPropagation && e.stopPropagation(),
          b.prevInputValue_str != b.input_do.screen.value && (b.isMobile_bl, b.positionList()),
          b.prevInputValue_str = b.input_do.screen.value,
          b.scrHandler_do && (b.updateScrollBarSizeActiveAndDeactivate(), b.updateScrollBarHandlerAndContent(!1))
				},
        this.showNothingFound = function() {
					b.isShowNothingFound_bl || (b.isShowNothingFound_bl = !0, b.noSearchFound_do.setVisible(!0), b.noSearchFound_do.setY(parseInt((b.stageHeight - b.noSearchFound_do.getHeight() - b.searchBar_do.h) / 2)), b.noSearchFound_do.setAlpha(0), FWDAnimation.to(b.noSearchFound_do, .1, {
						alpha: 1,
						yoyo: !0,
						repeat: 4
					}))
				},
        this.hideNothingFound = function() {
					b.isShowNothingFound_bl && (b.isShowNothingFound_bl = !1, FWDAnimation.killTweensOf(b.noSearchFound_do), b.noSearchFound_do.setVisible(!1))
				},
        this.setupButtons = function() {
					b.searchBarButtons_ar = [],
          FWDMSPSimpleButton.setPrototype(),
          b.sortNButton_do = new FWDMSPSimpleButton(p.sortNN_img, p.sortNSPath_str, null, !0, p.useHEXColorsForSkin_bl, p.normalButtonsColor_str, p.selectedButtonsColor_str),
          b.searchBarButtons_ar.push(b.sortNButton_do),
          b.sortNButton_do.addListener(FWDMSPSimpleButton.MOUSE_UP, b.sortNButtonOnMouseUpHandler),
          b.searchBar_do.addChild(b.sortNButton_do),
					b.sortNButton_do.setX(410),
					FWDMSPSimpleButton.setPrototype(),
          b.sortAButton_do = new FWDMSPSimpleButton(p.sortAN_img, p.sortASPath_str, null, !0, p.useHEXColorsForSkin_bl, p.normalButtonsColor_str, p.selectedButtonsColor_str),
          b.searchBarButtons_ar.push(b.sortAButton_do),
          b.sortAButton_do.addListener(FWDMSPSimpleButton.MOUSE_UP, b.sortAButtonOnMouseUpHandler),
          b.searchBar_do.addChild(b.sortAButton_do),
					b.sortAButton_do.setX(450),
					FWDMSPComplexButton.setPrototype(),
          b.ascDscButton_do = new FWDMSPComplexButton(p.ascendingN_img, p.ascendingSpath_str, p.decendingN_img, p.decendingSpath_str, !0, p.useHEXColorsForSkin_bl, p.normalButtonsColor_str, p.selectedButtonsColor_str),
          b.ascDscButton_do.setX(500),
          b.searchBarButtons_ar.push(b.ascDscButton_do),
          b.ascDscButton_do.addListener(FWDMSPComplexButton.MOUSE_UP, b.ascDscMouseUpHandler),
          b.searchBar_do.addChild(b.ascDscButton_do),
					b.isSortedNumerical_bl ? b.disableSortNButton() : b.disableSortAButton()
				},
        this.ascDscMouseUpHandler = function() {
					b.srotAscending_bl ? (b.ascDscButton_do.setButtonState(0), b.srotAscending_bl = !1) : (b.ascDscButton_do.setButtonState(1), b.srotAscending_bl = !0), b.sortList()
				},
        this.sortAButtonOnMouseUpHandler = function() {
					b.disableSortAButton(),
          b.sortList()
				},
        this.sortNButtonOnMouseUpHandler = function() {
					b.disableSortNButton(),
          b.sortList()
				},
        this.disableSortAButton = function() {
					b.sortAButton_do.disableForGood(),
          b.sortAButton_do.setSelectedState(),
          b.sortNButton_do.enableForGood(),
          b.sortNButton_do.setNormalState(),
          b.isSortedNumerical_bl = !1
				},
        this.disableSortNButton = function() {
					b.sortNButton_do && (b.sortNButton_do.disableForGood(), b.sortNButton_do.setSelectedState(), b.sortAButton_do.enableForGood(), b.sortAButton_do.setNormalState()),
          b.isSortedNumerical_bl = !0
				},
        this.sortList = function() {
					b.isSortedNumerical_bl ? b.items_ar.sort(function(e, t) {
						return e.id < t.id ? -1 : e.id > t.id ? 1 : 0
					}) : b.items_ar.sort(function(e, t) {
						return e.titleText_str < t.titleText_str ? -1 : e.titleText_str > t.titleText_str ? 1 : 0
					}), b.srotAscending_bl || b.items_ar.reverse();
					for (var e = 0; e < b.items_ar.length; e++) b.items_ar[e].sortId = e;
					b.positionList(), b.updateScrollBarHandlerAndContent(!1)
				}, b.positionSearchBar = function() {
					var e, t = 0;
					if (inputWidth = b.stageWidth - 2 * b.startSpaceBetweenButtons - b.inputArrow_do.w - 12, 430 < inputWidth && (inputWidth = 430), b.showSortButtons_bl)
						for (var o = b.searchBarButtons_ar.length - 1; 0 <= o; o--)
            e = b.searchBarButtons_ar[o],
            o == b.searchBarButtons_ar.length - 1 ? e.setX(b.stageWidth - e.w - b.startSpaceBetweenButtons) : e.setX(b.searchBarButtons_ar[o + 1].x - e.w - b.spaceBetweenButtons), e.setY(b.searchSeparator_do.h + parseInt((b.searchBar_do.h - b.searchSeparator_do.h - e.h) / 2)), t += e.w + b.spaceBetweenButtons;
					t += b.startSpaceBetweenButtons, inputWidth -= t, b.mainSearchInput_do.setWidth(inputWidth), b.input_do.setWidth(inputWidth), b.mainSearchInput_do.setX(b.startSpaceBetweenButtons + b.inputSearchOffsetLeft), b.mainSearchInput_do.setY(parseInt(b.searchSeparator_do.h + parseInt((b.searchBar_do.h - b.searchSeparator_do.h - b.mainSearchInput_do.h) / 2))), b.titleBarRight_do.setX(b.mainSearchInput_do.w - b.titleBarRight_do.w), b.inputArrow_do.setX(parseInt(b.mainSearchInput_do.x + inputWidth) + 4), b.inputArrow_do.setY(b.searchSeparator_do.h + parseInt((b.searchBar_do.h - b.searchSeparator_do.h - b.inputArrow_do.h) / 2)), b.searchSeparator_do.setWidth(b.stageWidth), b.searchBar_do.setWidth(b.stageWidth), b.searchBar_do.setY(b.stageHeight - b.searchSeparator_do.h - b.searchBar_do.h)
				},
        this.setupDisable = function() {
					b.disable_do = new FWDMSPDisplayObject("div"),
						FWDMSPUtils.isIE && (b.disable_do.setBkColor("#FFFFFF"), b.disable_do.setAlpha(0)), b.addChild(b.disable_do)
				},
        this.showDisable = function() {
					b.disable_do && 0 == b.disable_do.w && (b.scrMainHolder_do ? b.disable_do.setWidth(b.stageWidth - b.scrollbarOffestWidth) : b.disable_do.setWidth(b.stageWidth), b.disable_do.setHeight(b.stageHeight))
				},
        this.hideDisable = function() {
					b.disable_do && 0 != b.disable_do.w && (b.disable_do.setWidth(0), b.disable_do.setHeight(0))
				},
        this.setupSeparator = function() {
					b.separator_do = new FWDMSPDisplayObject("div"),
          b.separator_do.hasTransform3d_bl = !1,
          b.separator_do.hasTransform2d_bl = !1,
          b.separator_do.getStyle().background = "url('" + b.playlistSeparator_img.src + "')",
          b.separator_do.setHeight(b.playlistSeparator_img.height),
          b.separator_do.setY(-b.separator_do.h)
				},
        this.setupScrollbar = function() {
					b.scrMainHolder_do = new FWDMSPDisplayObject("div"),
          b.scrMainHolder_do.setWidth(b.scrWidth),
          b.scrTrack_do = new FWDMSPDisplayObject("div"),
          b.scrTrack_do.setWidth(b.scrWidth),
          b.scrTrackTop_do = new FWDMSPDisplayObject("img"),
          b.scrTrackTop_do.setScreen(b.playlistScrBkTop_img),
          b.scrTrackMiddle_do = new FWDMSPDisplayObject("div"),
          b.scrTrackMiddle_do.getStyle().background = "url('" + p.scrBkMiddlePath_str + "')",
          b.scrTrackMiddle_do.setWidth(b.scrWidth),
          b.scrTrackMiddle_do.setY(b.scrTrackTop_do.h);
					var e = new Image;
					e.src = p.scrBkBottomPath_str,
          b.scrTrackBottom_do = new FWDMSPDisplayObject("img"),
          b.scrTrackBottom_do.setScreen(e),
          b.scrTrackBottom_do.setWidth(b.scrTrackTop_do.w),
          b.scrTrackBottom_do.setHeight(b.scrTrackTop_do.h),
          b.scrHandler_do = new FWDMSPDisplayObject("div"),
          b.scrHandler_do.setWidth(b.scrWidth),
          b.scrHandlerTop_do = new FWDMSPDisplayObject("img"),
          b.useHEXColorsForSkin_bl ? (b.scrHandlerTop_do = new FWDMSPDisplayObject("div"),
          b.scrHandlerTop_do.setWidth(b.playlistScrDragTop_img.width),
          b.scrHandlerTop_do.setHeight(b.playlistScrDragTop_img.height),
          b.mainScrubberDragTop_canvas = FWDMSPUtils.getCanvasWithModifiedColor(b.playlistScrDragTop_img, b.normalButtonsColor_str).canvas,
          b.scrHandlerTop_do.screen.appendChild(b.mainScrubberDragTop_canvas)) : (b.scrHandlerTop_do = new FWDMSPDisplayObject("img"), b.scrHandlerTop_do.setScreen(b.playlistScrDragTop_img)),
          b.scrHandlerMiddle_do = new FWDMSPDisplayObject("div"),
          b.middleImage = new Image,
          b.middleImage.src = p.scrDragMiddlePath_str,
          b.useHEXColorsForSkin_bl ? b.middleImage.onload = function() {
							b.scrubberDragMiddle_canvas = FWDMSPUtils.getCanvasWithModifiedColor(b.middleImage, b.normalButtonsColor_str, !0),
							b.scrubberDragImage_img = b.scrubberDragMiddle_canvas.image,
							b.scrHandlerMiddle_do.getStyle().background = "url('" + b.scrubberDragImage_img.src + "') repeat-y"
						}
            : b.scrHandlerMiddle_do.getStyle().background = "url('" + p.scrDragMiddlePath_str + "')",
            b.scrHandlerMiddle_do.setWidth(b.scrWidth),
						b.scrHandlerMiddle_do.setY(b.scrHandlerTop_do.h),
            b.scrHandlerBottom_do = new FWDMSPDisplayObject("div"),
            b.scrHandlerBottom_img = new Image,
            b.scrHandlerBottom_img.src = p.scrDragMiddlePath_str,
            b.useHEXColorsForSkin_bl ? b.scrHandlerBottom_img.onload = function() {
							b.scrubberDragBottom_canvas = FWDMSPUtils.getCanvasWithModifiedColor(b.scrHandlerBottom_img, b.normalButtonsColor_str, !0),
							b.scrubberDragBottomImage_img = b.scrubberDragBottom_canvas.image,
							b.scrHandlerBottom_do.getStyle().background = "url('" + b.scrubberDragBottomImage_img.src + "') repeat-y"
						}
            : b.scrHandlerBottom_do.getStyle().background = "url('" + p.scrDragBottomPath_str + "')",
            b.scrHandlerBottom_do.setWidth(b.scrWidth),
						b.scrHandlerBottom_do.setWidth(b.scrHandlerTop_do.w),
            b.scrHandlerBottom_do.setHeight(b.scrHandlerTop_do.h),
            b.scrHandler_do.setButtonMode(!0),
            b.useHEXColorsForSkin_bl ? (b.scrHandlerLinesN_do = new FWDMSPDisplayObject("div"), b.scrHandlerLinesN_do.setWidth(b.playlistScrLines_img.width), b.scrHandlerLinesN_do.setHeight(b.playlistScrLines_img.height), b.mainhandlerN_canvas = FWDMSPUtils.getCanvasWithModifiedColor(b.playlistScrLines_img, b.selectedButtonsColor_str).canvas, b.scrHandlerLinesN_do.screen.appendChild(b.mainhandlerN_canvas)) : (b.scrHandlerLinesN_do = new FWDMSPDisplayObject("img"), b.scrHandlerLinesN_do.setScreen(b.playlistScrLines_img)),
            b.scrHandlerLinesS_img = new Image,
            b.scrHandlerLinesS_img.src = p.scrLinesSPath_str,
            b.useHEXColorsForSkin_bl ? (b.scrHandlerLinesS_do = new FWDMSPDisplayObject("div"), b.scrHandlerLinesS_img.onload = function() {
						b.scrHandlerLinesS_do.setWidth(b.scrHandlerLinesN_do.w),
            b.scrHandlerLinesS_do.setHeight(b.scrHandlerLinesN_do.h),
            b.scrubberLines_s_canvas = FWDMSPUtils.getCanvasWithModifiedColor(b.scrHandlerLinesS_img, b.selectedButtonsColor_str, !0),
            b.scrubbelinesSImage_img = b.scrubberLines_s_canvas.image,
						b.scrHandlerLinesS_do.getStyle().background = "url('" + b.scrubbelinesSImage_img.src + "') repeat-y"
						}
          )
          : (b.scrHandlerLinesS_do = new FWDMSPDisplayObject("img"), b.scrHandlerLinesS_do.setScreen(b.scrHandlerLinesS_img), b.scrHandlerLinesS_do.setWidth(b.scrHandlerLinesN_do.w), b.scrHandlerLinesS_do.setHeight(b.scrHandlerLinesN_do.h)),
          b.scrHandlerLinesS_do.setAlpha(0),
          b.scrHandlerLines_do = new FWDMSPDisplayObject("div"),
          b.scrHandlerLines_do.hasTransform3d_bl = !1,
          b.scrHandlerLines_do.hasTransform2d_bl = !1,
          b.scrHandlerLines_do.setWidth(b.scrHandlerLinesN_do.w),
          b.scrHandlerLines_do.setHeight(b.scrHandlerLinesN_do.h),
          b.scrHandlerLines_do.setButtonMode(!0),
          b.scrTrack_do.addChild(b.scrTrackTop_do),
          b.scrTrack_do.addChild(b.scrTrackMiddle_do),
          b.scrTrack_do.addChild(b.scrTrackBottom_do),
          b.scrHandler_do.addChild(b.scrHandlerTop_do),
          b.scrHandler_do.addChild(b.scrHandlerMiddle_do),
          b.scrHandler_do.addChild(b.scrHandlerBottom_do),
          b.scrHandlerLines_do.addChild(b.scrHandlerLinesN_do),
          b.scrHandlerLines_do.addChild(b.scrHandlerLinesS_do),
          b.scrMainHolder_do.addChild(b.scrTrack_do),
					b.scrMainHolder_do.addChild(b.scrHandler_do),
          b.scrMainHolder_do.addChild(b.scrHandlerLines_do),
          b.mainHolder_do.addChild(b.scrMainHolder_do),
          b.scrHandler_do.screen.addEventListener ? (b.scrHandler_do.screen.addEventListener("mouseover", b.scrollBarHandlerOnMouseOver), b.scrHandler_do.screen.addEventListener("mouseout", b.scrollBarHandlerOnMouseOut), b.scrHandler_do.screen.addEventListener("mousedown", b.scrollBarHandlerOnMouseDown), b.scrHandlerLines_do.screen.addEventListener("mouseover", b.scrollBarHandlerOnMouseOver), b.scrHandlerLines_do.screen.addEventListener("mouseout", b.scrollBarHandlerOnMouseOut), b.scrHandlerLines_do.screen.addEventListener("mousedown", b.scrollBarHandlerOnMouseDown))
          : b.scrHandler_do.screen.attachEvent && (b.scrHandler_do.screen.attachEvent("onmouseover", b.scrollBarHandlerOnMouseOver), b.scrHandler_do.screen.attachEvent("onmouseout", b.scrollBarHandlerOnMouseOut), b.scrHandler_do.screen.attachEvent("onmousedown", b.scrollBarHandlerOnMouseDown), b.scrHandlerLines_do.screen.attachEvent("onmouseover", b.scrollBarHandlerOnMouseOver), b.scrHandlerLines_do.screen.attachEvent("onmouseout", b.scrollBarHandlerOnMouseOut), b.scrHandlerLines_do.screen.attachEvent("onmousedown", b.scrollBarHandlerOnMouseDown))
				},
        this.scrollBarHandlerOnMouseOver = function(e) {
					FWDAnimation.to(b.scrHandlerLinesS_do, .8, {
						alpha: 1,
						ease: Expo.easeOut
					})
				},
        this.scrollBarHandlerOnMouseOut = function(e) {
					b.isDragging_bl || FWDAnimation.to(b.scrHandlerLinesS_do, .8, {
						alpha: 0,
						ease: Expo.easeOut
					})
				},
        this.scrollBarHandlerOnMouseDown = function(e) {
					if (b.allowToScrollAndScrollBarIsActive_bl) {
						var t = FWDMSPUtils.getViewportMouseCoordinates(e);
						b.isDragging_bl = !0, b.yPositionOnPress = b.scrHandler_do.y, b.lastPresedY = t.screenY, FWDAnimation.killTweensOf(b.scrHandler_do),
							b.showDisable(),
							window.addEventListener ? (window.addEventListener("mousemove", b.scrollBarHandlerMoveHandler), window.addEventListener("mouseup", b.scrollBarHandlerEndHandler)) : document.attachEvent && (document.attachEvent("onmousemove", b.scrollBarHandlerMoveHandler), document.attachEvent("onmouseup", b.scrollBarHandlerEndHandler)), b.prevSortId = -1
					}
				}, this.scrollBarHandlerMoveHandler = function(e) {
					e.preventDefault && e.preventDefault();
					var t = FWDMSPUtils.getViewportMouseCoordinates(e);
					b.scrollBarHandlerFinalY = Math.round(b.yPositionOnPress + t.screenY - b.lastPresedY), b.scrollBarHandlerFinalY >= b.scrTrack_do.h - b.scrHandler_do.h - 1 ? b.scrollBarHandlerFinalY = b.scrTrack_do.h - b.scrHandler_do.h - 1 : b.scrollBarHandlerFinalY <= 0 && (b.scrollBarHandlerFinalY = 0), b.scrHandler_do.setY(b.scrollBarHandlerFinalY), FWDAnimation.to(b.scrHandlerLines_do, .8, {
						y: b.scrollBarHandlerFinalY + parseInt((b.scrHandler_do.h - b.scrHandlerLines_do.h) / 2),
						ease: Quart.easeOut
					}), b.updateScrollBarHandlerAndContent(!0, !0)
				},
        b.scrollBarHandlerEndHandler = function(e) {
					var t = FWDMSPUtils.getViewportMouseCoordinates(e);
					b.isDragging_bl = !1, FWDMSPUtils.hitTest(b.scrHandler_do.screen, t.screenX, t.screenY) || FWDAnimation.to(b.scrHandlerLinesS_do, .8, {
							alpha: 0,
							ease: Expo.easeOut
						}), b.scrollBarHandlerFinalY = -1 * parseInt((b.scrTrack_do.h - b.scrHandler_do.h) * (b.playListFinalY / ((b.totalSearchedItems - b.nrOfVisiblePlaylistItems) * b.itemHeight))), b.scrollBarHandlerFinalY.y < 0 ? b.scrollBarHandlerFinalY = 0 : b.scrollBarHandlerFinalY > b.scrTrack_do.h - b.scrHandler_do.h - 1 && (b.scrollBarHandlerFinalY = b.scrTrack_do.h - b.scrHandler_do.h - 1), b.hideDisable(), FWDAnimation.killTweensOf(b.scrHandler_do),
						FWDAnimation.to(b.scrHandler_do, .5, {
							y: b.scrollBarHandlerFinalY,
							ease: Quart.easeOut
						}), window.removeEventListener ? (window.removeEventListener("mousemove", b.scrollBarHandlerMoveHandler), window.removeEventListener("mouseup", b.scrollBarHandlerEndHandler)) : document.detachEvent && (document.detachEvent("onmousemove", b.scrollBarHandlerMoveHandler), document.detachEvent("onmouseup", b.scrollBarHandlerEndHandler))
				},
        this.updateScrollBarSizeActiveAndDeactivate = function() {
					if (b.allowToScrollAndScrollBarIsActive_bl) {
						var e = 0;
						b.allowToScrollAndScrollBarIsActive_bl = !0,
						b.searchBar_do && (e = b.searchBar_do.h),
						b.scrMainHolder_do.setHeight(b.stageHeight - b.separator_do.h - e),
						b.scrTrack_do.setHeight(b.stageHeight - b.separator_do.h - e),
						b.scrTrackMiddle_do.setHeight(b.scrTrack_do.h - 2 * b.scrTrackTop_do.h),
						b.scrTrackBottom_do.setY(b.scrTrackMiddle_do.y + b.scrTrackMiddle_do.h),
						b.scrHandler_do.setHeight(Math.min(b.stageHeight - b.separator_do.h - e, Math.round((b.stageHeight - b.separator_do.h - e) / b.itemsTotalHeight * b.stageHeight))),
						b.scrHandlerMiddle_do.setHeight(b.scrHandler_do.h - 2 * b.scrHandlerTop_do.h),
						b.scrHandlerTop_do.setY(b.scrHandlerMiddle_do.y + b.scrHandlerMiddle_do.h),
						b.scrHandlerLines_do.setY(b.scrollBarHandlerFinalY + parseInt((b.scrHandler_do.h - b.scrHandlerLines_do.h) / 2)),
						b.scrMainHolder_do.setX(b.stageWidth - b.scrWidth),
						b.updateScrollBarHandlerAndContent()
					}
					else b.allowToScrollAndScrollBarIsActive_bl = !1,
					b.scrMainHolder_do.setX(-500),
					b.scrHandler_do.setY(0)
				},
        this.updateScrollBarHandlerAndContent = function(e, t) {
					if (b.curItem_do && b.allowToScrollAndScrollBarIsActive_bl && (b.curItem_do && (b.sortId = b.curItem_do.sortId), b.prevSortId != b.sortId || t)) {
						var o = 0, s = 0;
						b.addAtThePlaylistEnd_bl ? b.sortId = b.totalPlayListItems - 1 : b.addAtThePlaylistBeggingin_bl && (b.sortId = 0),
						b.prevSortId = b.sortId,
						b.isDragging_bl && !b.isMobile_bl ? ("Infinity" == (o = b.scrHandler_do.y / (b.scrMainHolder_do.h - b.scrHandler_do.h)) ? o = 0 : 1 <= o && (scrollPercent = 1),
						b.playListFinalY = Math.round(o * (b.totalSearchedItems - b.nrOfVisiblePlaylistItems)) * b.itemHeight * -1) : ((s = b.totalSearchedItems != b.totalPlayListItems ? 0 : parseInt(b.sortId / b.nrOfVisiblePlaylistItems) * b.nrOfVisiblePlaylistItems) + b.nrOfVisiblePlaylistItems >= b.totalPlayListItems && (s = b.totalPlayListItems - b.nrOfVisiblePlaylistItems), s < 0 && (s = 0), b.playListFinalY = parseInt(s * b.itemHeight * -1), b.scrMainHolder_do && (b.scrollBarHandlerFinalY = -1 * Math.round((b.scrMainHolder_do.h - b.scrHandler_do.h) * (b.playListFinalY / ((b.totalSearchedItems - b.nrOfVisiblePlaylistItems) * b.itemHeight))), b.scrollBarHandlerFinalY < 0 ? b.scrollBarHandlerFinalY = 0 : b.scrollBarHandlerFinalY > b.scrMainHolder_do.h - b.scrHandler_do.h - 1 && (b.scrollBarHandlerFinalY = b.scrMainHolder_do.h - b.scrHandler_do.h - 1), FWDAnimation.killTweensOf(b.scrHandler_do), FWDAnimation.killTweensOf(b.scrHandlerLines_do), e ? (FWDAnimation.to(b.scrHandler_do, .5, {
							y: b.scrollBarHandlerFinalY,
							ease: Quart.easeOut
						}), FWDAnimation.to(b.scrHandlerLines_do, .8, {
							y: b.scrollBarHandlerFinalY + parseInt((b.scrHandler_do.h - b.scrHandlerLinesN_do.h) / 2),
							ease: Quart.easeOut
						})) : (b.scrHandler_do.setY(b.scrollBarHandlerFinalY), b.scrHandlerLines_do.setY(b.scrollBarHandlerFinalY + parseInt((b.scrHandler_do.h - b.scrHandlerLinesN_do.h) / 2))))), b.prevPlaylistY != b.playListFinalY && (b.prevPlaylistY = b.playListFinalY, isNaN(b.playListFinalY) || (b.lastListY != b.playListFinalY && (FWDAnimation.killTweensOf(b.itemsHolder_do), e ? FWDAnimation.to(b.itemsHolder_do, .5, {
							y: b.playListFinalY,
							ease: Quart.easeOut
						}) : b.itemsHolder_do.setY(b.playListFinalY)), b.lastListY = b.playListFinalY))
					}
				},
        this.addMouseWheelSupport = function() {
					window.addEventListener ? (b.screen.addEventListener("mousewheel", b.mouseWheelHandler), b.screen.addEventListener("DOMMouseScroll", b.mouseWheelHandler)) : document.attachEvent && b.screen.attachEvent("onmousewheel", b.mouseWheelHandler)
				},
        this.mouseWheelHandler = function(e) {
					if (b.allowToScrollAndScrollBarIsActive_bl && !b.isDragging_bl && (!b.comboBox_do || !b.comboBox_do.isShowed_bl)) {
						var t = e.detail || e.wheelDelta;
						if (e.wheelDelta && (t *= -1), FWDMSPUtils.isOpera && (t *= -1), 0 < t ? b.playListFinalY -= b.itemHeight : b.playListFinalY += b.itemHeight, leftId = parseInt(b.playListFinalY / b.itemHeight), 0 <= leftId ? leftId = 0 : Math.abs(leftId) + b.nrOfVisiblePlaylistItems >= b.totalSearchedItems && (leftId = -1 * (b.totalSearchedItems - b.nrOfVisiblePlaylistItems)), b.prevSortId = -1, b.prevPlaylistY = -100, b.playListFinalY = leftId * b.itemHeight, b.lastListY != b.playListFinalY) {
							if (b.scrollBarHandlerFinalY = -1 * Math.round((b.scrMainHolder_do.h - b.scrHandler_do.h) * (b.playListFinalY / ((b.totalSearchedItems - b.nrOfVisiblePlaylistItems) * b.itemHeight))), b.scrollBarHandlerFinalY < 0 ? b.scrollBarHandlerFinalY = 0 : b.scrollBarHandlerFinalY > b.scrMainHolder_do.h - b.scrHandler_do.h - 1 && (b.scrollBarHandlerFinalY = b.scrMainHolder_do.h - b.scrHandler_do.h - 1), FWDAnimation.killTweensOf(b.itemsHolder_do), FWDAnimation.to(b.itemsHolder_do, .5, {
									y: b.playListFinalY,
									ease: Expo.easeOut
								}), FWDAnimation.killTweensOf(b.scrHandler_do), FWDAnimation.to(b.scrHandler_do, .5, {
									y: b.scrollBarHandlerFinalY,
									ease: Expo.easeOut
								}), FWDAnimation.to(b.scrHandlerLines_do, .8, {
									y: b.scrollBarHandlerFinalY + parseInt((b.scrHandler_do.h - b.scrHandlerLinesN_do.h) / 2),
									ease: Quart.easeOut
								}), b.lastListY = b.playListFinalY, !e.preventDefault) return !1;
							e.preventDefault()
						}
					}
				},
        b.setupMobileScrollbar = function() {
					b.hasPointerEvent_bl ? b.screen.addEventListener("pointerdown", b.scrollBarTouchStartHandler) : b.screen.addEventListener("touchstart", b.scrollBarTouchStartHandler), b.updateMobileScrollBarId_int = setInterval(b.updateMobileScrollBar, 16)
				},
        b.scrollBarTouchStartHandler = function(e) {
					if (!(b.stageHeight > b.itemsTotalHeight || b.comboBox_do && b.comboBox_do.isShowed_bl)) {
						e.preventDefault && e.preventDefault(),
							FWDAnimation.killTweensOf(b.itemsHolder_do);
						var t = FWDMSPUtils.getViewportMouseCoordinates(e);
						b.isDragging_bl = !0, b.isScrollingOnMove_bl = !1, b.lastPresedY = t.screenY, b.checkLastPresedY = t.screenY, b.hasPointerEvent_bl ? (window.addEventListener("pointerup", b.scrollBarTouchEndHandler), window.addEventListener("pointermove", b.scrollBarTouchMoveHandler)) : (window.addEventListener("touchend", b.scrollBarTouchEndHandler), window.addEventListener("touchmove", b.scrollBarTouchMoveHandler)),
							clearInterval(b.updateMoveMobileScrollbarId_int),
							b.updateMoveMobileScrollbarId_int = setInterval(b.updateMoveMobileScrollbar, 20)
					}
				},
        b.scrollBarTouchMoveHandler = function(e) {
					e.preventDefault && e.preventDefault(), b.showDisable();
					var t = FWDMSPUtils.getViewportMouseCoordinates(e);
					(t.screenY >= b.checkLastPresedY + 6 || t.screenY <= b.checkLastPresedY - 6) && (b.isScrollingOnMove_bl = !0);
					var o = t.screenY - b.lastPresedY;
					b.playListFinalY += o, b.playListFinalY = Math.round(b.playListFinalY), b.lastPresedY = t.screenY, b.vy = 2 * o
				},
        b.scrollBarTouchEndHandler = function(e) {
					b.isDragging_bl = !1, clearInterval(b.updateMoveMobileScrollbarId_int),
						clearTimeout(b.disableOnMoveId_to), b.disableOnMoveId_to = setTimeout(function() {
							b.hideDisable()
						}, 50), b.hasPointerEvent_bl ? (window.removeEventListener("pointerup", b.scrollBarTouchEndHandler), window.removeEventListener("pointermove", b.scrollBarTouchMoveHandler)) : (window.removeEventListener("touchend", b.scrollBarTouchEndHandler), window.removeEventListener("touchmove", b.scrollBarTouchMoveHandler))
				},
        b.updateMoveMobileScrollbar = function() {
					b.itemsHolder_do.setY(b.playListFinalY)
				},
         b.updateMobileScrollBar = function(e) {
					b.isDragging_bl || FWDAnimation.isTweening(b.itemsHolder_do) || (b.vy *= b.friction, b.playListFinalY += b.vy, 0 < b.playListFinalY ? (b.vy2 = .3 * (0 - b.playListFinalY), b.vy *= b.friction, b.playListFinalY += b.vy2) : b.playListFinalY < b.stageHeight - b.separator_do.h - b.itemsTotalHeight - b.searchBar_do.h && (b.vy2 = .3 * (b.stageHeight - b.separator_do.h - b.itemsTotalHeight - b.searchBar_do.h - b.playListFinalY), b.vy *= b.friction, b.playListFinalY += b.vy2), b.stageHeight >
          b.itemsTotalHeight && (b.playListFinalY = 0), b.itemsHolder_do.setY(Math.round(b.playListFinalY)))
				},
        this.hide = function() {
					b.isShowed_bl = !1
				},
        this.show = function(e) {
					e && (b.isShowed_bl = !0), b.setX(0)
				},
        b.updateHEXColors = function(e, t) {
					b.normalColor_str = e,
					b.selectedColor_str = t,
					b.sortNButton_do && b.sortNButton_do.updateHEXColors(e, t),
					b.sortAButton_do && b.sortAButton_do.updateHEXColors(e, t),
					b.ascDscButton_do && b.ascDscButton_do.updateHEXColors(e, t),
					FWDMSPUtils.changeCanvasHEXColor(b.inputArrow_img, b.mainScrubberDragLeft_canvas, e);
					for (var o = 0; o < b.items_ar.length; o++) b.items_ar[o].updateHEXColors(e, t)
				},
        this.init()
		};
		n.setPrototype = function() {
				n.prototype = new FWDMSPDisplayObject("div")
			},
      n.CHANGE_PLAYLIST = "changePlaylist",
      n.PLAY = "play",
			n.PAUSE = "pause",
      n.UPDATE_TRACK_TITLE_if_FOLDER = "update_trak_title",
      n.prototype = null,
      window.FWDMSPPlaylist = n
	}(),
	function() {
		var k = function(e, t, o, s, i, n, l, r, a, d, u, c, h, _, f, p, m, b, g, S, y, v, P, T, w, D, B, M, F, W, H, C, E) {
			var O = this;
			k.prototype;
			this.playlistItemGrad1_img = l,
      this.playlistItemGrad2_img = r,
      this.playlistItemProgress_img = a,
      this.playlistItemProgress2_img = d,
      this.playlistPlayButtonN_img = u,
      this.playlistDownloadButtonN_img = o,
      this.playlistDownloadButtonS_str = s,
      this.playlistBuyButtonN_img = i,
			this.playlistBuyButtonS_str = n,
      this.progress_do = null,
      this.playPause_do = null,
      this.playN_do = null,
      this.playS_do = null,
      this.pauseN_do = null,
			this.pauseS_do = null,
      this.titleText_do = null,
			this.grad_do = null,
      this.durationText_do = null,
			this.dumy_do = null,
      this.title_str = e,
      this.titleText_str = t,
      O.useHEXColorsForSkin_bl = W,
			O.normalButtonsColor_str = H,
      O.selectedButtonsColor_str = C,
      this.playlistItemBk1Path_str = c,
      this.playlistItemBk2Path_str = h,
      this.playlistPlayButtonN_str = _,
      this.playlistPlayButtonS_str = f,
      this.playlistPauseButtonN_str = p,
      this.playlistPauseButtonS_str = m,
      this.titleNormalColor_str = b,
      this.trackTitleSelected_str = g,
      this.durationColor_str = S,
      this.itemHeight = O.playlistItemGrad1_img.height,
			this.id = y,
      this.sortId = y,
      this.playPauseButtonOffsetLeftAndRight = v,
      this.trackTitleOffsetLeft = P,
      this.duration = F,
      this.durationOffsetRight = T,
      this.textHeight,
      this.durationWidth = 0,
      this.titleWidth = 0,
      this.playPauseButtonWidth = O.playlistPlayButtonN_img.width,
      this.playPauseButtonHeight = O.playlistPlayButtonN_img.height,
      this.progressPercent = 0,
      this.stageWidth = 0,
      this.downloadButtonOffsetRight = w,
      this.type = -1,
			this.setTextsSizeId_to,
      this.showDownloadButton_bl = B,
      this.showBuyButton_bl = M,
      this.showPlayPauseButton_bl = D,
      this.showDuration_bl = F,
      this.isActive_bl = !1,
      this.isSelected_bl = !1,
      this.isMobile_bl = FWDMSPUtils.isMobile,
      this.hasPointerEvent_bl = FWDMSPUtils.hasPointerEvent,
      O.init = function() {
					O.setupProgress(),
          O.setupTitle(),
          O.showPlayPauseButton_bl && O.setupPlayPauseButton(),
          O.setupGrad(),
          O.showDuration_bl && O.setupDuration(),
          O.setNormalState(!1, !0),
          O.setupDumy(),
          O.showDownloadButton_bl && O.setupDownloadButton(),
          O.showBuyButton_bl && O.setupBuyButton(),
          O.id % 2 == 0 ? (O.getStyle().background = "url('" + O.playlistItemBk1Path_str + "')",
					                 O.grad_do.getStyle().background = "url('" + O.playlistItemGrad1_img.src + "')",
													 O.progress_do.getStyle().background = "url('" + O.playlistItemProgress_img.src + "')",
													 O.type = 1)
                        : (O.getStyle().background = "url('" + O.playlistItemBk2Path_str + "')",
												   O.grad_do.getStyle().background = "url('" + O.playlistItemGrad2_img.src + "')",
													 O.progress_do.getStyle().background = "url('" + O.playlistItemProgress2_img.src + "')",
													 O.type = 2),
          O.isMobile_bl ? O.hasPointerEvent_bl ? (O.dumy_do.screen.addEventListener("pointerup", O.onMouseUp),
					                                        O.dumy_do.screen.addEventListener("pointerover", O.onMouseOver),
																									O.dumy_do.screen.addEventListener("pointerout", O.onMouseOut))
																							: O.dumy_do.screen.addEventListener("touchend", O.onMouseUp)
																							: O.dumy_do.screen.addEventListener
																							? (O.dumy_do.screen.addEventListener("mouseover", O.onMouseOver),
	        O.dumy_do.screen.addEventListener("mouseout", O.onMouseOut),
					O.dumy_do.screen.addEventListener("mouseup", O.onMouseUp)) : O.screen.attachEvent && (O.dumy_do.screen.attachEvent("onmouseover", O.onMouseOver),
																																																O.dumy_do.screen.attachEvent("onmouseout", O.onMouseOut),
																																																O.dumy_do.screen.attachEvent("onmouseup", O.onMouseUp))
				},
        O.onMouseOver = function(e, t) {
					O.isActive_bl || e.pointerType && "mouse" != e.pointerType || O.setSelectedState(!0)
				},
        O.onMouseOut = function(e) {
					O.isActive_bl || e.pointerType && "mouse" != e.pointerType || O.setNormalState(!0)
				},
        O.onMouseUp = function(e) {
					E.isScrollingOnMove_bl || 2 == e.button || (e.preventDefault && e.preventDefault(), O.dispatchEvent(k.MOUSE_UP, {
						id: O.id
					}))
				},
        O.changeSource = function(e) {
					0 == e ? 1 != O.type && (O.grad_do.getStyle().background = "url('" + O.playlistItemGrad1_img.src + "')", O.getStyle().background = "url('" + O.playlistItemBk1Path_str + "')", O.progress_do.getStyle().background = "url('" + O.playlistItemProgress_img.src + "')", O.type = 1) : 2 != O.type && (O.grad_do.getStyle().background = "url('" + O.playlistItemGrad2_img.src + "')", O.getStyle().background = "url('" + O.playlistItemBk2Path_str + "')", O.progress_do.getStyle().background = "url('" + O.playlistItemProgress2_img.src + "')", O.type = 2)
				},
        O.resize = function(e, t) {
					if ((!FWDMSPUtils.isIEAndLessThen9 || O.textHeight) && null != O) {
						O.stageWidth = e;
						var o = 0,
							s = parseInt((t - O.textHeight) / 2) + 1;
						O.playPause_do ? (O.titleText_do.setX(2 * O.playPauseButtonOffsetLeftAndRight + O.playPause_do.w + O.trackTitleOffsetLeft - 2), O.playPause_do.setY(parseInt((t - O.playPause_do.h) / 2))) : O.titleText_do.setX(O.trackTitleOffsetLeft), O.titleText_do.setY(s), O.buyButton_do && O.downloadButton_do ? (o = O.durationText_do ? (O.durationText_do.setX(e - O.durationWidth - O.durationOffsetRight + 1), O.durationText_do.setY(s), O.durationText_do.x) : e, O.downloadButton_do.setX(o - O.downloadButton_do.w - O.downloadButtonOffsetRight + 3), O.downloadButton_do.setY(parseInt((t - O.downloadButton_do.h) / 2)), O.buyButton_do.setX(O.downloadButton_do.x - O.buyButton_do.w - 4), O.buyButton_do.setY(parseInt((t - O.buyButton_do.h) / 2)), O.titleText_do.x + O.titleWidth + O.downloadButton_do.w + O.buyButton_do.w + O.downloadButtonOffsetRight + 4 > o ? O.grad_do.setX(O.buyButton_do.x - O.downloadButtonOffsetRight + 2) : O.grad_do.setX(-300)) : O.downloadButton_do ? (o = O.durationText_do ? (O.durationText_do.setX(e - O.durationWidth - O.durationOffsetRight + 1), O.durationText_do.setY(s), O.durationText_do.x) : e, O.downloadButton_do.setX(o - O.downloadButton_do.w - O.downloadButtonOffsetRight + 3), O.downloadButton_do.setY(parseInt((t - O.downloadButton_do.h) / 2)), O.titleText_do.x + O.titleWidth + O.downloadButton_do.w + O.downloadButtonOffsetRight > o ? O.grad_do.setX(O.downloadButton_do.x - O.downloadButtonOffsetRight + 2) : O.grad_do.setX(-300)) : O.buyButton_do ? (o = O.durationText_do ? (O.durationText_do.setX(e - O.durationWidth - O.durationOffsetRight + 1), O.durationText_do.setY(s), O.durationText_do.x) : e, O.buyButton_do.setX(o - O.buyButton_do.w - O.downloadButtonOffsetRight + 3), O.buyButton_do.setY(parseInt((t - O.buyButton_do.h) / 2)), O.titleText_do.x + O.titleWidth + O.buyButton_do.w + O.downloadButtonOffsetRight > o ? O.grad_do.setX(O.buyButton_do.x - O.downloadButtonOffsetRight + 2) : O.grad_do.setX(-300)) : O.durationText_do ? (O.durationText_do.setX(e - O.durationWidth - O.durationOffsetRight + 1), O.durationText_do.setY(s), O.titleText_do.x + O.titleWidth > O.durationText_do.x ? O.grad_do.setX(O.durationText_do.x - O.durationOffsetRight + 2) : O.grad_do.setX(-300)) : O.downloadButton_do ? (O.downloadButton_do.setX(e - O.downloadButton_do.w - O.downloadButtonOffsetRight + 2), O.titleText_do.x + O.titleWidth > O.downloadButton_do.x ? O.grad_do.setX(O.downloadButton_do.x - O.downloadButtonOffsetRight + 2) : O.grad_do.setX(-300), O.downloadButton_do.setY(parseInt((t - O.downloadButton_do.h) / 2))) : O.titleText_do.x + O.titleWidth > e - 10 ? O.grad_do.setX(e - 15) : O.grad_do.setX(-300),
							O.dumy_do.setWidth(e), O.dumy_do.setHeight(t), O.setWidth(e), O.setHeight(t)
					}
				},
        this.setupDownloadButton = function() {
					FWDMSPSimpleSizeButton.setPrototype(),
					O.downloadButton_do = new FWDMSPSimpleSizeButton(O.playlistDownloadButtonS_str,
						                                               O.playlistDownloadButtonN_img.src,
																													 O.playlistDownloadButtonN_img.width,
																													 O.playlistDownloadButtonN_img.height,
																													 O.useHEXColorsForSkin_bl,
																													 O.normalButtonsColor_str,
																													 O.selectedButtonsColor_str),
					O.downloadButton_do.getStyle().position = "absolute",
					O.downloadButton_do.addListener(FWDMSPSimpleSizeButton.CLICK,
						                              O.dwButtonClickHandler),
																					O.addChild(O.downloadButton_do)
				},
        this.dwButtonClickHandler = function() {
					O.dispatchEvent(k.DOWNLOAD, {
						id: O.id
					})
				},
        this.setupBuyButton = function() {
					FWDMSPSimpleSizeButton.setPrototype(),
					O.buyButton_do = new FWDMSPSimpleSizeButton(O.playlistBuyButtonS_str,
						                                          O.playlistBuyButtonN_img.src,
																											O.playlistBuyButtonN_img.width,
																											O.playlistBuyButtonN_img.height,
																											O.useHEXColorsForSkin_bl,
																											O.normalButtonsColor_str,
																											O.selectedButtonsColor_str),
					O.buyButton_do.getStyle().position = "absolute",
					O.buyButton_do.addListener(FWDMSPSimpleSizeButton.CLICK, O.buyButtonClickHandler),
					O.addChild(O.buyButton_do)
				},
        this.buyButtonClickHandler = function() {
					O.dispatchEvent(k.BUY, {
						id: O.id
					})
				},
        this.setupProgress = function() {
					O.progress_do = new FWDMSPDisplayObject("div"),
					O.progress_do.setHeight(a.height),
					O.addChild(O.progress_do)
				},
        this.updateProgressPercent = function(e) {
					null != O && O.progressPercent != e && (O.progressPercent = e, O.progress_do.setWidth(parseInt(O.stageWidth * e)))
				},
        this.setupPlayPauseButton = function() {
					O.playPause_do = new FWDMSPDisplayObject("div"),
          O.playPause_do.setWidth(O.playPauseButtonWidth),
          O.playPause_do.setHeight(O.playPauseButtonHeight),
          O.playN_do = new FWDMSPDisplayObject("div"),
          O.useHEXColorsForSkin_bl ? (O.playNImage_img = new Image,
						                          O.playNImage_img.src = O.playlistPlayButtonN_str,
																			O.playNImage_img.onload = function() {
																				var e = FWDMSPUtils.getCanvasWithModifiedColor(O.playNImage_img, O.normalButtonsColor_str, !0);
																				O.playNImageCanvas = e.canvas, O.playNImageBackground = e.image, O.playN_do.getStyle().background = "url('" + O.playNImageBackground.src + "')"
																			})
          													: O.playN_do.getStyle().background = "url('" + O.playlistPlayButtonN_str + "') no-repeat",
					O.playN_do.setWidth(O.playPauseButtonWidth),
          O.playN_do.setHeight(O.playPauseButtonHeight),
          O.playS_do = new FWDMSPDisplayObject("div"),
          O.useHEXColorsForSkin_bl ? (O.playSImage_img = new Image,
						                          O.playSImage_img.src = O.playlistPlayButtonS_str,
																			O.playSImage_img.onload = function() {
																				var e = FWDMSPUtils.getCanvasWithModifiedColor(O.playSImage_img, O.selectedButtonsColor_str, !0);
																				O.playSImageCanvas = e.canvas, O.playSImageBackground = e.image, O.playS_do.getStyle().background = "url('" + O.playSImageBackground.src + "')"
																			})
																		: O.playS_do.getStyle().background = "url('" + O.playlistPlayButtonS_str + "') no-repeat",
					O.playS_do.setWidth(O.playPauseButtonWidth),
					O.playS_do.setHeight(O.playPauseButtonHeight),
					O.playS_do.setAlpha(0),
					O.pauseN_do = new FWDMSPDisplayObject("div"),
					O.useHEXColorsForSkin_bl ? (O.pauseNImage_img = new Image,
						                          O.pauseNImage_img.src = O.playlistPauseButtonN_str,
																			O.pauseNImage_img.onload = function() {
																				var e = FWDMSPUtils.getCanvasWithModifiedColor(O.pauseNImage_img, O.normalButtonsColor_str, !0);
																				O.pauseNImageCanvas = e.canvas, O.pauseNImageBackground = e.image, O.pauseN_do.getStyle().background = "url('" + O.pauseNImageBackground.src + "')"
																			})
																		: O.pauseN_do.getStyle().background = "url('" + O.playlistPauseButtonN_str + "') no-repeat",
						O.pauseN_do.setWidth(O.playPauseButtonWidth),
						O.pauseN_do.setHeight(O.playPauseButtonHeight),
						O.pauseS_do = new FWDMSPDisplayObject("div"),
						O.useHEXColorsForSkin_bl ? (O.pauseSImage_img = new Image,
							                          O.pauseSImage_img.src = O.playlistPauseButtonS_str,
																				O.pauseSImage_img.onload = function() {
																					var e = FWDMSPUtils.getCanvasWithModifiedColor(O.pauseSImage_img, O.selectedButtonsColor_str, !0);
																					O.pauseSImageCanvas = e.canvas, O.pauseSImageBackground = e.image, O.pauseS_do.getStyle().background = "url('" + O.pauseSImageBackground.src + "')"
																				})
																			: O.pauseS_do.getStyle().background = "url('" + O.playlistPauseButtonS_str + "') no-repeat",
						O.pauseS_do.setWidth(O.playPauseButtonWidth),
						O.pauseS_do.setHeight(O.playPauseButtonHeight),
						O.pauseN_do.setX(-300),
						O.pauseS_do.setX(-300),
						O.pauseS_do.setAlpha(0),
						O.playPause_do.setX(O.playPauseButtonOffsetLeftAndRight),
						O.playPause_do.addChild(O.playN_do),
						O.playPause_do.addChild(O.playS_do),
						O.playPause_do.addChild(O.pauseN_do),
						O.playPause_do.addChild(O.pauseS_do),
						O.addChild(O.playPause_do)
				},
        this.setupTitle = function() {
					O.titleText_do = new FWDMSPDisplayObject("div"),
          FWDMSPUtils.isApple && (O.titleText_do.hasTransform3d_bl = !1,
						                      O.titleText_do.hasTransform2d_bl = !1),
					O.titleText_do.setOverflow("visible"),
					O.titleText_do.getStyle().fontFamily = "Arial",
					O.titleText_do.getStyle().fontSize = "12px",
					O.titleText_do.getStyle().whiteSpace = "nowrap",
					O.titleText_do.getStyle().textAlign = "left",
					O.titleText_do.getStyle().fontSmoothing = "antialiased",
					O.titleText_do.getStyle().webkitFontSmoothing = "antialiased",
					O.titleText_do.getStyle().textRendering = "optimizeLegibility",
					O.titleText_do.setInnerHTML(O.title_str),
					O.addChild(O.titleText_do)
				},
        this.updateTitle = function() {
					null != O && O.titleText_do.setInnerHTML(O.title_str)
				},
        this.setTextSizes = function(e) {
					null != O && (O.textHeight && !e || (O.titleWidth = O.titleText_do.screen.offsetWidth, O.textHeight = O.titleText_do.screen.offsetHeight, O.durationText_do && (O.durationWidth = O.durationText_do.screen.offsetWidth), O.grad_do.setWidth(150)))
				},
        this.setupGrad = function() {
					O.grad_do = new FWDMSPDisplayObject("div"),
          O.grad_do.setOverflow("visible"),
					FWDMSPUtils.isApple && (O.grad_do.hasTransform3d_bl = !1, O.grad_do.hasTransform2d_bl = !1),
          O.grad_do.setHeight(O.itemHeight),
          O.addChild(O.grad_do)
				},
        this.setupDuration = function() {
					O.durationText_do = new FWDMSPDisplayObject("div"),
          FWDMSPUtils.isApple && (O.durationText_do.hasTransform3d_bl = !1, O.durationText_do.hasTransform2d_bl = !1),
          O.durationText_do.setOverflow("visible"),
					O.durationText_do.getStyle().fontFamily = "Arial",
          O.durationText_do.getStyle().fontSize = "12px",
          O.durationText_do.getStyle().whiteSpace = "nowrap",
          O.durationText_do.getStyle().textAlign = "left",
          O.durationText_do.getStyle().color = O.titleColor_str,
          O.durationText_do.getStyle().fontSmoothing = "antialiased",
          O.durationText_do.getStyle().webkitFontSmoothing = "antialiased",
          O.durationText_do.getStyle().textRendering = "optimizeLegibility",
          O.durationText_do.getStyle().color = O.durationColor_str,
          O.durationText_do.setInnerHTML(O.duration),
          O.addChild(O.durationText_do)
				},
        this.setupDumy = function() {
					O.dumy_do = new FWDMSPDisplayObject("div"),
          O.dumy_do.setButtonMode(!0),
          FWDMSPUtils.isIE && (O.dumy_do.setBkColor("#FFFFFF"),
          O.dumy_do.setAlpha(.001)),
          O.addChild(O.dumy_do)
				},
        this.setNormalState = function(e, t) {
					(O.isSelected_bl || t) && (O.isSelected_bl = !1, e ? (FWDAnimation.to(O.titleText_do.screen, .8, {
						css: {
							color: O.titleNormalColor_str
						},
						ease: Expo.easeOut
					}), O.durationText_do && FWDAnimation.to(O.durationText_do.screen, .8, {
						css: {
							color: O.durationColor_str
						},
						ease: Expo.easeOut
					}), O.playPause_do && (FWDAnimation.to(O.pauseS_do, .8, {
						alpha: 0,
						ease: Expo.easeOut
					}), FWDAnimation.to(O.playS_do, .8, {
						alpha: 0,
						ease: Expo.easeOut
					}))) : (FWDAnimation.killTweensOf(O.titleText_do), O.titleText_do.getStyle().color = O.titleNormalColor_str, O.durationText_do && (O.durationText_do.getStyle().color = O.durationColor_str), O.playPause_do && (FWDAnimation.killTweensOf(O.pauseS_do), FWDAnimation.killTweensOf(O.playS_do), O.pauseS_do.setAlpha(0), O.playS_do.setAlpha(0))))
				},
				this.setSelectedState = function(e) {
					O.isSelected_bl || (O.isSelected_bl = !0, e ? (FWDAnimation.to(O.titleText_do.screen, .8, {
						css: {
							color: O.trackTitleSelected_str
						},
						ease: Expo.easeOut
					}), O.durationText_do && FWDAnimation.to(O.durationText_do.screen, .8, {
						css: {
							color: O.trackTitleSelected_str
						},
						ease: Expo.easeOut
					}), O.playPause_do && (FWDAnimation.to(O.pauseS_do, .8, {
						alpha: 1,
						ease: Expo.easeOut
					}), FWDAnimation.to(O.playS_do, .8, {
						alpha: 1,
						ease: Expo.easeOut
					}))) : (FWDAnimation.killTweensOf(O.titleText_do), O.durationText_do && (O.durationText_do.getStyle().color = O.trackTitleSelected_str), O.titleText_do.getStyle().color = O.trackTitleSelected_str, O.playPause_do && (FWDAnimation.killTweensOf(O.pauseS_do), FWDAnimation.killTweensOf(O.playS_do), O.pauseS_do.setAlpha(1), O.playS_do.setAlpha(1))))
				},
				this.setActive = function() {
					O.isActive_bl || (O.isActive_bl = !0, O.setSelectedState(!0))
				},
				this.setInActive = function() {
					O.isActive_bl && (O.isActive_bl = !1, O.setNormalState(!0), O.updateProgressPercent(0), O.showPlayButton())
				},
				this.showPlayButton = function() {
					null != O && O.playN_do && (O.playN_do.setX(0),
					                            O.playS_do.setX(0),
																			O.pauseN_do.setX(-300),
																			O.pauseS_do.setX(-300))
				},
				this.showPauseButton = function() {
					O.playN_do && (O.playN_do.setX(-300),
					O.playS_do.setX(-300),
					O.pauseN_do.setX(0),
					O.pauseS_do.setX(0))
				},
				this.destroy = function() {
					this.playlistItemGrad1_img = null,
          this.playlistItemProgress_img = null,
          this.playlistPlayButtonN_img = null,
          this.playlistDownloadButtonN_img = null,
          this.playlistDownloadButtonS_str = null,
          this.playlistBuyButtonN_img = null,
          this.playlistBuyButtonS_str = null,
          this.progress_do = null,
          this.playPause_do = null,
          this.playN_do = null,
          this.playS_do = null,
          this.pauseN_do = null,
					this.pauseS_do = null,
          this.titleText_do = null,
          this.grad_do = null,
          this.durationText_do = null,
          this.dumy_do = null,
          this.title_str = null,
          this.playlistItemBk1Path_str = null,
          this.playlistItemBk2Path_str = null,
          this.playlistPlayButtonN_str = null,
					this.playlistPlayButtonS_str = null,
          this.playlistPauseButtonN_str = null,
          this.playlistPauseButtonS_str = null,
          this.titleNormalColor_str = null,
          this.trackTitleSelected_str = null,
          this.durationColor_str = S, O.setInnerHTML(""), O = null,
          k.prototype = null
				}, O.updateHEXColors = function(e, t) {
					if (O.normalColor_str = e,
						  O.selectedColor_str = t, O.buyButton_do && O.buyButton_do.updateHEXColors(e, t),
							O.downloadButton_do && O.downloadButton_do.updateHEXColors(e, t),
							O.playNImage_img)
						{
						var o = FWDMSPUtils.changeCanvasHEXColor(O.playNImage_img, O.playNImageCanvas, e, !0),
							s = FWDMSPUtils.changeCanvasHEXColor(O.playSImage_img, O.playSImageCanvas, t, !0);
						O.playN_do.getStyle().background = "url('" + o.src + "')",
						O.playS_do.getStyle().background = "url('" + s.src + "')";
						var i = FWDMSPUtils.changeCanvasHEXColor(O.pauseNImage_img, O.pauseNImageCanvas, e, !0),
							n = FWDMSPUtils.changeCanvasHEXColor(O.pauseSImage_img, O.pauseSImageCanvas, t, !0);
						O.pauseN_do.getStyle().background = "url('" + i.src + "')",
						O.pauseS_do.getStyle().background = "url('" + n.src + "')"
					}
				}, this.init()
		};
		k.setPrototype = function() {
			k.prototype = new FWDMSPDisplayObject("div")
		},
    k.PLAY = "play",
    k.PAUSE = "pause",
    k.MOUSE_UP = "mouseUp",
    k.DOWNLOAD = "download",
    k.BUY = "buy",
    k.prototype = null,
    window.FWDMSPPlaylistItem = k
	}(),
	function(e) {
		var r = function(e, t, o, s, i, n) {
			var l = this;
			r.prototype;
			this.imageSource_img = null,
      this.image_sdo = null,
			this.imageSourcePath_str = e,
      this.segmentWidth = t,
      this.segmentHeight = o,
      this.totalSegments = s,
			this.totalWidth = t * s,
      this.animDelay = i || 300,
      this.count = 0,
      this.delayTimerId_int,
      this.isShowed_bl = !1,
      this.skipFirstFrame_bl = n,
			this.init = function() {
					l.setWidth(l.segmentWidth),
          l.setHeight(l.segmentHeight),
          l.imageSource_img = new Image,
          l.imageSource_img.src = l.imageSourcePath_str,
					l.image_sdo = new FWDMSPDisplayObject("img"),
          l.image_sdo.setScreen(l.imageSource_img), l.image_sdo.setWidth(l.totalWidth),
          l.image_sdo.setHeight(l.segmentHeight),
          l.addChild(this.image_sdo),
          l.hide(!1)
				},
        this.start = function() {
					null != l && (clearInterval(l.delayTimerId_int),
					              l.delayTimerId_int = setInterval(l.updatePreloader, l.animDelay))
				},
        this.stop = function() {
					clearInterval(l.delayTimerId_int),
					l.image_sdo.setX(0)
				},
        this.updatePreloader = function() {
					if (null != l) {
						l.count++,
						l.count > l.totalSegments - 1 && (l.skipFirstFrame_bl ? l.count = 1 : l.count = 0);
						var e = l.count * l.segmentWidth;
						l.image_sdo.setX(-e)
					}
				}, this.show = function() {
					this.setVisible(!0), this.start(),
						FWDAnimation.killTweensOf(this),
						FWDAnimation.to(this, 1, {
							alpha: 1
						}), this.isShowed_bl = !0
				}, this.hide = function(e) {
					this.isShowed_bl && (FWDAnimation.killTweensOf(this),
															e ? FWDAnimation.to(this, 1, {
																	alpha: 0,
																	onComplete: this.onHideComplete
																	})
																: (this.setVisible(!1), this.setAlpha(0)),
																this.isShowed_bl = !1)
				},
				this.onHideComplete = function() {
					l.stop(),
					l.setVisible(!1),
					l.dispatchEvent(r.HIDE_COMPLETE)
				}, this.setForFixedPosition = function() {
					l.hasTransform3d_bl = !1,
					l.hasTransform2d_bl = !1,
					l.image_sdo.hasTransform3d_bl = !1,
					l.image_sdo.hasTransform2d_bl = !1
				}, this.init()
		};
		r.setPrototype = function() {
			r.prototype = new FWDMSPDisplayObject("div")
		}, r.HIDE_COMPLETE = "hideComplete", r.prototype = null, e.FWDMSPPreloader = r
	}(window),
	function(e) {
		var l = function(e, t, o, s, i) {
			var n = this;
			l.prototype;
			this.buttonRef_do = e,
      this.bkColor = t,
      this.text_do = null,
      this.pointer_do = null,
      this.fontColor_str = o,
			this.pointerWidth = 7,
      this.pointerHeight = 4,
      this.showWithDelayId_to,
      this.isMobile_bl = FWDMSPUtils.isMobile,
      this.isShowed_bl = !0,
      this.init = function() {
					n.setOverflow("visible"),
          n.setupMainContainers(),
          n.setLabel(s),
          n.hide(),
          n.setVisible(!1),
          n.getStyle().backgroundColor = n.bkColor,
					n.getStyle().zIndex = 9999999999999,
          n.getStyle().pointerEvents = "none"
				},
        this.setupMainContainers = function() {
					n.pointerHolder_do = new FWDMSPDisplayObject("div"),
          n.pointerHolder_do.setOverflow("visible"),
          n.addChild(n.pointerHolder_do),
          n.text_do = new FWDMSPDisplayObject("div"),
          n.text_do.hasTransform3d_bl = !1,
          n.text_do.hasTransform2d_bl = !1,
          n.text_do.setDisplay("inline-block"),
          n.text_do.getStyle().fontFamily = "Arial",
          n.text_do.getStyle().fontSize = "12px",
          n.text_do.getStyle().color = n.fontColor_str,
          n.text_do.getStyle().whiteSpace = "nowrap",
          n.text_do.getStyle().fontSmoothing = "antialiased",
					n.text_do.getStyle().webkitFontSmoothing = "antialiased",
          n.text_do.getStyle().textRendering = "optimizeLegibility",
          n.text_do.getStyle().padding = "6px",
          n.text_do.getStyle().paddingTop = "4px",
          n.text_do.getStyle().paddingBottom = "4px",
					n.addChild(n.text_do),
          n.pointer_do = new FWDMSPDisplayObject("div"),
          n.pointer_do.setBkColor(n.bkColor),
          n.pointer_do.screen.style = "border: 4px solid transparent; border-top-color: " + n.bkColor + ";",
          n.pointerHolder_do.addChild(n.pointer_do)
				},
        this.setLabel = function(e) {
					void 0 !== e && (n.text_do.setInnerHTML(e), setTimeout(function() {
						null != n && (n.setWidth(n.text_do.getWidth()), n.setHeight(n.text_do.getHeight()), n.positionPointer())
					}, 20))
				},
        this.positionPointer = function(e) {
					var t, o;
					e = e || 0, t = parseInt((n.w - 8) / 2) + e,
						o = n.h, n.pointerHolder_do.setX(t), n.pointerHolder_do.setY(o)
				},
        this.show = function() {
					n.isShowed_bl = !0, clearTimeout(n.hideWithDelayId_to),
          FWDAnimation.killTweensOf(n),
          clearTimeout(n.showWithDelayId_to),
          n.showWithDelayId_to = setTimeout(n.showFinal)
				},
        this.showFinal = function() {
					n.setVisible(!0), FWDAnimation.to(n, .4, {
						alpha: 1,
						onComplete: function() {
							n.setVisible(!0)
						},
						ease: Quart.easeOut
					})
				}, this.hide = function() {
					n.isShowed_bl && (clearTimeout(n.showWithDelayId_to), clearTimeout(n.hideWithDelayId_to),
          n.hideWithDelayId_to = setTimeout(function() {
						FWDAnimation.killTweensOf(n),
							n.setVisible(!1), n.isShowed_bl = !1, n.setAlpha(0)
					}, 100))
				}, this.init()
		};
		l.setPrototype = function() {
			l.prototype = null,
      l.prototype = new FWDMSPDisplayObject("div")
		},
    l.CLICK = "onClick",
    l.MOUSE_DOWN = "onMouseDown",
    l.prototype = null
	}(window),
	function(t) {
		var e = function(o, s) {
			var f = this;
			e.prototype;
			this.embedColoseN_img = o.embedColoseN_img,
      this.bk_do = null,
      this.mainHolder_do = null,
      this.closeButton_do = null,
      this.buttons_ar = [],
			this.embedWindowCloseButtonMargins = 0,
      this.totalWidth = 0,
      this.stageWidth = 0,
      this.stageHeight = 0,
      this.minMarginXSpace = 20,
      this.hSpace = 20,
      this.minHSpace = 10,
      this.vSpace = 15,
      this.isShowed_bl = !1,
      this.isMobile_bl = FWDMSPUtils.isMobile,
      this.init = function() {
          f.mainHolder_do = new FWDMSPDisplayObject("div"),
          f.mainHolder_do.hasTransform3d_bl = !1,
          f.mainHolder_do.hasTransform2d_bl = !1,
          f.bk_do = new FWDMSPDisplayObject("div"),
          f.bk_do.getStyle().width = "100%",
          f.bk_do.getStyle().height = "100%",
          f.bk_do.setAlpha(.9),
					f.bk_do.getStyle().background = "url('" + f.embedWindowBackground_str + "')",
					FWDMSPSimpleButton.setPrototype(),
          f.closeButton_do = new FWDMSPSimpleButton(o.embedWindowClosePathS_str,
																										void 0,
																										!0,
																										o.useHEXColorsForSkin_bl,
																										o.normalButtonsColor_str,
																										o.selectedButtonsColor_str),
          f.closeButton_do.addListener(FWDMSPSimpleButton.MOUSE_UP, f.closeButtonOnMouseUpHandler),
          f.addChild(f.mainHolder_do),
          f.mainHolder_do.addChild(f.bk_do),
          f.mainHolder_do.addChild(f.closeButton_do),
					this.setupButtons()
				},
        this.closeButtonOnMouseUpHandler = function() {
					f.isShowed_bl && f.hide(!0)
				},
        this.positionAndResize = function() {
					f.stageWidth = s.stageWidth,
          f.stageHeight = s.stageHeight;
					var e = f.stageWidth - f.closeButton_do.w - f.embedWindowCloseButtonMargins,
					t = 0;
					t = s.playlist_do && s.position_str == FWDMSP.POSITION_TOP ? s.playlist_do.h : f.embedWindowCloseButtonMargins,
					f.closeButton_do.setX(e),
					f.closeButton_do.setY(0),
					f.setY(t),
					f.setWidth(f.stageWidth),
					f.setHeight(f.stageHeight),
					f.mainHolder_do.setWidth(f.stageWidth),
					f.mainHolder_do.setHeight(f.stageHeight),
					f.positionButtons()
				},
        this.positionButtons = function() {
					var e, t, o, s = [],
						i = [],
						n = [],
						l = 0,
						r = 0,
						a = 0;
					s[a] = [0],
					i[a] = f.buttons_ar[0].totalWidth,
					n[a] = f.buttons_ar[0].totalWidth,
					f.totalButtons = f.buttons_ar.length;
					for (var d = 1; d < f.totalButtons; d++)
					   e = f.buttons_ar[d],
						 i[a] + e.totalWidth + f.minHSpace > f.stageWidth - f.minMarginXSpace ? (s[++a] = [],
							                                                                       s[a].push(d),
																																										 i[a] = e.totalWidth,
																																										 n[a] = e.totalWidth)
																																									: (s[a].push(d),
																																									   i[a] += e.totalWidth + f.minHSpace,
																																										 n[a] += e.totalWidth);
					l = parseInt((f.stageHeight - ((a + 1) * (e.totalHeight + f.vSpace) - f.vSpace)) / 2);
					for (d = 0; d < a + 1; d++) {
						var u, c = 0;
						if (1 < s[d].length) {
							u = Math.min((f.stageWidth - f.minMarginXSpace - n[d]) / (s[d].length - 1), f.hSpace);
							var h = n[d] + u * (s[d].length - 1);
							c = parseInt((f.stageWidth - h) / 2)
						} else c = parseInt((f.stageWidth - i[d]) / 2);
						0 < d && (l += e.h + f.vSpace);
						for (var _ = 0; _ < s[d].length; _++) e = f.buttons_ar[s[d][_]], o = 0 == _ ? c : (t = f.buttons_ar[s[d][_] - 1]).finalX + t.totalWidth + u, e.finalX = o, e.finalY = l, r < e.finalY && (r = e.finalY), f.buttonsBarTotalHeight = r + e.totalHeight + f.startY, e.setX(e.finalX), e.setY(e.finalY)
					}
				}, this.show = function(e) {
					f.isShowed_bl || (f.isShowed_bl = !0,
														s.main_do.addChild(f),
					                 (!FWDMSPUtils.isMobile || FWDMSPUtils.isMobile && FWDMSPUtils.hasPointerEvent) && s.main_do.setSelectable(!0),
													 f.positionAndResize(),
													 clearTimeout(f.hideCompleteId_to),
													 clearTimeout(f.showCompleteId_to),
													 f.mainHolder_do.setY(-f.stageHeight),
													 f.showCompleteId_to = setTimeout(f.showCompleteHandler, 900),
													 setTimeout(function() {
														 FWDAnimation.to(f.mainHolder_do, .8, {
															 y: 0,
															 delay: .1,
															 ease: Expo.easeInOut
														 })
													 }, 100))
				},
				this.showCompleteHandler = function() {}, this.hide = function(e) {
					f.isShowed_bl && (f.isShowed_bl = !1,
						                s.customContextMenu_do && s.customContextMenu_do.enable(),
														clearTimeout(f.hideCompleteId_to),
														clearTimeout(f.showCompleteId_to),
														(!FWDMSPUtils.isMobile || FWDMSPUtils.isMobile && FWDMSPUtils.hasPointerEvent) && s.main_do.setSelectable(!1),
														f.hideCompleteId_to = setTimeout(f.hideCompleteHandler, 800),
														FWDAnimation.killTweensOf(f.mainHolder_do),
														e ? FWDAnimation.to(f.mainHolder_do, .8, {
																y: -f.stageHeight,
																ease: Expo.easeInOut
																})
															: f.hideCompleteHandler())
				}, this.hideCompleteHandler = function() {
					s.main_do.contains(f) && s.main_do.removeChild(f),
					f.dispatchEvent(e.HIDE_COMPLETE)
				}, this.updateHEXColors = function(e, t) {
					-1 != o.skinPath_str.indexOf("hex_white") ? f.selectedColor_str = "#FFFFFF" : f.selectedColor_str = t,
					 f.closeButton_do.updateHEXColors(e, f.selectedColor_str)
				}, this.init()
		};
		e.setPrototype = function() {
			e.prototype = new FWDMSPDisplayObject("div")
		}, e.HIDE_COMPLETE = "hideComplete", e.prototype = null, t.FWDMSPShareWindow = e
	}(window),
	function(e) {
		var d = function(e, t, o, s, i, n, l, r) {
			var a = this;
			d.prototype;
			this.nImg = e,
      this.sPath_str = t,
      this.dPath_str = o,
			this.n_sdo,
      this.s_sdo,
      this.d_sdo,
      this.totalWidth = this.nImg.width,
      this.totalHeight = this.nImg.height,
      this.useHEXColorsForSkin_bl = i,
      this.normalButtonsColor_str = n,
      this.selectedButtonsColor_str = l,
      this.inverseHEXColors_bl = r,
      this.isShowed_bl = !0,
			this.isSetToDisabledState_bl = !1,
      this.isDisabled_bl = !1,
      this.isDisabledForGood_bl = !1,
      this.isSelectedFinal_bl = !1,
      this.isActive_bl = !1,
      this.isMobile_bl = FWDMSPUtils.isMobile,
      this.hasPointerEvent_bl = FWDMSPUtils.hasPointerEvent,
      this.allowToCreateSecondButton_bl = !a.isMobile_bl || a.hasPointerEvent_bl || s,
      a.init = function() {
					a.setupMainContainers()
				},
        a.setupMainContainers = function() {
					if (a.useHEXColorsForSkin_bl ? (a.n_sdo = new FWDMSPTransformDisplayObject("div"),
					                                a.n_sdo.setWidth(a.totalWidth),
																					a.n_sdo.setHeight(a.totalHeight),
																					a.n_sdo_canvas = FWDMSPUtils.getCanvasWithModifiedColor(a.nImg, a.normalButtonsColor_str).canvas,
																					a.n_sdo.screen.appendChild(a.n_sdo_canvas))
																			 : (a.n_sdo = new FWDMSPTransformDisplayObject("img"),
																			    a.n_sdo.setScreen(a.nImg)),
						a.addChild(a.n_sdo),
						a.allowToCreateSecondButton_bl) {
							a.img1 = new Image,
            	a.img1.src = a.sPath_str;
							var e = new Image;
							a.sImg = e,
            	a.useHEXColorsForSkin_bl ? (a.s_sdo = new FWDMSPTransformDisplayObject("div"),
							                            a.s_sdo.setWidth(a.totalWidth),
																					a.s_sdo.setHeight(a.totalHeight),
																					a.img1.onload = function() {
																						a.inverseHEXColors_bl ? a.s_sdo_canvas = FWDMSPUtils.getCanvasWithModifiedColor(a.img1, a.normalButtonsColor_str).canvas
																						                      : a.s_sdo_canvas = FWDMSPUtils.getCanvasWithModifiedColor(a.img1, a.selectedButtonsColor_str).canvas,
																						a.s_sdo.screen.appendChild(a.s_sdo_canvas)
																					})
																				: (a.s_sdo = new FWDMSPDisplayObject("img"),
																				   a.s_sdo.setScreen(a.img1),
																					 a.s_sdo.setWidth(a.totalWidth),
																					 a.s_sdo.setHeight(a.totalHeight)),
							a.s_sdo.setAlpha(0),
							a.addChild(a.s_sdo),
							a.dPath_str && (e.src = a.dPath_str, a.d_sdo = new FWDMSPDisplayObject("img"),
															a.d_sdo.setScreen(e),
															a.d_sdo.setWidth(a.totalWidth),
															a.d_sdo.setHeight(a.totalHeight),
															a.d_sdo.setX(-100),
															a.addChild(a.d_sdo))
					}
					a.setWidth(a.totalWidth),
          a.setHeight(a.totalHeight),
          a.setButtonMode(!0),
          a.screen.style.yellowOverlayPointerEvents = "none",
          a.isMobile_bl ? a.hasPointerEvent_bl
					              ? (a.screen.addEventListener("pointerup", a.onMouseUp),
												   a.screen.addEventListener("pointerover", a.onMouseOver),
													 a.screen.addEventListener("pointerout", a.onMouseOut))
          			 				: a.screen.addEventListener("touchend", a.onMouseUp)
          							: a.screen.addEventListener
												? (a.screen.addEventListener("mouseover", a.onMouseOver),
												   a.screen.addEventListener("mouseout", a.onMouseOut),
													 a.screen.addEventListener("mouseup", a.onMouseUp))
          			 				: a.screen.attachEvent && (a.screen.attachEvent("onmouseover", a.onMouseOver),
												                           a.screen.attachEvent("onmouseout", a.onMouseOut),
																									 a.screen.attachEvent("onmouseup", a.onMouseUp))
				},
        a.onMouseOver = function(e) {
					if (!(a.isDisabledForGood_bl || e.pointerType && e.pointerType != e.MSPOINTER_TYPE_MOUSE && "mouse" != e.pointerType)) {
						if (a.isDisabled_bl || a.isSelectedFinal_bl) return;
						a.dispatchEvent(d.MOUSE_OVER, {
							e: e
						}), a.setSelectedState()
					}
				}, a.onMouseOut = function(e) {
					if (!(a.isDisabledForGood_bl || e.pointerType && e.pointerType != e.MSPOINTER_TYPE_MOUSE && "mouse" != e.pointerType)) {
						if (a.isDisabled_bl || a.isSelectedFinal_bl) return;
						a.dispatchEvent(d.MOUSE_OUT, {
							e: e
						}), a.setNormalState()
					}
				},
        a.onMouseUp = function(e) {
					a.isDisabledForGood_bl || (e.preventDefault && e.preventDefault(), a.isDisabled_bl || 2 == e.button || a.dispatchEvent(d.MOUSE_UP, {
						e: e
					}))
				}, a.setSelected = function() {
					a.isSelectedFinal_bl = !0, a.s_sdo && (FWDAnimation.killTweensOf(a.s_sdo), FWDAnimation.to(a.s_sdo, .8, {
						alpha: 1,
						ease: Expo.easeOut
					}))
				},
        a.setUnselected = function() {
					a.isSelectedFinal_bl = !1, a.s_sdo && FWDAnimation.to(a.s_sdo, .8, {
						alpha: 0,
						delay: .1,
						ease: Expo.easeOut
					})
				},
        this.setNormalState = function() {
					FWDAnimation.killTweensOf(a.s_sdo),
						FWDAnimation.to(a.s_sdo, .5, {
							alpha: 0,
							ease: Expo.easeOut
						})
				},
        this.setSelectedState = function() {
					FWDAnimation.killTweensOf(a.s_sdo),
						FWDAnimation.to(a.s_sdo, .5, {
							alpha: 1,
							delay: .1,
							ease: Expo.easeOut
						})
				},
        this.setDisabledState = function() {
					a.isSetToDisabledState_bl || (a.isSetToDisabledState_bl = !0, a.d_sdo && a.d_sdo.setX(0))
				},
        this.setEnabledState = function() {
					a.isSetToDisabledState_bl && (a.isSetToDisabledState_bl = !1, a.d_sdo && a.d_sdo.setX(-100))
				},
        this.disable = function() {
					a.isDisabledForGood_bl || a.isDisabled_bl || (a.isDisabled_bl = !0, FWDAnimation.killTweensOf(a), a.setButtonMode(!1), FWDAnimation.to(a, .6, {
						alpha: .4
					}), a.setNormalState())
				},
        this.enable = function() {
					!a.isDisabledForGood_bl && a.isDisabled_bl && (a.isDisabled_bl = !1, FWDAnimation.killTweensOf(a), a.setButtonMode(!0), FWDAnimation.to(a, .6, {
						alpha: 1
					}))
				},
        this.disableForGood = function() {
					a.isDisabledForGood_bl = !0, a.setButtonMode(!1)
				},
        this.disableForGood = function() {
					a.isDisabledForGood_bl = !0, a.setButtonMode(!1)
				},
        this.enableForGood = function() {
					a.isDisabledForGood_bl = !1, a.setButtonMode(!0)
				},
        this.showDisabledState = function() {
					0 != a.d_sdo.x && a.d_sdo.setX(0)
				},
        this.hideDisabledState = function() {
					-100 != a.d_sdo.x && a.d_sdo.setX(-100)
				},
        this.show = function() {
					a.isShowed_bl || (a.isShowed_bl = !0, FWDAnimation.killTweensOf(a), FWDMSPUtils.isIEAndLessThen9 ? (FWDMSPUtils.isIEAndLessThen9 || (a.setAlpha(0), FWDAnimation.to(a, .4, {
						alpha: 1,
						delay: .4
					})), a.setVisible(!0)) : FWDMSPUtils.isIEWebKit ? (FWDAnimation.killTweensOf(a.n_sdo), a.n_sdo.setScale2(0), FWDAnimation.to(a.n_sdo, .8, {
						scale: 1,
						delay: .4,
						onStart: function() {
							a.setVisible(!0)
						},
						ease: Elastic.easeOut
					})) : (a.setScale2(0), FWDAnimation.to(a, .8, {
						scale: 1,
						delay: .4,
						onStart: function() {
							a.setVisible(!0)
						},
						ease: Elastic.easeOut
					})))
				},
        this.hide = function(e) {
					a.isShowed_bl && (a.isShowed_bl = !1,
														FWDAnimation.killTweensOf(a),
														FWDAnimation.killTweensOf(a.n_sdo),
														a.setVisible(!1))
				},
        a.updateHEXColors = function(e, t) {
					FWDMSPUtils.changeCanvasHEXColor(a.nImg, a.n_sdo_canvas, e),
					FWDMSPUtils.changeCanvasHEXColor(a.img1, a.s_sdo_canvas, t)
				}, a.init()
		};
		d.setPrototype = function() {
				d.prototype = null, d.prototype = new FWDMSPTransformDisplayObject("div")
			},
      d.CLICK = "onClick",
      d.MOUSE_OVER = "onMouseOver",
      d.MOUSE_OUT = "onMouseOut",
			d.MOUSE_UP = "onMouseDown",
      d.prototype = null,
      e.FWDMSPSimpleButton = d
	}(window),
	function(e) {
		var a = function(e, t, o, s, i, n, l) {
			var r = this;
			a.prototype;
			this.nImg_img = null,
      this.sImg_img = null,
      this.n_do,
			this.s_do,
      this.useHEXColorsForSkin_bl = i,
      this.normalButtonsColor_str = l,
      this.selectedButtonsColor_str = n,
      this.nImgPath_str = e,
      this.sImgPath_str = t,
      this.buttonWidth = o,
			this.buttonHeight = s,
      this.isMobile_bl = FWDMSPUtils.isMobile,
      this.hasPointerEvent_bl = FWDMSPUtils.hasPointerEvent,
      this.isDisabled_bl = !1,
      this.init = function() {
					r.setupMainContainers(),
          r.setWidth(r.buttonWidth),
          r.setHeight(r.buttonHeight),
          r.setButtonMode(!0)
				},
        this.setupMainContainers = function() {
					r.nImg = new Image,
          r.nImg.src = r.nImgPath_str,
          r.useHEXColorsForSkin_bl ? (r.n_do = new FWDMSPTransformDisplayObject("div"),
					                            r.n_do.setWidth(r.buttonWidth),
																			r.n_do.setHeight(r.buttonHeight),
																			r.nImg.onload = function() {
																				r.n_do_canvas = FWDMSPUtils.getCanvasWithModifiedColor(r.nImg, r.normalButtonsColor_str).canvas,
																				r.n_do.screen.appendChild(r.n_do_canvas)
																			})
          												 : (r.n_do = new FWDMSPDisplayObject("img"),
																	    r.n_do.setScreen(r.nImg),
																			r.n_do.setWidth(r.buttonWidth),
																			r.n_do.setHeight(r.buttonHeight)),
						r.addChild(r.n_do),
						r.sImg = new Image,
						r.sImg.src = r.sImgPath_str,
						r.useHEXColorsForSkin_bl ? (r.s_do = new FWDMSPTransformDisplayObject("div"),
						                            r.s_do.setWidth(r.buttonWidth),
																				r.s_do.setHeight(r.buttonHeight),
																				r.sImg.onload = function() {
																					r.s_do_canvas = FWDMSPUtils.getCanvasWithModifiedColor(r.sImg, r.selectedButtonsColor_str).canvas,
																					                                                       r.s_do.screen.appendChild(r.s_do_canvas)
																																															 })
          														: (r.s_do = new FWDMSPDisplayObject("img"),
																			   r.s_do.setScreen(r.sImg),
																				 r.s_do.setWidth(r.buttonWidth),
																				 r.s_do.setHeight(r.buttonHeight)),
						r.addChild(r.s_do),
						r.hasPointerEvent_bl ? (r.screen.addEventListener("pointerup", r.onMouseUp),
						                        r.screen.addEventListener("pointerover", r.setNormalState),
																		r.screen.addEventListener("pointerout", r.setSelectedState))
																 : r.screen.addEventListener && (r.isMobile_bl || (r.screen.addEventListener("mouseover", r.setNormalState),
																                                 									 r.screen.addEventListener("mouseout", r.setSelectedState),
          																															 					 r.screen.addEventListener("mouseup", r.onMouseUp)),
          																															 					 r.screen.addEventListener("touchend", r.onMouseUp))
				},
        this.setNormalState = function(e) {
					FWDAnimation.killTweensOf(r.s_do),
						FWDAnimation.to(r.s_do, .5, {
							alpha: 0,
							ease: Expo.easeOut
						})
				},
        this.setSelectedState = function(e) {
					FWDAnimation.killTweensOf(r.s_do),
						FWDAnimation.to(r.s_do, .5, {
							alpha: 1,
							ease: Expo.easeOut
						})
				},
        this.onMouseUp = function(e) {
					r.dispatchEvent(a.CLICK)
				},
        r.updateHEXColors = function(e, t) {
					FWDMSPUtils.changeCanvasHEXColor(r.nImg, r.n_do_canvas, t), FWDMSPUtils.changeCanvasHEXColor(r.sImg, r.s_do_canvas, e)
				}, this.destroy = function() {
					FWDAnimation.killTweensOf(r.n_do), r.n_do.destroy(), this.s_do.destroy(), r.screen.onmouseover = null, r.screen.onmouseout = null, r.screen.onclick = null, r.nImg_img = null, r.sImg_img = null, r = null, a.prototype = null
				}, r.init()
		};
		a.setPrototype = function() {
			a.prototype = null,
      a.prototype = new FWDMSPTransformDisplayObject("div", "relative")
		},
    a.CLICK = "onClick",
    a.prototype = null,
    e.FWDMSPSimpleSizeButton = a
	}(window),
	function(a) {
		var d = function(e, t, o, s, i, n, l) {
			var r = this;
			d.prototype;
			this.buttonRef_do = e,
      this.bkPath_str = t,
      this.pointerPath_str = o,
      this.text_do = null,
      this.pointer_do = null,
      this.pointerUp_do = null,
			this.fontColor_str = n,
			this.pointerWidth = 7,
      this.pointerHeight = 4,
      this.showWithDelayId_to,
      this.isMobile_bl = FWDMSPUtils.isMobile,
      this.isShowed_bl = !0,
      this.init = function() {
					r.setOverflow("visible"),
          r.setupMainContainers(),
          r.hide(),
          r.getStyle().background = "url('" + r.bkPath_str + "')",
          r.getStyle().zIndex = 9999999999
				},
        this.setupMainContainers = function() {
					r.text_do = new FWDMSPDisplayObject("div"),
          r.text_do.hasTransform3d_bl = !1,
          r.text_do.hasTransform2d_bl = !1,
          r.text_do.setDisplay("inline"),
          r.text_do.getStyle().fontFamily = "Arial",
          r.text_do.getStyle().fontSize = "12px",
          r.text_do.getStyle().color = r.fontColor_str,
          r.text_do.getStyle().whiteSpace = "nowrap",
          r.text_do.getStyle().fontSmoothing = "antialiased",
					r.text_do.getStyle().webkitFontSmoothing = "antialiased",
          r.text_do.getStyle().textRendering = "optimizeLegibility",
          r.text_do.getStyle().padding = "6px",
          r.text_do.getStyle().paddingTop = "4px",
          r.text_do.getStyle().paddingBottom = "4px",
					r.setLabel(),
          r.addChild(r.text_do);
					var e = new Image;
					e.src = r.pointerPath_str,
          r.pointer_do = new FWDMSPDisplayObject("img"),
          r.pointer_do.setScreen(e),
          r.pointer_do.setWidth(r.pointerWidth),
          r.pointer_do.setHeight(r.pointerHeight),
          r.addChild(r.pointer_do);
					var t = new Image;
					r.pointerUp_do = new FWDMSPDisplayObject("img"),
          r.pointerUp_do.setScreen(t),
          r.pointerUp_do.setWidth(r.pointerWidth),
          r.pointerUp_do.setHeight(r.pointerHeight),
					r.addChild(r.pointerUp_do)
				},
        this.setLabel = function(e) {
					r.text_do.setInnerHTML(i),
          setTimeout(function() {
						null != r && (r.setWidth(r.text_do.getWidth()),
            r.setHeight(r.text_do.getHeight()),
            r.positionPointer())
					}, 50)
				},
        this.positionPointer = function(e, t) {
					var o, s;
					e = e || 0, o = parseInt((r.w - r.pointerWidth) / 2) + e, t ? (s = -3, r.pointerUp_do.setX(o), r.pointerUp_do.setY(s), r.pointer_do.setX(0), r.pointer_do.setY(0)) : (s = r.h, r.pointer_do.setX(o), r.pointer_do.setY(s), r.pointerUp_do.setX(0), r.pointerUp_do.setY(0))
				},
        this.show = function() {
					r.isShowed_bl || (r.isShowed_bl = !0, FWDAnimation.killTweensOf(r), clearTimeout(r.showWithDelayId_to), r.showWithDelayId_to = setTimeout(r.showFinal), a.addEventListener ? a.addEventListener("mousemove", r.moveHandler) : document.attachEvent && (document.detachEvent("onmousemove", r.moveHandler), document.attachEvent("onmousemove", r.moveHandler)))
				},
        this.showFinal = function() {
					r.setVisible(!0), r.setAlpha(0), FWDAnimation.to(r, .4, {
						alpha: 1,
						onComplete: function() {
							r.setVisible(!0)
						},
						ease: Quart.easeOut
					})
				},
        this.moveHandler = function(e) {
					var t = FWDMSPUtils.getViewportMouseCoordinates(e);
					FWDMSPUtils.hitTest(r.buttonRef_do.screen, t.screenX, t.screenY) || r.hide()
				},
        this.hide = function() {
					r.isShowed_bl && (clearTimeout(r.showWithDelayId_to), a.removeEventListener ? a.removeEventListener("mousemove", r.moveHandler) : document.detachEvent && document.detachEvent("onmousemove", r.moveHandler), FWDAnimation.killTweensOf(r), r.setVisible(!1), r.isShowed_bl = !1)
				},
        this.init()
		};
		d.setPrototype = function() {
			d.prototype = null, d.prototype = new FWDMSPDisplayObject("div", "fixed")
		}, d.CLICK = "onClick", d.MOUSE_DOWN = "onMouseDown", d.prototype = null
	}(window),

	window.FWDMSPTransformDisplayObject = function(e, t, o, s) {
			this.listeners = {
				events_ar: []
			};
			var i = this;
			if ("div" != e && "img" != e && "canvas" != e) throw Error("Type is not valid! " + e);
			this.type = e,
	    this.children_ar = [],
	    this.style,
	    this.screen,
			this.numChildren,
	    this.transform,
	    this.position = t || "absolute",
	    this.overflow = o || "hidden",
	    this.display = s || "block",
	    this.visible = !0,
			this.buttonMode,
	    this.x = 0,
	    this.y = 0,
	    this.scale = 1,
	    this.rotation = 0,
	    this.w = 0,
	    this.h = 0,
	    this.rect,
	    this.alpha = 1,
	    this.innerHTML = "",
			this.opacityType = "",
	    this.isHtml5_bl = !1,
	    this.hasTransform2d_bl = FWDMSPUtils.hasTransform2d,
	    this.init = function() {
					this.setScreen()
				},
	      this.getTransform = function() {
					for (var e, t = ["transform", "msTransform", "WebkitTransform", "MozTransform", "OTransform"]; e = t.shift();)
						if (void 0 !== this.screen.style[e]) return e;
					return !1
				},
	      this.getOpacityType = function() {
					return void 0 !== this.screen.style.opacity ? "opacity" : "filter"
				},
	      this.setScreen = function(e) {
					"img" == this.type && e ? this.screen = e : this.screen = document.createElement(this.type), this.setMainProperties()
				},
	      this.setMainProperties = function() {
					this.transform = this.getTransform(),
	        this.setPosition(this.position),
	        this.setOverflow(this.overflow),
	        this.opacityType = this.getOpacityType(),
	        "opacity" == this.opacityType && (this.isHtml5_bl = !0),
	        "filter" == i.opacityType && (i.screen.style.filter = "inherit"),
					this.screen.style.left = "0px",
	        this.screen.style.top = "0px",
	        this.screen.style.margin = "0px",
	        this.screen.style.padding = "0px",
	        this.screen.style.maxWidth = "none",
	        this.screen.style.maxHeight = "none",
	        this.screen.style.border = "none",
	        this.screen.style.lineHeight = "1",
	        this.screen.style.backgroundColor = "transparent",
					this.screen.style.MozImageRendering = "optimizeSpeed",
	        this.screen.style.WebkitImageRendering = "optimizeSpeed",
	        "img" == e && (this.setWidth(this.screen.width),
	        this.setHeight(this.screen.height),
	        this.screen.onmousedown = function(e) {
							return !1
						})
				},
	      this.setSelectable = function(e) {
					if (!e) {
						try {
							this.screen.style.userSelect = "none"
						} catch (e) {}
						try {
							this.screen.style.MozUserSelect = "none"
						} catch (e) {}
						try {
							this.screen.style.webkitUserSelect = "none"
						} catch (e) {}
						try {
							this.screen.style.khtmlUserSelect = "none"
						} catch (e) {}
						try {
							this.screen.style.oUserSelect = "none"
						} catch (e) {}
						try {
							this.screen.style.msUserSelect = "none"
						} catch (e) {}
						try {
							this.screen.msUserSelect = "none"
						} catch (e) {}
						this.screen.ondragstart = function(e) {
							return !1
						}, this.screen.onselectstart = function() {
							return !1
						}, this.screen.style.webkitTouchCallout = "none"
					}
				},
	      this.getScreen = function() {
					return i.screen
				},
	      this.setVisible = function(e) {
					this.visible = e, 1 == this.visible ? this.screen.style.visibility = "visible" : this.screen.style.visibility = "hidden"
				},
	      this.getVisible = function() {
					return this.visible
				},
	      this.setResizableSizeAfterParent = function() {
					this.screen.style.width = "100%", this.screen.style.height = "100%"
				},
	      this.getStyle = function() {
					return this.screen.style
				},
	      this.setOverflow = function(e) {
					i.overflow = e, i.screen.style.overflow = i.overflow
				},
	      this.setPosition = function(e) {
					i.position = e, i.screen.style.position = i.position
				},
	      this.setDisplay = function(e) {
					this.display = e, this.screen.style.display = this.display
				},
	      this.setButtonMode = function(e) {
					this.buttonMode = e, 1 == this.buttonMode ? this.screen.style.cursor = "pointer" : this.screen.style.cursor = "default"
				},
	      this.setBkColor = function(e) {
					i.screen.style.backgroundColor = e
				},
	      this.setInnerHTML = function(e) {
					i.innerHTML = e, i.screen.innerHTML = i.innerHTML
				},
	      this.getInnerHTML = function() {
					return i.innerHTML
				},
	      this.getRect = function() {
					return i.screen.getBoundingClientRect()
				},
	      this.setAlpha = function(e) {
					i.alpha = e, "opacity" == i.opacityType ? i.screen.style.opacity = i.alpha : "filter" == i.opacityType && (i.screen.style.filter = "alpha(opacity=" + 100 * i.alpha + ")", i.screen.style.filter = "progid:DXImageTransform.Microsoft.Alpha(Opacity=" + Math.round(100 * i.alpha) + ")")
				},
	      this.getAlpha = function() {
					return i.alpha
				},
	      this.getRect = function() {
					return this.screen.getBoundingClientRect()
				},
	      this.getGlobalX = function() {
					return this.getRect().left
				},
	      this.getGlobalY = function() {
					return this.getRect().top
				},
	      this.setX = function(e) {
					i.x = e, i.hasTransform2d_bl ? i.screen.style[i.transform] = "translate(" + i.x + "px," + i.y + "px) scale(" + i.scale + " , " + i.scale + ") rotate(" + i.rotation + "deg)" : i.screen.style.left = i.x + "px"
				},
	      this.getX = function() {
					return i.x
				},
	      this.setY = function(e) {
					i.y = e, i.hasTransform2d_bl ? i.screen.style[i.transform] = "translate(" + i.x + "px," + i.y + "px) scale(" + i.scale + " , " + i.scale + ") rotate(" + i.rotation + "deg)" : i.screen.style.top = i.y + "px"
				},
	      this.getY = function() {
					return i.y
				},
	      this.setScale2 = function(e) {
					i.scale = e, i.hasTransform2d_bl && (i.screen.style[i.transform] = "translate(" + i.x + "px," + i.y + "px) scale(" + i.scale + " , " + i.scale + ") rotate(" + i.rotation + "deg)")
				},
	      this.getScale = function() {
					return i.scale
				},
	      this.setRotation = function(e) {
					i.rotation = e, i.hasTransform2d_bl && (i.screen.style[i.transform] = "translate(" + i.x + "px," + i.y + "px) scale(" + i.scale + " , " + i.scale + ") rotate(" + i.rotation + "deg)")
				},
	      i.setWidth = function(e) {
					i.w = e, "img" == i.type && (i.screen.width = i.w), i.screen.style.width = i.w + "px"
				},
	      this.getWidth = function() {
					return "div" == i.type ? 0 != i.screen.offsetWidth ? i.screen.offsetWidth : i.w : "img" == i.type ? 0 != i.screen.offsetWidth ? i.screen.offsetWidth : 0 != i.screen.width ? i.screen.width : i._w : "canvas" == i.type ? 0 != i.screen.offsetWidth ? i.screen.offsetWidth : i.w : void 0
				},
	      i.setHeight = function(e) {
					i.h = e, "img" == i.type && (i.screen.height = i.h), i.screen.style.height = i.h + "px"
				},
	      this.getHeight = function() {
					return "div" == i.type ? 0 != i.screen.offsetHeight ? i.screen.offsetHeight : i.h : "img" == i.type ? 0 != i.screen.offsetHeight ? i.screen.offsetHeight : 0 != i.screen.height ? i.screen.height : i.h : "canvas" == i.type ? 0 != i.screen.offsetHeight ? i.screen.offsetHeight : i.h : void 0
				},
	      this.getNumChildren = function() {
					return i.children_ar.length
				},
	      this.addChild = function(e) {
					this.contains(e) && this.children_ar.splice(FWDMSPUtils.indexOfArray(this.children_ar, e), 1),
					this.children_ar.push(e),
					this.screen.appendChild(e.screen)
				},
	      this.removeChild = function(e) {
					if (!this.contains(e)) throw Error("##removeChild()## Child doesn't exist, it can't be removed!");
					this.children_ar.splice(FWDMSPUtils.indexOfArray(this.children_ar, e), 1),
					this.screen.removeChild(e.screen)
				},
	      this.contains = function(e) {
					return -1 != FWDMSPUtils.indexOfArray(this.children_ar, e)
				},
	      this.addChildAtZero = function(e) {
					0 == this.numChildren ? (this.children_ar.push(e), this.screen.appendChild(e.screen)) : (this.screen.insertBefore(e.screen, this.children_ar[0].screen), this.contains(e) && this.children_ar.splice(FWDMSPUtils.indexOfArray(this.children_ar, e), 1), this.children_ar.unshift(e))
				},
	      this.getChildAt = function(e) {
					if (e < 0 || e > this.numChildren - 1) throw Error("##getChildAt()## Index out of bounds!");
					if (0 == this.numChildren) throw Errror("##getChildAt## Child dose not exist!");
					return this.children_ar[e]
				},
	      this.removeChildAtZero = function() {
					this.screen.removeChild(this.children_ar[0].screen),
						this.children_ar.shift()
				},
	      this.addListener = function(e, t) {
					if (null == e) throw Error("type is required.");
					if ("object" == typeof e) throw Error("type must be of type String.");
					if ("function" != typeof t) throw Error("listener must be of type Function.");
					var o = {};
					o.type = e, o.listener = t, (o.target = this).listeners.events_ar.push(o)
				},
	      this.dispatchEvent = function(e, t) {
					if (null == e) throw Error("type is required.");
					if ("object" == typeof e) throw Error("type must be of type String.");
					for (var o = 0, s = this.listeners.events_ar.length; o < s; o++)
						if (this.listeners.events_ar[o].target === this && this.listeners.events_ar[o].type === e) {
							if (t)
								for (var i in t) this.listeners.events_ar[o][i] = t[i];
							this.listeners.events_ar[o].listener.call(this, this.listeners.events_ar[o]);
							break
						}
				},
	      this.removeListener = function(e, t) {
					if (null == e) throw Error("type is required.");
					if ("object" == typeof e) throw Error("type must be of type String.");
					if ("function" != typeof t) throw Error("listener must be of type Function." + e);
					for (var o = 0, s = this.listeners.events_ar.length; o < s; o++)
						if (this.listeners.events_ar[o].target === this && this.listeners.events_ar[o].type === e && this.listeners.events_ar[o].listener === t) {
							this.listeners.events_ar.splice(o, 1);
							break
						}
				},
	      this.disposeImage = function() {
					"img" == this.type && (this.screen.src = null)
				},
	      this.destroy = function() {
					try {
						this.screen.parentNode.removeChild(this.screen)
					} catch (e) {}
					this.screen.onselectstart = null,
	        this.screen.ondragstart = null,
	        this.screen.ontouchstart = null,
	        this.screen.ontouchmove = null,
	        this.screen.ontouchend = null,
	        this.screen.onmouseover = null,
	        this.screen.onmouseout = null,
	        this.screen.onmouseup = null,
	        this.screen.onmousedown = null,
					this.screen.onmousemove = null,
	        this.screen.onclick = null,
	        delete this.screen,
	        delete this.style,
	        delete this.rect,
	        delete this.selectable,
					delete this.buttonMode,
	        delete this.position,
					delete this.overflow,
	        delete this.visible,
					delete this.innerHTML,
	        delete this.numChildren,
					delete this.x,
	        delete this.y,
	        delete this.w,
					delete this.h,
	        delete this.opacityType,
					delete this.isHtml5_bl,
	        delete this.hasTransform2d_bl,
	        this.children_ar = null,
	        this.style = null,
	        this.screen = null,
	        this.numChildren = null,
	        this.transform = null,
	        this.position = null,
	        this.overflow = null,
	        this.display = null,
	        this.visible = null,
	        this.buttonMode = null,
					this.globalX = null,
	        this.globalY = null,
	        this.x = null,
	        this.y = null,
					this.w = null,
	        this.h = null,
	        this.rect = null,
					this.alpha = null,
	        this.innerHTML = null,
	        this.opacityType = null,
	        this.isHtml5_bl = null,
	        this.hasTransform3d_bl = null,
	        this.hasTransform2d_bl = null,
	        i = null
				},
	      this.init()
		}
