import { writable } from 'svelte/store';
import { browser } from '$app/environment';

interface AuthState {
    isAuthenticated: boolean;
    token: string | null;
    user: {
        name: string;
        email?: string;
    } | null;
}

// Function to get auth state from cookies
function getAuthStateFromCookies(): AuthState {
  if (!browser) {
    return {
      isAuthenticated: false,
      token: null,
      user: null
    };
  }

  try {
    // Check for auth cookie
    const cookies = document.cookie.split(';').map(cookie => cookie.trim());
    const authCookie = cookies.find(cookie => cookie.startsWith('auth='));
    
    if (authCookie) {
      const authData = JSON.parse(decodeURIComponent(authCookie.substring(5)));
      return {
        isAuthenticated: true,
        token: authData.token || null,
        user: authData.user || null
      };
    }
  } catch (error) {
    console.error('Error parsing auth cookie', error);
  }
  
  return {
    isAuthenticated: false,
    token: null,
    user: null
  };
}

// Function to save auth state to cookies
function saveAuthStateToCookies(state: AuthState) {
  if (browser && state.isAuthenticated && state.token) {
    const authData = {
      token: state.token,
      user: state.user
    };
    
    // Set cookie with a long expiration (30 days)
    const expiryDate = new Date();
    expiryDate.setDate(expiryDate.getDate() + 30);
    
    document.cookie = `auth=${encodeURIComponent(JSON.stringify(authData))}; expires=${expiryDate.toUTCString()}; path=/; SameSite=Lax`;
  } else if (browser) {
    // Clear cookie on logout
    document.cookie = 'auth=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;';
  }
}

const createAuthStore = () => {
    // Initialize with persisted state if available
    const initialState = getAuthStateFromCookies();
    
    const { subscribe, set, update } = writable<AuthState>(initialState);

    return {
        subscribe,
        login: (token: string, userData?: { name: string; email?: string }) => {
            const newState = { isAuthenticated: true, token, user: userData || null };
            set(newState);
            saveAuthStateToCookies(newState);
        },
        logout: () => {
            const newState = { isAuthenticated: false, token: null, user: null };
            set(newState);
            saveAuthStateToCookies(newState);
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
  console.log('Callback');
  console.log(import.meta.env.VITE_API_ENDPOINT);

  const response = await fetch(
      `http://${endpoint}/api/v1/auth/callback`,
      {
          method: 'POST',
          headers: {
              'Content-Type': 'application/json',
          },
          body: JSON.stringify({ code, state }),
          credentials: 'include'
      }
  );
  if (response.ok) {
      const data = await response.json();
      const token = data.token || data;
      const userData = data.user ? {
        name: data.user.name || 'User',
        email: data.user.email
      } : null;
      
      auth.login(token, userData);
      return true;
  }
  return true;
};
