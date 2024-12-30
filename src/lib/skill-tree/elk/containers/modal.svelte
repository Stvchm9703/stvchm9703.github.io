<script lang="ts">
  import { run, self } from 'svelte/legacy';

  let dataShow = $state(false);

  interface Props {
    title?: string;
    description?: string;
    isShow?: boolean;
    // $: () => (isShow ? (dataShow = true) : void 0);
    onCloseClick?: any;
    children?: import('svelte').Snippet;
  }

  let {
    title = "",
    description = "",
    isShow = false,
    onCloseClick = () => {},
    children
  }: Props = $props();

  const internalCloseClick = () => {
    dataShow = false;
    onCloseClick();
  };
  run(() => {
    if (isShow) dataShow = true;
  });

  const children_render = $derived(children);
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="cds--modal {dataShow ? 'is-visible' : ''}" data-modal="true" onclick={self(internalCloseClick)}>
  <div class="cds--modal-container cds--modal-container">
    <div class="cds--modal-header cds--modal-header">
      <p
        class="cds--modal-header__label cds--type-delta cds--modal-header__label cds--type-delta"
        id="modal-title"
      >
        {title}
      </p>

      <button
        class="cds--modal-close cds--modal-close"
        type="button"
        data-modal-close=""
        aria-label="close modal"
        data-modal-primary-focus=""
        onclick={internalCloseClick}
      >
        <svg
          focusable="false"
          preserveAspectRatio="xMidYMid meet"
          xmlns="http://www.w3.org/2000/svg"
          fill="currentColor"
          aria-label="Close"
          width="20"
          height="20"
          viewBox="0 0 32 32"
          role="img"
          class="cds--modal-close__icon cds--modal-close__icon"
        >
          <path
            d="M24 9.4L22.6 8 16 14.6 9.4 8 8 9.4 14.6 16 8 22.6 9.4 24 16 17.4 22.6 24 24 22.6 17.4 16 24 9.4z"
          ></path>
        </svg>
      </button>
    </div>

    <div class="cds--modal-content cds--modal-content">
      {@render children_render?.()}
    </div>

    <div class="cds--modal-footer cds--modal-footer">
      <div class="cds--cc-modal-footer-spacer"></div>
      <!-- <button
          class="cds--btn cds--btn--primary cds--btn cds--btn--primary"
          type="button"
          data-modal-primary-focus="">Download as CSV</button
        > -->
    </div>
  </div>
</div>
