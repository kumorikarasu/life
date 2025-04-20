<script lang="ts">
import { initializeApp } from "firebase/app";
import { getMessaging, getToken } from "firebase/messaging";

import '../assets/app.css'
import Navbar from '$lib/Navbar.svelte'
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
</script>

<svelte:head>
  <title>SimBru - Home</title>
</svelte:head>

<main>
  <Navbar />
  
  <div class="container mx-auto py-8">
    <div class="bg-base-100 rounded-lg shadow-md p-8 text-center">
      <h1 class="text-4xl font-bold mb-6">Welcome to SimBru</h1>
      
      {#if $auth.isAuthenticated}
        <div class="py-4">
          <h2 class="text-2xl font-semibold mb-4">Your Simulations</h2>
          <p class="mb-6">Manage your existing simulations or create new ones.</p>
          <div class="flex justify-center gap-4">
            <a href="/sims" class="btn btn-primary">
              View Your Sims
            </a>
          </div>
        </div>
      {:else}
        <div class="py-4">
          <p class="mb-6">Please login to access and manage your simulations.</p>
          <button class="btn btn-primary" on:click={login}>Login</button>
        </div>
      {/if}
    </div>
  </div>
</main>

<style lang="postcss">
  :global(html) {
    background-color: hsl(var(--b2));
  }
</style>
