<script>
  // load from language db
  let mFileIcon = `background-image: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="18" height="18" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="7" height="7"></rect><rect x="14" y="3" width="7" height="7"></rect><rect x="14" y="14" width="7" height="7"></rect><rect x="3" y="14" width="7" height="7"></rect></svg>')`
  let mSettingsIcon;
  let mExitIcon;
</script>

<ul class="menu">
  <li class="item">
    Kefer
  </li>
  <li class="item" style={mFileIcon}>
    <a href="/" on:click={() => { window.location.href = "/"; }}>.</a>
  </li>
  <li class="item">
    <a href="/">
      <svg
        viewBox="0 0 24 24"
        width="30%"
        height="30%"
        stroke="currentColor"
        stroke-width="2"
        fill="none"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
        <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"
        ></path>
      </svg>
    </a>
  </li>
  <li class="item">
    <a href="/">X </a>
  </li>
</ul>

<style>
  .menu {
    align-items: center;
    background-color: var(--bg-color);
    column-gap: 0.5rem;
    display: flex;
    flex-direction: row;
    flex-wrap: nowrap;
    justify-content: center;
    padding: 0;
    row-gap: 1rem;
    width: 100%;
  }
  .item {
    align-items: baseline;
    color: var(--main-color);
    background-repeat: no-repeat;
    min-width: 50px;
    display: inline-flex;
    flex-grow: 1;
    column-gap: 0.5rem;
  }
  svg {
    color: var(--main-color);
  }
  @media all and (max-width: 1000px) {
    ul {
      flex-wrap: wrap;
    }

    li {
      flex: 1 1 50%;
      min-width: 50px;
    }
  }
</style>
