<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  const NUM_FADERS = 16;


  let amps = Array(NUM_FADERS).fill(0);
  let row1 = [...Array(NUM_FADERS/2).keys()];
  let row2 = [...Array(NUM_FADERS/2).keys()];
  row2.forEach((n, i, arr ) => { let num = NUM_FADERS / 2; arr[i] = n + num; });
  console.log(row1)
  console.log(row2)

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
    let inputs = [...document.getElementsByTagName('input')];
    amps.forEach(async (n, i) => {
      n = 0;
      inputs.find((e, i) => {if (e.id == "amp"+i) { e.value = "0"; e.defaultValue = "0";}});
      await invoke("amp", {n: i, val: n})
    });
  }
</script>

<main class="container">

  <div class="amps">
  {#each row1 as n}
  <input id="amp{n}" type="range" bind:value={amps[n]} oninput={amp} step="0.001" min="0" max="100" defaultvalue="0">
  {/each}
  </div>
  <div class="amps">
  {#each row2 as n}
  <input id="amp{n}" type="range" bind:value={amps[n]} oninput={amp} step="0.001" min="0" max="100" defaultvalue="0">
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
  margin-top: 10%;
  rotate: -90deg;
  margin-left: 2em;
}

.amps {
  display: flex;
  flex-direction: row;
  flex-grow: 1;
  margin-top: 12em;
  width: 100%;
  justify-content: center;
  align-content: space-between;
}
</style>
