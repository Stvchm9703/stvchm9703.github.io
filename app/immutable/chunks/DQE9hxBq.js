import{A as t,B as s,D as a,Dt as e,Et as r,Ft as l,L as i,Lt as o,P as c,Pt as n,Q as d,V as v,_ as p,c as f,dt as m,ft as h,j as g,l as $,pt as x,st as b,v as u,z as k}from"./DC_otEb6.js";import{t as _}from"./Nk5D0k5d.js";import{t as j}from"./CF56sTGR.js";import{t as y}from"./CU3vjlEi.js";import{t as M}from"./sfM9m55h.js";function w(t,l){e(l,!0);
/**
	* @license @lucide/svelte v0.562.0 - ISC
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
let i=f(l,["$$slots","$$events","$$legacy"]);const c=[["path",{d:"M8 2v4"}],["path",{d:"M16 2v4"}],["rect",{width:"18",height:"18",x:"3",y:"4",rx:"2"}],["path",{d:"M3 10h18"}]];j(t,$({name:"calendar"},()=>i,{get iconNode(){return c},children:(t,e)=>{var r=s(),i=h(r);a(i,()=>l.children??o),k(t,r)},$$slots:{default:!0}})),r()}var z=v('<a class="block text-sm text-slate-800 hover:text-slate-500 hover:underline" target="_self"> </a> <i class="h-4 border-r-1 border-r-solid -mb-1"></i>',1),D=v('<article><div class="group"><a class="block font-serif text-xl font-bold mb-2 group-hover:underline capitalize" target="_self"> </a> <div class="flex items-center gap-2 text-muted-foreground my-3"><!> <div class="flex items-center gap-1"><!> <span class="text-sm"> </span></div></div> <p class="font-serif block line-clamp-3 my-3"> </p> <div class="flex gap-2"></div></div></article>');function L(s,a){e(a,!0);const o=f(a,["$$slots","$$events","$$legacy","id","title","title_slot","content","content_slot","post_date","link","tags","serie"]);var v=D();p(v,(t,s)=>({id:t,title:a.title,class:s,...o}),[()=>M("post-card",a.id),()=>_(["block border-b pb-6",a?.class])]);var j=m(v),L=m(j),P=m(L,!0);l(L);var A=x(L,2),B=m(A),C=t=>{var s=z(),e=h(s),r=m(e,!0);l(e),n(2),b(()=>{u(e,"href",a.serie.url),i(r,a.serie.name)}),k(t,s)};c(B,t=>{a.serie&&t(C)});var E=x(B,2),F=m(E);w(F,{class:"h-3 w-3"});var N=x(F,2),Q=m(N,!0);l(N),l(E),l(A);var V=x(A,2),q=m(V,!0);l(V);var G=x(V,2);t(G,21,()=>a.tags,g,(t,s)=>{y(t,$(()=>d(s)))}),l(G),l(j),l(v),b(()=>{u(L,"href",a.link),i(P,a.title),i(Q,a.post_date),i(q,a.content)}),k(s,v),r()}export{w as n,L as t};