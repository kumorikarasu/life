<script lang="ts">
  export let name: string
  export let value: number
  let bg = '#333'

  function getColor(val) {

  }

  $: bg = "hsl(" + (Math.round(value / 30) * 30) + ", 90%, 50%)"
</script>

<div class="mb-4">
<p class="text-base-content">{name}</p>
<input style="--range-shdw: {bg}" type="range" min="0" max="100" bind:value={value} class="range" />
<div class="flex justify-between text-xs text-base-content opacity-70 px-1 mt-1">
  <span>0</span>
  <span>25</span>
  <span>50</span>
  <span>75</span>
  <span>100</span>
</div>
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
