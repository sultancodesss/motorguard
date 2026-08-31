Router.register('ride-history', (el) => {
  el.innerHTML = `
    ${buildTopBar({ title: 'Ride History', showAvatar: true, showSettings: true })}

    <div class="screen-body">
      <!-- Page header -->
      <div style="padding:var(--space-lg) var(--space-md) var(--space-sm)">
        <h1 class="text-large-title">Ride History</h1>
        <p class="text-body text-on-surface-var" style="margin-top:4px">Your past rides and performance</p>
      </div>

      <!-- Segmented control -->
      <div style="padding:0 var(--space-md) var(--space-md)">
        <div class="seg-control" id="ride-filter">
          <button class="seg-btn active" data-filter="all">All Rides</button>
          <button class="seg-btn"        data-filter="commute">Commutes</button>
          <button class="seg-btn"        data-filter="weekend">Weekend</button>
        </div>
      </div>

      <!-- Ride list -->
      <div id="rides-list">
        <div style="padding:var(--space-xl);text-align:center;color:var(--on-surface-variant)">
          <div class="spinner" style="margin:0 auto;border-color:rgba(0,88,188,0.3);border-top-color:var(--primary)"></div>
          <p class="text-subhead" style="margin-top:var(--space-md)">Loading rides…</p>
        </div>
      </div>
    </div>

    ${buildBottomNav('ride-history')}`;

  attachNavHandlers(el);

  // Segmented control
  el.querySelector('#ride-filter').addEventListener('click', e => {
    const btn = e.target.closest('.seg-btn');
    if (!btn) return;
    el.querySelectorAll('.seg-btn').forEach(b => b.classList.remove('active'));
    btn.classList.add('active');
  });

  loadRides(el);
});

async function loadRides(el) {
  try {
    const data = await Api.listRides();
    const rides = data?.rides || [];
    renderRides(el, rides);
  } catch (err) {
    el.querySelector('#rides-list').innerHTML = `
      <div class="empty-state">
        <span class="material-symbols-outlined">error_outline</span>
        <p class="text-subhead">Failed to load rides</p>
        <button class="btn btn-primary btn-pill" onclick="loadRides(document.getElementById('screen-ride-history'))">Retry</button>
      </div>`;
  }
}

function renderRides(el, rides) {
  const container = el.querySelector('#rides-list');

  if (!rides.length) {
    container.innerHTML = `
      <div class="empty-state">
        <span class="material-symbols-outlined">two_wheeler</span>
        <p class="text-title-3">No rides yet</p>
        <p class="text-body text-on-surface-var">Start your first ride from the Home tab</p>
        <button class="btn btn-primary btn-pill" onclick="Router.go('home')">Start a Ride</button>
      </div>`;
    return;
  }

  // Group by month
  const grouped = {};
  rides.forEach(r => {
    const d = new Date(r.created_at);
    const key = d.toLocaleDateString('en-US', { month: 'long', year: 'numeric' });
    if (!grouped[key]) grouped[key] = [];
    grouped[key].push(r);
  });

  let html = '';
  for (const [month, monthRides] of Object.entries(grouped)) {
    html += `
      <div style="padding:var(--space-md) var(--space-md) var(--space-sm)">
        <h2 class="text-title-3">${month}</h2>
      </div>
      <div style="margin:0 var(--space-md);background:var(--surface-container-lowest);border-radius:var(--radius-lg);overflow:hidden">
        ${monthRides.map((r, i) => `
          <div class="ride-card${i > 0 ? '' : ''}" style="${i > 0 ? 'border-top:1px solid var(--outline-variant)' : ''}">
            <div class="ride-thumb">
              <div class="map-placeholder">
                <span class="material-symbols-outlined" style="font-size:28px;opacity:0.5">route</span>
              </div>
            </div>
            <div class="ride-info">
              <div style="display:flex;align-items:flex-start;justify-content:space-between;gap:4px">
                <span class="ride-name">${r.name || 'Ride'}</span>
                <span class="text-caption text-on-surface-var" style="flex-shrink:0">${formatTime(r.created_at)}</span>
              </div>
              <div class="ride-route">${formatDate(r.created_at)}</div>
              <div class="ride-stats">
                <div class="ride-stat-item">
                  <span class="material-symbols-outlined">straighten</span>
                  ${formatDistance(r.distance_miles)}
                </div>
                <div class="ride-stat-item">
                  <span class="material-symbols-outlined">schedule</span>
                  ${formatDuration(r.duration_seconds)}
                </div>
                ${r.safety_score != null ? `
                <div class="ride-score ${r.safety_score >= 90 ? 'good' : 'warn'}" style="margin-left:auto">
                  <span class="material-symbols-outlined" style="font-variation-settings:'FILL' 1">
                    ${r.safety_score >= 90 ? 'verified' : 'warning'}
                  </span>
                  ${Math.round(r.safety_score)}
                </div>` : ''}
              </div>
            </div>
          </div>`).join('')}
      </div>`;
  }

  container.innerHTML = html;
}
