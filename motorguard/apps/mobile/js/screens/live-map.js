let _mapInstance = null;
let _wsGroupConn = null;

Router.register('live-map', (el) => {
  el.innerHTML = `
    ${buildTopBar({ title: 'Live Map', showAvatar: true, showSettings: true })}

    <!-- Full-screen map -->
    <div id="map-container" style="
      position:absolute;top:var(--top-bar-h);bottom:var(--bottom-nav-h);left:0;right:0;
    ">
      <div id="leaflet-map" style="width:100%;height:100%"></div>

      <!-- Hazard markers (positioned absolutely inside map) -->
      <div style="position:absolute;top:35%;left:45%;transform:translate(-50%,-50%);z-index:500">
        <div class="hazard-marker" id="hazard-oil" title="Oil Spill">
          <span class="material-symbols-outlined" style="font-size:22px;color:#f59e0b">oil_barrel</span>
        </div>
      </div>
      <div style="position:absolute;top:55%;left:30%;transform:translate(-50%,-50%);z-index:500">
        <div class="hazard-marker" id="hazard-pot" title="Pothole">
          <span class="material-symbols-outlined" style="font-size:22px;color:#ef4444">road</span>
        </div>
      </div>
      <div style="position:absolute;top:40%;left:70%;transform:translate(-50%,-50%);z-index:500">
        <div class="hazard-marker" id="hazard-rain" title="Wet Road">
          <span class="material-symbols-outlined" style="font-size:22px;color:#3b82f6">rainy</span>
        </div>
      </div>
    </div>

    <!-- FAB: Report Hazard -->
    <button class="fab" id="fab-hazard" style="z-index:600;bottom:calc(var(--bottom-nav-h) + 16px)">
      <span class="material-symbols-outlined">add_alert</span>
    </button>

    <!-- Bottom sheet overlay -->
    <div class="sheet-overlay" id="hazard-sheet-overlay">
      <div class="sheet">
        <div class="sheet-handle"></div>
        <div id="sheet-content"></div>
      </div>
    </div>

    ${buildBottomNav('live-map')}`;

  attachNavHandlers(el);

  // Init Leaflet map
  if (_mapInstance) {
    _mapInstance.remove();
    _mapInstance = null;
  }

  const map = L.map('leaflet-map', { zoomControl: false, attributionControl: false });
  _mapInstance = map;

  L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
    maxZoom: 19,
  }).addTo(map);

  // Try to get user location
  if (navigator.geolocation) {
    navigator.geolocation.getCurrentPosition(
      pos => {
        map.setView([pos.coords.latitude, pos.coords.longitude], 15);
        L.circleMarker([pos.coords.latitude, pos.coords.longitude], {
          radius: 10,
          fillColor: '#0058bc',
          color: 'white',
          weight: 3,
          fillOpacity: 1,
        }).addTo(map).bindPopup('You are here');
      },
      () => map.setView([37.7749, -122.4194], 13)
    );
  } else {
    map.setView([37.7749, -122.4194], 13);
  }

  // Hazard definitions
  const hazards = {
    'hazard-oil': { icon: 'oil_barrel', title: 'Oil Spill', color: '#f59e0b', desc: 'Slippery oil detected on road surface. Reduce speed and avoid sudden maneuvers.' },
    'hazard-pot': { icon: 'road', title: 'Pothole', color: '#ef4444', desc: 'Large pothole reported ahead. Move to the left lane if possible.' },
    'hazard-rain': { icon: 'rainy', title: 'Wet Road', color: '#3b82f6', desc: 'Road surface is wet from recent rain. Maintain extra following distance.' },
  };

  const overlay = el.querySelector('#hazard-sheet-overlay');
  const content = el.querySelector('#sheet-content');

  function openSheet(hazardId) {
    const h = hazards[hazardId];
    if (!h) return;
    content.innerHTML = `
      <div style="display:flex;align-items:center;gap:var(--space-md);margin-bottom:var(--space-md)">
        <div style="
          width:48px;height:48px;border-radius:var(--radius-md);
          background:${h.color}20;
          display:flex;align-items:center;justify-content:center;flex-shrink:0
        ">
          <span class="material-symbols-outlined" style="font-size:26px;color:${h.color}">${h.icon}</span>
        </div>
        <div>
          <div class="text-headline">${h.title}</div>
          <div class="text-footnote text-on-surface-var">Road Hazard Alert</div>
        </div>
      </div>
      <p class="text-body text-on-surface-var" style="margin-bottom:var(--space-lg)">${h.desc}</p>
      <div style="display:grid;grid-template-columns:1fr 1fr;gap:var(--space-sm)">
        <button class="btn btn-secondary" id="sheet-dismiss">Dismiss</button>
        <button class="btn btn-primary">Got It</button>
      </div>`;

    overlay.classList.add('open');

    content.querySelector('#sheet-dismiss').addEventListener('click', closeSheet);
    content.querySelector('.btn-primary').addEventListener('click', closeSheet);
  }

  function closeSheet() {
    overlay.classList.remove('open');
  }

  overlay.addEventListener('click', e => { if (e.target === overlay) closeSheet(); });

  // Wire hazard markers
  Object.keys(hazards).forEach(id => {
    const marker = el.querySelector(`#${id}`);
    if (marker) marker.addEventListener('click', () => openSheet(id));
  });

  // FAB
  el.querySelector('#fab-hazard').addEventListener('click', () => {
    showToast('Hazard reporting coming soon', 'info');
  });

}, (el) => {
  if (_wsGroupConn) {
    _wsGroupConn.close();
    _wsGroupConn = null;
  }
});
