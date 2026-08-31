/**
 * Reusable UI component factory functions.
 * Each returns an HTML string or DOM element.
 */

function buildTopBar({ title, centerTitle, showAvatar, showSettings, onBack, onSettings }) {
  const avatarHtml = showAvatar ? `
    <div class="bar-avatar" id="bar-avatar">
      <span class="material-symbols-outlined" style="font-size:18px">person</span>
    </div>` : '';

  const backHtml = onBack ? `
    <button class="bar-action" id="bar-back" aria-label="Back">
      <span class="material-symbols-outlined">chevron_left</span>
    </button>` : '';

  const settingsHtml = showSettings ? `
    <button class="bar-action" id="bar-settings" aria-label="Settings">
      <span class="material-symbols-outlined">settings</span>
    </button>` : `<div style="width:36px"></div>`;

  return `
    <header class="top-bar">
      ${onBack ? backHtml : avatarHtml}
      <h1 class="bar-title${centerTitle ? ' center' : ''}">${title}</h1>
      ${settingsHtml}
    </header>`;
}

function buildBottomNav(activeTab) {
  const tabs = [
    { id: 'home',         icon: 'home',         label: 'Home'    },
    { id: 'live-map',     icon: 'explore',      label: 'Map'     },
    { id: 'ride-history', icon: 'history',      label: 'History' },
    { id: 'groups',       icon: 'group',        label: 'Groups'  },
    { id: 'profile',      icon: 'person',       label: 'Profile' },
  ];

  const items = tabs.map(t => `
    <button class="nav-item${t.id === activeTab ? ' active' : ''}" data-nav="${t.id}" aria-label="${t.label}">
      <span class="material-symbols-outlined">${t.icon}</span>
      <span class="nav-label">${t.label}</span>
    </button>`).join('');

  return `<nav class="bottom-nav">${items}</nav>`;
}

function attachNavHandlers(container) {
  container.querySelectorAll('[data-nav]').forEach(btn => {
    btn.addEventListener('click', () => Router.go(btn.dataset.nav));
  });
}

function buildAvatarStack(names = [], max = 3) {
  const shown = names.slice(0, max);
  const colors = ['#0058bc','#4c4aca','#9e3d00','#1a7f37'];
  return `<div class="avatar-stack">
    ${shown.map((n, i) => `
      <div class="avatar" style="background:${colors[i % colors.length]}" title="${n}">
        ${n ? n[0].toUpperCase() : '?'}
      </div>`).join('')}
    ${names.length > max ? `<div class="avatar" style="background:var(--outline)">+${names.length - max}</div>` : ''}
  </div>`;
}

function showToast(message, type = 'info') {
  const existing = document.getElementById('mg-toast');
  if (existing) existing.remove();

  const bg = type === 'error' ? 'var(--error)' :
             type === 'success' ? '#1a7f37' : 'var(--inverse-surface)';

  const toast = document.createElement('div');
  toast.id = 'mg-toast';
  toast.style.cssText = `
    position:fixed; bottom:calc(var(--bottom-nav-h) + 16px);
    left:50%; transform:translateX(-50%);
    background:${bg}; color:white;
    padding:10px 20px; border-radius:var(--radius-full);
    font-size:var(--text-subhead); font-weight:500;
    z-index:9999; white-space:nowrap;
    animation: toastIn 0.25s ease;
    box-shadow: 0 4px 16px rgba(0,0,0,0.2);
  `;
  toast.textContent = message;

  const style = document.createElement('style');
  style.textContent = `@keyframes toastIn{from{opacity:0;transform:translateX(-50%) translateY(12px)}to{opacity:1;transform:translateX(-50%) translateY(0)}}`;
  document.head.appendChild(style);
  document.body.appendChild(toast);
  setTimeout(() => toast.remove(), 3000);
}

function formatDistance(miles) {
  return miles >= 100 ? `${Math.round(miles)} mi` : `${miles.toFixed(1)} mi`;
}

function formatDuration(seconds) {
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  return h > 0 ? `${h}h ${m}m` : `${m} min`;
}

function formatDate(iso) {
  const d = new Date(iso);
  return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
}

function formatTime(iso) {
  const d = new Date(iso);
  return d.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' });
}
