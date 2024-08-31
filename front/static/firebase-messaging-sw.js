importScripts('https://www.gstatic.com/firebasejs/8.10.1/firebase-app.js');
importScripts('https://www.gstatic.com/firebasejs/8.10.1/firebase-messaging.js');

// "Default" Firebase configuration (prevents errors)
const defaultConfig = {
  apiKey: true,
  projectId: true,
  messagingSenderId: true,
  appId: true,
};

try {
  const urlParams = new URLSearchParams(location.search);
  self.firebaseConfig = Object.fromEntries(urlParams);
} catch (err) {
  console.error('Failed to add event listener', err);
}

firebase.initializeApp(self.firebaseConfig || defaultConfig);
const messaging = firebase.messaging();

if (messaging) {
  try {
    messaging.onBackgroundMessage((payload) => {
      console.log('[firebase-messaging-sw.js] Received background message ', payload);
      self.registration.showNotification(payload.notification.title, payload.notification);
    });
  } catch (error) {
    console.error('Error with firebase message', error);
  }
}

// Handling Notification click
self.addEventListener('notificationclick', (event) => {
    event.notification.close(); // CLosing the notification when clicked
    const urlToOpen = event?.notification?.data?.url || 'https://www.test.com/';
    // Open the URL in the default browser.
    event.waitUntil(
      clients.matchAll({
        type: 'window',
      })
      .then((windowClients) => {
        // Check if there is already a window/tab open with the target URL
        for (const client of windowClients) {
          if (client.url === urlToOpen && 'focus' in client) {
            return client.focus();
          }
        }
        // If not, open a new window/tab with the target URL
        if (clients.openWindow) {
          return clients.openWindow(urlToOpen);
        }
      })
    );
  });
