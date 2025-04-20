<script lang="ts">
import { onMount } from 'svelte';
import { auth, login } from "$lib/auth";
import Navbar from '$lib/Navbar.svelte';
import { theme } from "$lib/theme";
import '../../assets/app.css';

export let data;

let sims = data.sims;
let newSimName = '';
let loading = false;
let errorMessage = '';
let successMessage = '';
let deleteLoading = false;
let simToDelete = null;
let showDeleteConfirm = false;
let showCreateModal = false; // New state variable to control the create sim modal

// Function to refresh the token if needed
async function refreshAuthIfNeeded() {
  const authState = $auth;
  if (!authState.isAuthenticated || !authState.token) {
    errorMessage = 'You must be logged in to create a sim';
    return false;
  }
  return true;
}

// Function to handle creating a new sim
async function handleCreateSim(event) {
  // Prevent default form submission behavior
  event.preventDefault();
  
  if (!newSimName.trim()) {
    errorMessage = 'Please enter a sim name';
    return;
  }
  
  // Check auth status before proceeding
  if (!await refreshAuthIfNeeded()) {
    return;
  }
  
  errorMessage = '';
  successMessage = '';
  loading = true;
  
  try {
    const newSim = await data.createSim(newSimName);
    if (newSim) {
      sims = [...sims, newSim];
      newSimName = '';
      successMessage = `Sim "${newSim.name}" created successfully!`;
      
      // Hide success message after 5 seconds
      setTimeout(() => {
        successMessage = '';
      }, 5000);
      
      // Close the modal after successful creation
      showCreateModal = false;
    } else {
      errorMessage = 'Failed to create sim. You may need to log out and log back in.';
    }
  } catch (error) {
    console.error('Error creating sim:', error);
    errorMessage = 'Error creating sim. You may need to log out and log back in.';
  } finally {
    loading = false;
  }
}

function navigateToSim(id: number) {
  window.location.href = `/sim/${id}`;
}

function handleLogout() {
  auth.logout();
  // Redirect to login page
  window.location.href = '/';
}

// Function to open delete confirmation modal
function confirmDelete(sim) {
  simToDelete = sim;
  showDeleteConfirm = true;
}

// Function to cancel delete
function cancelDelete() {
  simToDelete = null;
  showDeleteConfirm = false;
}

// Function to handle deleting a sim
async function handleDeleteSim() {
  if (!simToDelete || !await refreshAuthIfNeeded()) {
    return;
  }
  
  deleteLoading = true;
  errorMessage = '';
  
  try {
    const success = await data.deleteSim(simToDelete.id);
    if (success) {
      // Remove the deleted sim from the list
      sims = sims.filter(s => s.id !== simToDelete.id);
      successMessage = `Sim "${simToDelete.name}" deleted successfully!`;
      
      // Hide success message after 5 seconds
      setTimeout(() => {
        successMessage = '';
      }, 5000);
      
      // Close the confirmation dialog
      showDeleteConfirm = false;
      simToDelete = null;
    } else {
      errorMessage = 'Failed to delete sim. You may need to log out and log back in.';
    }
  } catch (error) {
    console.error('Error deleting sim:', error);
    errorMessage = 'Error deleting sim. You may need to log out and log back in.';
  } finally {
    deleteLoading = false;
  }
}

// Function to handle theme toggle
function toggleTheme() {
  theme.toggle();
}

// Function to open create modal
function openCreateModal() {
  errorMessage = '';
  newSimName = '';
  showCreateModal = true;
}

// Function to close create modal
function closeCreateModal() {
  showCreateModal = false;
  errorMessage = '';
}
</script>

<svelte:head>
  <title>Manage Sims</title>
</svelte:head>

<main>
  <Navbar />
  
  <div class="container mx-auto px-4 py-8">
    <div class="flex justify-between items-center mb-8">
      <h1 class="text-3xl font-bold">Manage Your Sims</h1>
      
      {#if $auth.isAuthenticated}
        <button class="btn btn-primary" on:click={openCreateModal}>
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          Create New Sim
        </button>
      {/if}
    </div>
    
    {#if successMessage}
      <div class="alert alert-success mb-6">
        <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
        <span>{successMessage}</span>
      </div>
    {/if}
    
    {#if $auth.isAuthenticated}
      <!-- Sims List -->
      <div class="bg-base-100 rounded-lg shadow-md p-6">
        <h2 class="text-xl font-semibold mb-4">Your Sims</h2>
        
        {#if sims.length === 0}
          <p class="text-base-content opacity-60 py-4">You don't have any sims yet. Click the "Create New Sim" button to get started!</p>
        {:else}
          <div class="overflow-x-auto">
            <table class="table w-full">
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Stats Count</th>
                  <th>Actions</th>
                </tr>
              </thead>
              <tbody>
                {#each sims as sim}
                  <tr class="hover">
                    <td>{sim.name}</td>
                    <td>{sim.stats?.length || 0}</td>
                    <td class="flex gap-2">
                      <button 
                        class="btn btn-sm btn-primary"
                        on:click={() => navigateToSim(sim.id)}
                      >
                        View/Edit
                      </button>
                      <button 
                        class="btn btn-sm btn-error"
                        on:click={() => confirmDelete(sim)}
                      >
                        Delete
                      </button>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}
      </div>
    {:else}
      <div class="bg-base-100 rounded-lg shadow-md p-8 text-center">
        <h2 class="text-2xl mb-4">Please login to manage your sims</h2>
        <button class="btn btn-primary" on:click={login}>Login</button>
      </div>
    {/if}
  </div>
  
  <!-- Create Sim Modal -->
  {#if showCreateModal}
    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-base-100 rounded-lg p-6 max-w-md w-full">
        <h3 class="text-xl font-semibold mb-4">Create New Sim</h3>
        
        <form on:submit={handleCreateSim} class="flex flex-col gap-4">
          <div>
            <label for="simName" class="block text-sm font-medium mb-1">Sim Name</label>
            <input 
              type="text" 
              id="simName" 
              bind:value={newSimName} 
              placeholder="Enter sim name" 
              class="input input-bordered w-full" 
              autofocus
            />
            {#if errorMessage}
              <div class="alert alert-error mt-3 text-sm">
                <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                <span>{errorMessage}</span>
              </div>
            {/if}
          </div>
          <div class="flex justify-end gap-3 mt-2">
            <button 
              type="button" 
              class="btn btn-ghost" 
              on:click={closeCreateModal}
              disabled={loading}
            >
              Cancel
            </button>
            <button 
              type="submit" 
              class="btn btn-primary" 
              disabled={loading || !newSimName.trim()}
            >
              {loading ? 'Creating...' : 'Create Sim'}
            </button>
          </div>
          {#if errorMessage}
            <div class="text-center">
              <button 
                type="button" 
                class="btn btn-outline btn-sm mt-2" 
                on:click={handleLogout}
              >
                Log Out and Try Again
              </button>
            </div>
          {/if}
        </form>
      </div>
    </div>
  {/if}
  
  <!-- Delete Confirmation Modal -->
  {#if showDeleteConfirm}
    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-base-100 rounded-lg p-6 max-w-md w-full">
        <h3 class="text-lg font-semibold mb-4">Confirm Delete</h3>
        <p class="mb-6">Are you sure you want to delete the sim "{simToDelete?.name}"? This action cannot be undone.</p>
        
        {#if errorMessage}
          <div class="alert alert-error mb-4 text-sm">
            <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
            <span>{errorMessage}</span>
          </div>
        {/if}
        
        <div class="flex justify-end gap-3">
          <button 
            class="btn btn-ghost" 
            on:click={cancelDelete}
            disabled={deleteLoading}
          >
            Cancel
          </button>
          <button 
            class="btn btn-error" 
            on:click={handleDeleteSim}
            disabled={deleteLoading}
          >
            {deleteLoading ? 'Deleting...' : 'Delete'}
          </button>
        </div>
      </div>
    </div>
  {/if}
</main>

<style lang="postcss">
  :global(html) {
    background-color: hsl(var(--b2));
  }
</style>