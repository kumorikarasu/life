<script lang="ts">
  export let name: string
  export let value: number
  export let decay_rate: number = 1/60  // Updated to 1 every 60 seconds (1/60 per second)
  export let onEdit: (name: string) => void = () => {}
  export let onDelete: (name: string) => void = () => {}
  export let onValueChange: (name: string, value: number) => void = () => {}
  let bg = '#333'
  
  // Initialize currentValue with the provided value
  let currentValue = value
  
  // Visual representation of decaying value (doesn't affect actual backend value)
  let visualValue = value
  let decayInterval: number | null = null
  let timeSinceLastDecay = 0
  let lastUpdateTime = Date.now()
  
  // Update the background color based on visualValue
  $: bg = "hsl(" + (Math.round(visualValue / 30) * 30) + ", 90%, 50%)"
  
  // Keep currentValue and visualValue in sync with external value changes
  $: if (value !== undefined && value !== null) {
    currentValue = value;
    visualValue = value;
    lastUpdateTime = Date.now();
    timeSinceLastDecay = 0;
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
  
  function handleInput(event) {
    // Update local value for visual feedback without sending to parent
    currentValue = Number(event.target.value);
    
    // Also update visualValue when user manually adjusts the slider
    visualValue = currentValue;
    
    // Reset decay timers when manually adjusting
    lastUpdateTime = Date.now();
    timeSinceLastDecay = 0;
  }
  
  function handleChange() {
    // Only send to parent when slider is released
    if (currentValue !== value) {
      onValueChange(name, currentValue);
    }
  }
  
  // Start decay visualization on component mount
  import { onMount, onDestroy } from 'svelte';
  
  onMount(() => {
    startDecayVisualization();
  });
  
  onDestroy(() => {
    stopDecayVisualization();
  });
  
  function startDecayVisualization() {
    if (decayInterval) return;
    
    // Update every 100ms for smooth visual decay
    decayInterval = window.setInterval(() => {
      const now = Date.now();
      const deltaTime = (now - lastUpdateTime) / 1000; // Convert to seconds
      lastUpdateTime = now;
      
      // Accumulate time since last decay
      timeSinceLastDecay += deltaTime;
      
      // Skip decay calculation if decay_rate is 0 (infinite)
      if (decay_rate === 0) {
        return;
      }
      
      // Calculate how much to decay based on decay_rate and elapsed time
      const decayAmount = decay_rate * deltaTime;
      
      // Only update if there's a noticeable change (>= 0.01)
      if (decayAmount >= 0.01) {
        visualValue = Math.max(0, visualValue - decayAmount);
      }
    }, 100);
  }
  
  function stopDecayVisualization() {
    if (decayInterval) {
      window.clearInterval(decayInterval);
      decayInterval = null;
    }
  }
  
  // Reset visual decay to actual value
  function resetVisualValue() {
    visualValue = value;
    lastUpdateTime = Date.now();
    timeSinceLastDecay = 0;
  }
</script>

<div class="mb-4">
  <div class="flex justify-between items-center">
    <p class="text-base-content">{name} <span class="text-sm font-normal opacity-80">{Math.round(visualValue)}</span></p>
    <div class="flex space-x-2">
      <button 
        class="btn btn-sm btn-ghost" 
        title="Edit stat"
        on:click={() => onEdit(name)}
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
        </svg>
      </button>
      <button 
        class="btn btn-sm btn-ghost text-error" 
        title="Delete stat"
        on:click={() => onDelete(name)}
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
        </svg>
      </button>
    </div>
  </div>
  <input 
    style="--range-shdw: {bg}" 
    type="range" 
    min="0" 
    max="100" 
    value={visualValue}
    class="range" 
    on:input={handleInput}
    on:change={handleChange}
  />
  <div class="flex justify-between text-xs text-base-content opacity-70 px-1 mt-1">
    <span>0</span>
    <span>25</span>
    <span>50</span>
    <span>75</span>
    <span>100</span>
  </div>
</div>

<style>
  p {
    font-size: 1.3rem;
    line-height: 3rem;
    margin: 10;
  }
  .range {
    width: 100%;
    --range-shdw: var(--bg, #fff);
  }
</style>
