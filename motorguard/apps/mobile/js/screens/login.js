Router.register('login', (el) => {
  el.innerHTML = `
    <div class="screen-body no-nav" style="padding:0">
      <!-- Status bar spacer -->
      <div style="height:44px"></div>

      <div style="padding:var(--space-lg) var(--space-md)">
        <!-- Logo -->
        <div style="display:flex;flex-direction:column;align-items:center;gap:var(--space-md);margin-bottom:var(--space-xl)">
          <div style="
            width:64px;height:64px;border-radius:16px;
            background:var(--primary);
            display:flex;align-items:center;justify-content:center;
            box-shadow:0 4px 20px rgba(0,88,188,0.35)
          ">
            <span class="material-symbols-outlined" style="font-size:36px;color:white;font-variation-settings:'FILL' 1">
              shield_with_heart
            </span>
          </div>
          <div style="text-align:center">
            <h1 class="text-large-title">Welcome to MotorGuard</h1>
            <p class="text-body text-on-surface-var" style="margin-top:7px; margin-left: 17px; max-width:320px">
              Enter your phone number to start your safe ride
            </p>
          </div>
        </div>

<div id="phone-input-wrap" style="
  display:flex;align-items:center;
  background:var(--surface-container-low);
  border:1.5px solid var(--outline-variant);
  border-radius:var(--radius-md);
  height:56px;overflow:hidden;
  transition:border-color 0.15s;
">
  <!-- Country code dropdown -->
  <select id="country-code" style="
    border:none;
    background:transparent;
    padding:0 var(--space-md);
    height:100%;
    font-weight:600;
    font-size:var(--text-body);
    flex-shrink:0;
  ">
    <option value="+1">🇺🇸 +1</option>
    <option value="+91">🇮🇳 +91</option>
    <option value="+44">🇬🇧 +44</option>
    <option value="+61">🇦🇺 +61</option>
    <option value="+81">🇯🇵 +81</option>
    <!-- बाकी 200+ countries JSON से auto-fill कर सकते हो -->
  </select>

  <!-- Phone number input -->
  <input
    type="tel"
    id="phone-input"
    placeholder="(555) 000-0000"
    maxlength="14"
    autocomplete="tel"
    style="
      flex:1;
      border:none;
      background:transparent;
      padding:0 var(--space-md);
      font-family:inherit;
      font-size:var(--text-body);
      color:var(--on-surface);
      outline:none;
      height:100%;
    "
  />
</div>


        <!-- Terms -->
        <p class="text-footnote text-on-surface-var" style="margin-top:var(--space-sm);text-align:center">
          By continuing you agree to our
          <a href="#" style="color:var(--primary);text-decoration:none">Terms of Service</a>
          and
          <a href="#" style="color:var(--primary);text-decoration:none">Privacy Policy</a>
        </p>

        <!-- Send Code button -->
        <button id="btn-send" class="btn btn-primary btn-full-pill" style="margin-top:var(--space-lg)" disabled>
          <span id="btn-send-content">Send Code</span>
        </button>

        <!-- Ghost button -->
        <button class="btn btn-ghost btn-full-pill" style="margin-top:var(--space-sm)">
          Sign in with Email
        </button>

        <!-- Quick demo shortcut -->
        <div style="text-align:center;margin-top:var(--space-sm)">
          <button id="btn-demo" style="
            background:none;border:none;cursor:pointer;
            font-family:inherit;font-size:var(--text-footnote);
            color:var(--on-surface-variant);text-decoration:underline;
          ">Skip — enter demo mode</button>
        </div>

        <!-- Divider -->
        <div style="display:flex;align-items:center;gap:var(--space-md);margin:var(--space-md) 0">
          <div style="flex:1;height:1px;background:var(--outline-variant)"></div>
          <span class="text-footnote text-on-surface-var">or sign in with</span>
          <div style="flex:1;height:1px;background:var(--outline-variant)"></div>
        </div>

        <!-- OAuth buttons -->
        <div style="display:grid;grid-template-columns:1fr 1fr;gap:var(--space-sm)">
          <button class="btn btn-ghost" style="height:48px;border-radius:var(--radius-md);gap:8px;font-size:var(--text-subhead)">
            <svg width="18" height="18" viewBox="0 0 24 24">
              <path d="M17.05 20.28c-.98.95-2.05.8-3.08.35-1.09-.46-2.09-.48-3.24 0-1.44.62-2.2.44-3.06-.35C2.79 15.25 3.51 7.7 9.05 7.41c1.42.07 2.38.74 3.2.8 1.21-.26 2.38-.96 3.7-.84 1.58.14 2.77.74 3.53 1.9-3.29 1.97-2.52 6.25.87 7.47-.62 1.63-1.43 3.22-3.3 4.54zM12.03 7.25c-.15-2.23 1.66-4.07 3.74-4.25.29 2.58-2.34 4.5-3.74 4.25z" fill="#000"/>
            </svg>
            Apple
          </button>
          <button class="btn btn-ghost" style="height:48px;border-radius:var(--radius-md);gap:8px;font-size:var(--text-subhead)">
            <svg width="18" height="18" viewBox="0 0 24 24">
              <path d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z" fill="#4285F4"/>
              <path d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z" fill="#34A853"/>
              <path d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z" fill="#FBBC05"/>
              <path d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z" fill="#EA4335"/>
            </svg>
            Google
          </button>
        </div>
      </div>
    </div>`;

  const phoneInput = el.querySelector('#phone-input');
  const sendBtn = el.querySelector('#btn-send');
  const wrap = el.querySelector('#phone-input-wrap');

  phoneInput.addEventListener('input', () => {
    const v = phoneInput.value.replace(/\D/g, '');
    sendBtn.disabled = v.length < 7;
    wrap.style.borderColor = v.length > 0 ? 'var(--primary)' : 'var(--outline-variant)';
  });

  phoneInput.addEventListener('focus', () => {
    wrap.style.borderColor = 'var(--primary)';
  });
  phoneInput.addEventListener('blur', () => {
    if (!phoneInput.value) wrap.style.borderColor = 'var(--outline-variant)';
  });

  sendBtn.addEventListener('click', async () => {
    const digits = phoneInput.value.replace(/\D/g, '');
    const phone = `+1${digits}`;
    const content = el.querySelector('#btn-send-content');

    sendBtn.disabled = true;
    content.innerHTML = `<div class="spinner"></div>`;

    try {
      await Api.requestOtp(phone);
      Store.set('pendingPhone', phone);
      showToast('Code sent!', 'success');
      Router.go('otp', { phone });
    } catch (err) {
      showToast(err.message || 'Failed to send code', 'error');
    } finally {
      content.textContent = 'Send Code';
      sendBtn.disabled = false;
    }
  });

  // Auto-focus
  setTimeout(() => phoneInput.focus(), 100);

  // Demo shortcut — skip login entirely
  el.querySelector('#btn-demo').addEventListener('click', async () => {
    const resp = await Api.verifyOtp('+15550000000', '123456');
    Store.persist('accessToken', resp.access_token);
    Store.persist('refreshToken', resp.refresh_token);
    Store.set('user', resp.user);
    Router.go('home');
  });
});
