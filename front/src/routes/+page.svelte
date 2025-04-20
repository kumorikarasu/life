<script lang="ts">
import { initializeApp } from "firebase/app";
import { getMessaging, getToken } from "firebase/messaging";

import '../assets/app.css'
import svelteLogo from '../assets/svelte.svg'
import Counter from '../lib/Counter.svelte'
import Range from '../lib/Range.svelte'
import Navbar from '../lib/Navbar.svelte'
import { firebaseConfig, registerSW } from "../lib/firebase.ts";
import { auth, login } from "../lib/auth";
import { theme } from "../lib/theme";

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
  <Navbar simName={data?.sim?.name || ''} />
  
  {#if $auth.isAuthenticated}
    {#if data?.sim}
      <div class="container mx-auto py-6">
        <div class="bg-base-100 rounded-lg shadow-md p-6 mb-6">
          <h1 class="text-2xl font-bold mb-4">{data.sim.name}</h1>
          <div class="flex justify-between items-center mb-6">
            <p class="text-base-content opacity-70">Sim ID: {data.sim.id}</p>
            <a href="/sims" class="btn btn-sm btn-outline">Back to All Sims</a>
          </div>
          
          {#if data.sim.stats && data.sim.stats.length > 0}
            <div class="grid grid-cols-1 gap-8 px-4">
              {#each data.sim.stats as stat}
                <Range name={stat.name} bind:value={stat.value}/>
              {/each}
            </div>
          {:else}
            <div class="text-center py-8">
              <p class="text-base-content opacity-60">This sim doesn't have any stats yet.</p>
            </div>
          {/if}
        </div>
      </div>
    {:else}
      <div class="container mx-auto mt-8 text-center">
        <div class="bg-base-100 rounded-lg shadow-md p-8">
          <h2 class="text-xl mb-4">No sim selected or sim not found</h2>
          <a href="/sims" class="btn btn-primary">Browse Your Sims</a>
        </div>
      </div>
    {/if}
  {:else}
    <div class="container mx-auto mt-8 text-center">
      <div class="bg-base-100 rounded-lg shadow-md p-8">
        <h2 class="text-2xl mb-4">Please login to view your sim</h2>
        <button class="btn btn-primary" on:click={login}>Login</button>
      </div>
    </div>
  {/if}
</main>

<style lang="postcss">
  :global(html) {
    background-color: hsl(var(--b2));
  }
</style>
