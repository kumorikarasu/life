<script lang="ts">
import { initializeApp } from "firebase/app";
import { getMessaging, getToken } from "firebase/messaging";

import '../assets/app.css'
import svelteLogo from '../assets/svelte.svg'
import Counter from '../lib/Counter.svelte'
import Range from '../lib/Range.svelte'
import { firebaseConfig, registerSW } from "../lib/firebase.ts";

const app = initializeApp(firebaseConfig);
let messaging = null;
if (typeof window !== 'undefined'){
  messaging = getMessaging(app);
}

async function requestPermission() {
  const registration = await registerSW();
  const permission = await Notification.requestPermission();
  if (permission === "granted") {
    // Generate Device Token for notification
    const token = await getToken(messaging, {
      vapidKey:  import.meta.env.VITE_VAPID_KEY,
      serviceWorkerRegistration: registration,
    });
  } else if (permission === "denied") {
    console.log("Denied for the notification");
  }
}
requestPermission();

export let data;
$: data.saveData(data);
</script>

<main>
  <div class="navbar bg-base-100">
    <a href="/" class="btn btn-ghost text-xl">Sim {data.sim.name}</a>
  </div>
  <div class="container mx-auto">
    <div class="grid grid-cols-1 gap-8 px-4">
      {#each data.sim.stats as [key, value]}
      <Range name="{key}" bind:value={value}/>
      {/each}
    </div>
  </div>
</main>

<style lang="postcss">
  :global(html) {
    background-color: theme(colors.gray.100);
  }
</style>
