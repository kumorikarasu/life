<script lang="ts">
  export let name: string
  let bg = '#333'
</script>

<div>
<p>{name}</p>
<input type="range" min="0" max="100" value="40" class="range" />
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
