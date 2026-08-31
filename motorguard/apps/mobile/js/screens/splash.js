Router.register('splash', (el) => {
  el.innerHTML = `
    <div style="
      display:flex;flex-direction:column;align-items:center;justify-content:center;
      height:100dvh;gap:var(--space-lg);background:var(--primary);
    ">
      <div style="
        width:88px;height:88px;border-radius:22px;
        background:rgba(255,255,255,0.18);
        display:flex;align-items:center;justify-content:center;
        animation:splashBounce 1.6s ease-in-out infinite;
      ">
        <span class="material-symbols-outlined" style="font-size:48px;color:white;font-variation-settings:'FILL' 1">
          shield_with_heart
        </span>
      </div>
      <div style="text-align:center">
        <div style="font-size:32px;font-weight:800;color:white;letter-spacing:-0.8px">MotorGuard</div>
        <div style="font-size:15px;color:rgba(255,255,255,0.72);margin-top:6px;letter-spacing:0.3px">
          Ride Safe. Ride Together.
        </div>
      </div>
      <div style="display:flex;gap:7px;margin-top:8px">
        <div style="width:8px;height:8px;border-radius:50%;background:white;opacity:0.9;animation:dot 1.2s 0s ease-in-out infinite"></div>
        <div style="width:8px;height:8px;border-radius:50%;background:white;opacity:0.6;animation:dot 1.2s 0.2s ease-in-out infinite"></div>
        <div style="width:8px;height:8px;border-radius:50%;background:white;opacity:0.3;animation:dot 1.2s 0.4s ease-in-out infinite"></div>
      </div>
    </div>
    <style>
      @keyframes splashBounce{0%,100%{transform:scale(1) rotate(-2deg)}50%{transform:scale(1.07) rotate(2deg)}}
      @keyframes dot{0%,100%{opacity:0.3;transform:scale(0.8)}50%{opacity:1;transform:scale(1.15)}}
    </style>`;

  setTimeout(() => {
    const token = Store.get('accessToken');
    if (token && token !== 'null') {
      // Token exists — go straight home with cached/mock user
      const cached = Store.get('user');
      if (cached) {
        Router.go('home');
      } else {
        Api.getMe()
          .then(user => { Store.set('user', user); Router.go('home'); })
          .catch(() => Router.go('login'));
      }
    } else {
      Router.go('login');
    }
  }, 1800);
});
