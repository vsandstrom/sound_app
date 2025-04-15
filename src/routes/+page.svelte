<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
    import { parse } from "svelte/compiler";

  const NUM_FADERS = 16;
  const NUM_FILTER = 12;
  let amps = Array(NUM_FADERS).fill("0");
  let filt = Array(NUM_FADERS).fill("0");
  let mod: "0";
  let fm = "0";
  let fb = "0";
  let mix = "0";
  let row1 = [...Array(NUM_FADERS/2).keys()];
  let row2 = [...Array(NUM_FADERS/2).keys()];
  let row3 = [...Array(NUM_FILTER).keys()];
  row2.forEach((n, i, arr ) => { let num = NUM_FADERS / 2; arr[i] = n + num; });

  const amp = async (e: Event) => {
    e.preventDefault();
    const id = parseInt((e.currentTarget as HTMLInputElement).id.substring(3));
    await invoke("amp", {n: id, val: parseAndNormalize(amps[id])});
  }
  
  const filter = async (e: Event) => {
    e.preventDefault();
    const id = parseInt((e.currentTarget as HTMLInputElement).id.substring(3));
    await invoke("filter", {n: id, val: parseAndNormalize(filt[id])});
  }

  const modulation = async (e: Event) => {
    e.preventDefault();
    await invoke("modulation", {val: parseAndNormalize(mod)});
  }
  
  const freqmod = async (e: Event) => {
    e.preventDefault();
    await invoke("fm", {val: parseAndNormalize(fm)});
  }
  
  const feedback = async (e: Event) => {
    e.preventDefault();
    await invoke("fb", {val: parseAndNormalize(fb)});
  }

  const filterMix = async(e: Event) => {
    e.preventDefault();
    await invoke("mix", {val: parseAndNormalize(mix)})
  }
  

  const panic = async (e: Event) => {
    e.preventDefault();
    let inputs = [...document.getElementsByTagName('input')];
    amps.forEach(async (n, i) => {
      n = 0;
      inputs.find((e, i) => {if (e.id == "amp"+i) { e.value = "0"; e.defaultValue = "0";}});
      await invoke("amp", {n: i, val: n})
    });
  }

  const parseAndNormalize = (val: string): number => { return parseFloat(val) / 100.0 }
</script>

<main class="container">

  <div class="modulation">
    <input id="modulation" type="range" bind:value={mod} oninput={modulation} step="0.001" min="0" max="100" defaultvalue="0">
    <input id="freqmod" type="range" bind:value={fm} oninput={freqmod} step="0.001" min="0" max="100" defaultvalue="0">
    <input id="feedback" type="range" bind:value={fb} oninput={feedback} step="0.001" min="0" max="100" defaultvalue="0">
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
  <div class="amps amps2 filters">
    {#each row3 as n}
    <input class="filter" id="flt{n}" type="range" bind:value={filt[n]} oninput={filter} step="0.001" min="0" max="100" defaultvalue="0">
    {/each}
  </div>
  <div class="modulation">
    <input id="mix" type="range" bind:value={mix} oninput={filterMix} step="0.001" min="0" max="100" defaultvalue="0">
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

.filter {
  margin-top: 10%;
  rotate: -90deg;
  margin-left: -7.5em;
}

.modulation {
  margin-top: 10%;
  margin-left: 8%;
}

#modulation, #freqmod, #feedback, #mix {
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

.filters {
  margin-left: 1em;

}
</style>
