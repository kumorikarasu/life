<script lang="ts">
  export let name: string
  export let value: number
  let bg = '#333'

  function getColor(val) {

  }

  $: bg = "hsl(" + (Math.round(value / 30) * 30) + ", 90%, 50%)"
</script>

<div>
<p>{name}</p>
<input style="--range-shdw: {bg}" type="range" min="0" max="100" bind:value={value} class="range" />
</div>

<style>
  p {
    font-size: 1.3rem;
    line-height: 3rem;
    margin: 10;

  }
  .range {
    width: 100%;
    --range-shdw: var(--bg, #fff);
  }
</style>
