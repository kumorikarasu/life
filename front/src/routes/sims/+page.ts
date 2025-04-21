export const ssr = false;

import { auth } from '$lib/auth';
import { get } from 'svelte/store';

type SimStat = {
  name: string;
  value: number;
  decay_rate: number | null;
};

type Sim = {
  id: number;
  name: string;
  user_id: number;
  stats: SimStat[];
};

export async function load({ fetch }): Promise<{ 
  sims: Sim[], 
  createSim: (name: string) => Promise<Sim | null>,
  deleteSim: (id: number) => Promise<boolean>
}> {
  const authState = get(auth);

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

  // Fetch user's sims
  let sims: Sim[] = [];
  if (authState.isAuthenticated) {
    try {
      const response = await fetchWithAuth(`${import.meta.env.VITE_API_ENDPOINT}/api/v1/sim/active_user`);
      if (response.ok) {
        sims = await response.json();
      } else {
        console.error('Failed to fetch sims:', response.status, response.statusText);
      }
    } catch (error) {
      console.error('Error fetching sims:', error);
    }
  }

  // Function to create a new sim
  const createSim = async (name: string): Promise<Sim | null> => {
    if (!authState.isAuthenticated) return null;
    
    try {
      // Make sure we're using the correct payload structure that the backend expects
      const simData = { 
        name, 
        user_id: 0, // The backend will override this with the authenticated user's ID
        stats: [] 
      };
      
      console.log('Creating sim with data:', simData);
      console.log('Auth token present:', !!authState.token);
      
      const response = await fetchWithAuth(`${import.meta.env.VITE_API_ENDPOINT}/api/v1/sim`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${authState.token}` // Explicitly add token here as well
        },
        body: JSON.stringify(simData)
      });
      
      if (response.ok) {
        return await response.json();
      } else {
        const errorText = await response.text();
        console.error('Failed to create sim:', response.status, response.statusText, errorText);
      }
    } catch (error) {
      console.error('Error creating sim:', error);
    }
    
    return null;
  };

  // Function to delete a sim
  const deleteSim = async (id: number): Promise<boolean> => {
    if (!authState.isAuthenticated) return false;
    
    try {
      const response = await fetchWithAuth(`${import.meta.env.VITE_API_ENDPOINT}/api/v1/sim/${id}`, {
        method: 'DELETE',
        headers: {
          'Authorization': `Bearer ${authState.token}`
        }
      });
      
      if (response.ok) {
        return true;
      } else {
        const errorText = await response.text();
        console.error('Failed to delete sim:', response.status, response.statusText, errorText);
      }
    } catch (error) {
      console.error('Error deleting sim:', error);
    }
    
    return false;
  };

  return {
    sims,
    createSim,
    deleteSim
  };
}
