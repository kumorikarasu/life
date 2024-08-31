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

</script>

<main>
<div class="navbar bg-base-100">
<a href="/" class="btn btn-ghost text-xl">Sim Bruna</a>
</div>

<div class="container mx-auto">

  <div class="grid grid-cols-1 gap-8 px-4">
        <Range name="Energy" />
        <Range name="Hunger" />
        <Range name="Fun" />
        <Range name="Social" />
        <Range name="Hygiene" />
  </div>
</main>

<style lang="postcss">
  :global(html) {
    background-color: theme(colors.gray.100);
  }
</style>
