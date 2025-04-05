<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  const NUM_FADERS = 12;

  let amps = Array(NUM_FADERS).fill(0);
  const amp = async (e: Event) => {
    e.preventDefault();
    const el = e.currentTarget as HTMLInputElement;
    const id = parseInt(el.id.substring(3));
    // const val = parseFloat(el.value) / 100.0;
    const val = parseFloat(amps[id]) / 100.0;
    console.log(id + ": " + val);
    await invoke("amp", {n: id, val: val});
  }

  const panic = async () => {
    amps.forEach(async (n, i) => {
      n = 0;
      await invoke("amp", {n: i, val: n})
    });
  }
</script>

<main class="container">

  <div class="amps">
  {#each amps as _, n}
  <input class="amps" id="amp{n}" type="range" bind:value={amps[n]} oninput={amp} step="0.001" min="0" max="100" defaultvalue="0">
  {/each}
  </div>

  <button onclick={panic}>PANIC</button>
</main>

<style>
main {
  height: 100%;
  width: 100%;
}

input {
  margin-top: 20%;
  rotate: -90deg;
}

.amps {
  display: flex;
  flex-direction: row;
  flex-grow: 1;
  margin-top: 12em;
  width: 20em;
  justify-content: center;
  align-content: space-between;
}
</style>
