<script>
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let greetMsg = ""

  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("translation", { name })
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <p>Additional settings:</p>
    <input id="greet-input" placeholder="Input details..." bind:value={name} />
    <button type="submit">Show translation</button>
  </form>
  <p>{greetMsg}</p>
</div>

