<script lang="ts">
  import { auth, login } from "$lib/auth";
  import { theme } from "$lib/theme";
  import { browser } from '$app/environment';
  import { onMount } from 'svelte';
  import { writable } from 'svelte/store';

  // Create a writable store for autoSort state that can be imported by other components
  export const autoSortStore = writable(false);

  export let simName: string | null = null;

  // Auto-sort state
  let autoSortEnabled = false;
  // Local storage key for auto-sort preference
  const AUTO_SORT_STORAGE_KEY = 'simBru_autoSortEnabled';

  // Load auto-sort preference from local storage on mount
  onMount(() => {
    if (browser) {
      const savedPreference = localStorage.getItem(AUTO_SORT_STORAGE_KEY);
      if (savedPreference !== null) {
        autoSortEnabled = savedPreference === 'true';
        // Update the store when loading from localStorage
        autoSortStore.set(autoSortEnabled);
      }
      
      // Debug: Log auth state to check if picture URL is present
      console.log('Auth state:', $auth);
    }
  });

  function toggleAutoSort() {
    autoSortEnabled = !autoSortEnabled;
    
    // Save preference to local storage
    if (browser) {
      localStorage.setItem(AUTO_SORT_STORAGE_KEY, autoSortEnabled.toString());
      // Update the store when the toggle is changed
      autoSortStore.set(autoSortEnabled);
    }
  }

  function toggleTheme() {
    theme.toggle();
  }

  function handleLogout() {
    auth.logout();
    // Redirect to login page
    window.location.href = '/';
  }

  // Track dropdown state
  let isDropdownOpen = false;
  function toggleDropdown() {
    isDropdownOpen = !isDropdownOpen;
  }
  
  // Close dropdown when clicking outside
  function handleClickOutside(event: MouseEvent) {
    const target = event.target as HTMLElement;
    const dropdown = document.getElementById('settingsDropdown');
    const button = document.getElementById('settingsButton');
    
    if (dropdown && button && !dropdown.contains(target) && !button.contains(target)) {
      isDropdownOpen = false;
    }
  }

  // Get user initial for avatar fallback
  function getUserInitial(name: string | undefined): string {
    if (!name) return 'U';
    return name.charAt(0).toUpperCase();
  }

  // Calculate complementary color to primary for the avatar
  // We're using a predefined complementary color that will work well with purple
  const avatarBgColor = "bg-secondary";
</script>

<svelte:window on:click={handleClickOutside} />

<div class="navbar bg-primary text-primary-content shadow-lg">
  <a href={simName ? `/sim/${simName.split(' ')[1]}` : "/"} class="btn btn-ghost text-xl">
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
    {#if $auth.isAuthenticated}
      <div class="relative">
        <button 
          id="settingsButton"
          class="btn btn-ghost btn-circle" 
          on:click={toggleDropdown}
          aria-label="User menu"
        >
          {#if $auth.user?.picture}
            <img 
              src={$auth.user.picture} 
              alt="Profile" 
              class="rounded-full w-10 h-10 object-cover"
              onerror="this.onerror=null; this.style.display='none'; this.nextElementSibling.style.display='flex';"
            />
            <!-- Fallback avatar that's initially hidden but will be shown if image fails to load -->
            <div class="avatar placeholder" style="display: none;">
              <div class={`${avatarBgColor} text-secondary-content rounded-full w-10 h-10`}>
                <span class="text-lg font-semibold">{getUserInitial($auth.user?.name)}</span>
              </div>
            </div>
          {:else}
            <div class="avatar placeholder">
              <div class={`${avatarBgColor} text-secondary-content rounded-full w-10 h-10`}>
                <span class="text-lg font-semibold">{getUserInitial($auth.user?.name)}</span>
              </div>
            </div>
          {/if}
        </button>
        
        {#if isDropdownOpen}
          <div 
            id="settingsDropdown"
            class="absolute right-0 mt-2 w-48 rounded-md shadow-lg bg-base-100 text-base-content z-50"
          >
            <div class="py-1 rounded-md bg-base-100 shadow-xs">
              <!-- User info section -->
              <div class="px-4 py-3 border-b border-base-300">
                <div class="font-medium">{$auth.user?.name || 'User'}</div>
                {#if $auth.user?.email}
                  <div class="text-xs text-base-content/70 truncate">{$auth.user.email}</div>
                {/if}
              </div>

              <div class="px-4 py-2 flex items-center justify-between">
                <span>Theme</span>
                <button class="btn btn-ghost btn-sm btn-circle" on:click={toggleTheme} aria-label="Toggle theme">
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
              </div>

              <div class="border-t border-base-300"></div>
              
              <div class="px-4 py-2 flex items-center justify-between">
                <span>Auto-sort Stats</span>
                <div class="form-control">
                  <label class="cursor-pointer">
                    <input 
                      type="checkbox" 
                      checked={autoSortEnabled} 
                      on:change={toggleAutoSort}
                      class="toggle toggle-primary toggle-sm" 
                    />
                  </label>
                </div>
              </div>

              <div class="border-t border-base-300"></div>
              
              <button 
                class="w-full text-left block px-4 py-2 text-sm hover:bg-base-200"
                on:click={handleLogout}
              >
                Logout
              </button>
            </div>
          </div>
        {/if}
      </div>
    {:else}
      <button class="btn btn-primary" on:click={login}>Login</button>
    {/if}
  </div>
</div>