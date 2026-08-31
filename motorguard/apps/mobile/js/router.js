/**
 * Simple screen router — shows one screen at a time.
 * Screens register themselves via Router.register(id, mountFn, unmountFn).
 */
const Router = (() => {
  const screens   = {};
  let   current   = null;

  function register(id, mount, unmount) {
    screens[id] = { mount, unmount: unmount || (() => {}) };
  }

  function go(id, params = {}) {
    const app = document.getElementById('app');

    // Unmount previous
    if (current && screens[current]) {
      screens[current].unmount();
      const el = document.getElementById(`screen-${current}`);
      if (el) el.classList.add('hidden');
    }

    Store.set('previousScreen', current);
    Store.set('currentScreen', id);
    current = id;

    // Mount new
    if (!document.getElementById(`screen-${id}`)) {
      const el = document.createElement('div');
      el.id = `screen-${id}`;
      el.className = 'screen';
      app.appendChild(el);
    }

    const el = document.getElementById(`screen-${id}`);
    el.classList.remove('hidden');

    if (screens[id]) screens[id].mount(el, params);
  }

  function back() {
    const prev = Store.get('previousScreen');
    if (prev) go(prev);
  }

  return { register, go, back };
})();
