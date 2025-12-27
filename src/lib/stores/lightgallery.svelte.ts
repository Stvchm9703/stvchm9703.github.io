// import { $state } from "svelte";
import lightGallery from "lightgallery";
import lgZoom from "lightgallery/plugins/zoom";
import "lightgallery/css/lightgallery.css";
import "lightgallery/css/lg-thumbnail.css";
import "lightgallery/css/lg-zoom.css";

import "./lightgallery.css";
// import { getContext, setContext } from "svelte";
let lightgallery = null;
let items = [];
export const initLightGallery = (el: HTMLElement, page_img_item: any[]) => {
  let lgElm = document.getElementsByClassName("lg-container");

  if (lightgallery === null && lgElm.length === 0) {
    items = page_img_item.map((item, idx) => ({
      id: idx,
      path: item.url,
      src: `/blog/assets/${item.url}`,
      thumb: `/blog/assets/${item.url}`,
      subHtml: "",
    }));

    lightgallery = lightGallery(el, {
      speed: 300,
      mode: "lg-fade",
      plugins: [lgZoom],
      dynamic: true,
      download: false,
      dynamicEl: items,
    });
  }
};

export const openGallery = (images: string) => {
  const itemsIdx = items.find((p) => p.path === images);
  if (lightgallery !== null) {
    lightgallery?.openGallery(itemsIdx.id);
  }
};
