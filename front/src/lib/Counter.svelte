<script lang="ts">
  const sw = navigator.serviceWorker
  let count: number = 0
  const increment = () => {
    count += 1;
    new PushEvent('push', { data: "hi" });
  }
</script>

<button on:click={increment}>
  count is {count}
</button>
