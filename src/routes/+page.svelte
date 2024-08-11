<script lang="ts">
  //import { invoke } from "@tauri-apps/api/core";
  import InputGroup from "../lib/InputGroup.svelte";
  import Navigation from "../lib/Navigation.svelte";
  //import Zodiac from "../lib/Zodiac.svelte";

  let greetMsg = "";
  let innerWidth = 0;
  let innerHeight = 0;

  // Default colors setup
  let mainColor = "#1c62a8";

  // Predefined colors
  const lightColors = ["#FFFFFF", "#333333", "#FF5733"];
  const darkColors = ["#000000", "#CCCCCC", "#FFC300"];

  //$: condition = innerWidth*1.33 <= innerHeight

  function switch_mode() {
    const isDarkMode = document.body.classList.contains("dark-mode");
    const colors = isDarkMode ? darkColors : lightColors;
    //color = color === "black" ? "white" : "black";
    //bgColor = bgColor === "white" ? "black" : "white";
    document.documentElement.style.setProperty("--color", colors[0]);
    document.documentElement.style.setProperty("--bg-color", colors[1]);
    // Toggle the dark-mode class on the body
    document.body.classList.toggle("dark-mode");
  }

  function select_color(event: Event) {
    if (event.target instanceof HTMLInputElement) {
      mainColor = event.target.value;
      document.documentElement.style.setProperty("--main-color", mainColor);
    } else {
      console.log("Different event type, not setting color");
    }
  }
</script>

<svelte:window bind:innerWidth bind:innerHeight />

<main class="container">
  <nav>
    <Navigation></Navigation>
  </nav>
  <div class="content">
    <main>
      <!--Here be the wheel <Zodiac></Zodiac> -->
    </main>
    <aside>
        <button on:click={switch_mode}>Switch mode </button>
        <input type="color" bind:value={mainColor} on:change={select_color} />
        <p>Input details about first event:</p>
        <!-- <DateInput bind:value={dateFirst} timePrecision="second" /> -->
        <p>Input details about second event:</p>
        <!-- <DateInput bind:value={dateSecond} timePrecision="second"/> -->
        <slot/>
      <InputGroup></InputGroup>
    </aside>
  </div>
  <footer>Kefer Astrology (c) 2024</footer>
</main>

  <p>{greetMsg}</p>

<style>
  :root {
    /* start with specific values setup */
    /* later will have a default setting implemented */
    --color: #839496;
    --bg-color: #002B36;
    --main-color: #1c62a8;
  }

  :global(body body.dark-mode) {
		color: var(--main-color);
		transition: background-color 0.3s;
    background-color: var(--bg-color)
	}
  :global(body.dark-mode) {
		color: var(--main-color);
		transition: background-color 0.3s;
    background-color: var(--bg-color)
	}

  .content {
    display: flex;
    flex-direction: row;
    /*min-height: calc(100vh - 40vh);
    background-color: #f0f0f0;*/
  }
  nav {
    display: flex;
    flex-direction: row;
    background: var(--bg-color);
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
    background: var(--bg-color);
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
