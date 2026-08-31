/**
 * MotorGuard — app entry point.
 * Starts the router on the splash screen.
 */
document.addEventListener('DOMContentLoaded', () => {
  // Restore tokens from localStorage (already done by store.js at parse time)
  // Kick off the app
  Router.go('splash');
});
