<script>
  import InputGroup from "./lib/InputGroup.svelte";
  import Navigation from "./lib/Navigation.svelte";
  import Zodiac from "./lib/Zodiac.svelte";
  let innerWidth = 0;
  let innerHeight = 0;

  //$: condition = innerWidth*1.33 <= innerHeight
</script>

<svelte:window bind:innerWidth bind:innerHeight />

<main class="container">
  <nav>
    <Navigation></Navigation>
  </nav>
  <div class="content">
    <main>
      <!--Here be the wheel  --><Zodiac></Zodiac>
    </main>
    <aside>
      <InputGroup></InputGroup>
    </aside>
  </div>
  <footer>datum, summary</footer>
</main>

<style>
  .content {
    display: flex;
    flex-direction: row;
    /*min-height: calc(100vh - 40vh);
    background-color: #f0f0f0;*/
  }
  nav {
    display: flex;
    flex-direction: row;
    background: orangered;
    /*
    margin: 0 auto;
    max-width: var(--main-width);
    padding: 0 var(--main-side-padding);
    width: 100%;
    */
  }
  aside {
    display: flex;
    flex-direction: column;
    background: rgb(209, 209, 89);
    /*width: 100%;
    order: -1;
    flex: 0 0 20vw;*/
  }
  .container {
    display: flex;
    flex-direction: column;
    /*flex-wrap: nowrap;
    height: 100%;*/
  }

  @media all and (min-width: 768px) {
    .content {
      flex-direction: row;
      flex-wrap: wrap;
    }
    main {
      flex: 2;
      order: 2;
      min-height: 80vh;
    }
    aside {
      order: 1;
      flex: 1;
    }
  }
</style>
