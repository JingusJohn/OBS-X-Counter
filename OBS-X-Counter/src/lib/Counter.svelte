<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { CounterType } from "./models";
  export let counter: CounterType;

  const dispatch = createEventDispatcher();

  let editingName: boolean = false;

  function update() {
    counter = counter;
    dispatch("update", {
      counter
    });
  }

  function handleIncrement(event) {
    counter.count++;
    update();
  }

  function handleDecrement(event) {
    if (counter.count > 0) {
      counter.count--;
      update();
    }
  }
</script>

<div class="row">
  {#if editingName}
    <input class="counter-name-input" bind:value={counter.title} />
    <button on:click={handleIncrement}>Save</button>
  {:else}
    <h2 on:click={() => (editingName = true)}>{counter.title}</h2>
  {/if}
</div>

<div>
  <button on:click={handleIncrement}>+</button>
  <p>{counter.count}</p>
  <button on:click={handleDecrement}>-</button>
</div>
