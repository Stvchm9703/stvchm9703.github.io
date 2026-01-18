import{H as t,I as s,It as a,Lt as e,M as r,N as l,Ot as i,U as o,V as c,b as n,c as d,et as p,ht as v,k as m,kt as f,l as h,lt as g,mt as $,pt as b,y as x,z as u,zt as k}from"./DcaEFj2i.js";import{t as y}from"./BtALZkK9.js";import{t as _}from"./DX-5ybwC.js";import{t as j}from"./jUFu-Rfa.js";import{t as M}from"./sfM9m55h.js";function z(s,a){f(a,!0);
/**
	* @license @lucide/svelte v0.561.0 - ISC
	*
	* ISC License
	*
	* Copyright (c) for portions of Lucide are held by Cole Bemis 2013-2023 as part of Feather (MIT). All other copyright (c) for Lucide are held by Lucide Contributors 2025.
	*
	* Permission to use, copy, modify, and/or distribute this software for any
	* purpose with or without fee is hereby granted, provided that the above
	* copyright notice and this permission notice appear in all copies.
	*
	* THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
	* WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
	* MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
	* ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
	* WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
	* ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
	* OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
	*
	* ---
	*
	* The MIT License (MIT) (for portions derived from Feather)
	*
	* Copyright (c) 2013-2023 Cole Bemis
	*
	* Permission is hereby granted, free of charge, to any person obtaining a copy
	* of this software and associated documentation files (the "Software"), to deal
	* in the Software without restriction, including without limitation the rights
	* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
	* copies of the Software, and to permit persons to whom the Software is
	* furnished to do so, subject to the following conditions:
	*
	* The above copyright notice and this permission notice shall be included in all
	* copies or substantial portions of the Software.
	*
	* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
	* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
	* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
	* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
	* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
	* SOFTWARE.
	*
	*/
let e=d(a,["$$slots","$$events","$$legacy"]);const r=[["path",{d:"M8 2v4"}],["path",{d:"M16 2v4"}],["rect",{width:"18",height:"18",x:"3",y:"4",rx:"2"}],["path",{d:"M3 10h18"}]];_(s,h({name:"calendar"},()=>e,{get iconNode(){return r},children:(s,e)=>{var r=t(),l=$(r);m(l,()=>a.children??k),c(s,r)},$$slots:{default:!0}})),i()}var w=o('<a class="block text-sm text-slate-800 hover:text-slate-500 hover:underline" target="_self"> </a> <i class="h-4 border-r-1 border-r-solid -mb-1"></i>',1),I=o('<article><div class="group"><a class="block font-serif text-xl font-bold mb-2 group-hover:underline capitalize" target="_self"> </a> <div class="flex items-center gap-2 text-muted-foreground my-3"><!> <div class="flex items-center gap-1"><!> <span class="text-sm"> </span></div></div> <p class="font-serif block line-clamp-3 my-3"> </p> <div class="flex gap-2"></div></div></article>');function N(t,o){f(o,!0);const m=d(o,["$$slots","$$events","$$legacy","id","title","title_slot","content","content_slot","post_date","link","tags","serie"]);var k=I();x(k,(t,s)=>({id:t,title:o.title,class:s,...m}),[()=>M("post-card",o.id),()=>y(["block border-b pb-6",o?.class])]);var _=b(k),N=b(_),C=b(N,!0);e(N);var H=v(N,2),L=b(H),O=t=>{var s=w(),r=$(s),l=b(r,!0);e(r),a(2),g(()=>{n(r,"href",o.serie.url),u(l,o.serie.name)}),c(t,s)};s(L,t=>{o.serie&&t(O)});var U=v(L,2),V=b(U);z(V,{class:"h-3 w-3"});var q=v(V,2),A=b(q,!0);e(q),e(U),e(H);var B=v(H,2),D=b(B,!0);e(B);var E=v(B,2);r(E,21,()=>o.tags,l,(t,s)=>{j(t,h(()=>p(s)))}),e(E),e(_),e(k),g(()=>{n(N,"href",o.link),u(C,o.title),u(A,o.post_date),u(D,o.content)}),c(t,k),i()}export{z as n,N as t};