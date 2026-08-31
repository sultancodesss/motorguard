Router.register('profile', (el) => {
  const user = Store.get('user') || { name: 'Rider', phone: '', stats: {} };
  const stats = user.stats || { total_rides: 0, total_miles: 0, safety_score: 0 };
  const initials = (user.name || 'R').split(' ').map(w => w[0]).join('').toUpperCase().slice(0, 2);

  el.innerHTML = `
    ${buildTopBar({ title: 'Profile', centerTitle: true, showSettings: true })}

    <div class="screen-body">
      <!-- Profile header -->
      <div style="
        display:flex;flex-direction:column;align-items:center;
        padding:var(--space-lg) var(--space-md) var(--space-md);gap:var(--space-sm)
      ">
        <div class="profile-avatar">${initials}</div>
        <div style="text-align:center">
          <div class="text-title-1">${user.name || 'Rider'}</div>
          <div style="
            display:inline-flex;align-items:center;gap:4px;
            background:rgba(0,88,188,0.1);color:var(--primary);
            border-radius:var(--radius-full);padding:4px 12px;margin-top:6px;
            font-size:var(--text-footnote);font-weight:600
          ">
            <span class="material-symbols-outlined" style="font-size:14px;font-variation-settings:'FILL' 1">verified</span>
            Pro Rider
          </div>
        </div>
      </div>

      <!-- Stats bento -->
      <div style="padding:0 var(--space-md) var(--space-md);display:grid;grid-template-columns:1fr 1fr 1fr;gap:var(--space-sm)">
        <div class="stat-card">
          <div class="stat-value">${stats.total_rides}</div>
          <div class="stat-label">Rides</div>
        </div>
        <div class="stat-card">
          <div class="stat-value">${formatDistance(stats.total_miles || 0)}</div>
          <div class="stat-label">Miles</div>
        </div>
        <div class="stat-card">
          <div class="stat-value tertiary">${Math.round(stats.safety_score || 0)}%</div>
          <div class="stat-label">Safety</div>
        </div>
      </div>

      <!-- Safety & Security -->
      <div style="padding:0 var(--space-md) var(--space-sm)">
        <h2 class="text-title-3" style="margin-bottom:var(--space-sm)">Safety &amp; Security</h2>
        <div class="inset-list" style="margin:0">
          <div class="inset-list-item" id="nav-emergency-contacts">
            <div class="list-icon" style="background:rgba(186,26,26,0.1)">
              <span class="material-symbols-outlined" style="color:var(--error);font-variation-settings:'FILL' 1">contact_emergency</span>
            </div>
            <div class="list-item-text">
              <div class="item-title">Emergency Contacts</div>
              <div class="item-sub" id="contacts-count">Loading…</div>
            </div>
            <div class="list-chevron"><span class="material-symbols-outlined">chevron_right</span></div>
          </div>
          <div class="inset-list-item">
            <div class="list-icon" style="background:rgba(0,88,188,0.1)">
              <span class="material-symbols-outlined" style="color:var(--primary);font-variation-settings:'FILL' 1">two_wheeler</span>
            </div>
            <div class="list-item-text">
              <div class="item-title">Ride Settings</div>
              <div class="item-sub">Safety score thresholds</div>
            </div>
            <div class="list-chevron"><span class="material-symbols-outlined">chevron_right</span></div>
          </div>
        </div>
      </div>

      <!-- Hardware -->
      <div style="padding:0 var(--space-md) var(--space-sm)">
        <h2 class="text-title-3" style="margin-bottom:var(--space-sm)">Hardware</h2>
        <div class="inset-list" style="margin:0">
          <div class="inset-list-item">
            <div class="list-icon" style="background:rgba(76,74,202,0.1)">
              <span class="material-symbols-outlined" style="color:var(--secondary);font-variation-settings:'FILL' 1">bluetooth</span>
            </div>
            <div class="list-item-text">
              <div class="item-title">Device Connectivity</div>
              <div class="item-sub" style="color:#1a7f37">● Connected</div>
            </div>
            <div class="list-chevron"><span class="material-symbols-outlined">chevron_right</span></div>
          </div>
        </div>
      </div>

      <!-- General -->
      <div style="padding:0 var(--space-md) var(--space-md)">
        <h2 class="text-title-3" style="margin-bottom:var(--space-sm)">General</h2>
        <div class="inset-list" style="margin:0">
          <div class="inset-list-item">
            <div class="list-icon" style="background:var(--surface-container)">
              <span class="material-symbols-outlined" style="color:var(--on-surface-variant);font-variation-settings:'FILL' 1">manage_accounts</span>
            </div>
            <div class="list-item-text">
              <div class="item-title">Account</div>
              <div class="item-sub">${user.phone || ''}</div>
            </div>
            <div class="list-chevron"><span class="material-symbols-outlined">chevron_right</span></div>
          </div>
          <div class="inset-list-item">
            <div class="list-icon" style="background:var(--surface-container)">
              <span class="material-symbols-outlined" style="color:var(--on-surface-variant);font-variation-settings:'FILL' 1">help</span>
            </div>
            <div class="list-item-text">
              <div class="item-title">Help &amp; Support</div>
            </div>
            <div class="list-chevron"><span class="material-symbols-outlined">chevron_right</span></div>
          </div>
        </div>
      </div>

      <!-- Sign out -->
      <div style="padding:0 var(--space-md) var(--space-xl)">
        <button id="sign-out-btn" class="btn" style="
          background:transparent;color:var(--error);
          font-size:var(--text-callout);font-weight:500;
          border:1.5px solid rgba(186,26,26,0.2);
          border-radius:var(--radius-xl);
        ">Sign Out</button>
      </div>
    </div>

    ${buildBottomNav('profile')}

    <!-- Emergency contacts sheet -->
    <div class="sheet-overlay" id="contacts-sheet">
      <div class="sheet">
        <div class="sheet-handle"></div>
        <div style="display:flex;align-items:center;justify-content:space-between;margin-bottom:var(--space-md)">
          <h2 class="text-title-2">Emergency Contacts</h2>
          <button id="close-contacts-sheet" style="background:none;border:none;cursor:pointer;color:var(--primary);font-family:inherit">Done</button>
        </div>
        <div id="contacts-list">Loading…</div>
        <button class="btn btn-primary btn-full-pill" style="margin-top:var(--space-md)" id="add-contact-btn">
          <span class="material-symbols-outlined">add</span> Add Contact
        </button>
      </div>
    </div>`;

  attachNavHandlers(el);

  // Load contact count
  Api.listContacts().then(data => {
    const contacts = data?.contacts || [];
    const countEl = el.querySelector('#contacts-count');
    if (countEl) countEl.textContent = `${contacts.length} contact${contacts.length !== 1 ? 's' : ''} added`;
    Store.set('contacts', contacts);
    renderContactsList(el, contacts);
  }).catch(() => { });

  // Contacts sheet
  const sheet = el.querySelector('#contacts-sheet');
  el.querySelector('#nav-emergency-contacts').addEventListener('click', () => sheet.classList.add('open'));
  el.querySelector('#close-contacts-sheet').addEventListener('click', () => sheet.classList.remove('open'));

  // Add contact
  el.querySelector('#add-contact-btn').addEventListener('click', () => {
    showAddContactDialog(el, sheet);
  });

  // Sign out
  el.querySelector('#sign-out-btn').addEventListener('click', async () => {
    if (!confirm('Sign out of MotorGuard?')) return;
    try {
      await Api.logout();
    } catch (_) { }
    Store.persist('accessToken', null);
    Store.persist('refreshToken', null);
    Store.set('user', null);
    Router.go('login');
  });
});

function renderContactsList(el, contacts) {
  const list = el.querySelector('#contacts-list');
  if (!list) return;

  if (!contacts.length) {
    list.innerHTML = `
      <div class="empty-state" style="padding:var(--space-lg) 0">
        <span class="material-symbols-outlined">contact_emergency</span>
        <p class="text-subhead">No emergency contacts yet</p>
      </div>`;
    return;
  }

  list.innerHTML = `
    <div class="inset-list" style="margin:0">
      ${contacts.map(c => `
        <div class="inset-list-item">
          <div class="list-icon" style="background:rgba(186,26,26,0.1)">
            <span class="material-symbols-outlined" style="color:var(--error);font-variation-settings:'FILL' 1">person</span>
          </div>
          <div class="list-item-text">
            <div class="item-title">${c.name}</div>
            <div class="item-sub">${c.phone}${c.relationship ? ' · ' + c.relationship : ''}</div>
          </div>
          <button data-contact-id="${c.id}" class="delete-contact-btn"
            style="background:none;border:none;cursor:pointer;color:var(--error);padding:4px">
            <span class="material-symbols-outlined" style="font-size:20px">delete</span>
          </button>
        </div>`).join('')}
    </div>`;

  list.querySelectorAll('.delete-contact-btn').forEach(btn => {
    btn.addEventListener('click', async () => {
      if (!confirm('Remove this contact?')) return;
      try {
        await Api.deleteContact(btn.dataset.contactId);
        const contacts = Store.get('contacts').filter(c => c.id !== btn.dataset.contactId);
        Store.set('contacts', contacts);
        renderContactsList(el, contacts);
        showToast('Contact removed', 'success');
      } catch (err) {
        showToast('Failed to remove contact', 'error');
      }
    });
  });
}

function showAddContactDialog(el, sheet) {
  const overlay = document.createElement('div');
  overlay.style.cssText = `
    position:fixed;inset:0;background:rgba(0,0,0,0.5);z-index:9000;
    display:flex;align-items:flex-end;
  `;
  overlay.innerHTML = `
    <div style="
      background:var(--surface-container-lowest);
      border-radius:var(--radius-xl) var(--radius-xl) 0 0;
      width:100%;max-width:430px;margin:0 auto;
      padding:var(--space-sm) var(--space-md) 40px;
    ">
      <div style="width:36px;height:4px;border-radius:2px;background:var(--outline-variant);margin:0 auto var(--space-md)"></div>
      <h2 class="text-title-2" style="margin-bottom:var(--space-md)">Add Contact</h2>
      <div style="display:flex;flex-direction:column;gap:var(--space-sm)">
        <input type="text"  id="nc-name" class="input-field" placeholder="Full name" />
        <input type="tel"   id="nc-phone" class="input-field" placeholder="+1 555 000 0000" />
        <input type="text"  id="nc-rel"  class="input-field" placeholder="Relationship (e.g. spouse)" />
        <button id="nc-submit" class="btn btn-primary btn-full-pill">Add Contact</button>
        <button id="nc-cancel" class="btn btn-secondary btn-full-pill">Cancel</button>
      </div>
    </div>`;
  document.getElementById('app').appendChild(overlay);

  overlay.querySelector('#nc-cancel').addEventListener('click', () => overlay.remove());
  overlay.addEventListener('click', e => { if (e.target === overlay) overlay.remove(); });

  overlay.querySelector('#nc-submit').addEventListener('click', async () => {
    const name = overlay.querySelector('#nc-name').value.trim();
    const phone = overlay.querySelector('#nc-phone').value.trim();
    const rel = overlay.querySelector('#nc-rel').value.trim();
    if (!name || !phone) { showToast('Name and phone are required', 'error'); return; }
    try {
      const contact = await Api.createContact({ name, phone, relationship: rel || null });
      const contacts = [...(Store.get('contacts') || []), contact];
      Store.set('contacts', contacts);
      renderContactsList(el, contacts);
      const countEl = el.querySelector('#contacts-count');
      if (countEl) countEl.textContent = `${contacts.length} contact${contacts.length !== 1 ? 's' : ''} added`;
      overlay.remove();
      showToast('Contact added!', 'success');
    } catch (err) {
      showToast(err.message || 'Failed to add contact', 'error');
    }
  });
}
