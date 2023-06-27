<nav class="navbar w-auto bg-base-100 sticky top-0 bottom-0 z-8000">
  <section class="flex-1">
    <a class="btn btn-ghost normal-case text-xl" href="/"></a>
  </section>
  <section class="flex-none">
    <ul class="menu menu-horizontal px-1">
      <li><a>Item 1</a></li>
      <li tabindex="0">
        <a href="/blog">
          Blog
          <svg
            class="fill-current"
            xmlns="http://www.w3.org/2000/svg"
            width="20"
            height="20"
            viewBox="0 0 24 24"
            ><path
              d="M7.41,8.58L12,13.17L16.59,8.58L18,10L12,16L6,10L7.41,8.58Z"
            /></svg
          >
        </a>
        <ul class="p-2 bg-base-100">
          <li><a href="/blog?tag=dev">Developer</a></li>
          <li><a href="/blog?tag=data-analysis">Data Analysis</a></li>
        </ul>
      </li>
      <li><a>Item 3</a></li>
    </ul>
  </section>
</nav>
