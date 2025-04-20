import { writable } from 'svelte/store';
import { browser } from '$app/environment';

type Theme = 'light' | 'dark';

// Function to get theme from localStorage
function getStoredTheme(): Theme {
  if (!browser) return 'light';
  
  try {
    const storedTheme = localStorage.getItem('theme');
    if (storedTheme === 'dark' || storedTheme === 'light') {
      return storedTheme;
    }
    
    // Check user preference if no stored theme
    if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
      return 'dark';
    }
  } catch (error) {
    console.error('Error getting theme from localStorage', error);
  }
  
  return 'light';
}

// Function to apply theme to document
function applyTheme(theme: Theme) {
  if (!browser) return;
  
  try {
    document.documentElement.setAttribute('data-theme', theme);
    localStorage.setItem('theme', theme);
  } catch (error) {
    console.error('Error applying theme', error);
  }
}

// Create theme store
const createThemeStore = () => {
  const initialTheme = getStoredTheme();
  
  // Initialize theme on document
  if (browser) {
    applyTheme(initialTheme);
  }
  
  const { subscribe, set, update } = writable<Theme>(initialTheme);
  
  return {
    subscribe,
    setTheme: (theme: Theme) => {
      set(theme);
      applyTheme(theme);
    },
    toggle: () => {
      update(currentTheme => {
        const newTheme = currentTheme === 'light' ? 'dark' : 'light';
        applyTheme(newTheme);
        return newTheme;
      });
    }
  };
};

export const theme = createThemeStore();