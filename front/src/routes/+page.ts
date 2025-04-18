export const prerender = true;
export const ssr = false;

import { auth } from '$lib/auth';
import { get } from 'svelte/store';

type Sim = {
  name: string
  stats: Array<{
    name: string, 
    value: string
  }>
}

export async function load({fetch} ) : Promise<any> {
  let saving = false;
  const authState = get(auth);

  const fetchWithAuth = async (url: string, options: RequestInit = {}) => {
    if (authState.isAuthenticated && authState.token) {
      options.headers = {
        ...options.headers,
        'Authorization': `Bearer ${authState.token}`
      };
    }
    return fetch(url, options);
  };

  let sim = null;
  if (authState.isAuthenticated) {
    const req = await fetchWithAuth(`http://${import.meta.env.VITE_API_ENDPOINT}/api/v1/sim/1`);
    if (req.ok) {
      sim = await req.json();
    }
  }

  return {
    sim: sim as Sim,
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
            body: JSON.stringify(data)
          });
          saving = false;
        });
    }
  }
}
