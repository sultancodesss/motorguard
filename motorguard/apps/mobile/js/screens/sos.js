let _sosTimer    = null;
let _sosSeconds  = 10;
let _sosState    = 'idle';

Router.register('sos', (el) => {
  _sosSeconds = 10;
  _sosState   = 'countdown';

  el.innerHTML = `
    <div style="
      min-height:100dvh;
      display:flex;flex-direction:column;
      background:var(--surface);
      position:relative;overflow:hidden;
    ">
      <!-- Error tint overlay -->
      <div style="position:absolute;inset:0;background:rgba(186,26,26,0.07);pointer-events:none"></div>

      <!-- Status bar spacer -->
      <div style="height:44px"></div>

      <div style="flex:1;display:flex;flex-direction:column;align-items:center;justify-content:center;padding:var(--space-lg);gap:var(--space-lg)">
        <!-- Warning icon + title -->
        <div style="text-align:center">
          <span class="material-symbols-outlined" style="
            font-size:48px;color:var(--error);
            font-variation-settings:'FILL' 1;
            display:block;margin-bottom:var(--space-md)
          ">warning</span>
          <h1 class="text-title-1" style="color:var(--on-surface)">Crash Detected</h1>
          <p class="text-body text-on-surface-var" style="margin-top:var(--space-sm);max-width:280px;margin-left:auto;margin-right:auto">
            We detected a possible crash. Emergency services will be contacted automatically.
          </p>
        </div>

        <!-- Countdown circle -->
        <div class="countdown-wrap" id="countdown-wrap">
          <div class="countdown-circle-bg pulse-ring"></div>
          <svg width="192" height="192" viewBox="0 0 192 192">
            <circle cx="96" cy="96" r="88"
              fill="none"
              stroke="rgba(186,26,26,0.15)"
              stroke-width="8"
            />
            <circle id="progress-ring" cx="96" cy="96" r="88"
              fill="none"
              stroke="var(--error)"
              stroke-width="8"
              stroke-linecap="round"
              stroke-dasharray="553"
              stroke-dashoffset="0"
            />
          </svg>
          <div class="countdown-number" id="countdown-num">10</div>
        </div>

        <p class="text-subhead text-on-surface-var" style="letter-spacing:0.08em;text-transform:uppercase">
          Seconds until dispatch
        </p>
      </div>

      <!-- Buttons -->
      <div style="padding:var(--space-md) var(--space-lg) 40px;display:flex;flex-direction:column;gap:var(--space-sm)">
        <button class="btn btn-error btn-full-pill" id="call-now-btn" style="height:56px">
          <span class="material-symbols-outlined" style="font-variation-settings:'FILL' 1">phone_in_talk</span>
          Call Emergency Now
        </button>
        <button class="btn btn-secondary btn-full-pill" id="cancel-sos-btn" style="height:56px">
          I am Okay (Cancel)
        </button>
      </div>
    </div>`;

  const numEl  = el.querySelector('#countdown-num');
  const ring   = el.querySelector('#progress-ring');
  const CIRCUM = 553; // 2 * π * 88

  function updateRing(seconds) {
    const pct    = seconds / 10;
    const offset = CIRCUM * (1 - pct);
    ring.style.strokeDashoffset = offset;
    numEl.textContent = seconds;
  }

  updateRing(10);

  _sosTimer = setInterval(() => {
    _sosSeconds--;
    updateRing(_sosSeconds);

    if (_sosSeconds <= 0) {
      clearInterval(_sosTimer);
      _sosTimer = null;
      if (_sosState === 'countdown') dispatchSos(el);
    }
  }, 1000);

  // Cancel
  el.querySelector('#cancel-sos-btn').addEventListener('click', () => {
    _sosState = 'cancelled';
    clearInterval(_sosTimer);
    _sosTimer = null;
    showToast('SOS cancelled. Stay safe!', 'success');
    Router.go(Store.get('previousScreen') || 'home');
  });

  // Call now
  el.querySelector('#call-now-btn').addEventListener('click', () => {
    clearInterval(_sosTimer);
    _sosTimer = null;
    dispatchSos(el);
  });

}, (el) => {
  if (_sosTimer) {
    clearInterval(_sosTimer);
    _sosTimer = null;
  }
});

async function dispatchSos(el) {
  _sosState = 'dispatching';

  // Get location
  let lat = 37.7749, lon = -122.4194, acc = 10;
  try {
    await new Promise((resolve) => {
      navigator.geolocation.getCurrentPosition(
        pos => { lat = pos.coords.latitude; lon = pos.coords.longitude; acc = pos.coords.accuracy; resolve(); },
        () => resolve(),
        { timeout: 5000 }
      );
    });
  } catch (_) {}

  try {
    const resp = await Api.triggerSos({
      latitude: lat, longitude: lon, accuracy: acc, trigger: 'crash_detection',
    });
    Store.set('sosEventId', resp.id);
    Store.set('sosState', 'active');
    showSosActive(el, resp);
  } catch (err) {
    showToast('SOS sent (offline mode — contacts will be notified when online)', 'error');
    Router.go('home');
  }
}

function showSosActive(el, event) {
  el.innerHTML = `
    <div style="
      min-height:100dvh;background:var(--error);
      display:flex;flex-direction:column;align-items:center;justify-content:center;
      padding:var(--space-lg);gap:var(--space-lg);text-align:center;
    ">
      <span class="material-symbols-outlined" style="
        font-size:72px;color:white;
        font-variation-settings:'FILL' 1
      ">emergency</span>

      <div style="color:white">
        <div class="text-title-1" style="color:white">SOS Active</div>
        <p class="text-body" style="color:rgba(255,255,255,0.85);margin-top:var(--space-sm)">
          Emergency services have been notified.
          ${event.contacts_notified} contact(s) alerted.
        </p>
      </div>

      <div style="
        background:rgba(255,255,255,0.15);
        border-radius:var(--radius-xl);
        padding:var(--space-md);width:100%;max-width:300px;
      ">
        <div class="text-footnote" style="color:rgba(255,255,255,0.75);margin-bottom:4px">Your location</div>
        <div class="text-callout" style="color:white;font-weight:600">
          ${event.latitude.toFixed(5)}, ${event.longitude.toFixed(5)}
        </div>
      </div>

      <button id="resolve-sos-btn" style="
        background:white;color:var(--error);border:none;border-radius:var(--radius-full);
        height:56px;width:100%;max-width:300px;
        font-family:inherit;font-size:var(--text-callout);font-weight:700;cursor:pointer;
        display:flex;align-items:center;justify-content:center;gap:8px;
      ">
        <span class="material-symbols-outlined" style="font-variation-settings:'FILL' 1">check_circle</span>
        I'm Safe — Resolve SOS
      </button>
    </div>`;

  el.querySelector('#resolve-sos-btn').addEventListener('click', async () => {
    try {
      await Api.resolveSos(event.id, 'false_alarm');
      Store.set('sosState', 'idle');
      Store.set('sosEventId', null);
      showToast('SOS resolved. Glad you\'re safe!', 'success');
      Router.go('home');
    } catch (err) {
      showToast('Failed to resolve SOS', 'error');
    }
  });
}
