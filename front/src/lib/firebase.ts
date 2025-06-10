export type FirebaseConfig = {
  apiKey: string;
  authDomain: string;
  projectId: string;
  storageBucket: string;
  messagingSenderId: string;
  appId: string;
};


// Firebase config will be fetched from API
let firebaseConfig: FirebaseConfig | null = null;

export async function getFirebaseConfig(): Promise<FirebaseConfig> {
    if (firebaseConfig) {
        return firebaseConfig;
    }
    
    try {
        const response = await fetch('/api/v1/config/firebase');
        if (!response.ok) {
            throw new Error('Failed to fetch Firebase config');
        }
        firebaseConfig = await response.json();
        return firebaseConfig;
    } catch (error) {
        console.error('Failed to load Firebase config:', error);
        // Fallback to environment variables for development
        firebaseConfig = {
            apiKey: import.meta.env.VITE_FIREBASE_API_KEY as string || '',
            authDomain: import.meta.env.VITE_FIREBASE_AUTH_DOMAIN as string || '',
            projectId: import.meta.env.VITE_FIREBASE_PROJECT_ID as string || '',
            storageBucket: import.meta.env.VITE_FIREBASE_STORAGE_BUCKET as string || '',
            messagingSenderId: import.meta.env.VITE_FIREBASE_MESSAGING_SENDER_ID as string || '',
            appId: import.meta.env.VITE_FIREBASE_APP_ID as string || '',
        };
        return firebaseConfig;
    }
}

export async function registerSW () {
    if ('serviceWorker' in navigator) {
        const config = await getFirebaseConfig();
        const UrlFirebaseConfig = new URLSearchParams(config);
        const swUrl = `/firebase-messaging-sw.js?${UrlFirebaseConfig}`;
        
        const registration = await navigator.serviceWorker.register(swUrl);
        console.log('Service Worker registered');
        return registration;
    }
    console.log('Service Worker not supported');
    return null;
}
