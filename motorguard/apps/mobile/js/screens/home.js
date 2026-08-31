Router.register('home', (el) => {
  const user     = Store.get('user') || { name: 'Alex', stats: { total_rides: 124, total_miles: 2847.3, safety_score: 98 } };
  const hour     = new Date().getHours();
  const greeting = hour < 12 ? 'Good morning' : hour < 17 ? 'Good afternoon' : 'Good evening';
  const initials = (user.name || 'R').split(' ').map(w => w[0]).join('').toUpperCase().slice(0, 2);
  const stats    = user.stats || { total_rides: 0, total_miles: 0, safety_score: 0 };

  el.innerHTML = `
    ${buildTopBar({ title: 'MotorGuard', showAvatar: true, showSettings: true })}
    <div class="screen-body">

      <!-- Welcome -->
      <div style="padding:var(--space-lg) var(--space-md) var(--space-md)">
        <h1 class="text-large-title">${greeting}, ${user.name || 'Rider'}.</h1>
        <p class="text-body text-on-surface-var" style="margin-top:5px">Your bike is ready — let's ride safely.</p>
      </div>

      <!-- Start Ride card -->
      <div style="padding:0 var(--space-md)">
        <div class="start-ride-card" id="start-ride-btn" role="button" tabindex="0">
          <div class="ride-icon">
            <span class="material-symbols-outlined" style="font-variation-settings:'FILL' 1">two_wheeler</span>
          </div>
          <div style="color:white;margin-top:var(--space-sm)">
            <div class="text-title-2" style="color:white">Start Ride</div>
            <div class="text-body" style="color:rgba(255,255,255,0.82);margin-top:4px">
              GPS tracking · live safety monitoring
            </div>
            <div class="text-footnote" style="color:rgba(255,255,255,0.6);margin-top:var(--space-md)">
              Tap to begin →
            </div>
          </div>
        </div>
      </div>

      <!-- Quick actions 2-col grid -->
      <div style="padding:var(--space-md);display:grid;grid-template-columns:1fr 1fr;gap:var(--space-sm)">

        <!-- SOS card -->
        <div class="sos-card" id="sos-btn" role="button" tabindex="0">
          <div class="sos-icon-wrap pulse-ring">
            <span class="material-symbols-outlined" style="font-variation-settings:'FILL' 1">emergency</span>
          </div>
          <div style="text-align:center;margin-top:4px">
            <div class="text-headline" style="color:var(--on-error-container)">SOS Alert</div>
            <div class="text-caption text-on-surface-var">Emergency dispatch</div>
          </div>
        </div>

        <!-- Weather card -->
        <div class="weather-card">
          <div style="display:flex;align-items:center;gap:var(--space-sm)">
            <span class="material-symbols-outlined" style="font-size:30px;color:#5b8dd9">partly_cloudy_day</span>
            <div>
              <div class="text-title-3">72°F</div>
              <div class="text-caption text-on-surface-var">Clear skies</div>
            </div>
          </div>
          <div style="margin-top:var(--space-xs);display:flex;align-items:center;gap:4px">
            <span class="material-symbols-outlined" style="font-size:14px;color:#1a7f37">check_circle</span>
            <span class="text-footnote" style="color:#1a7f37;font-weight:500">Optimal riding conditions</span>
          </div>
          <div class="text-caption text-on-surface-var" style="margin-top:2px">Wind 8 mph · Humidity 52%</div>
        </div>
      </div>

      <!-- Active Groups -->
      <div class="section-header">
        <span class="text-title-3">Active Groups</span>
        <button class="text-callout text-primary"
          style="background:none;border:none;cursor:pointer;font-family:inherit;font-weight:500"
          onclick="Router.go('groups')">See All</button>
      </div>
      <div id="home-groups" style="padding:0 var(--space-md)">
        <div style="text-align:center;padding:var(--space-lg)">
          <div class="spinner" style="margin:0 auto;border-color:rgba(0,88,188,0.25);border-top-color:var(--primary)"></div>
        </div>
      </div>

      <!-- Stats -->
      <div class="section-header" style="margin-top:var(--space-xs)">
        <span class="text-title-3">Your Stats</span>
      </div>
      <div style="padding:0 var(--space-md) var(--space-xl);display:grid;grid-template-columns:1fr 1fr 1fr;gap:var(--space-sm)">
        <div class="stat-card">
          <div class="stat-value">${stats.total_rides}</div>
          <div class="stat-label">Rides</div>
        </div>
        <div class="stat-card">
          <div class="stat-value">${formatDistance(stats.total_miles || 0)}</div>
          <div class="stat-label">Distance</div>
        </div>
        <div class="stat-card">
          <div class="stat-value tertiary">${Math.round(stats.safety_score || 0)}%</div>
          <div class="stat-label">Safety</div>
        </div>
      </div>
    </div>

    ${buildBottomNav('home')}`;

  // Avatar initials
  const av = el.querySelector('#bar-avatar');
  if (av) { av.innerHTML = ''; av.textContent = initials; }

  attachNavHandlers(el);

  // Start Ride
  el.querySelector('#start-ride-btn').addEventListener('click', () => startRideFlow());

  // SOS
  el.querySelector('#sos-btn').addEventListener('click', () => Router.go('sos'));

  // Load groups preview
  Api.listGroups().then(data => {
    const groups = data?.groups || [];
    const container = el.querySelector('#home-groups');
    if (!container) return;

    if (!groups.length) {
      container.innerHTML = `
        <div class="empty-state">
          <span class="material-symbols-outlined">group</span>
          <p class="text-subhead">No active groups</p>
          <button class="btn btn-primary btn-pill" onclick="Router.go('groups')">Find Groups</button>
        </div>`;
      return;
    }

    const memberGroups = groups.filter(g => g.is_member).slice(0, 2);
    const colors = ['var(--primary-container)','var(--secondary-container)'];
    container.innerHTML = memberGroups.map((g, i) => `
      <div class="card" style="margin-bottom:var(--space-sm);cursor:pointer" onclick="Router.go('groups')">
        <div style="padding:var(--space-md);display:flex;align-items:center;gap:var(--space-md)">
          <div style="
            width:48px;height:48px;border-radius:var(--radius-md);
            background:${colors[i % colors.length]};
            display:flex;align-items:center;justify-content:center;flex-shrink:0
          ">
            <span class="material-symbols-outlined" style="font-size:24px;color:white;font-variation-settings:'FILL' 1">
              sports_motorsports
            </span>
          </div>
          <div style="flex:1;min-width:0">
            <div class="text-headline" style="overflow:hidden;text-overflow:ellipsis;white-space:nowrap">${g.name}</div>
            <div class="text-footnote text-on-surface-var">${g.member_count.toLocaleString()} members</div>
          </div>
          <span class="badge badge-live">Live</span>
        </div>
      </div>`).join('');
  }).catch(() => {
    const c = el.querySelector('#home-groups');
    if (c) c.innerHTML = '';
  });

  // Active ride banner (if ride in progress)
  if (Store.get('activeRide')) showActiveRideBanner();
});

// ── Ride flow ─────────────────────────────────────────────────────────────────
async function startRideFlow() {
  try {
    const ride = await Api.createRide('My Ride');
    await Api.startRide(ride.id);
    Store.set('activeRide', ride);
    Store.set('rideStartTime', Date.now());
    showToast('Ride started! GPS tracking active.', 'success');
    showActiveRideBanner();
    startGpsTracking(ride.id);
  } catch (err) {
    showToast(err.message || 'Could not start ride', 'error');
  }
}

function showActiveRideBanner() {
  if (document.getElementById('active-ride-bar')) return;

  const bar = document.createElement('div');
  bar.id = 'active-ride-bar';
  bar.className = 'active-ride-bar';
  bar.innerHTML = `
    <div style="display:flex;align-items:center;gap:6px">
      <span class="material-symbols-outlined" style="font-size:14px;animation:pulse-ring 1.2s infinite">fiber_manual_record</span>
      <span>Ride in progress</span>
    </div>
    <button id="finish-ride-btn" style="
      background:white;color:var(--primary);border:none;
      border-radius:var(--radius-full);padding:4px 14px;
      font-family:inherit;font-size:var(--text-caption);font-weight:700;cursor:pointer
    ">Finish</button>`;
  document.getElementById('app').appendChild(bar);

  bar.querySelector('#finish-ride-btn').addEventListener('click', async () => {
    const ride = Store.get('activeRide');
    if (!ride) return;
    try {
      const finished = await Api.finishRide(ride.id);
      Store.set('activeRide', null);
      stopGpsTracking();
      bar.remove();
      showToast(`Ride complete! ${formatDistance(finished.distance_miles || 0)} · Score: ${Math.round(finished.safety_score || 0)}`, 'success');
      setTimeout(() => Router.go('ride-history'), 500);
    } catch (err) {
      showToast('Failed to finish ride', 'error');
    }
  });
}

let gpsWatchId = null;
function startGpsTracking(rideId) {
  if (!navigator.geolocation) return;
  const buffer = [];
  gpsWatchId = navigator.geolocation.watchPosition(
    pos => {
      buffer.push({
        latitude:  pos.coords.latitude,
        longitude: pos.coords.longitude,
        altitude:  pos.coords.altitude,
        speed:     pos.coords.speed ? pos.coords.speed * 2.237 : 0,
        accuracy:  pos.coords.accuracy,
        timestamp: new Date(pos.timestamp).toISOString(),
      });
      if (buffer.length >= 5) {
        Api.addPoints(rideId, [...buffer]).catch(() => {});
        buffer.length = 0;
      }
    },
    () => {},
    { enableHighAccuracy: true, maximumAge: 3000, timeout: 10000 }
  );
}

function stopGpsTracking() {
  if (gpsWatchId !== null) {
    navigator.geolocation.clearWatch(gpsWatchId);
    gpsWatchId = null;
  }
}
