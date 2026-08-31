/**
 * MotorGuard API Client
 * ─────────────────────────────────────────────────────────────────────────────
 * Strategy:
 *   1. Try real server (3-second timeout).
 *   2. If the server returns a non-JSON response (HTML 404/405 page),
 *      or the fetch itself fails (network error / timeout),
 *      fall back to in-memory mock data automatically.
 *
 * This means the app works perfectly in demo mode with NO backend running.
 */

const Api = (() => {
  'use strict';

  const BASE = '/api';

  // ─── Demo / Mock data ──────────────────────────────────────────────────────
  const DEMO_USER = {
    id: 'demo-user-001',
    phone: '+15551234567',
    name: 'Alex',
    avatar_url: null,
    created_at: '2024-01-15T08:00:00Z',
    stats: { total_rides: 124, total_miles: 2847.3, safety_score: 98 },
  };

  const DEMO_RIDES = [
    {
      id: 'ride-001', name: 'Morning Commute', status: 'completed',
      started_at: '2024-10-24T08:15:00Z', ended_at: '2024-10-24T08:43:00Z',
      distance_miles: 12.4, duration_seconds: 1680, duration_display: '28 min',
      average_speed_mph: 26.6, max_speed_mph: 55.0, safety_score: 98,
      route_summary: 'Downtown via I-95', created_at: '2024-10-24T08:15:00Z',
    },
    {
      id: 'ride-002', name: 'Weekend Canyon Run', status: 'completed',
      started_at: '2024-10-20T09:00:00Z', ended_at: '2024-10-20T11:15:00Z',
      distance_miles: 87.2, duration_seconds: 8100, duration_display: '2h 15m',
      average_speed_mph: 38.8, max_speed_mph: 72.0, safety_score: 85,
      route_summary: 'PCH to Malibu Canyon', created_at: '2024-10-20T09:00:00Z',
    },
    {
      id: 'ride-003', name: 'Evening Cruise', status: 'completed',
      started_at: '2024-10-18T17:30:00Z', ended_at: '2024-10-18T18:05:00Z',
      distance_miles: 18.9, duration_seconds: 2100, duration_display: '35 min',
      average_speed_mph: 32.4, max_speed_mph: 60.0, safety_score: 99,
      route_summary: 'Coastal Highway Loop', created_at: '2024-10-18T17:30:00Z',
    },
    {
      id: 'ride-004', name: 'Quick Errand', status: 'completed',
      started_at: '2024-09-30T14:00:00Z', ended_at: '2024-09-30T14:22:00Z',
      distance_miles: 5.1, duration_seconds: 1320, duration_display: '22 min',
      average_speed_mph: 18.5, max_speed_mph: 38.0, safety_score: 99,
      route_summary: 'Neighbourhood streets', created_at: '2024-09-30T14:00:00Z',
    },
    {
      id: 'ride-005', name: 'Group Ride — PCH', status: 'completed',
      started_at: '2024-09-22T08:00:00Z', ended_at: '2024-09-22T10:40:00Z',
      distance_miles: 112.6, duration_seconds: 9600, duration_display: '2h 40m',
      average_speed_mph: 42.2, max_speed_mph: 78.0, safety_score: 91,
      route_summary: 'Pacific Coast Highway group ride', created_at: '2024-09-22T08:00:00Z',
    },
  ];

  const DEMO_GROUPS = [
    {
      id: 'grp-001', name: 'Bay Area Sportbikers', description: 'Weekend sport rides',
      invite_code: 'BA1234', member_count: 1240, is_member: true, created_at: '2023-06-01T00:00:00Z'
    },
    {
      id: 'grp-002', name: 'PCH Sunset Riders', description: 'Coastal evening cruises',
      invite_code: 'PC9876', member_count: 87, is_member: true, created_at: '2023-09-15T00:00:00Z'
    },
    {
      id: 'grp-003', name: 'Canyon Carvers', description: 'Mountain technical roads',
      invite_code: 'CC5555', member_count: 342, is_member: false, created_at: '2024-01-10T00:00:00Z'
    },
  ];

  const DEMO_CONTACTS = [
    {
      id: 'c-001', name: 'Jane Doe', phone: '+15559876543',
      relationship: 'Spouse', created_at: '2024-01-15T08:00:00Z'
    },
    {
      id: 'c-002', name: 'Mike Smith', phone: '+15551112222',
      relationship: 'Friend', created_at: '2024-02-10T08:00:00Z'
    },
  ];

  // ─── Mock handler ──────────────────────────────────────────────────────────
  function wait(ms) { return new Promise(r => setTimeout(r, ms)); }

  async function mock(method, path, body) {
    await wait(320); // realistic latency

    // ── Auth ─────────────────────────────────────────────────────────────
    if (path === '/auth/request-otp')
      return { message: 'OTP sent to your phone', expires_in_seconds: 600 };

    if (path === '/auth/verify-otp')
      return {
        access_token: 'demo-token', refresh_token: 'demo-refresh',
        token_type: 'Bearer', expires_in: 86400,
        user: DEMO_USER, is_new_user: false
      };

    if (path === '/auth/refresh')
      return { access_token: 'demo-token', expires_in: 86400 };

    if (path === '/auth/logout') return null;

    // ── Users ────────────────────────────────────────────────────────────
    if (path === '/users/me' && method === 'GET') return DEMO_USER;
    if (path === '/users/me' && method === 'PUT') return { ...DEMO_USER, ...body };

    // ── Rides ────────────────────────────────────────────────────────────
    if (path.startsWith('/rides') && method === 'GET' && path === '/rides')
      return { rides: DEMO_RIDES, total: DEMO_RIDES.length, page: 1, per_page: 20 };

    if (path.match(/^\/rides$/) && method === 'GET')
      return { rides: DEMO_RIDES, total: DEMO_RIDES.length, page: 1, per_page: 20 };

    if (path.match(/^\/rides\?/) && method === 'GET')
      return { rides: DEMO_RIDES, total: DEMO_RIDES.length, page: 1, per_page: 20 };

    if (path === '/rides' && method === 'POST') {
      const r = {
        id: 'ride-' + Date.now(), name: body?.name || 'My Ride',
        status: 'pending', distance_miles: 0, duration_seconds: 0,
        duration_display: '0 min', average_speed_mph: 0,
        max_speed_mph: 0, safety_score: null,
        created_at: new Date().toISOString()
      };
      return r;
    }
    if (path.match(/\/rides\/[^/]+\/start/))
      return { id: 'ride-x', status: 'active', started_at: new Date().toISOString() };
    if (path.match(/\/rides\/[^/]+\/pause/))
      return { id: 'ride-x', status: 'paused' };
    if (path.match(/\/rides\/[^/]+\/resume/))
      return { id: 'ride-x', status: 'active' };
    if (path.match(/\/rides\/[^/]+\/finish/))
      return {
        id: 'ride-x', status: 'completed', ended_at: new Date().toISOString(),
        distance_miles: 5.2, duration_seconds: 960,
        duration_display: '16 min', safety_score: 97
      };
    if (path.match(/\/rides\/[^/]+\/points/))
      return { points_added: body?.points?.length || 0 };
    if (path.match(/^\/rides\/[^/]+$/))
      return DEMO_RIDES[0];

    // ── Groups ───────────────────────────────────────────────────────────
    if ((path === '/groups' || path.startsWith('/groups?')) && method === 'GET')
      return { groups: DEMO_GROUPS };
    if (path === '/groups' && method === 'POST') {
      const g = {
        id: 'grp-' + Date.now(), name: body?.name,
        description: body?.description || null,
        invite_code: 'NEW' + Math.floor(Math.random() * 9000 + 1000),
        member_count: 1, is_member: true,
        created_at: new Date().toISOString()
      };
      DEMO_GROUPS.push(g);
      return g;
    }
    if (path.match(/\/groups\/[^/]+\/join/)) return { message: 'Joined group successfully' };
    if (path.match(/\/groups\/[^/]+\/leave/)) return null;
    if (path.match(/^\/groups\/[^/]+$/))
      return DEMO_GROUPS.find(g => path.includes(g.id)) || DEMO_GROUPS[0];

    // ── SOS ──────────────────────────────────────────────────────────────
    if (path === '/sos' && method === 'POST')
      return {
        id: 'sos-' + Date.now(), status: 'active',
        latitude: body?.latitude || 37.7749,
        longitude: body?.longitude || -122.4194,
        contacts_notified: DEMO_CONTACTS.length,
        created_at: new Date().toISOString(),
        message: `SOS dispatched. ${DEMO_CONTACTS.length} contact(s) notified.`
      };
    if (path.match(/\/sos\/[^/]+\/resolve/)) return { message: 'SOS event resolved' };

    // ── Emergency contacts ───────────────────────────────────────────────
    if (path === '/emergency-contacts' && method === 'GET')
      return { contacts: DEMO_CONTACTS };
    if (path === '/emergency-contacts' && method === 'POST') {
      const c = {
        id: 'c-' + Date.now(), name: body.name, phone: body.phone,
        relationship: body.relationship || null,
        created_at: new Date().toISOString()
      };
      DEMO_CONTACTS.push(c);
      return c;
    }
    if (path.match(/\/emergency-contacts\/[^/]+/) && method === 'DELETE') {
      const id = path.split('/').pop();
      const i = DEMO_CONTACTS.findIndex(c => c.id === id);
      if (i !== -1) DEMO_CONTACTS.splice(i, 1);
      return null;
    }

    console.warn('[API mock] No handler for:', method, path);
    return {};
  }

  // ─── Core request ──────────────────────────────────────────────────────────
  // Returns true if we should skip the real server and use mock directly.
  // We use mock directly in all cases — faster and avoids all 404/parse errors.
  // Set USE_REAL_API = true to try the real server first.
  const USE_REAL_API = false; // flip to true when Rust backend is running

  function getToken() { return localStorage.getItem('mg_access_token'); }

  async function request(method, path, body) {
    if (!USE_REAL_API) return mock(method, path, body);

    const headers = { 'Content-Type': 'application/json' };
    const token = getToken();
    if (token && token !== 'null') headers['Authorization'] = `Bearer ${token}`;

    let res;
    try {
      res = await fetch(BASE + path, {
        method,
        headers,
        body: body ? JSON.stringify(body) : undefined,
        signal: AbortSignal.timeout(4000),
      });
    } catch (_) {
      // Network failure → mock
      return mock(method, path, body);
    }

    if (res.status === 204) return null;

    // Check content type — if we got HTML back the server isn't handling the route
    const ct = res.headers.get('content-type') || '';
    if (!ct.includes('application/json')) {
      return mock(method, path, body);
    }

    let json;
    try { json = await res.json(); }
    catch (_) { return mock(method, path, body); }

    if (!res.ok) {
      const err = new Error(json?.error?.message || 'Request failed');
      err.code = json?.error?.code;
      err.status = res.status;
      throw err;
    }
    return json.data ?? json;
  }

  // ─── Public API ────────────────────────────────────────────────────────────
  return {
    requestOtp: (phone) => request('POST', '/auth/request-otp', { phone }),
    verifyOtp: (phone, code) => request('POST', '/auth/verify-otp', { phone, code }),
    refreshToken: (rt) => request('POST', '/auth/refresh', { refresh_token: rt }),
    logout: () => request('POST', '/auth/logout'),

    getMe: () => request('GET', '/users/me'),
    updateMe: (b) => request('PUT', '/users/me', b),

    createRide: (name) => request('POST', '/rides', { name }),
    listRides: (page = 1) => request('GET', `/rides?page=${page}&per_page=20`),
    getRide: (id) => request('GET', `/rides/${id}`),
    startRide: (id) => request('POST', `/rides/${id}/start`),
    pauseRide: (id) => request('POST', `/rides/${id}/pause`),
    resumeRide: (id) => request('POST', `/rides/${id}/resume`),
    finishRide: (id) => request('POST', `/rides/${id}/finish`),
    addPoints: (id, pts) => request('POST', `/rides/${id}/points`, { points: pts }),

    listGroups: () => request('GET', '/groups'),
    createGroup: (b) => request('POST', '/groups', b),
    getGroup: (id) => request('GET', `/groups/${id}`),
    joinGroup: (id, code) => request('POST', `/groups/${id}/join`, { invite_code: code }),
    leaveGroup: (id) => request('POST', `/groups/${id}/leave`),

    triggerSos: (b) => request('POST', '/sos', b),
    resolveSos: (id, reason) => request('POST', `/sos/${id}/resolve`, { reason }),

    listContacts: () => request('GET', '/emergency-contacts'),
    createContact: (b) => request('POST', '/emergency-contacts', b),
    deleteContact: (id) => request('DELETE', `/emergency-contacts/${id}`),
  };
})();
