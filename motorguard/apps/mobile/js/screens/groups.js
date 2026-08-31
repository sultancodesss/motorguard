Router.register('groups', (el) => {
  el.innerHTML = `
    ${buildTopBar({ title: 'Groups', showAvatar: true, showSettings: true })}

    <div class="screen-body">
      <!-- Header -->
      <div style="padding:var(--space-lg) var(--space-md) var(--space-sm);display:flex;align-items:flex-end;justify-content:space-between">
        <div>
          <h1 class="text-large-title">Active Rides</h1>
          <p class="text-body text-on-surface-var" style="margin-top:4px">Group riding sessions near you</p>
        </div>
        <button class="btn btn-primary btn-pill" id="create-group-btn">
          <span class="material-symbols-outlined" style="font-size:16px">add</span>
          Create
        </button>
      </div>

      <!-- Active group cards -->
      <div id="active-groups" style="padding:0 var(--space-md)">
        <!-- Populated by JS -->
      </div>

      <!-- My Groups -->
      <div class="section-header" style="margin-top:var(--space-md)">
        <span class="text-title-3">My Groups</span>
        <span class="text-callout text-primary" style="cursor:pointer">See All</span>
      </div>

      <div id="my-groups" style="padding:0 var(--space-md)">
        <!-- Populated by JS -->
      </div>

      <!-- Browse / Explore -->
      <div style="padding:var(--space-md)">
        <div style="
          border:1.5px dashed var(--outline-variant);
          border-radius:var(--radius-xl);
          padding:var(--space-lg);
          display:flex;flex-direction:column;align-items:center;gap:var(--space-sm);
          cursor:pointer;
          text-align:center;
        " id="explore-card">
          <span class="material-symbols-outlined" style="font-size:36px;color:var(--primary)">explore</span>
          <div class="text-headline">Find more groups</div>
          <p class="text-subhead text-on-surface-var">Discover riding communities near you</p>
        </div>
      </div>
    </div>

    <!-- FAB: create group -->
    <button class="fab" id="fab-group" style="bottom:calc(var(--bottom-nav-h) + 16px)">
      <span class="material-symbols-outlined">group_add</span>
    </button>

    ${buildBottomNav('groups')}

    <!-- Create Group modal sheet -->
    <div class="sheet-overlay" id="create-group-sheet">
      <div class="sheet">
        <div class="sheet-handle"></div>
        <h2 class="text-title-2" style="margin-bottom:var(--space-md)">Create Group</h2>
        <div style="display:flex;flex-direction:column;gap:var(--space-sm)">
          <input type="text" id="group-name-input" class="input-field" placeholder="Group name" maxlength="80" />
          <input type="text" id="group-desc-input" class="input-field" placeholder="Description (optional)" maxlength="300" />
          <button class="btn btn-primary btn-full-pill" id="create-group-submit">Create Group</button>
          <button class="btn btn-secondary btn-full-pill" id="create-group-cancel">Cancel</button>
        </div>
      </div>
    </div>`;

  attachNavHandlers(el);

  // Load groups
  Api.listGroups().then(data => {
    const groups = data?.groups || [];
    renderActiveGroups(el, groups.filter(g => g.is_member));
    renderMyGroups(el, groups.filter(g => g.is_member));
  }).catch(() => {
    renderActiveGroups(el, []);
    renderMyGroups(el, []);
  });

  // Create group sheet
  const sheet = el.querySelector('#create-group-sheet');

  function openSheet() { sheet.classList.add('open'); }
  function closeSheet() { sheet.classList.remove('open'); }

  el.querySelector('#create-group-btn').addEventListener('click', openSheet);
  el.querySelector('#fab-group').addEventListener('click', openSheet);
  el.querySelector('#create-group-cancel').addEventListener('click', closeSheet);
  sheet.addEventListener('click', e => { if (e.target === sheet) closeSheet(); });

  el.querySelector('#create-group-submit').addEventListener('click', async () => {
    const name = el.querySelector('#group-name-input').value.trim();
    const desc = el.querySelector('#group-desc-input').value.trim();
    if (!name) { showToast('Group name is required', 'error'); return; }

    try {
      await Api.createGroup({ name, description: desc || null });
      closeSheet();
      showToast('Group created!', 'success');
      // Reload
      const data = await Api.listGroups();
      renderMyGroups(el, data?.groups || []);
    } catch (err) {
      showToast(err.message || 'Failed to create group', 'error');
    }
  });

  el.querySelector('#explore-card').addEventListener('click', () => {
    showToast('Group discovery coming soon', 'info');
  });
});

function renderActiveGroups(el, groups) {
  const container = el.querySelector('#active-groups');
  if (!groups.length) {
    container.innerHTML = `
      <div class="card" style="margin-bottom:var(--space-sm)">
        <div style="
          height:160px;background:linear-gradient(135deg,#e8f0fe,#c5d8f8);
          display:flex;align-items:center;justify-content:center;
        ">
          <span class="material-symbols-outlined" style="font-size:40px;color:var(--primary);opacity:0.4">two_wheeler</span>
        </div>
        <div style="padding:var(--space-md)">
          <div class="badge badge-scheduled" style="margin-bottom:var(--space-sm)">Scheduled</div>
          <div class="text-headline">PCH Sunset Run</div>
          <div style="display:flex;align-items:center;gap:6px;margin-top:4px">
            ${buildAvatarStack(['A', 'B', 'C'])}
            <span class="text-footnote text-on-surface-var">0 riders active</span>
          </div>
        </div>
      </div>`;
    return;
  }

  container.innerHTML = groups.slice(0, 3).map(g => `
    <div class="card" style="margin-bottom:var(--space-sm);cursor:pointer">
      <div style="height:140px;background:linear-gradient(135deg,#e8f0fe,#c5d8f8);position:relative;display:flex;align-items:center;justify-content:center">
        <span class="material-symbols-outlined" style="font-size:40px;color:var(--primary);opacity:0.4">route</span>
        <div style="position:absolute;top:var(--space-sm);left:var(--space-sm)">
          <span class="badge badge-live">Live</span>
        </div>
        <div style="position:absolute;bottom:var(--space-sm);right:var(--space-sm)">
          ${buildAvatarStack([g.name[0], 'B', 'C'])}
        </div>
      </div>
      <div style="padding:var(--space-md);display:flex;align-items:center;justify-content:space-between">
        <div>
          <div class="text-headline">${g.name}</div>
          <div style="display:flex;align-items:center;gap:4px;margin-top:2px">
            <span class="material-symbols-outlined" style="font-size:14px;color:var(--on-surface-variant)">group</span>
            <span class="text-footnote text-on-surface-var">${g.member_count} members</span>
          </div>
        </div>
        <span class="material-symbols-outlined text-on-surface-var">chevron_right</span>
      </div>
    </div>`).join('');
}

function renderMyGroups(el, groups) {
  const container = el.querySelector('#my-groups');
  if (!groups.length) {
    container.innerHTML = `
      <div class="inset-list">
        <div class="inset-list-item" style="justify-content:center;color:var(--on-surface-variant)">
          <span class="text-subhead">No groups yet. Create or join one!</span>
        </div>
      </div>`;
    return;
  }

  const colors = [
    { bg: 'rgba(76,74,202,0.1)', fg: 'var(--secondary)' },
    { bg: 'rgba(158,61,0,0.1)', fg: 'var(--tertiary)' },
    { bg: 'rgba(0,88,188,0.1)', fg: 'var(--primary)' },
  ];

  container.innerHTML = `
    <div class="inset-list">
      ${groups.map((g, i) => {
    const c = colors[i % colors.length];
    return `
        <div class="inset-list-item">
          <div class="list-icon" style="background:${c.bg}">
            <span class="material-symbols-outlined" style="color:${c.fg};font-variation-settings:'FILL' 1">sports_motorsports</span>
          </div>
          <div class="list-item-text">
            <div class="item-title">${g.name}</div>
            <div class="item-sub">${g.member_count} members</div>
          </div>
          <div class="list-chevron">
            <span class="material-symbols-outlined">chevron_right</span>
          </div>
        </div>`;
  }).join('')}
    </div>`;
}
