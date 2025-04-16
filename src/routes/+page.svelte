<script lang="ts">
  //import { invoke } from "@tauri-apps/api/core";
  import InputGroup from "$lib/InputGroup.svelte";
  import Navigation from "$lib/Navigation.svelte";
  //import Zodiac from "$lib/Zodiac_custom.svelte";
  import Zodiac from "$lib/Zodiac_custom.svelte";
  import CollapsibleSection from "$lib/CollapsibleSection.svelte";

  let greetMsg = "";
  let innerWidth = 0;
  let innerHeight = 0;
  let isResizing = false;
  let startX = 0;
  let startWidth = 0;

  // Default colors setup
  let mainColor = "#1c62a8";

  // Predefined colors [ foreground, shade, background ]
  const lightColors = ["#1c1e1e", "#3c4752", "#6896a3"];
  const darkColors = ["#e8eeee", "#0d2d4e", "#0d429e"];

  //$: condition = innerWidth*1.33 <= innerHeight

  function switch_mode() {
    const isDarkMode = document.body.classList.contains("dark-mode");
    const colors = isDarkMode ? darkColors : lightColors;
    //color = color === "black" ? "white" : "black";
    //bgColor = bgColor === "white" ? "black" : "white";
    document.documentElement.style.setProperty("--color", colors[0]);
    document.documentElement.style.setProperty("--main-color", colors[1]);
    document.documentElement.style.setProperty("--bg-color", colors[2]);
    // Toggle the dark-mode class on the body
    document.body.classList.toggle("dark-mode");
  }

  function select_color(event: Event) {
    // main color replacement
    if (event.target instanceof HTMLInputElement) {
      mainColor = event.target.value;
      document.documentElement.style.setProperty("--main-color", mainColor);
    } else {
      console.log("Different event type, not setting color");
    }
  }

  function startResize(event: MouseEvent) {
    isResizing = true;
    startX = event.clientX;
    startWidth = document.querySelector('aside')?.offsetWidth || 0;
    document.addEventListener('mousemove', resize);
    document.addEventListener('mouseup', stopResize);
  }

  function resize(event: MouseEvent) {
    if (isResizing) {
      const newWidth = startWidth + (event.clientX - startX);
      const aside = document.querySelector('aside');
      if (aside) {
        aside.style.width = `${newWidth}px`;
      }
    }
  }

  function stopResize() {
    isResizing = false;
    document.removeEventListener('mousemove', resize);
    document.removeEventListener('mouseup', stopResize);
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
      <Zodiac></Zodiac>
    </main>
    <aside>
      <!-- <div class="resize-handle" role="separator" on:mousedown={startResize} aria-orientation="vertical">
      </div> -->
      <CollapsibleSection headerText={'Radek M'} >
        <div class="content">
            Look at all this fun content
        </div>
        <button on:click={switch_mode}>Switch mode </button>
        <input type="color" bind:value={mainColor} on:change={select_color} />
      </CollapsibleSection>
      <CollapsibleSection headerText={'Navigátor časoprostoru'} >
        <div class="content">
          <p>Input details about first event:</p>
          <!-- <DateInput bind:value={dateFirst} timePrecision="second" /> -->
          <p>Input details about second event:</p>
          <!-- <DateInput bind:value={dateSecond} timePrecision="second"/> -->
        </div>
      </CollapsibleSection>
      <CollapsibleSection headerText={'Tranzity'} >
        <div class="content">
          <InputGroup></InputGroup>
        </div>
      </CollapsibleSection>
    </aside>
  </div>
  <footer>Kefer Astrology (c) 2024</footer>
</main>

  <p>{greetMsg}</p>

<style>
  :root {
    /* start with specific values setup */
    /* later will have a default setting implemented */
    --color: #1c1e1e;
    --main-color: #3c4752;
    --bg-color: #6896a3;
  }

  :global(body body.dark-mode) {
		color: var(--main-color);
		/* transition: background-color 0.3s ease; */
    background-color: var(--bg-color)
	}

  .container {
    background: linear-gradient(135deg, var(--bg-color), var(--main-color));
    display: flex;
    margin: 0;
    padding: 0;
    flex-direction: column;
    transition: background-color 0.3s ease
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
  }
  aside {
    display: flex;
    flex-direction: column;
    min-width: 5vw; /* Minimum width to prevent collapsing */
    max-width: 22vw; /* Maximum width to prevent excessive expansion */
    overflow: auto; /* Ensures content is scrollable if it overflows */
    position: relative;
    padding: 1vw;
    resize: horizontal; /* Allows horizontal resizing */
    width: 10vw; /* Initial width */
  }

  .resize-handle {
    width: 10px;
    cursor: ew-resize;
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.1);
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
