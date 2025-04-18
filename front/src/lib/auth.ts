import { writable } from 'svelte/store';

interface AuthState {
    isAuthenticated: boolean;
    token: string | null;
}

const createAuthStore = () => {
    const { subscribe, set, update } = writable<AuthState>({
        isAuthenticated: false,
        token: null
    });

    return {
        subscribe,
        login: (token: string) => {
            set({ isAuthenticated: true, token });
        },
        logout: () => {
            set({ isAuthenticated: false, token: null });
        }
    };
};

export const auth = createAuthStore();

export const login = async () => {
    window.location.href = `http://${import.meta.env.VITE_API_ENDPOINT}/api/v1/auth/login`;
};

export const handleCallback = async (code: string, state: string) => {
  // If the endpoint is localhost, we need to change the URL to the local IP address
  var endpoint = import.meta.env.VITE_API_ENDPOINT;

  if (endpoint.includes('localhost')) {
    endpoint = "api:8000"
  }

  const response = await fetch(
      `http://${endpoint}/api/v1/auth/callback`,
      {
          method: 'POST',
          headers: {
              'Content-Type': 'application/json',
          },
          body: JSON.stringify({ code, state })
      }
  );
  console.log(JSON.stringify(response));
  if (response.ok) {
      const token = await response.text();
      auth.login(token);
      return true;
  }
  return false;
};
