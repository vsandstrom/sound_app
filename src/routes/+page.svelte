<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  const NUM_FADERS = 16;


  let amps = Array(NUM_FADERS).fill("0");
  let mod: "0";
  let row1 = [...Array(NUM_FADERS/2).keys()];
  let row2 = [...Array(NUM_FADERS/2).keys()];
  row2.forEach((n, i, arr ) => { let num = NUM_FADERS / 2; arr[i] = n + num; });

  const amp = async (e: Event) => {
    e.preventDefault();
    const el = e.currentTarget as HTMLInputElement;
    const id = parseInt(el.id.substring(3));
    const val = normalize_amps(parseFloat(amps[id]));
    console.log("osv " + id + ": " + val);
    await invoke("amp", {n: id, val: val});
  }

  const modulation = async (e: Event) => {
    e.preventDefault();
    const val = normalize_amps(parseFloat(mod));
    console.log("modulation: " + val);
    await invoke("modulation", {val: val});
}

  const panic = async () => {
    let inputs = [...document.getElementsByTagName('input')];
    amps.forEach(async (n, i) => {
      n = 0;
      inputs.find((e, i) => {if (e.id == "amp"+i) { e.value = "0"; e.defaultValue = "0";}});
      await invoke("amp", {n: i, val: n})
    });
  }


  const normalize_amps = (val: number): number => {
    return val / 100.0
  }
</script>

<main class="container">

  <div class="modulation">
    <input id="modulation" type="range" bind:value={mod} oninput={modulation} step="0.001" min="0" max="100" defaultvalue="0">
  </div>
  <div class="amps">
  {#each row1 as n}
  <input class="amp" id="amp{n}" type="range" bind:value={amps[n]} oninput={amp} step="0.001" min="0" max="100" defaultvalue="0">
  {/each}
  </div>
  <div class="amps amps2">
  {#each row2 as n}
  <input class="amp" id="amp{n}" type="range" bind:value={amps[n]} oninput={amp} step="0.001" min="0" max="100" defaultvalue="0">
  {/each}
  </div>

  <button onclick={panic}>PANIC</button>
</main>

<style>
main {
  justify-content: center;
  height: 100%;
  width: 100%;
}

.amp {
  margin-top: 10%;
  rotate: -90deg;
  margin-left: -5em;
}

.modulation {
  margin-top: 10%;
  margin-left: 8%;
}

#modulation {
  width: 200px;
}

.amps {
  margin-top: 4%;
  display: flex;
  align-self: center;
  justify-content: center;
  flex-direction: row;
  flex-grow: 1;
  width: 100%;
  justify-content: center;
  align-content: space-between;
}

.amps2 {
  margin-top: 10%;
}
</style>
