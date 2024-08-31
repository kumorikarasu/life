export type FirebaseConfig = {
  apiKey: string;
  authDomain: string;
  projectId: string;
  storageBucket: string;
  messagingSenderId: string;
  appId: string;
};


export const firebaseConfig: FirebaseConfig = {
    apiKey: import.meta.env.VITE_FIREBASE_API_KEY as string,
    authDomain: import.meta.env.VITE_FIREBASE_AUTH_DOMAIN as string,
    projectId: import.meta.env.VITE_FIREBASE_PROJECT_ID as string,
    storageBucket: import.meta.env.VITE_FIREBASE_STORAGE_BUCKET as string,
    messagingSenderId: import.meta.env.VITE_FIREBASE_MESSAGING_SENDER_ID as string,
    appId: import.meta.env.VITE_FIREBASE_APP_ID as string,
};

const UrlFirebaseConfig = new URLSearchParams(firebaseConfig);
const swUrl = `/firebase-messaging-sw.js?${UrlFirebaseConfig}`;

export async function registerSW () {
    if ('serviceWorker' in navigator) {
        const registration = await navigator.serviceWorker.register(swUrl);
        console.log('Service Worker registered');
        return registration;
    }
    console.log('Service Worker not supported');
    return null;
}
