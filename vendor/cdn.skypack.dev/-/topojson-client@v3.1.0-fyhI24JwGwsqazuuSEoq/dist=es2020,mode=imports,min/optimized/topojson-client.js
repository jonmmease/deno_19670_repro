function P(n){return n}function w(n){if(n==null)return P;var u,s,c=n.scale[0],i=n.scale[1],l=n.translate[0],y=n.translate[1];return function(b,f){f||(u=s=0);var t=2,r=b.length,e=new Array(r);for(e[0]=(u+=b[0])*c+l,e[1]=(s+=b[1])*i+y;t<r;)e[t]=b[t],++t;return e}}function M(n){var u=w(n.transform),s,c=Infinity,i=c,l=-c,y=-c;function b(t){t=u(t),t[0]<c&&(c=t[0]),t[0]>l&&(l=t[0]),t[1]<i&&(i=t[1]),t[1]>y&&(y=t[1])}function f(t){switch(t.type){case"GeometryCollection":t.geometries.forEach(f);break;case"Point":b(t.coordinates);break;case"MultiPoint":t.coordinates.forEach(b);break}}n.arcs.forEach(function(t){for(var r=-1,e=t.length,a;++r<e;)a=u(t[r],r),a[0]<c&&(c=a[0]),a[0]>l&&(l=a[0]),a[1]<i&&(i=a[1]),a[1]>y&&(y=a[1])});for(s in n.objects)f(n.objects[s]);return[c,i,l,y]}function q(n,u){for(var s,c=n.length,i=c-u;i<--c;)s=n[i],n[i++]=n[c],n[c]=s}function z(n,u){return typeof u=="string"&&(u=n.objects[u]),u.type==="GeometryCollection"?{type:"FeatureCollection",features:u.geometries.map(function(s){return k(n,s)})}:k(n,u)}function k(n,u){var s=u.id,c=u.bbox,i=u.properties==null?{}:u.properties,l=E(n,u);return s==null&&c==null?{type:"Feature",properties:i,geometry:l}:c==null?{type:"Feature",id:s,properties:i,geometry:l}:{type:"Feature",id:s,bbox:c,properties:i,geometry:l}}function E(n,u){var s=w(n.transform),c=n.arcs;function i(r,e){e.length&&e.pop();for(var a=c[r<0?~r:r],o=0,p=a.length;o<p;++o)e.push(s(a[o],o));r<0&&q(e,p)}function l(r){return s(r)}function y(r){for(var e=[],a=0,o=r.length;a<o;++a)i(r[a],e);return e.length<2&&e.push(e[0]),e}function b(r){for(var e=y(r);e.length<4;)e.push(e[0]);return e}function f(r){return r.map(b)}function t(r){var e=r.type,a;switch(e){case"GeometryCollection":return{type:e,geometries:r.geometries.map(t)};case"Point":a=l(r.coordinates);break;case"MultiPoint":a=r.coordinates.map(l);break;case"LineString":a=y(r.arcs);break;case"MultiLineString":a=r.arcs.map(y);break;case"Polygon":a=f(r.arcs);break;case"MultiPolygon":a=r.arcs.map(f);break;default:return null}return{type:e,coordinates:a}}return t(u)}function A(n,u){var s={},c={},i={},l=[],y=-1;u.forEach(function(t,r){var e=n.arcs[t<0?~t:t],a;e.length<3&&!e[1][0]&&!e[1][1]&&(a=u[++y],u[y]=t,u[r]=a)}),u.forEach(function(t){var r=b(t),e=r[0],a=r[1],o,p;if(o=i[e])if(delete i[o.end],o.push(t),o.end=a,p=c[a]){delete c[p.start];var h=p===o?o:o.concat(p);c[h.start=o.start]=i[h.end=p.end]=h}else c[o.start]=i[o.end]=o;else if(o=c[a])if(delete c[o.start],o.unshift(t),o.start=e,p=i[e]){delete i[p.end];var d=p===o?o:p.concat(o);c[d.start=p.start]=i[d.end=o.end]=d}else c[o.start]=i[o.end]=o;else o=[t],c[o.start=e]=i[o.end=a]=o});function b(t){var r=n.arcs[t<0?~t:t],e=r[0],a;return n.transform?(a=[0,0],r.forEach(function(o){a[0]+=o[0],a[1]+=o[1]})):a=r[r.length-1],t<0?[a,e]:[e,a]}function f(t,r){for(var e in t){var a=t[e];delete r[a.start],delete a.start,delete a.end,a.forEach(function(o){s[o<0?~o:o]=1}),l.push(a)}}return f(i,c),f(c,i),u.forEach(function(t){s[t<0?~t:t]||l.push([t])}),l}function _(n){return E(n,G.apply(this,arguments))}function G(n,u,s){var c,i,l;if(arguments.length>1)c=j(n,u,s);else for(i=0,c=new Array(l=n.arcs.length);i<l;++i)c[i]=i;return{type:"MultiLineString",arcs:A(n,c)}}function j(n,u,s){var c=[],i=[],l;function y(e){var a=e<0?~e:e;(i[a]||(i[a]=[])).push({i:e,g:l})}function b(e){e.forEach(y)}function f(e){e.forEach(b)}function t(e){e.forEach(f)}function r(e){switch(l=e,e.type){case"GeometryCollection":e.geometries.forEach(r);break;case"LineString":b(e.arcs);break;case"MultiLineString":case"Polygon":f(e.arcs);break;case"MultiPolygon":t(e.arcs);break}}return r(u),i.forEach(s==null?function(e){c.push(e[0].i)}:function(e){s(e[0].g,e[e.length-1].g)&&c.push(e[0].i)}),c}function B(n){for(var u=-1,s=n.length,c,i=n[s-1],l=0;++u<s;)c=i,i=n[u],l+=c[0]*i[1]-c[1]*i[0];return Math.abs(l)}function F(n){return E(n,C.apply(this,arguments))}function C(n,u){var s={},c=[],i=[];u.forEach(l);function l(f){switch(f.type){case"GeometryCollection":f.geometries.forEach(l);break;case"Polygon":y(f.arcs);break;case"MultiPolygon":f.arcs.forEach(y);break}}function y(f){f.forEach(function(t){t.forEach(function(r){(s[r=r<0?~r:r]||(s[r]=[])).push(f)})}),c.push(f)}function b(f){return B(E(n,{type:"Polygon",arcs:[f]}).coordinates[0])}return c.forEach(function(f){if(!f._){var t=[],r=[f];for(f._=1,i.push(t);f=r.pop();)t.push(f),f.forEach(function(e){e.forEach(function(a){s[a<0?~a:a].forEach(function(o){o._||(o._=1,r.push(o))})})})}}),c.forEach(function(f){delete f._}),{type:"MultiPolygon",arcs:i.map(function(f){var t=[],r;if(f.forEach(function(h){h.forEach(function(d){d.forEach(function(g){s[g<0?~g:g].length<2&&t.push(g)})})}),t=A(n,t),(r=t.length)>1)for(var e=1,a=b(t[0]),o,p;e<r;++e)(o=b(t[e]))>a&&(p=t[0],t[0]=t[e],t[e]=p,a=o);return t}).filter(function(f){return f.length>0})}}function S(n,u){for(var s=0,c=n.length;s<c;){var i=s+c>>>1;n[i]<u?s=i+1:c=i}return s}function I(n){var u={},s=n.map(function(){return[]});function c(h,d){h.forEach(function(g){g<0&&(g=~g);var v=u[g];v?v.push(d):u[g]=[d]})}function i(h,d){h.forEach(function(g){c(g,d)})}function l(h,d){h.type==="GeometryCollection"?h.geometries.forEach(function(g){l(g,d)}):h.type in y&&y[h.type](h.arcs,d)}var y={LineString:c,MultiLineString:i,Polygon:i,MultiPolygon:function(h,d){h.forEach(function(g){i(g,d)})}};n.forEach(l);for(var b in u)for(var f=u[b],t=f.length,r=0;r<t;++r)for(var e=r+1;e<t;++e){var a=f[r],o=f[e],p;(p=s[a])[b=S(p,o)]!==o&&p.splice(b,0,o),(p=s[o])[b=S(p,a)]!==a&&p.splice(b,0,a)}return s}function L(n){if(n==null)return P;var u,s,c=n.scale[0],i=n.scale[1],l=n.translate[0],y=n.translate[1];return function(b,f){f||(u=s=0);var t=2,r=b.length,e=new Array(r),a=Math.round((b[0]-l)/c),o=Math.round((b[1]-y)/i);for(e[0]=a-u,u=a,e[1]=o-s,s=o;t<r;)e[t]=b[t],++t;return e}}function T(n,u){if(n.transform)throw new Error("already quantized");if(!u||!u.scale){if(!((y=Math.floor(u))>=2))throw new Error("n must be \u22652");f=n.bbox||M(n);var s=f[0],c=f[1],i=f[2],l=f[3],y;u={scale:[i-s?(i-s)/(y-1):1,l-c?(l-c)/(y-1):1],translate:[s,c]}}else f=n.bbox;var b=L(u),f,t,r=n.objects,e={};function a(h){return b(h)}function o(h){var d;switch(h.type){case"GeometryCollection":d={type:"GeometryCollection",geometries:h.geometries.map(o)};break;case"Point":d={type:"Point",coordinates:a(h.coordinates)};break;case"MultiPoint":d={type:"MultiPoint",coordinates:h.coordinates.map(a)};break;default:return h}return h.id!=null&&(d.id=h.id),h.bbox!=null&&(d.bbox=h.bbox),h.properties!=null&&(d.properties=h.properties),d}function p(h){var d=0,g=1,v=h.length,x,m=new Array(v);for(m[0]=b(h[0],0);++d<v;)((x=b(h[d],d))[0]||x[1])&&(m[g++]=x);return g===1&&(m[g++]=[0,0]),m.length=g,m}for(t in r)e[t]=o(r[t]);return{type:"Topology",bbox:f,transform:u,objects:e,arcs:n.arcs.map(p)}}export{M as bbox,z as feature,F as merge,C as mergeArcs,_ as mesh,G as meshArcs,I as neighbors,T as quantize,w as transform,L as untransform};export default null;
