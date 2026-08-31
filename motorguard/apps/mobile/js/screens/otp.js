Router.register('otp', (el, params = {}) => {
  const phone = params.phone || Store.get('pendingPhone') || '+1•••••••••••';
  const masked = phone.replace(/(\+\d{1,2})(\d{3})(\d{3})(\d{4})/, '$1 ($2) $3-••••');
  const digits = ['', '', '', '', '', ''];
  let resendSeconds = 55;
  let resendTimer = null;

  el.innerHTML = `
    <div class="screen-body no-nav">
      <!-- Back -->
      <div style="height:44px"></div>
      <div style="padding:var(--space-md)">
        <button id="otp-back" style="display:flex;align-items:center;gap:4px;background:none;border:none;cursor:pointer;color:var(--primary);font-family:inherit;font-size:var(--text-callout)">
          <span class="material-symbols-outlined" style="font-size:20px">chevron_left</span>
          Back
        </button>
      </div>

      <div style="padding:0 var(--space-md) var(--space-lg)">
        <h1 class="text-large-title">Verify Phone</h1>
        <p class="text-body" style="margin-top:var(--space-sm)">
          Enter the 6-digit code sent to
          <strong>${masked}</strong>
        </p>

        <!-- Demo mode hint -->
        <div style="
          display:flex;align-items:center;gap:8px;
          background:rgba(0,88,188,0.08);
          border:1.5px solid rgba(0,88,188,0.18);
          border-radius:var(--radius-md);
          padding:10px var(--space-md);
          margin-top:var(--space-md);
        ">
          <span class="material-symbols-outlined" style="font-size:18px;color:var(--primary);flex-shrink:0">info</span>
          <span style="font-size:var(--text-footnote);color:var(--primary);font-weight:500">
            Demo mode — enter any 6 digits (e.g. <strong>123456</strong>)
          </span>
        </div>

        <!-- OTP boxes -->
        <div class="otp-boxes" style="margin-top:var(--space-xl)" id="otp-boxes">
          ${[0, 1, 2, 3, 4, 5].map(i => `
            <div class="otp-box" data-idx="${i}" id="otp-box-${i}">
              <span id="otp-digit-${i}"></span>
            </div>`).join('')}
        </div>

        <!-- Resend -->
        <div style="text-align:center;margin-top:var(--space-lg)">
          <span class="text-subhead text-on-surface-var" id="resend-label">Resend Code in 0:<span id="resend-secs">55</span></span>
          <button id="btn-resend" class="text-subhead text-primary" style="display:none;background:none;border:none;cursor:pointer;font-family:inherit">
            Resend Code
          </button>
        </div>

        <!-- Verify button -->
        <button id="btn-verify" class="btn btn-primary btn-full-pill" style="margin-top:var(--space-xl)" disabled>
          Verify &amp; Continue
        </button>
      </div>
    </div>

    <!-- Keypad -->
    <div class="keypad" id="otp-keypad">
      ${[
      ['1', ''], ['2', 'ABC'], ['3', 'DEF'],
      ['4', 'GHI'], ['5', 'JKL'], ['6', 'MNO'],
      ['7', 'PQRS'], ['8', 'TUV'], ['9', 'WXYZ'],
      ['', ''], ['0', '+'], ['⌫', '']
    ].map(([num, alpha]) => `
        <button class="keypad-key${num === '⌫' || num === '' ? ' special' : ''}" data-key="${num}">
          ${num === '⌫'
        ? `<span class="material-symbols-outlined" style="font-size:20px">backspace</span>`
        : num === ''
          ? ``
          : `<span class="key-num">${num}</span>${alpha ? `<span class="key-alpha">${alpha}</span>` : ''}`
      }
        </button>`).join('')}
    </div>`;

  let activeIdx = 0;

  function highlightBox(idx) {
    document.querySelectorAll('.otp-box').forEach((b, i) => {
      b.classList.toggle('active', i === idx);
      b.classList.toggle('filled', digits[i] !== '');
    });
    activeIdx = idx;
  }

  function checkComplete() {
    const code = digits.join('');
    const btn = el.querySelector('#btn-verify');
    btn.disabled = code.length < 6;
  }

  function setError() {
    document.querySelectorAll('.otp-box').forEach(b => b.classList.add('error'));
    setTimeout(() => document.querySelectorAll('.otp-box').forEach(b => b.classList.remove('error')), 1000);
  }

  // Keypad handler
  el.querySelector('#otp-keypad').addEventListener('click', (e) => {
    const key = e.target.closest('[data-key]')?.dataset.key;
    if (!key) return;

    if (key === '⌫') {
      if (digits[activeIdx] !== '') {
        digits[activeIdx] = '';
        el.querySelector(`#otp-digit-${activeIdx}`).textContent = '';
        highlightBox(activeIdx);
      } else if (activeIdx > 0) {
        activeIdx--;
        digits[activeIdx] = '';
        el.querySelector(`#otp-digit-${activeIdx}`).textContent = '';
        highlightBox(activeIdx);
      }
    } else if (key === '*') {
      // star key — no action
    } else if (/^\d$/.test(key) && activeIdx < 6) {
      digits[activeIdx] = key;
      el.querySelector(`#otp-digit-${activeIdx}`).textContent = key;
     if(activeIdx <5){
      activeIdx++;
     }
     highlightBox(activeIdx)
    }
    checkComplete();
  });

  // Verify handler
  el.querySelector('#btn-verify').addEventListener('click', async () => {
    const code = digits.join('');
    const btn = el.querySelector('#btn-verify');
    btn.disabled = true;
    btn.innerHTML = '<div class="spinner"></div>';

    try {
      const resp = await Api.verifyOtp(phone, code);
      Store.persist('accessToken', resp.access_token);
      Store.persist('refreshToken', resp.refresh_token);
      Store.set('user', resp.user);
      Router.go('home');
    } catch (err) {
      setError();
      showToast(err.message || 'Invalid code', 'error');
      btn.disabled = false;
      btn.textContent = 'Verify & Continue';
    }
  });

  // Back
  el.querySelector('#otp-back').addEventListener('click', () => Router.go('login'));

  // Resend countdown
  function tickResend() {
    resendSeconds--;
    const secEl = el.querySelector('#resend-secs');
    if (secEl) secEl.textContent = String(resendSeconds).padStart(2, '0');
    if (resendSeconds <= 0) {
      clearInterval(resendTimer);
      const label = el.querySelector('#resend-label');
      const btn = el.querySelector('#btn-resend');
      if (label) label.style.display = 'none';
      if (btn) btn.style.display = 'inline';
    }
  }
  resendTimer = setInterval(tickResend, 1000);

  el.querySelector('#btn-resend').addEventListener('click', async () => {
    try {
      await Api.requestOtp(phone);
      showToast('New code sent!', 'success');
      resendSeconds = 55;
      el.querySelector('#resend-label').style.display = '';
      el.querySelector('#btn-resend').style.display = 'none';
      resendTimer = setInterval(tickResend, 1000);
    } catch (err) {
      showToast('Failed to resend', 'error');
    }
  });

  highlightBox(0);

}, (el) => {
  // Cleanup resend timer — handled by closure but safe to leave
});
