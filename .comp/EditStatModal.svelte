<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  
  // Create event dispatcher
  const dispatch = createEventDispatcher();
  
  // Props
  export let statName: string = '';
  export let loading: boolean = false;
  export let errorMessage: string = '';
  export let decayRate: number = 1/60;
  export let isGrowth: boolean = false;
  
  // State variables for stat editing
  let editingStatDecayRate = decayRate;
  let editingStatDecaySlider = getSliderFromDecayRate(decayRate);
  let editingStatIsGrowth = isGrowth;
  
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
  
  // Function to handle decay slider change in edit modal
  function handleEditDecaySliderChange(event) {
    const sliderValue = parseFloat(event.target.value);
    editingStatDecaySlider = sliderValue;
    editingStatDecayRate = getDecayRateFromSlider(sliderValue);
  }
  
  // Function to close the modal
  function closeModal() {
    dispatch('close');
  }
  
  // Function to handle editing a stat
  function handleEditStat(event) {
    event.preventDefault();
    
    // Validate decay rate is non-negative before applying any direction
    if (editingStatDecayRate < 0) {
      dispatch('error', 'Rate value cannot be negative');
      return;
    }
    
    // Apply the growth mode by making the decay rate negative if needed
    const finalDecayRate = editingStatIsGrowth ? -editingStatDecayRate : editingStatDecayRate;
    
    // Send the edited stat data to the parent component
    dispatch('save', {
      name: statName,
      decayRate: finalDecayRate,
      isGrowth: editingStatIsGrowth
    });
  }
</script>

<div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
  <div class="bg-base-100 rounded-lg p-6 max-w-md w-full">
    <h3 class="text-xl font-semibold mb-4">Edit Stat: {statName}</h3>
    
    <form on:submit={handleEditStat} class="flex flex-col gap-4">
      <div>
        <!-- Growth/Decay Mode Toggle at the top -->
        <div class="form-control mb-4 bg-base-200 p-3 rounded-md">
          <label class="cursor-pointer label justify-start gap-4">
            <input 
              type="checkbox" 
              class="toggle toggle-success" 
              bind:checked={editingStatIsGrowth}
              disabled={editingStatDecayRate === 0}
            />
            <div>
              <span class="label-text font-medium text-base">
                {editingStatIsGrowth ? 'Growth Mode' : 'Decay Mode'}
              </span>
              <p class="text-xs opacity-70 mt-1">
                This stat will {editingStatIsGrowth ? 'increase' : 'decrease'} over time
              </p>
            </div>
          </label>
        </div>
        
        <!-- Rate Slider with visual indicators and manual input -->
        <div class="mb-5">
          <div class="flex justify-between items-center mb-1">
            <label for="editDecaySlider" class="block text-sm font-medium">
              {editingStatIsGrowth ? 'Growth Rate' : 'Decay Rate'}
            </label>
            <div class="dropdown dropdown-end">
              <button tabindex="0" class="cursor-pointer badge {editingStatDecayRate === 0 ? 'badge-neutral' : editingStatIsGrowth ? 'badge-success' : 'badge-error'}">
                {editingStatDecayRate === 0 ? 'Infinite' : getDecayRateDescription(editingStatDecayRate)}
                <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 ml-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                </svg>
              </button>
              <div tabindex="0" class="dropdown-content z-[1] p-3 shadow-lg bg-base-200 rounded-box w-72">
                <div class="form-control w-full">
                  <label class="label">
                    <span class="label-text">Enter exact rate</span>
                  </label>
                  <div class="flex gap-2">
                    <input 
                      type="number" 
                      min="0.001" 
                      step="0.01" 
                      placeholder="Rate" 
                      class="input input-bordered w-full" 
                      on:change={(e) => {
                        const value = parseFloat(e.target.value);
                        const unit = document.getElementById('rateUnitEdit').value;
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
                          editingStatDecayRate = ratePerSecond;
                          editingStatDecaySlider = getSliderFromDecayRate(ratePerSecond);
                        }
                      }}
                    />
                    <select 
                      id="rateUnitEdit" 
                      class="select select-bordered" 
                      defaultValue="minute"
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
                    <span class="label-text">Time to fully {editingStatIsGrowth ? 'grow' : 'decay'}</span>
                  </label>
                  <div class="flex gap-2">
                    <input 
                      type="number" 
                      min="1" 
                      step="1" 
                      placeholder="Time" 
                      class="input input-bordered w-full" 
                      on:change={(e) => {
                        const value = parseFloat(e.target.value);
                        const unit = document.getElementById('timeUnitEdit').value;
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
                          editingStatDecayRate = newRate;
                          editingStatDecaySlider = getSliderFromDecayRate(newRate);
                        }
                      }}
                    />
                    <select 
                      id="timeUnitEdit" 
                      class="select select-bordered" 
                      defaultValue="minute"
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
            id="editDecaySlider" 
            min="0" 
            max="300" 
            step="1" 
            bind:value={editingStatDecaySlider} 
            on:input={handleEditDecaySliderChange}
            class="range {editingStatIsGrowth ? 'range-success' : 'range-error'}"
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
          {#if editingStatDecayRate === 0}
            <div class="flex items-center gap-3">
              <div class="badge badge-lg badge-neutral">Infinite</div>
              <p class="text-sm">This stat will not change over time</p>
            </div>
          {:else}
            <div class="flex items-center gap-3">
              <p class="text-sm">
                <span class="font-semibold">{getTimeDescription(getTimeToFullFromRate(editingStatDecayRate))}</span> to 
                {editingStatIsGrowth ? 'fully grow' : 'fully decay'}
              </p>
            </div>
          {/if}
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
          on:click={closeModal}
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