<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  let name = "";
  // greetMsg left only for example
  let greetMsg = ""

  function createCounter() {
    dispatch('create', {
      name
    });
  }

  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name })
  }
</script>

<div>
  <div class="row">
    <input id="counter-name-input" placeholder="Name your counter..." bind:value={name} />
    <button on:click={createCounter}>
      Count
    </button>
  </div>
</div>
