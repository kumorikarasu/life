export const prerender = false;
export const ssr = false;

import { auth } from '$lib/auth';
import { get } from 'svelte/store';

type SimStat = {
  name: string;
  value: number;
  decay_rate: number | null;
  order_index?: number;
};

type Sim = {
  id: number;
  name: string;
  user_id: number;
  stats: SimStat[];
}

export async function load({ fetch, params }) {
  let saving = false;
  const authState = get(auth);
  
  // Get sim ID from route parameter
  const simId = params.id;

  const fetchWithAuth = async (url: string, options: RequestInit = {}) => {
    if (authState.isAuthenticated && authState.token) {
      options.credentials = 'include';
      options.headers = {
        ...options.headers,
        'Authorization': `Bearer ${authState.token}`
      };
    }
    return fetch(url, options);
  };

  let sim = null;
  if (authState.isAuthenticated) {
    console.log('Authenticated, loading sim:', simId);
    const req = await fetchWithAuth(`http://${import.meta.env.VITE_API_ENDPOINT}/api/v1/sim/${simId}`);
    if (req.ok) {
      sim = await req.json();
    }
  }

  return {
    sim,
    saveData: async (data: Sim) => {
      if (!authState.isAuthenticated || !data) return;
      
      // Don't save if no sim data or no ID (prevents unnecessary saves)
      if (!data.id) return;
      
      // Create delay to only save after 1 second without changes
      if (saving) return;
      saving = true;
      await new Promise(resolve => setTimeout(resolve, 1000))
        .then(async () => {
          // Use PUT for updating existing sims
          await fetchWithAuth(`http://${import.meta.env.VITE_API_ENDPOINT}/api/v1/sim/${data.id}`, {
            method: 'PUT',
            headers: {
              'Content-Type': 'application/json'
            },
            body: JSON.stringify(data),
          });
          saving = false;
        });
    },
    createStat: async (simId: number, statName: string, initialValue: number = 50, decayRate: number = 0.1): Promise<SimStat | null> => {
      if (!authState.isAuthenticated) return null;
      
      try {
        // Get the current number of stats to set the order index
        let orderIndex = 0;
        if (sim && sim.stats) {
          orderIndex = sim.stats.length; // Place new stat at the end
        }
        
        // Create the new stat with the provided values or defaults
        const statData = { 
          name: statName, 
          value: initialValue, // Use the provided initial value
          decay_rate: decayRate, // Use the provided decay rate
          order_index: orderIndex // Add it at the end of the list
        };
        
        const response = await fetchWithAuth(`http://${import.meta.env.VITE_API_ENDPOINT}/api/v1/sim/${simId}/stat`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${authState.token}`
          },
          body: JSON.stringify(statData)
        });
        
        if (response.ok) {
          return await response.json();
        } else {
          const errorText = await response.text();
          console.error('Failed to create stat:', response.status, response.statusText, errorText);
        }
      } catch (error) {
        console.error('Error creating stat:', error);
      }
      
      return null;
    },
    updateStat: async (simId: number, statName: string, value: number, decayRate: number = 0.1): Promise<boolean> => {
      if (!authState.isAuthenticated) return false;
      
      try {
        // Update the stat with new values
        const statData = { 
          name: statName, 
          value: value,
          decay_rate: decayRate
        };
        
        const response = await fetchWithAuth(`http://${import.meta.env.VITE_API_ENDPOINT}/api/v1/sim/${simId}/stat/${statName}`, {
          method: 'PUT',
          headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${authState.token}`
          },
          body: JSON.stringify(statData)
        });
        
        return response.ok;
      } catch (error) {
        console.error('Error updating stat:', error);
        return false;
      }
    },
    deleteStat: async (simId: number, statName: string): Promise<boolean> => {
      if (!authState.isAuthenticated) return false;
      
      try {
        const response = await fetchWithAuth(`http://${import.meta.env.VITE_API_ENDPOINT}/api/v1/sim/${simId}/stat/${statName}`, {
          method: 'DELETE',
          headers: {
            'Authorization': `Bearer ${authState.token}`
          }
        });
        
        return response.ok;
      } catch (error) {
        console.error('Error deleting stat:', error);
        return false;
      }
    },
    updateStatsOrder: async (simId: number, statOrders: Array<[string, number]>): Promise<boolean> => {
      if (!authState.isAuthenticated) return false;
      
      try {
        const response = await fetchWithAuth(`http://${import.meta.env.VITE_API_ENDPOINT}/api/v1/sim/${simId}/stats/order`, {
          method: 'PUT',
          headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${authState.token}`
          },
          body: JSON.stringify({ stat_orders: statOrders })
        });
        
        return response.ok;
      } catch (error) {
        console.error('Error updating stats order:', error);
        return false;
      }
    }
  }
}