<script lang="ts">
import '../../../assets/app.css'
import Range from '$lib/Range.svelte'
import Navbar from '$lib/Navbar.svelte'
import { auth, login } from "$lib/auth";
import { browser } from '$app/environment';
import { onMount } from 'svelte';

export let data;

// State variables for stat creation and editing
let showStatModal = false; // Single modal for both create and edit
let showDeleteStatConfirm = false; // Modal for confirming stat deletion
let isEditMode = false; // Flag to determine if we're editing or creating
let statName = '';
let statValue = 50;
let statDecayRate = 1/60; // Default to 1 unit per minute
let statDecaySlider = 50; // Default slider position (mid-range)
let statIsGrowth = false; // Track whether stat should grow instead of decay
// For displaying formatted values in modal
let rateDisplayValue = 0; // For the displayed rate in the selected unit
let rateUnit = "minute"; // Default unit for rate
let timeDisplayValue = 0; // For the displayed time in the selected unit
let timeUnit = "minute"; // Default unit for time
let statToDelete = '';
let errorMessage = '';
let successMessage = '';
let loading = false;
// Store stat order in this array
let statOrder = [];
// Keep track of the manual order (when auto-sort is off)
let manualStatOrder = [];
// Use a reactive variable to track auto-sort state
let autoSortEnabled = false;
// Track sort direction (true = descending/highest first, false = ascending/lowest first)
let sortDescending = true;
// Track whether sliders are currently being dragged
let isCreateSliderDragging = false;
let isEditSliderDragging = false;
// For displaying formatted values in edit modal
let editingRateDisplayValue = 0; // For the displayed rate in the selected unit
let editingRateUnit = "minute"; // Default unit for rate
let editingTimeDisplayValue = 0; // For the displayed time in the selected unit
let editingTimeUnit = "minute"; // Default unit for time

// Local storage keys
const AUTO_SORT_STORAGE_KEY = 'simBru_autoSortEnabled';
const SORT_DIRECTION_STORAGE_KEY = 'simBru_sortDescending';

// For tracking when stat values change (to trigger auto-sort)
let statsSignal = 0;

// Set up a reactive statement to update autoSortEnabled when localStorage changes
function checkAutoSortSetting() {
  if (browser) {
    const savedPreference = localStorage.getItem(AUTO_SORT_STORAGE_KEY);
    return savedPreference === 'true';
  }
  return false;
}

// Check sort direction from localStorage
function checkSortDirectionSetting() {
  if (browser) {
    const savedDirection = localStorage.getItem(SORT_DIRECTION_STORAGE_KEY);
    return savedDirection !== 'false'; // Default to true (descending) if not set
  }
  return true;
}

// Check for localStorage changes using addEventListener
onMount(() => {
  if (browser) {
    // Initialize on mount
    autoSortEnabled = checkAutoSortSetting();
    sortDescending = checkSortDirectionSetting();
    
    // Listen for storage events (when changed from another tab/window)
    window.addEventListener('storage', (event) => {
      if (event.key === AUTO_SORT_STORAGE_KEY) {
        autoSortEnabled = event.newValue === 'true';
      } else if (event.key === SORT_DIRECTION_STORAGE_KEY) {
        sortDescending = event.newValue !== 'false';
      }
    });
  }
});

// Create a custom event handler to detect changes from the navbar
function handleStorageChange() {
  if (browser) {
    autoSortEnabled = checkAutoSortSetting();
  }
}

// Set an interval to periodically check localStorage
let checkInterval;
onMount(() => {
  checkInterval = setInterval(handleStorageChange, 300); // Check every 300ms
  return () => clearInterval(checkInterval); // Clean up on component destroy
});

// Set up a scheduled update to sync backend values with visual decay
let syncInterval;

// Schedule periodic updates to sync actual values with database
onMount(() => {
  // Check for auto-sort setting
  autoSortEnabled = checkAutoSortSetting();
  sortDescending = checkSortDirectionSetting();
  
  // Listen for storage events (when changed from another tab/window)
  if (browser) {
    window.addEventListener('storage', (event) => {
      if (event.key === AUTO_SORT_STORAGE_KEY) {
        autoSortEnabled = event.newValue === 'true';
      } else if (event.key === SORT_DIRECTION_STORAGE_KEY) {
        sortDescending = event.newValue !== 'false';
      }
    });
  }
  
  // Set up interval to periodically check localStorage and handle storage change
  checkInterval = setInterval(handleStorageChange, 300);
  
  return () => {
    clearInterval(checkInterval);
  };
});

// Initialize manual order from database order when data first loads
$: if (data?.sim?.stats && manualStatOrder.length === 0) {
  // Initial load from database
  const sortedStats = [...data.sim.stats].sort((a, b) => {
    const aIndex = typeof a.order_index === 'number' ? a.order_index : 0;
    const bIndex = typeof b.order_index === 'number' ? b.order_index : 0;
    return aIndex - bIndex;
  });
  
  // Set both statOrder and manualStatOrder
  manualStatOrder = sortedStats.map(stat => stat.name);
  statOrder = [...manualStatOrder];
}

// Update statOrder when data.sim.stats changes or when autoSortEnabled changes
// or when statsSignal changes (indicating values have been modified)
$: if (data?.sim?.stats && manualStatOrder.length > 0) {
  if (autoSortEnabled) {
    // Including statsSignal in the dependency array to trigger re-sort when values change
    const triggerReactivity = statsSignal; // This does nothing but forces reactivity
    
    // Sort stats by value, respecting the sort direction
    const sortedStats = [...data.sim.stats].sort((a, b) => {
      return sortDescending ? b.value - a.value : a.value - b.value;
    });
    statOrder = sortedStats.map(stat => stat.name);
  } else {
    // When auto-sort is disabled, use our manually maintained order
    // which preserves any reordering the user has done
    statOrder = [...manualStatOrder];
  }
}

// Toggle auto-sort function and save to local storage
function toggleAutoSort() {
  autoSortEnabled = !autoSortEnabled;
  
  // Save preference to local storage
  if (browser) {
    localStorage.setItem(AUTO_SORT_STORAGE_KEY, autoSortEnabled.toString());
  }
}

// Toggle sort direction and save to local storage
function toggleSortDirection() {
  sortDescending = !sortDescending;
  
  // Save preference to local storage
  if (browser) {
    localStorage.setItem(SORT_DIRECTION_STORAGE_KEY, sortDescending.toString());
  }
}

// Function to reorder stats
async function handleReorderStat(statName, direction, positions = 1) {
  // Don't allow manual reordering when auto-sort is enabled
  if (autoSortEnabled) return;
  
  const currentIndex = statOrder.indexOf(statName);
  if (currentIndex < 0) return;
  
  // Calculate the target index based on direction and positions to move
  let newIndex;
  if (direction === 'up') {
    newIndex = Math.max(0, currentIndex - positions);
  } else {
    newIndex = Math.min(statOrder.length - 1, currentIndex + positions);
  }
  
  // Don't do anything if the index wouldn't change
  if (newIndex === currentIndex) return;
  
  // Create a copy of the array
  const newOrder = [...statOrder];
  
  // Remove the item from its current position
  newOrder.splice(currentIndex, 1);
  
  // Insert it at the new position
  newOrder.splice(newIndex, 0, statName);
  
  // Update the statOrder array
  statOrder = newOrder;
  manualStatOrder = [...newOrder]; // Update manual order as well
  
  // Save the new order
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

// Function to calculate time to fully decay/grow based on decay rate
function getTimeToFullFromRate(decayRate) {
  // For infinite (rate of 0), return Infinity
  if (decayRate === 0) return Infinity;
  
  // Time to decay/grow fully = 100 / rate per second
  // Using absolute value to handle both growth and decay
  const absRate = Math.abs(decayRate);
  const seconds = 100 / absRate;
  
  // Convert to minutes and round to nearest integer
  return Math.round(seconds / 60);
}

// Function to calculate decay rate from time to fully decay/grow
function getRateFromTimeToFull(minutes) {
  // For infinite (0 minutes), return 0 rate
  if (minutes === 0) return 0;
  
  // Rate per second = 100 / (minutes * 60)
  return 100 / (minutes * 60);
}

// Function to get human-readable time description
function getTimeDescription(minutes) {
  if (!isFinite(minutes)) {
    return "Never";
  } else if (minutes < 1) {
    return "Less than a minute";
  } else if (minutes < 60) {
    return `${minutes} minute${minutes !== 1 ? 's' : ''}`;
  } else if (minutes < 1440) { // Less than 1 day (24 hours)
    const hours = Math.floor(minutes / 60);
    const remainingMinutes = minutes % 60;
    
    if (remainingMinutes === 0) {
      return `${hours} hour${hours !== 1 ? 's' : ''}`;
    } else {
      return `${hours} hour${hours !== 1 ? 's' : ''} and ${remainingMinutes} minute${remainingMinutes !== 1 ? 's' : ''}`;
    }
  } else { // 1 day or more
    const days = Math.floor(minutes / 1440);
    const remainingHours = Math.floor((minutes % 1440) / 60);
    const remainingMinutes = minutes % 60;
    
    let result = `${days} day${days !== 1 ? 's' : ''}`;
    
    if (remainingHours > 0) {
      result += ` ${remainingHours} hour${remainingHours !== 1 ? 's' : ''}`;
    }
    
    if (remainingMinutes > 0) {
      result += ` ${remainingMinutes} minute${remainingMinutes !== 1 ? 's' : ''}`;
    }
    
    return result;
  }
}

// Suggested stat types that would make sense for a life simulation
const suggestedStats = [
  'Energy', 'Hunger', 'Thirst', 'Sleep', 'Hygiene', 
  'Fun', 'Social', 'Stress', 'Health', 'Fitness',
  'Mood', 'Intelligence', 'Creativity', 'Money', 'Career'
];

// Function to open the unified stat modal (for both create and edit)
function openStatModal(existingStatName = null) {
  errorMessage = '';
  isEditMode = existingStatName !== null;
  
  // Reset form fields
  if (isEditMode) {
    // Edit mode - load existing stat data
    const stat = data.sim.stats.find(s => s.name === existingStatName);
    if (stat) {
      statName = stat.name;
      statValue = stat.value;
      
      // Check if the decay rate is negative, which indicates growth mode
      const absDecayRate = Math.abs(stat.decay_rate || 1/60);
      statIsGrowth = stat.decay_rate < 0;
      statDecayRate = absDecayRate;
      statDecaySlider = getSliderFromDecayRate(absDecayRate);
      
      // Set default unit to minute and calculate initial display values
      rateUnit = "minute";
      rateDisplayValue = Math.round(absDecayRate * 60); // Default to per minute
      
      // Initialize time values as well
      timeUnit = "minute";
      timeDisplayValue = Math.round(getTimeToFullFromRate(absDecayRate));
    }
  } else {
    // Create mode - use defaults
    statName = '';
    statValue = 0; // Default to 0 instead of 50
    statDecayRate = 1/60; // Reset to default (1 per minute)
    statDecaySlider = 150; // Reset to default (mid-range)
    statIsGrowth = false; // Reset growth mode to false
    
    // Initialize display values
    rateUnit = "minute";
    rateDisplayValue = 1; // 1 per minute
    timeUnit = "minute";
    timeDisplayValue = 100; // 100 minutes to fully decay
  }
  
  showStatModal = true;
}

// Function to close the unified stat modal
function closeStatModal() {
  showStatModal = false;
  errorMessage = '';
}

// Function to handle the decay slider changes in unified modal
function handleDecaySliderChange(event) {
  const sliderValue = parseFloat(event.target.value);
  statDecaySlider = sliderValue;
  statDecayRate = getDecayRateFromSlider(sliderValue);
  
  // Update the displayed rate value based on selected unit
  if (rateUnit === 'second') {
    rateDisplayValue = Math.round(statDecayRate);
  } else if (rateUnit === 'minute') {
    rateDisplayValue = Math.round(statDecayRate * 60);
  } else if (rateUnit === 'hour') {
    rateDisplayValue = Math.round(statDecayRate * 3600);
  } else if (rateUnit === 'day') {
    rateDisplayValue = Math.round(statDecayRate * 86400);
  }
  
  // Update the displayed time value
  const minutes = getTimeToFullFromRate(statDecayRate);
  if (timeUnit === 'minute') {
    timeDisplayValue = Math.round(minutes);
  } else if (timeUnit === 'hour') {
    timeDisplayValue = Math.round(minutes / 60);
  } else if (timeUnit === 'day') {
    timeDisplayValue = Math.round(minutes / 1440);
  } else {
    timeDisplayValue = Math.round(minutes);
  }
}

// Function to handle both creating and editing a stat
async function handleSaveStat(event) {
  event.preventDefault();
  
  // Validate the form
  if (!statName.trim()) {
    errorMessage = 'Please enter a stat name';
    return;
  }
  
  // Validate value is between 0 and 100
  if (statValue < 0 || statValue > 100) {
    errorMessage = 'Value must be between 0 and 100';
    return;
  }

  // Validate decay rate is non-negative before applying direction
  if (statDecayRate < 0) {
    errorMessage = 'Rate value cannot be negative';
    return;
  }
  
  // For create mode, check if the stat already exists
  if (!isEditMode && data.sim.stats.some(stat => stat.name.toLowerCase() === statName.trim().toLowerCase())) {
    errorMessage = 'A stat with this name already exists';
    return;
  }
  
  errorMessage = '';
  loading = true;
  
  try {
    // Apply growth mode by making decay rate negative if needed
    const finalDecayRate = statIsGrowth ? -statDecayRate : statDecayRate;
    
    if (isEditMode) {
      // Updating an existing stat
      const success = await data.updateStat(data.sim.id, statName, statValue, finalDecayRate);
      if (success) {
        // Update in the sim object
        data.sim.stats = data.sim.stats.map(s => {
          if (s.name === statName) {
            return {
              ...s,
              value: statValue,
              decay_rate: finalDecayRate
            };
          }
          return s;
        });
        
        // Show success message
        const modeText = statIsGrowth ? 'Growth' : 'Decay';
        successMessage = `${modeText} rate for "${statName}" updated successfully!`;
      } else {
        errorMessage = `Failed to update ${statIsGrowth ? 'growth' : 'decay'} rate. Please try again.`;
        loading = false;
        return;
      }
    } else {
      // Creating a new stat
      const newStat = await data.createStat(data.sim.id, statName.trim(), statValue, finalDecayRate);
      if (newStat) {
        // Add the new stat to the sim object
        data.sim.stats = [...data.sim.stats, newStat];
        
        // Show success message
        const modeText = statIsGrowth ? 'Growth' : 'Decay';
        successMessage = `Stat "${newStat.name}" with ${modeText} mode created successfully!`;
      } else {
        errorMessage = 'Failed to create stat. Please try again.';
        loading = false;
        return;
      }
    }
    
    // Close modal and clean up
    showStatModal = false;
    
    // Clear success message after 3 seconds
    setTimeout(() => {
      successMessage = '';
    }, 3000);
  } catch (error) {
    console.error(`Error ${isEditMode ? 'updating' : 'creating'} stat:`, error);
    errorMessage = `An error occurred while ${isEditMode ? 'updating' : 'creating'} the stat.`;
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
    // Only update if the value has actually changed significantly (at least 1 point)
    if (Math.abs(stat.value - newValue) >= 1) {
      // Determine if this is an automatic update (decay or growth) or a manual change
      // For decay: value decreases by small amount
      // For growth: value increases by small amount
      const isAutomaticUpdate = 
        (Math.abs(stat.value - newValue) < 5) && 
        ((stat.decay_rate > 0 && newValue < stat.value) || // Decay mode decreasing
         (stat.decay_rate < 0 && newValue > stat.value));  // Growth mode increasing
      
      // For automatic updates, we only want to update the local state without database write
      // For manual changes, we also update the database
      if (!isAutomaticUpdate) {
        // This is likely a manual change, so update the database
        const success = await data.updateStat(data.sim.id, statName, newValue, stat.decay_rate || 1/60);
        if (!success) {
          console.error('Failed to update stat value in database');
          return; // Don't update local state if database update failed
        }
      }
      
      // Update the local state regardless (for both automatic updates and manual changes)
      data.sim.stats = data.sim.stats.map(s => {
        if (s.name === statName) {
          return { ...s, value: newValue };
        }
        return s;
      });
      
      // Update the statsSignal to trigger auto-sort
      statsSignal += 1;
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
              {#if !autoSortEnabled}
                <p class="text-sm text-base-content opacity-80 italic flex items-center">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 inline-block mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4" />
                  </svg>
                  Drag stats to reorder them
                </p>
              {:else}
                <button 
                  class="text-sm btn btn-sm btn-ghost flex items-center gap-1 opacity-80"
                  on:click={toggleSortDirection}
                  title={sortDescending ? "Currently: Highest to lowest" : "Currently: Lowest to highest"}
                >
                  {#if sortDescending}
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4h13M3 8h9m-9 4h6m4 0l4-4m0 0l4 4m-4-4v12" />
                    </svg>
                    <span>Highest to lowest</span>
                  {:else}
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4h13M3 8h9m-9 4h9m5-4v12m0 0l-4-4m4 4l4-4" />
                    </svg>
                    <span>Lowest to highest</span>
                  {/if}
                </button>
              {/if}
            </div>
            <div class="flex gap-2">
              <button class="btn btn-primary btn-sm" on:click={() => openStatModal()}>
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                </svg>
                Add Stat
              </button>
              <a href="/sims" class="btn btn-sm btn-outline">Back to All Sims</a>
            </div>
          </div>
          
          <!-- Auto-sort toggle moved to navbar dropdown -->
          
          {#if data.sim.stats && data.sim.stats.length > 0}
            <div class="grid grid-cols-1 gap-3 px-4">
              {#each statOrder as statName, index}
                {#each data.sim.stats as stat}
                  {#if stat.name === statName}
                    <Range 
                      name={stat.name} 
                      value={stat.value} 
                      decay_rate={stat.decay_rate || 1/60}
                      onEdit={(name) => openStatModal(name)}
                      onDelete={confirmDeleteStat}
                      onValueChange={handleStatValueChange}
                      onReorder={handleReorderStat}
                      draggable={!autoSortEnabled}
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

<!-- Unified Stat Modal (for both Add and Edit) -->
{#if showStatModal}
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
    <div class="bg-base-100 rounded-lg p-6 max-w-md w-full">
      <h3 class="text-xl font-semibold mb-4">
        {isEditMode ? `Edit Stat: ${statName}` : 'Add New Stat'}
      </h3>
      
      <form on:submit={handleSaveStat} class="flex flex-col gap-4">
        {#if !isEditMode}
          <div>
            <label for="statName" class="block text-sm font-medium mb-1">Stat Name</label>
            <input 
              type="text" 
              id="statName" 
              bind:value={statName} 
              placeholder="Enter stat name" 
              class="input input-bordered w-full" 
              autofocus
              disabled={isEditMode}
            />
          </div>
          <!-- Initial value field removed, will default to 0 -->
        {/if}

        <div>
          <!-- Growth/Decay Mode Toggle at the top -->
          <div class="form-control mb-4 bg-base-200 p-3 rounded-md">
            <label class="cursor-pointer label justify-start gap-4">
              <input 
                type="checkbox" 
                class="toggle toggle-success" 
                bind:checked={statIsGrowth}
                disabled={statDecayRate === 0}
              />
              <div>
                <span class="label-text font-medium text-base">
                  {statIsGrowth ? 'Growth Mode' : 'Decay Mode'}
                </span>
                <p class="text-xs opacity-70 mt-1">
                  This stat will {statIsGrowth ? 'increase' : 'decrease'} over time
                </p>
              </div>
            </label>
          </div>
          
          <!-- Rate Slider with visual indicators and manual input -->
          <div class="mb-5">
            <div class="flex justify-between items-center mb-1">
              <label for="statDecaySlider" class="block text-sm font-medium">
                {statIsGrowth ? 'Growth Rate' : 'Decay Rate'}
              </label>
              <div class="dropdown dropdown-end">
                <label tabindex="0" class="cursor-pointer badge {statDecayRate === 0 ? 'badge-neutral' : statIsGrowth ? 'badge-success' : 'badge-error'}">
                  {statDecayRate === 0 ? 'Infinite' : getDecayRateDescription(statDecayRate)}
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 ml-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                  </svg>
                </label>
                <div tabindex="0" class="dropdown-content z-[1] p-3 shadow-lg bg-base-200 rounded-box w-72">
                  <div class="form-control w-full">
                    <label class="label">
                      <span class="label-text">Enter exact rate</span>
                    </label>
                    <div class="flex gap-2">
                      <input 
                        type="text" 
                        pattern="[0-9]*" 
                        inputmode="numeric"
                        placeholder="Rate" 
                        class="input input-bordered w-full max-w-xs flex-1"
                        value={rateDisplayValue}
                        on:change={(e) => {
                          const value = parseInt(e.target.value, 10) || 0;
                          const unit = document.getElementById('rateUnit').value;
                          let ratePerSecond;
                          
                          if (unit === 'second') {
                            ratePerSecond = value;
                          } else if (unit === 'minute') {
                            ratePerSecond = value / 60;
                          } else if (unit === 'hour') {
                            ratePerSecond = value / 3600;
                          } else if (unit === 'day') {
                            ratePerSecond = value / 86400;
                          }
                          
                          if (ratePerSecond) {
                            statDecayRate = ratePerSecond;
                            statDecaySlider = getSliderFromDecayRate(ratePerSecond);
                            
                            // Update the time display value when the rate changes
                            const minutes = getTimeToFullFromRate(ratePerSecond);
                            if (timeUnit === 'minute') {
                              timeDisplayValue = Math.round(minutes);
                            } else if (timeUnit === 'hour') {
                              timeDisplayValue = Math.round(minutes / 60);
                            } else if (timeUnit === 'day') {
                              timeDisplayValue = Math.round(minutes / 1440);
                            }
                            
                            rateDisplayValue = value;
                          }
                        }}
                      />
                      <select 
                        id="rateUnit" 
                        class="select select-bordered w-32"
                        bind:value={rateUnit}
                        on:change={(e) => {
                          const unit = e.target.value;
                          let displayValue;
                          
                          if (unit === 'second') {
                            displayValue = Math.round(statDecayRate);
                          } else if (unit === 'minute') {
                            displayValue = Math.round(statDecayRate * 60);
                          } else if (unit === 'hour') {
                            displayValue = Math.round(statDecayRate * 3600);
                          } else if (unit === 'day') {
                            displayValue = Math.round(statDecayRate * 86400);
                          }
                          
                          rateDisplayValue = displayValue;
                        }}
                      >
                        <option value="second">Per second</option>
                        <option value="minute">Per minute</option>
                        <option value="hour">Per hour</option>
                        <option value="day">Per day</option>
                      </select>
                    </div>
                  </div>
                  
                  <div class="form-control w-full mt-3">
                    <label class="label">
                      <span class="label-text">Time to fully {statIsGrowth ? 'grow' : 'decay'}</span>
                    </label>
                    <div class="flex gap-2">
                      <input 
                        type="text" 
                        pattern="[0-9]*" 
                        inputmode="numeric"
                        placeholder="Time" 
                        class="input input-bordered w-full max-w-xs flex-1" 
                        value={timeDisplayValue}
                        on:change={(e) => {
                          const value = parseInt(e.target.value, 10) || 0;
                          const unit = document.getElementById('timeUnit').value;
                          let minutes;
                          
                          if (unit === 'minute') {
                            minutes = value;
                          } else if (unit === 'hour') {
                            minutes = value * 60;
                          } else if (unit === 'day') {
                            minutes = value * 1440;
                          }
                          
                          if (minutes) {
                            const newRate = getRateFromTimeToFull(minutes);
                            statDecayRate = newRate;
                            statDecaySlider = getSliderFromDecayRate(newRate);
                            
                            // Update the rate display value when time is changed
                            if (rateUnit === 'second') {
                              rateDisplayValue = Math.round(newRate);
                            } else if (rateUnit === 'minute') {
                              rateDisplayValue = Math.round(newRate * 60);
                            } else if (rateUnit === 'hour') {
                              rateDisplayValue = Math.round(newRate * 3600);
                            } else if (rateUnit === 'day') {
                              rateDisplayValue = Math.round(newRate * 86400);
                            }
                            
                            timeDisplayValue = value;
                          }
                        }}
                      />
                      <select 
                        id="timeUnit" 
                        class="select select-bordered w-32" 
                        bind:value={timeUnit}
                        on:change={(e) => {
                          const unit = e.target.value;
                          const minutes = getTimeToFullFromRate(statDecayRate);
                          
                          if (unit === 'minute') {
                            timeDisplayValue = Math.round(minutes);
                          } else if (unit === 'hour') {
                            timeDisplayValue = Math.round(minutes / 60);
                          } else if (unit === 'day') {
                            timeDisplayValue = Math.round(minutes / 1440);
                          }
                        }}
                      >
                        <option value="minute">Minutes</option>
                        <option value="hour">Hours</option>
                        <option value="day">Days</option>
                      </select>
                    </div>
                  </div>
                </div>
              </div>
            </div>
            <input 
              type="range" 
              id="statDecaySlider" 
              min="0" 
              max="300" 
              step="1" 
              bind:value={statDecaySlider} 
              on:input={handleDecaySliderChange}
              class="range {statIsGrowth ? 'range-success' : 'range-error'}"
            />
            <div class="flex justify-between text-xs text-base-content opacity-70 px-1 mt-1">
              <span>Infinite</span>
              <span>Slow</span>
              <span>Medium</span>
              <span>Fast</span>
              <span>Very Fast</span>
            </div>
          </div>
          
          <!-- Combined information box -->
          <div class="bg-base-300 rounded-md p-4 mb-2">
            {#if statDecayRate === 0}
              <div class="flex items-center gap-3">
                <div class="badge badge-lg badge-neutral">Infinite</div>
                <p class="text-sm">This stat will not change over time</p>
              </div>
            {:else}
              <div class="flex items-center gap-3">
                <p class="text-sm">
                  <span class="font-semibold">{getTimeDescription(getTimeToFullFromRate(statDecayRate))}</span> to 
                  {statIsGrowth ? 'fully grow' : 'fully decay'}
                </p>
              </div>
            {/if}
          </div>
        </div>
        
        {#if !isEditMode}
          <div>
            <p class="text-sm font-medium mb-2">Suggested Stats:</p>
            <div class="flex flex-wrap gap-2">
              {#each suggestedStats as suggestion}
                <button 
                  type="button"
                  class="badge badge-outline hover:bg-primary hover:text-primary-content transition-colors cursor-pointer p-3"
                  on:click={() => statName = suggestion}
                >
                  {suggestion}
                </button>
              {/each}
            </div>
          </div>
        {/if}
        
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
            on:click={closeStatModal}
            disabled={loading}
          >
            Cancel
          </button>
          <button 
            type="submit" 
            class="btn btn-primary" 
            disabled={loading || (!isEditMode && !statName.trim())}
          >
            {loading ? (isEditMode ? 'Saving...' : 'Creating...') : (isEditMode ? 'Save Changes' : 'Create Stat')}
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