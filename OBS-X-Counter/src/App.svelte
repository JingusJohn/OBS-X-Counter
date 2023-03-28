<script lang="ts">
  import NewCounter from "./lib/NewCounter.svelte";
  import Counter from "./lib/Counter.svelte";
  import { counterSchema, type CounterType } from "./lib/models";
  import { listen } from "@tauri-apps/api/event";
  import {
    isRegistered,
    register,
    registerAll,
    unregister,
  } from "@tauri-apps/api/globalShortcut";
  import { onDestroy, onMount } from "svelte";

  async function registerShortcuts() {
    await registerAll(["CmdOrControl+Shift+Alt+Up", "CmdOrControl+Shift+Alt+Down"], (shortcut) => {
      message = shortcut;
      if (selected) {
        switch (shortcut) {
          case "CmdOrControl+Shift+Alt+Up": {
            selected.count++;
            selected = selected;
            break;
          }
          case "CmdOrControl+Shift+Alt+Down": {
            if (selected.count > 0 || selected.allowNegative) {
              selected.count--;
              selected = selected;
            }
            break;
          }
        }
      }
    });

    // await register("CmdOrControl+Shift+Alt+Down", () => {
    //   message = "decrement actuated";
    //   if (selected) {
    //     if (selected.count > 0) {
    //       selected.count--;
    //       selected = selected;
    //     }
    //   }
    // });
  }

  onMount(async () => {
    await registerShortcuts();
  });

  onDestroy(async () => {
    await unregister("CmdOrControl+Shift+Alt+Up");
    await unregister("CmdOrControl+Shift+Alt+Down");
  });

  let mode: string = "Counters";
  let counters: CounterType[] = []; // load from tauri command locally later
  let selected: CounterType;
  let newCounter: boolean = false;
  let message: string;

  function handleCreate(event) {
    const newCounter: CounterType = counterSchema.parse({
      title: event.detail.name,
      count: 0,
      formatMode: "equation",
    });
    counters.push(newCounter);
    counters = counters;
    selected = newCounter;
  }
</script>

<main class="container no-select">
  <h1>OBS-X-Counter</h1>
  <h1>{message}</h1>

  {#if counters.length < 1 || newCounter}
    <NewCounter on:create={handleCreate} />
  {/if}

  <!-- Counter bar here -->

  {#if selected}
    <Counter
      on:update={(event) => (selected = event.detail.counter)}
      counter={selected}
    />

    <div>
      {selected.count}
    </div>
    <div>
      {selected.formatMode}
    </div>
    <div>
      {selected.allowNegative}
    </div>
  {/if}
</main>
