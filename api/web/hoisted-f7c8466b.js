var O=typeof globalThis<"u"?globalThis:typeof window<"u"?window:typeof global<"u"?global:typeof self<"u"?self:{};function k(a){return a&&a.__esModule&&Object.prototype.hasOwnProperty.call(a,"default")?a.default:a}function F(a){if(a.__esModule)return a;var v=a.default;if(typeof v=="function"){var h=function c(){return this instanceof c?Reflect.construct(v,arguments,this.constructor):v.apply(this,arguments)};h.prototype=v.prototype}else h={};return Object.defineProperty(h,"__esModule",{value:!0}),Object.keys(a).forEach(function(c){var _=Object.getOwnPropertyDescriptor(a,c);Object.defineProperty(h,c,_.get?_:{enumerable:!0,get:function(){return a[c]}})}),h}var j={exports:{}};function D(a){throw new Error('Could not dynamically require "'+a+'". Please configure the dynamicRequireTargets or/and ignoreDynamicRequires option of @rollup/plugin-commonjs appropriately for this require call to work.')}var P={exports:{}};const $={},U=Object.freeze(Object.defineProperty({__proto__:null,default:$},Symbol.toStringTag,{value:"Module"})),M=F(U);var E;function N(){return E||(E=1,function(a,v){(function(h,c){a.exports=c()})(O,function(){var h=h||function(c,_){var u;if(typeof window<"u"&&window.crypto&&(u=window.crypto),typeof self<"u"&&self.crypto&&(u=self.crypto),typeof globalThis<"u"&&globalThis.crypto&&(u=globalThis.crypto),!u&&typeof window<"u"&&window.msCrypto&&(u=window.msCrypto),!u&&typeof O<"u"&&O.crypto&&(u=O.crypto),!u&&typeof D=="function")try{u=M}catch{}var I=function(){if(u){if(typeof u.getRandomValues=="function")try{return u.getRandomValues(new Uint32Array(1))[0]}catch{}if(typeof u.randomBytes=="function")try{return u.randomBytes(4).readInt32LE()}catch{}}throw new Error("Native crypto module could not be used to get secure random number.")},S=Object.create||function(){function t(){}return function(e){var r;return t.prototype=e,r=new t,t.prototype=null,r}}(),C={},b=C.lib={},m=b.Base=function(){return{extend:function(t){var e=S(this);return t&&e.mixIn(t),(!e.hasOwnProperty("init")||this.init===e.init)&&(e.init=function(){e.$super.init.apply(this,arguments)}),e.init.prototype=e,e.$super=this,e},create:function(){var t=this.extend();return t.init.apply(t,arguments),t},init:function(){},mixIn:function(t){for(var e in t)t.hasOwnProperty(e)&&(this[e]=t[e]);t.hasOwnProperty("toString")&&(this.toString=t.toString)},clone:function(){return this.init.prototype.extend(this)}}}(),p=b.WordArray=m.extend({init:function(t,e){t=this.words=t||[],e!=_?this.sigBytes=e:this.sigBytes=t.length*4},toString:function(t){return(t||g).stringify(this)},concat:function(t){var e=this.words,r=t.words,o=this.sigBytes,s=t.sigBytes;if(this.clamp(),o%4)for(var f=0;f<s;f++){var l=r[f>>>2]>>>24-f%4*8&255;e[o+f>>>2]|=l<<24-(o+f)%4*8}else for(var d=0;d<s;d+=4)e[o+d>>>2]=r[d>>>2];return this.sigBytes+=s,this},clamp:function(){var t=this.words,e=this.sigBytes;t[e>>>2]&=4294967295<<32-e%4*8,t.length=c.ceil(e/4)},clone:function(){var t=m.clone.call(this);return t.words=this.words.slice(0),t},random:function(t){for(var e=[],r=0;r<t;r+=4)e.push(I());return new p.init(e,t)}}),x=C.enc={},g=x.Hex={stringify:function(t){for(var e=t.words,r=t.sigBytes,o=[],s=0;s<r;s++){var f=e[s>>>2]>>>24-s%4*8&255;o.push((f>>>4).toString(16)),o.push((f&15).toString(16))}return o.join("")},parse:function(t){for(var e=t.length,r=[],o=0;o<e;o+=2)r[o>>>3]|=parseInt(t.substr(o,2),16)<<24-o%8*4;return new p.init(r,e/2)}},w=x.Latin1={stringify:function(t){for(var e=t.words,r=t.sigBytes,o=[],s=0;s<r;s++){var f=e[s>>>2]>>>24-s%4*8&255;o.push(String.fromCharCode(f))}return o.join("")},parse:function(t){for(var e=t.length,r=[],o=0;o<e;o++)r[o>>>2]|=(t.charCodeAt(o)&255)<<24-o%4*8;return new p.init(r,e)}},n=x.Utf8={stringify:function(t){try{return decodeURIComponent(escape(w.stringify(t)))}catch{throw new Error("Malformed UTF-8 data")}},parse:function(t){return w.parse(unescape(encodeURIComponent(t)))}},i=b.BufferedBlockAlgorithm=m.extend({reset:function(){this._data=new p.init,this._nDataBytes=0},_append:function(t){typeof t=="string"&&(t=n.parse(t)),this._data.concat(t),this._nDataBytes+=t.sigBytes},_process:function(t){var e,r=this._data,o=r.words,s=r.sigBytes,f=this.blockSize,l=f*4,d=s/l;t?d=c.ceil(d):d=c.max((d|0)-this._minBufferSize,0);var H=d*f,B=c.min(H*4,s);if(H){for(var A=0;A<H;A+=f)this._doProcessBlock(o,A);e=o.splice(0,H),r.sigBytes-=B}return new p.init(e,B)},clone:function(){var t=m.clone.call(this);return t._data=this._data.clone(),t},_minBufferSize:0});b.Hasher=i.extend({cfg:m.extend(),init:function(t){this.cfg=this.cfg.extend(t),this.reset()},reset:function(){i.reset.call(this),this._doReset()},update:function(t){return this._append(t),this._process(),this},finalize:function(t){t&&this._append(t);var e=this._doFinalize();return e},blockSize:16,_createHelper:function(t){return function(e,r){return new t.init(r).finalize(e)}},_createHmacHelper:function(t){return function(e,r){return new y.HMAC.init(t,r).finalize(e)}}});var y=C.algo={};return C}(Math);return h})}(P)),P.exports}(function(a,v){(function(h,c){a.exports=c(N())})(O,function(h){return function(c){var _=h,u=_.lib,I=u.WordArray,S=u.Hasher,C=_.algo,b=[],m=[];(function(){function g(y){for(var t=c.sqrt(y),e=2;e<=t;e++)if(!(y%e))return!1;return!0}function w(y){return(y-(y|0))*4294967296|0}for(var n=2,i=0;i<64;)g(n)&&(i<8&&(b[i]=w(c.pow(n,1/2))),m[i]=w(c.pow(n,1/3)),i++),n++})();var p=[],x=C.SHA256=S.extend({_doReset:function(){this._hash=new I.init(b.slice(0))},_doProcessBlock:function(g,w){for(var n=this._hash.words,i=n[0],y=n[1],t=n[2],e=n[3],r=n[4],o=n[5],s=n[6],f=n[7],l=0;l<64;l++){if(l<16)p[l]=g[w+l]|0;else{var d=p[l-15],H=(d<<25|d>>>7)^(d<<14|d>>>18)^d>>>3,B=p[l-2],A=(B<<15|B>>>17)^(B<<13|B>>>19)^B>>>10;p[l]=H+p[l-7]+A+p[l-16]}var T=r&o^~r&s,z=i&y^i&t^y&t,W=(i<<30|i>>>2)^(i<<19|i>>>13)^(i<<10|i>>>22),q=(r<<26|r>>>6)^(r<<21|r>>>11)^(r<<7|r>>>25),R=f+q+T+m[l]+p[l],L=W+z;f=s,s=o,o=r,r=e+R|0,e=t,t=y,y=i,i=R+L|0}n[0]=n[0]+i|0,n[1]=n[1]+y|0,n[2]=n[2]+t|0,n[3]=n[3]+e|0,n[4]=n[4]+r|0,n[5]=n[5]+o|0,n[6]=n[6]+s|0,n[7]=n[7]+f|0},_doFinalize:function(){var g=this._data,w=g.words,n=this._nDataBytes*8,i=g.sigBytes*8;return w[i>>>5]|=128<<24-i%32,w[(i+64>>>9<<4)+14]=c.floor(n/4294967296),w[(i+64>>>9<<4)+15]=n,g.sigBytes=w.length*4,this._process(),this._hash},clone:function(){var g=S.clone.call(this);return g._hash=this._hash.clone(),g}});_.SHA256=S._createHelper(x),_.HmacSHA256=S._createHmacHelper(x)}(Math),h.SHA256})})(j);var V=j.exports;const G=k(V);function J(){document.getElementById("username")?.value;var a=document.getElementById("password")?.value||"";return K(a).then(v=>{if(v==="c6f2bb56844c94dd2065b9425c32cd8246ee6f208937e476970cd3b0324d6365")Q(),console.log("Contraseña correcta"),localStorage.setItem("isLoggedIn","true"),window.location.href="http://103.23.60.158:80/home";else return window.location.href="http://103.23.60.158:80/index",console.log("Contraseña incorrecta"),!1}).catch(v=>{console.error("Error al calcular el hash:",v)}),!0}async function K(a){return G(a).toString()}async function Q(){try{var a=document.getElementById("username")?.value;const h=await(await fetch("http://103.23.60.158:80/loged_in",{method:"POST",headers:{"Content-Type":"application/json","Access-Control-Allow-Methods":"POST","Access-Control-Allow-Headers":"Content-Type"},body:JSON.stringify({mail:a})})).json();console.log(h)}catch(v){console.log(v)}}addEventListener("submit",J,!1);const X=localStorage.getItem("isLoggedIn")==="true";X&&(window.location.href="http://103.23.60.158:80/home");
