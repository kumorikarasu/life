<script lang="ts">
import { initializeApp } from "firebase/app";
import { getMessaging, getToken } from "firebase/messaging";

import '../assets/app.css'
import svelteLogo from '../assets/svelte.svg'
import Counter from '../lib/Counter.svelte'
import Range from '../lib/Range.svelte'
import { firebaseConfig, registerSW } from "../lib/firebase.ts";
import { auth, login } from "../lib/auth";

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
$: data.saveData(data.sim);
</script>

<main>
  <div class="navbar bg-base-100">
    <a href="/" class="btn btn-ghost text-xl">Sim {data?.sim?.name || ''}</a>
    <div class="flex-none">
      {#if $auth.isAuthenticated}
        <button class="btn btn-ghost" on:click={() => auth.logout()}>Logout</button>
      {:else}
        <button class="btn btn-primary" on:click={login}>Login</button>
      {/if}
    </div>
  </div>
  
  {#if $auth.isAuthenticated}
    <div class="container mx-auto">
      <div class="grid grid-cols-1 gap-8 px-4">
        {#each data?.sim?.stats || [] as stat }
        <Range name="{stat.name}" bind:value={stat.value}/>
        {/each}
      </div>
    </div>
  {:else}
    <div class="container mx-auto mt-8 text-center">
      <h2 class="text-2xl mb-4">Please login to view your sim</h2>
      <button class="btn btn-primary" on:click={login}>Login</button>
    </div>
  {/if}
</main>

<style lang="postcss">
  :global(html) {
    background-color: theme(colors.gray.100);
  }
</style>
