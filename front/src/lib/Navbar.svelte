<script lang="ts">
  import { auth, login } from "$lib/auth";
  import { theme } from "$lib/theme";

  export let simName: string | null = null;

  function toggleTheme() {
    theme.toggle();
  }

  function handleLogout() {
    auth.logout();
    // Redirect to login page
    window.location.href = '/';
  }
</script>

<div class="navbar bg-base-100">
  <a href="/" class="btn btn-ghost text-xl">
    {#if simName}
      Sim {simName}
    {:else}
      SimBru
    {/if}
  </a>
  <div class="flex-1 justify-center">
    {#if $auth.isAuthenticated}
      <a href="/sims" class="btn btn-ghost">Manage Sims</a>
    {/if}
  </div>
  <div class="flex-none">
    <button class="btn btn-ghost btn-circle" on:click={toggleTheme} aria-label="Toggle theme">
      {#if $theme === 'light'}
        <!-- Sun icon for light mode -->
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
          <path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386-1.591 1.591M21 12h-2.25m-.386 6.364-1.591-1.591M12 18.75V21m-4.773-4.227-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z" />
        </svg>
      {:else}
        <!-- Moon icon for dark mode -->
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
          <path stroke-linecap="round" stroke-linejoin="round" d="M21.752 15.002A9.72 9.72 0 0 1 18 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 0 0 3 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 0 0 9.002-5.998Z" />
        </svg>
      {/if}
    </button>
    {#if $auth.isAuthenticated}
      <div class="flex items-center gap-4">
        <span class="text-sm font-medium">Welcome, {$auth.user?.name || 'User'}</span>
        <button class="btn btn-ghost" on:click={handleLogout}>Logout</button>
      </div>
    {:else}
      <button class="btn btn-primary" on:click={login}>Login</button>
    {/if}
  </div>
</div>