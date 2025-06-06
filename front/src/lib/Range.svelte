<script lang="ts">
  export let name: string
  export let value: number
  export let decay_rate: number = 1/60  // Updated to 1 every 60 seconds (1/60 per second)
  export let onEdit: (name: string) => void = () => {}
  export let onDelete: (name: string) => void = () => {}
  export let onValueChange: (name: string, value: number) => void = () => {}
  export let onReorder: (name: string, direction: 'up' | 'down', positions?: number) => void = () => {}
  export let draggable: boolean = false
  export let index: number = 0
  export let totalStats: number = 1
  
  let bg = '#333'
  
  // Initialize currentValue with the provided value
  let currentValue = value
  
  // Visual representation of decaying value (doesn't affect actual backend value)
  let visualValue = value
  let decayInterval: number | null = null
  let lastUpdateTime = Date.now()
  let lastSyncTime = Date.now()
  let valueHasChanged = false
  
  // Track whether the slider is currently being interacted with
  let isSliderActive = false

  // How often to sync visual value with parent component (ms)
  const SYNC_INTERVAL = 2000 // 2 seconds
  
  // Drag and drop state
  let isDragging = false
  let dragStartY = 0
  let dragStartX = 0
  let translateY = 0
  let translateX = 0
  let containerHeight = 0
  let dragTargetIndex = null
  
  // Update the background color based on visualValue and growth/decay status
  $: bg = "hsl(" + (Math.round(visualValue / 30) * 30) + ", " + (decay_rate < 0 ? "90%, 70%" : "90%, 50%)")
  
  // Keep currentValue and visualValue in sync with external value changes
  $: if (value !== undefined && value !== null) {
    currentValue = value;
    visualValue = value;
    lastUpdateTime = Date.now();
  }
  
  function handleInput(event) {
    // Mark slider as active when user interacts with it
    isSliderActive = true;
    
    // Update local value for visual feedback without sending to parent
    currentValue = Number(event.target.value);
    
    // Also update visualValue when user manually adjusts the slider
    visualValue = currentValue;
    
    // Reset decay timers when manually adjusting
    lastUpdateTime = Date.now();
  }
  
  function handleChange() {
    // Mark slider as inactive when user releases it
    isSliderActive = false;
    
    // Only send to parent when slider is released
    if (currentValue !== value) {
      onValueChange(name, currentValue);
    }
  }
  
  // Add mousedown/touchstart handlers to track interaction start
  function handleSliderMouseDown() {
    isSliderActive = true;
  }
  
  // Mouse events for when user releases slider outside the component
  function handleGlobalMouseUp() {
    if (isSliderActive) {
      isSliderActive = false;
      
      // Sync value with parent when releasing the slider
      if (currentValue !== value) {
        onValueChange(name, currentValue);
      }
    }
  }
  
  // Drag and drop handlers
  function handleDragStart(event) {
    if (!draggable) return;
    
    // Prevent default behavior
    event.preventDefault();
    
    // Get initial position to track movement
    const container = event.target.closest('.stat-container');
    containerHeight = container.offsetHeight;
    
    // Set dragging state
    isDragging = true;
    dragStartY = event.clientY;
    dragStartX = event.clientX;
    translateY = 0;
    translateX = 0;
    dragTargetIndex = null;
    
    // Add event listeners for drag movement and end
    window.addEventListener('mousemove', handleDragMove);
    window.addEventListener('mouseup', handleDragEnd);
    
    // Change cursor
    document.body.style.cursor = 'grabbing';
  }
  
  function handleDragMove(event) {
    if (!isDragging) return;
    
    // Update position for dynamic movement in both X and Y directions
    const currentDragY = event.clientY;
    const currentDragX = event.clientX;
    translateY = currentDragY - dragStartY;
    translateX = currentDragX - dragStartX;
    
    // Limit horizontal movement to a reasonable range (e.g., +/- 100px)
    translateX = Math.max(-100, Math.min(100, translateX));
    
    // Calculate the target index based on current mouse position
    const currentMousePos = event.clientY;
    
    // Get all stat containers to calculate their positions
    const containers = Array.from(document.querySelectorAll('.stat-container'));
    let newTargetIndex = null;
    
    // Skip the dragged container in the comparison
    containers.forEach((container, idx) => {
      // Skip the container that's being dragged
      if (idx === index && isDragging) return;
      
      const rect = container.getBoundingClientRect();
      const containerTop = rect.top;
      const containerBottom = containerTop + rect.height;
      
      // If the mouse is within this container's bounds
      if (currentMousePos >= containerTop && currentMousePos <= containerBottom) {
        newTargetIndex = idx;
      }
    });
    
    // If we haven't found a container (mouse is above all containers)
    // and we're not already at the top, move to the top
    if (newTargetIndex === null && currentMousePos < containers[0]?.getBoundingClientRect().top) {
      newTargetIndex = 0;
    }
    
    // Set the target index if it's different from the current one
    if (newTargetIndex !== null && newTargetIndex !== index) {
      dragTargetIndex = newTargetIndex;
    } else {
      dragTargetIndex = null;
    }
  }
  
  function handleDragEnd() {
    if (!isDragging) return;
    
    // Check if we need to reorder based on final position
    if (dragTargetIndex !== null && dragTargetIndex !== index) {
      // Instead of just moving up or down one position,
      // move directly to the target position
      const distance = dragTargetIndex - index;
      const direction = distance > 0 ? 'down' : 'up';
      
      // Tell parent to reorder with the calculated number of positions
      onReorder(name, direction, Math.abs(distance));
    }
    
    // Reset dragging state
    isDragging = false;
    translateY = 0;
    translateX = 0;
    dragTargetIndex = null;
    
    // Remove event listeners
    window.removeEventListener('mousemove', handleDragMove);
    window.removeEventListener('mouseup', handleDragEnd);
    
    // Reset cursor
    document.body.style.cursor = 'default';
  }
  
  // Start decay visualization on component mount
  import { onMount, onDestroy } from 'svelte';
  
  onMount(() => {
    startDecayVisualization();
    window.addEventListener('mouseup', handleGlobalMouseUp);
  });
  
  onDestroy(() => {
    stopDecayVisualization();
    
    // Clean up event listeners if component is destroyed while dragging
    window.removeEventListener('mousemove', handleDragMove);
    window.removeEventListener('mouseup', handleDragEnd);
    window.removeEventListener('mouseup', handleGlobalMouseUp);
  });
  
  function startDecayVisualization() {
    if (decayInterval) return;
    
    // Update every 100ms for smooth visual decay
    decayInterval = window.setInterval(() => {
      // Skip decay if slider is currently being interacted with
      if (isSliderActive) {
        return;
      }
      
      const now = Date.now();
      const deltaTime = (now - lastUpdateTime) / 1000; // Convert to seconds
      lastUpdateTime = now;
      
      // Skip decay calculation if decay_rate is 0 (infinite)
      if (decay_rate === 0) {
        return;
      }
      
      // Calculate how much to change based on decay_rate and elapsed time
      const decayAmount = Math.abs(decay_rate) * deltaTime;
      
      // Only update if there's a noticeable change (>= 0.01)
      if (decayAmount >= 0.01) {
        const oldValue = visualValue;
        
        // If decay_rate is negative, the stat should grow instead of decay
        if (decay_rate < 0) {
          // For growth, increase the value up to 100
          visualValue = Math.min(100, visualValue + decayAmount);
        } else {
          // For decay, decrease the value down to 0
          visualValue = Math.max(0, visualValue - decayAmount);
        }
        
        // Check if the value has changed by at least 1 whole number
        if (Math.floor(oldValue) !== Math.floor(visualValue)) {
          valueHasChanged = true;
        }
        
        // If it's time to sync and the value has changed meaningfully
        if (valueHasChanged && (now - lastSyncTime > SYNC_INTERVAL)) {
          // Only update the actual value for auto-sort purposes
          if (Math.round(visualValue) !== Math.round(value)) {
            // Update the parent component with the new value
            onValueChange(name, Math.round(visualValue));
            lastSyncTime = now;
            valueHasChanged = false;
          }
        }
      }
    }, 100);
  }
  
  function stopDecayVisualization() {
    if (decayInterval) {
      window.clearInterval(decayInterval);
      decayInterval = null;
    }
  }
</script>

<div 
  class="stat-container mb-2 bg-base-200 p-4 rounded-lg shadow-md transition-transform duration-200" 
  class:draggable={draggable} 
  class:is-dragging={isDragging}
  class:drag-target={isDragging && dragTargetIndex !== null}
  class:growth-mode={decay_rate < 0}
  style={isDragging ? `transform: translate(${translateX}px, ${translateY}px);` : ''}
>
  <div class="flex justify-between items-center">
    <p 
      class="text-base-content font-bold" 
      class:cursor-grab={draggable}
      class:cursor-grabbing={isDragging}
      on:mousedown={handleDragStart}
    >
      {#if draggable}
        <span class="drag-handle mr-2">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 inline-block" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 8h16M4 16h16" />
          </svg>
        </span>
      {/if}
      {name} <span class="text-sm font-normal opacity-80 ml-4">{Math.round(visualValue)}</span>
      
      {#if decay_rate < 0}
        <span class="badge badge-sm badge-success ml-2 text-xs">Growth</span>
      {/if}
    </p>
    <div class="flex space-x-2">
      {#if draggable && index > 0}
        <button 
          class="btn btn-sm btn-ghost" 
          title="Move up"
          on:click={() => onReorder(name, 'up')}
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 15l7-7 7 7" />
          </svg>
        </button>
      {/if}
      {#if draggable && index < totalStats - 1}
        <button 
          class="btn btn-sm btn-ghost" 
          title="Move down"
          on:click={() => onReorder(name, 'down')}
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
          </svg>
        </button>
      {/if}
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
    on:mousedown={handleSliderMouseDown}
    on:touchstart={handleSliderMouseDown}
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
    margin: 0;
  }
  .range {
    width: 100%;
    --range-shdw: var(--bg, #fff);
  }
  
  .stat-container {
    border-left: 4px solid; 
    border-color: var(--range-shdw, #333);
  }
  
  .draggable {
    transition: box-shadow 0.2s ease-out, transform 0.05s ease-out;
    cursor: grab;
  }
  
  .is-dragging {
    opacity: 0.9;
    z-index: 1000;
    box-shadow: 0 8px 20px rgba(0, 0, 0, 0.3);
    transition: none; /* Remove transition for smoother dragging */
    position: relative; /* Ensure z-index works properly */
  }
  
  .drag-handle {
    cursor: grab;
    opacity: 0.6;
  }
  
  .drag-handle:hover {
    opacity: 1;
  }
  
  .cursor-grabbing {
    cursor: grabbing;
  }
  
  .drag-target {
    box-shadow: 0 0 15px rgba(0, 0, 255, 0.4);
  }
</style>
