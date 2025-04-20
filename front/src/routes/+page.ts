export const prerender = false;
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
}

export async function load({ fetch, url }) {
  let saving = false;
  const authState = get(auth);
  
  // Get sim ID from URL query parameter
  const simId = url.searchParams.get('sim') || '1';

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
      if (!authState.isAuthenticated) return;
      
      // Create delay to only save after 1 second without changes
      if (saving) return;
      saving = true;
      await new Promise(resolve => setTimeout(resolve, 1000))
        .then(async () => {
          await fetchWithAuth(`http://${import.meta.env.VITE_API_ENDPOINT}/api/v1/sim`, {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json'
            },
            body: JSON.stringify(data),
          });
          saving = false;
        });
    }
  }
}
