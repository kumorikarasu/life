<script lang="ts">
import '../../../assets/app.css'
import Range from '$lib/Range.svelte'
import Navbar from '$lib/Navbar.svelte'
import { auth, login } from "$lib/auth";

export let data;

// State variables for stat creation and editing
let showCreateStatModal = false;
let showEditStatModal = false;
let showDeleteStatConfirm = false;
let newStatName = '';
let newStatValue = 50;
let newStatDecayRate = 1/60; // Default to 1 unit per minute
let newStatDecaySlider = 50; // Default slider position (mid-range)
let editingStatName = '';
let editingStatValue = 50;
let editingStatDecayRate = 1/60; // Default to 1 unit per minute
let editingStatDecaySlider = 50; // Default slider position (mid-range)
let statToDelete = '';
let errorMessage = '';
let successMessage = '';
let loading = false;
// Store stat order in this array
let statOrder = [];

// Update statOrder when data.sim.stats changes
$: if (data?.sim?.stats && (!statOrder.length || statOrder.length !== data.sim.stats.length)) {
  // Sort stats by order_index if available, otherwise use the order received from the backend
  const sortedStats = [...data.sim.stats].sort((a, b) => {
    // If both stats have order_index, sort by that
    if (a.order_index !== undefined && b.order_index !== undefined) {
      return a.order_index - b.order_index;
    }
    // Fall back to the order received from the backend
    return 0;
  });
  
  // Extract just the names for our statOrder array
  statOrder = sortedStats.map(stat => stat.name);
}

// Function to reorder stats
async function handleReorderStat(statName, direction, positions = 1) {
  const currentIndex = statOrder.indexOf(statName);
  if (currentIndex < 0) return;
  
  // Calculate the target index based on direction and positions to move
  let newIndex;
  if (direction === 'up') {
    // Move up by the specified number of positions
    newIndex = Math.max(0, currentIndex - positions);
  } else if (direction === 'down') {
    // Move down by the specified number of positions
    newIndex = Math.min(statOrder.length - 1, currentIndex + positions);
  } else {
    // Invalid direction
    return;
  }
  
  // No need to reorder if the index hasn't changed
  if (newIndex === currentIndex) return;
  
  // Create a new array for the updated order
  const newOrder = [...statOrder];
  
  // Remove the stat from its current position
  newOrder.splice(currentIndex, 1);
  
  // Insert it at the new position
  newOrder.splice(newIndex, 0, statName);
  
  // Update the state
  statOrder = newOrder;
  
  // Save the new order to the backend
  saveStatOrder();
}

// Function to save the stat order to the backend
async function saveStatOrder() {
  // Create an array of [name, index] pairs
  const orderData = statOrder.map((name, index) => [name, index]);
  
  // Call the API to update the order
  try {
    const success = await data.updateStatsOrder(data.sim.id, orderData);
    if (!success) {
      console.error('Failed to save stat order');
    }
  } catch (error) {
    console.error('Error saving stat order:', error);
  }
}

// Function to convert slider value (0-300) to decay rate (logarithmic scale)
function getDecayRateFromSlider(sliderValue) {
  // Special case: If slider is at minimum (0), return 0 for infinite decay
  if (sliderValue === 0) {
    return 0; // Infinite (no decay)
  }
  
  // For the rest of the slider range (1-300), map to decay rates from 1 per day to 10 per second
  // Adjusted to use 1-300 instead of 0-300 for the decay range
  // Slider: 1   -> 1/(24*60*60) (1 per day)
  // Slider: 75  -> 1/(60*60) (1 per hour)
  // Slider: 150 -> 1/60 (1 per minute)
  // Slider: 225 -> 1/10 (1 per 10 seconds)
  // Slider: 300 -> 10 (10 per second)
  
  // Convert to logarithmic scale
  const minRate = 1/(24*60*60); // 1 per day
  const maxRate = 10; // 10 per second
  
  // Using an exponential function to map slider to decay rate
  const normalized = (sliderValue - 1) / 299; // 0 to 1 (adjusted for 1-300 range)
  const exponent = normalized * (Math.log(maxRate) - Math.log(minRate)) + Math.log(minRate);
  return Math.exp(exponent);
}

// Function to convert decay rate to slider value (0-300)
function getSliderFromDecayRate(decayRate) {
  // Special case for infinite decay (0)
  if (decayRate === 0) {
    return 0;
  }
  
  const minRate = 1/(24*60*60); // 1 per day
  const maxRate = 10; // 10 per second
  
  // Inverse of the exponential function used in getDecayRateFromSlider
  const normalized = (Math.log(decayRate) - Math.log(minRate)) / (Math.log(maxRate) - Math.log(minRate));
  // Adjust for 1-300 scale (instead of 0-300)
  return Math.max(1, Math.min(300, normalized * 299 + 1));
}

// Function to get human-readable description of decay rate
function getDecayRateDescription(decayRate) {
  // Check for infinite (no decay)
  if (decayRate === 0) {
    return "Infinite";
  }
  // Format the decay rate based on its magnitude
  else if (decayRate >= 1) {
    // More than 1 per second
    return `${decayRate.toFixed(1)} per second`;
  } else if (decayRate >= 1/60) {
    // More than 1 per minute
    return `${(decayRate * 60).toFixed(1)} per minute`;
  } else if (decayRate >= 1/3600) {
    // More than 1 per hour
    return `${(decayRate * 3600).toFixed(1)} per hour`;
  } else {
    // Per day
    return `${(decayRate * 86400).toFixed(1)} per day`;
  }
}

// Suggested stat types that would make sense for a life simulation
const suggestedStats = [
  'Energy', 'Hunger', 'Thirst', 'Sleep', 'Hygiene', 
  'Fun', 'Social', 'Stress', 'Health', 'Fitness',
  'Mood', 'Intelligence', 'Creativity', 'Money', 'Career'
];

// Function to open the create stat modal
function openCreateStatModal() {
  errorMessage = '';
  successMessage = '';
  newStatName = '';
  newStatValue = 50; // Reset to default
  newStatDecayRate = 1/60; // Reset to default (1 per minute)
  newStatDecaySlider = 50; // Reset to default (mid-range)
  showCreateStatModal = true;
}

// Function to close the create stat modal
function closeCreateStatModal() {
  showCreateStatModal = false;
  errorMessage = '';
}

// Function to handle decay slider change in create modal
function handleCreateDecaySliderChange(event) {
  const sliderValue = parseFloat(event.target.value);
  newStatDecaySlider = sliderValue;
  newStatDecayRate = getDecayRateFromSlider(sliderValue);
}

// Function to handle decay slider change in edit modal
function handleEditDecaySliderChange(event) {
  const sliderValue = parseFloat(event.target.value);
  editingStatDecaySlider = sliderValue;
  editingStatDecayRate = getDecayRateFromSlider(sliderValue);
}

// Function to create a new stat
async function handleCreateStat(event) {
  event.preventDefault();
  
  if (!newStatName.trim()) {
    errorMessage = 'Please enter a stat name';
    return;
  }
  
  // Check if stat already exists
  if (data.sim.stats.some(stat => stat.name.toLowerCase() === newStatName.trim().toLowerCase())) {
    errorMessage = 'A stat with this name already exists';
    return;
  }

  // Validate value is between 0 and 100
  if (newStatValue < 0 || newStatValue > 100) {
    errorMessage = 'Initial value must be between 0 and 100';
    return;
  }

  // Validate decay rate is non-negative
  if (newStatDecayRate < 0) {
    errorMessage = 'Decay rate cannot be negative';
    return;
  }
  
  errorMessage = '';
  loading = true;
  
  try {
    const newStat = await data.createStat(data.sim.id, newStatName.trim(), newStatValue, newStatDecayRate);
    if (newStat) {
      // Add the new stat to the sim object
      data.sim.stats = [...data.sim.stats, newStat];
      
      // Show success message and close modal
      successMessage = `Stat "${newStat.name}" created successfully!`;
      showCreateStatModal = false;
      
      // Clear success message after 3 seconds
      setTimeout(() => {
        successMessage = '';
      }, 3000);
    } else {
      errorMessage = 'Failed to create stat. Please try again.';
    }
  } catch (error) {
    console.error('Error creating stat:', error);
    errorMessage = 'An error occurred while creating the stat.';
  } finally {
    loading = false;
  }
}

// Function to open the edit stat modal
function openEditStatModal(statName) {
  errorMessage = '';
  const stat = data.sim.stats.find(s => s.name === statName);
  if (stat) {
    editingStatName = stat.name;
    editingStatValue = stat.value;
    editingStatDecayRate = stat.decay_rate || 1/60;
    editingStatDecaySlider = getSliderFromDecayRate(editingStatDecayRate);
    showEditStatModal = true;
  }
}

// Function to close the edit stat modal
function closeEditStatModal() {
  showEditStatModal = false;
  errorMessage = '';
}

// Function to handle editing a stat
async function handleEditStat(event) {
  event.preventDefault();
  
  // Validate decay rate is non-negative
  if (editingStatDecayRate < 0) {
    errorMessage = 'Decay rate cannot be negative';
    return;
  }
  
  errorMessage = '';
  loading = true;
  
  try {
    // Get the current value of the stat to pass it unchanged
    const stat = data.sim.stats.find(s => s.name === editingStatName);
    if (!stat) {
      errorMessage = 'Stat not found';
      loading = false;
      return;
    }
    
    const success = await data.updateStat(data.sim.id, editingStatName, stat.value, editingStatDecayRate);
    if (success) {
      // Update only the decay_rate in the sim object, leave value unchanged
      data.sim.stats = data.sim.stats.map(s => {
        if (s.name === editingStatName) {
          return {
            ...s,
            decay_rate: editingStatDecayRate
          };
        }
        return s;
      });
      
      // Show success message and close modal
      successMessage = `Decay rate for "${editingStatName}" updated successfully!`;
      showEditStatModal = false;
      
      // Clear success message after 3 seconds
      setTimeout(() => {
        successMessage = '';
      }, 3000);
    } else {
      errorMessage = 'Failed to update decay rate. Please try again.';
    }
  } catch (error) {
    console.error('Error updating decay rate:', error);
    errorMessage = 'An error occurred while updating the decay rate.';
  } finally {
    loading = false;
  }
}

// Function to open delete confirmation
function confirmDeleteStat(statName) {
  statToDelete = statName;
  showDeleteStatConfirm = true;
}

// Function to cancel delete
function cancelDeleteStat() {
  showDeleteStatConfirm = false;
  statToDelete = '';
}

// Function to handle deleting a stat
async function handleDeleteStat() {
  if (!statToDelete) return;
  
  loading = true;
  errorMessage = '';
  
  try {
    const success = await data.deleteStat(data.sim.id, statToDelete);
    if (success) {
      // Remove the stat from the sim object
      data.sim.stats = data.sim.stats.filter(stat => stat.name !== statToDelete);
      
      // Show success message and close modal
      successMessage = `Stat "${statToDelete}" deleted successfully!`;
      showDeleteStatConfirm = false;
      statToDelete = '';
      
      // Clear success message after 3 seconds
      setTimeout(() => {
        successMessage = '';
      }, 3000);
    } else {
      errorMessage = 'Failed to delete stat. Please try again.';
    }
  } catch (error) {
    console.error('Error deleting stat:', error);
    errorMessage = 'An error occurred while deleting the stat.';
  } finally {
    loading = false;
  }
}

// Function to handle stat value changes directly
async function handleStatValueChange(statName, newValue) {
  const stat = data.sim.stats.find(s => s.name === statName);
  if (stat) {
    // Only update if the value has actually changed
    if (stat.value !== newValue) {
      const success = await data.updateStat(data.sim.id, statName, newValue, stat.decay_rate || 1/60);
      if (!success) {
        // If update fails, revert to previous value
        data.sim.stats = data.sim.stats.map(s => {
          if (s.name === statName) {
            return { ...s, value: stat.value };
          }
          return s;
        });
        console.error('Failed to update stat value');
      }
    }
  }
}

// Function to select a suggested stat
function selectSuggestion(suggestion) {
  newStatName = suggestion;
}
</script>

<svelte:head>
  <title>{data?.sim?.name || 'Sim Details'}</title>
</svelte:head>

<main>
  <Navbar simName={data?.sim?.name || ''} />
  
  {#if $auth.isAuthenticated}
    {#if data?.sim}
      <div class="container mx-auto py-6">
        {#if successMessage}
          <div class="alert alert-success mb-4">
            <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
            <span>{successMessage}</span>
          </div>
        {/if}
        
        <div class="bg-base-100 rounded-lg shadow-md p-6 mb-6">
          <div class="flex justify-between items-center mb-6">
            <div class="flex items-center gap-4">
              <h1 class="text-2xl font-bold">{data.sim.name}</h1>
              <p class="text-sm text-base-content opacity-80 italic flex items-center">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 inline-block mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4" />
                </svg>
                Drag stats to reorder them
              </p>
            </div>
            <div class="flex gap-2">
              <button class="btn btn-primary btn-sm" on:click={openCreateStatModal}>
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                </svg>
                Add Stat
              </button>
              <a href="/sims" class="btn btn-sm btn-outline">Back to All Sims</a>
            </div>
          </div>
          
          {#if data.sim.stats && data.sim.stats.length > 0}
            <div class="grid grid-cols-1 gap-3 px-4">
              {#each statOrder as statName, index}
                {#each data.sim.stats as stat}
                  {#if stat.name === statName}
                    <Range 
                      name={stat.name} 
                      value={stat.value} 
                      decay_rate={stat.decay_rate || 1/60}
                      onEdit={openEditStatModal}
                      onDelete={confirmDeleteStat}
                      onValueChange={handleStatValueChange}
                      onReorder={handleReorderStat}
                      draggable={true}
                      index={index}
                      totalStats={statOrder.length}
                    />
                  {/if}
                {/each}
              {/each}
            </div>
          {:else}
            <div class="text-center py-8">
              <p class="text-base-content opacity-60">This sim doesn't have any stats yet. Click the "Add Stat" button to get started!</p>
            </div>
          {/if}
        </div>
      </div>
    {:else}
      <div class="container mx-auto mt-8 text-center">
        <div class="bg-base-100 rounded-lg shadow-md p-8">
          <h2 class="text-xl mb-4">Sim not found</h2>
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

<!-- Create Stat Modal -->
{#if showCreateStatModal}
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
    <div class="bg-base-100 rounded-lg p-6 max-w-md w-full">
      <h3 class="text-xl font-semibold mb-4">Add New Stat</h3>
      
      <form on:submit={handleCreateStat} class="flex flex-col gap-4">
        <div>
          <label for="statName" class="block text-sm font-medium mb-1">Stat Name</label>
          <input 
            type="text" 
            id="statName" 
            bind:value={newStatName} 
            placeholder="Enter stat name" 
            class="input input-bordered w-full" 
            autofocus
          />
        </div>

        <div>
          <label for="statValue" class="block text-sm font-medium mb-1">Initial Value (0-100)</label>
          <input 
            type="number" 
            id="statValue" 
            bind:value={newStatValue} 
            min="0"
            max="100"
            step="1"
            class="input input-bordered w-full" 
          />
        </div>

        <div>
          <label for="createDecaySlider" class="block text-sm font-medium mb-1">Decay Rate</label>
          <input 
            type="range" 
            id="createDecaySlider" 
            min="0" 
            max="300" 
            step="1" 
            bind:value={newStatDecaySlider} 
            on:input={handleCreateDecaySliderChange}
            class="range range-primary"
          />
          <div class="flex justify-between text-xs text-base-content opacity-70 px-1 mt-1">
            <span>Infinite</span>
            <span>Slow</span>
            <span>Medium</span>
            <span>Fast</span>
            <span>Very Fast</span>
          </div>
          <div class="bg-base-200 rounded-md p-3 mt-2">
            <p class="text-sm font-medium text-base-content">
              Current setting: <span class="font-bold">{getDecayRateDescription(newStatDecayRate)}</span>
            </p>
            <p class="text-xs text-base-content opacity-70 mt-1">
              {#if newStatDecayRate === 0}
                This stat will <span class="font-semibold">never decay</span> and will maintain its value indefinitely.
              {:else}
                This stat will decrease by 1 point {getDecayRateDescription(newStatDecayRate)}.
                The higher the decay rate, the more frequently you'll need to maintain this stat.
              {/if}
            </p>
          </div>
        </div>
        
        <div>
          <p class="text-sm font-medium mb-2">Suggested Stats:</p>
          <div class="flex flex-wrap gap-2">
            {#each suggestedStats as suggestion}
              <button 
                type="button"
                class="badge badge-outline hover:bg-primary hover:text-primary-content transition-colors cursor-pointer p-3"
                on:click={() => selectSuggestion(suggestion)}
              >
                {suggestion}
              </button>
            {/each}
          </div>
        </div>
        
        {#if errorMessage}
          <div class="alert alert-error mt-3 text-sm">
            <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
            <span>{errorMessage}</span>
          </div>
        {/if}
        
        <div class="flex justify-end gap-3 mt-2">
          <button 
            type="button" 
            class="btn btn-ghost" 
            on:click={closeCreateStatModal}
            disabled={loading}
          >
            Cancel
          </button>
          <button 
            type="submit" 
            class="btn btn-primary" 
            disabled={loading || !newStatName.trim()}
          >
            {loading ? 'Creating...' : 'Create Stat'}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}

<!-- Edit Stat Modal -->
{#if showEditStatModal}
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
    <div class="bg-base-100 rounded-lg p-6 max-w-md w-full">
      <h3 class="text-xl font-semibold mb-4">Edit Stat: {editingStatName}</h3>
      
      <form on:submit={handleEditStat} class="flex flex-col gap-4">
        <div>
          <label for="editDecaySlider" class="block text-sm font-medium mb-1">Decay Rate</label>
          <input 
            type="range" 
            id="editDecaySlider" 
            min="0" 
            max="300" 
            step="1" 
            bind:value={editingStatDecaySlider} 
            on:input={handleEditDecaySliderChange}
            class="range range-primary"
          />
          <div class="flex justify-between text-xs text-base-content opacity-70 px-1 mt-1">
            <span>Infinite</span>
            <span>Slow</span>
            <span>Medium</span>
            <span>Fast</span>
            <span>Very Fast</span>
          </div>
          <div class="bg-base-200 rounded-md p-3 mt-2">
            <p class="text-sm font-medium text-base-content">
              Current setting: <span class="font-bold">{getDecayRateDescription(editingStatDecayRate)}</span>
            </p>
            <p class="text-xs text-base-content opacity-70 mt-1">
              {#if editingStatDecayRate === 0}
                This stat will <span class="font-semibold">never decay</span> and will maintain its value indefinitely.
              {:else}
                This stat will decrease by 1 point {getDecayRateDescription(editingStatDecayRate)}.
                The higher the decay rate, the more frequently you'll need to maintain this stat.
              {/if}
            </p>
          </div>
        </div>
        
        {#if errorMessage}
          <div class="alert alert-error mt-3 text-sm">
            <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
            <span>{errorMessage}</span>
          </div>
        {/if}
        
        <div class="flex justify-end gap-3 mt-2">
          <button 
            type="button" 
            class="btn btn-ghost" 
            on:click={closeEditStatModal}
            disabled={loading}
          >
            Cancel
          </button>
          <button 
            type="submit" 
            class="btn btn-primary" 
            disabled={loading}
          >
            {loading ? 'Saving...' : 'Save Changes'}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}

<!-- Delete Stat Confirmation Modal -->
{#if showDeleteStatConfirm}
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
    <div class="bg-base-100 rounded-lg p-6 max-w-md w-full">
      <h3 class="text-xl font-semibold mb-4">Delete Stat</h3>
      
      <p class="mb-6">Are you sure you want to delete the stat "{statToDelete}"? This action cannot be undone.</p>
      
      {#if errorMessage}
        <div class="alert alert-error mb-4 text-sm">
          <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
          <span>{errorMessage}</span>
        </div>
      {/if}
      
      <div class="flex justify-end gap-3">
        <button 
          class="btn btn-ghost" 
          on:click={cancelDeleteStat}
          disabled={loading}
        >
          Cancel
        </button>
        <button 
          class="btn btn-error" 
          on:click={handleDeleteStat}
          disabled={loading}
        >
          {loading ? 'Deleting...' : 'Delete'}
        </button>
      </div>
    </div>
  </div>
{/if}

<style lang="postcss">
  :global(html) {
    background-color: hsl(var(--b2));
  }
</style>