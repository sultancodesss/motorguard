/**
 * Minimal reactive store — simple pub/sub around a plain object.
 */
const Store = (() => {
  const state = {
    user:          null,
    accessToken:   localStorage.getItem('mg_access_token')   || null,
    refreshToken:  localStorage.getItem('mg_refresh_token')  || null,
    rides:         [],
    activeRide:    null,       // ride being recorded right now
    groups:        [],
    contacts:      [],
    currentScreen: 'splash',
    previousScreen: null,
    rideStartTime: null,
    sosState:      'idle',     // idle | countdown | active | cancelled
    sosEventId:    null,
    sosCountdown:  10,
    map:           null,       // Leaflet map instance
    groupWs:       null,       // active WebSocket
  };

  const listeners = {};

  function get(key) { return state[key]; }

  function set(key, value) {
    state[key] = value;
    if (listeners[key]) listeners[key].forEach(fn => fn(value));
    if (listeners['*'])  listeners['*'].forEach(fn => fn(key, value));
  }

  function on(key, fn) {
    if (!listeners[key]) listeners[key] = [];
    listeners[key].push(fn);
  }

  function off(key, fn) {
    if (!listeners[key]) return;
    listeners[key] = listeners[key].filter(f => f !== fn);
  }

  function persist(key, value) {
    set(key, value);
    if (value === null) localStorage.removeItem(`mg_${key}`);
    else                localStorage.setItem(`mg_${key}`, value);
  }

  return { get, set, on, off, persist };
})();
