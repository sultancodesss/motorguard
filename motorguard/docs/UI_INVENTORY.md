# MotorGuard — UI Inventory

Extracted from `stitch_pixel_perfect_ui_clone/` design files.

---

## Design System

### Color Palette (Material Design 3)

| Token | Value | Usage |
|-------|-------|-------|
| `primary` | `#0058bc` | Buttons, active nav, links, brand |
| `primary-container` | `#0070eb` | Start Ride card background |
| `on-primary` | `#ffffff` | Text/icons on primary |
| `on-primary-container` | `#fefcff` | Text on primary container |
| `secondary` | `#4c4aca` | Secondary actions, group icons |
| `secondary-container` | `#6664e4` | Secondary card backgrounds |
| `tertiary` | `#9e3d00` | Warning states, scores |
| `tertiary-container` | `#c64f00` | Tertiary backgrounds |
| `error` | `#ba1a1a` | SOS, error states |
| `error-container` | `#ffdad6` | SOS background tint |
| `on-error` | `#ffffff` | Text on error |
| `on-error-container` | `#93000a` | Text on error container |
| `surface` | `#faf9fe` | Page background |
| `surface-container` | `#eeedf3` | Card backgrounds |
| `surface-container-low` | `#f4f3f8` | Input backgrounds |
| `surface-container-high` | `#e9e7ed` | Elevated cards |
| `surface-container-highest` | `#e3e2e7` | Dividers, highest elevation |
| `surface-container-lowest` | `#ffffff` | Pure white cards |
| `on-surface` | `#1a1b1f` | Primary text |
| `on-surface-variant` | `#414755` | Secondary text, captions |
| `outline` | `#717786` | Borders, inactive icons |
| `outline-variant` | `#c1c6d7` | Subtle borders, dividers |
| `background` | `#faf9fe` | App background |
| `inverse-surface` | `#2f3034` | Dark surface |
| `inverse-on-surface` | `#f1f0f5` | Text on dark surface |

### Typography (Inter font)

| Token | Size | Line Height | Weight | Letter Spacing |
|-------|------|-------------|--------|----------------|
| `large-title` | 34px | 41px | 700 | +0.37px |
| `large-title-mobile` | 31px | 38px | 700 | — |
| `title-1` | 28px | 34px | 700 | +0.36px |
| `title-2` | 22px | 28px | 700 | +0.35px |
| `title-3` | 20px | 25px | 600 | +0.38px |
| `headline` | 17px | 22px | 600 | -0.41px |
| `body` | 17px | 22px | 400 | -0.41px |
| `callout` | 16px | 21px | 400 | -0.32px |
| `subheadline` | 15px | 20px | 400 | -0.24px |
| `footnote` | 13px | 18px | 400 | -0.08px |
| `caption-1` | 12px | 16px | 400 | 0px |

### Spacing Scale

| Token | Value |
|-------|-------|
| `xs` | 4px |
| `sm` | 8px |
| `md` | 16px |
| `lg` | 24px |
| `xl` | 32px |
| `margin-mobile` | 16px |
| `gutter` | 16px |

### Border Radius

| Token | Value |
|-------|-------|
| `DEFAULT` | 4px |
| `lg` | 8px |
| `xl` | 12px (login) / 0.75rem (other screens) |
| `2xl` | 24px (cards) |
| `full` | 9999px (pills, avatars) |

### Effects
- **Backdrop blur** (`backdrop-filter: blur(20px)`) — top bar, bottom nav, keypad
- **Box shadow** — cards: `0 4px 24px rgba(0,0,0,0.08)`
- **Active states** — `scale(0.95–0.98)` spring transform on all interactive elements
- **Transitions** — 150–200ms ease

### Icons
- Google Material Symbols Outlined (variable font)
- Key icons: `shield_with_heart`, `two_wheeler`, `emergency`, `explore`, `history`, `group`, `person`, `home`, `add_alert`, `settings`, `warning`, `contact_emergency`, `sports_motorsports`, `mountain_flag`

---

## Screen Inventory

---

### Screen 1: Login

**File:** `motorguard_login/code.html`  
**Purpose:** Phone number entry for OTP authentication

**Components:**
- Status bar spacer (48px)
- Logo header — 64×64px rounded-xl `#0058bc` background with `shield_with_heart` icon (FILL=1)
- Title — "Welcome to MotorGuard" (large-title-mobile, bold)
- Subtitle — "Enter your phone number to start your safe ride" (body, on-surface-variant)
- Phone input row — surface-container-low bg, country flag + dial code, phone number input
- Terms footnote — links in primary color
- "Send Code" primary button — full-width, h-14, rounded-xl, primary bg, disabled 50% opacity until 7+ digits
- "Sign in with Email" ghost button
- "or sign in with" divider
- Apple + Google OAuth buttons (2-col grid, rounded-xl border)

**Colors:** surface background, primary button, on-surface text  
**Navigation:** → OTP Verification screen on Send Code  
**States:** Button disabled (< 7 digits), loading spinner on click, "Code Sent!" success state  
**Assets:** US flag image (country picker), Apple logo, Google logo

---

### Screen 2: OTP Verification

**File:** `motorguard_verify_otp/code.html`  
**Purpose:** 6-digit OTP entry with custom numeric keypad

**Components:**
- Back button header (chevron_left + "Back", primary color)
- Title — "Verify Phone" (large-title-mobile)
- Subtitle with masked phone number (bold on-surface)
- 6× OTP input boxes — w-12 h-14, surface-container-low bg, border-2, rounded-xl, title-2 font, active border highlights in primary
- Resend timer — "Resend Code in 0:55" countdown
- "Verify & Continue" primary button — rounded-full, h-14
- Cupertino numeric keypad — 3-col grid, surface bg keys, with letter labels (ABC, DEF…), backspace key
- Keypad footer blurred — surface-container-highest/60 + backdrop-blur-xl

**Colors:** Primary border on active input, error border on wrong code  
**Navigation:** ← Login, → Home Dashboard on success  
**States:** Empty inputs, filling, complete, wrong code (error border), loading  
**Interactions:** Auto-advance to next box, backspace navigates back

---

### Screen 3: Home Dashboard

**File:** `motorguard_home_dashboard/code.html`  
**Purpose:** Main hub — start ride, SOS, weather, active groups

**Components:**
- Fixed top bar — avatar (32px circle), "MotorGuard" (title-2, primary), settings icon
- Welcome section — "Good morning, Alex." (large-title-mobile), bike ready subtitle
- **Start Ride card** — primary-container bg, rounded-[24px], "Start Ride" title-2, subtitle, motorcycle icon, "Tap to begin →" footer; decorative blur circle in background
- **2×2 quick actions grid:**
  - SOS Alert card — error-container bg, pulsing error circle with `emergency` icon, "SOS Alert" headline
  - Weather card — #F2F2F7 bg, cloud icon + temperature, "Optimal conditions" / road conditions caption
- **Active Groups section** — title-3 header, group card with stacked avatars (3 overlapping circles), group name, member count, "Join" pill button
- Fixed bottom nav — 5 tabs: Home (active, primary), Live Map, History, Groups, Profile
- Desktop sidebar nav (hidden on mobile)

**Colors:** primary-container for Start Ride, error-container for SOS, #F2F2F7 for weather  
**Navigation:** Bottom nav → all main sections; Start Ride → ride flow  
**States:** Group active/inactive, SOS pulse animation  
**Assets:** User avatar, group member avatars

---

### Screen 4: Live Map

**File:** `motorguard_live_map/code.html`  
**Purpose:** Full-screen interactive map with hazard markers

**Components:**
- Fixed top bar (same as dashboard)
- **Full-screen map canvas** — 100dvh × 100vw, fixed, z-index 0, map image as background
- **Hazard markers** — 44×44px white circles with backdrop-blur, positioned absolutely:
  - Oil spill (`oil_barrel`, amber color) — top 35%, left 45%
  - Pothole (`road`, red color) — top 55%, left 30%
  - Rain/slick (`rainy`, blue color) — top 40%, left 70%
- **FAB** — `add_alert` icon, 56×56px, primary bg, fixed bottom-right, above nav
- **Bottom sheet** (hazard detail) — slides up from bottom, rounded-[24px] top corners, handle indicator, icon + title + description, Dismiss + Confirm buttons
- Fixed bottom nav (Live Map tab active)

**Colors:** White hazard markers, primary FAB, error-container in bottom sheet  
**Navigation:** Hazard tap → bottom sheet; FAB → report hazard  
**States:** Sheet open/closed (translateY animation), hazard type colors  
**Interactions:** Tap map → close sheet; tap hazard → open sheet with details

---

### Screen 5: Ride History

**File:** `motorguard_ride_history/code.html`  
**Purpose:** Chronological list of past rides with stats and safety scores

**Components:**
- Fixed top bar
- Page title — "Ride History" (large-title-mobile), subtitle
- **Segmented control** — "All Rides / Commutes / Weekend" filter pills, surface-container-high bg
- **Month section headers** — title-3, "October 2023", "September 2023"
- **Ride cards** (in surface-container-low rounded-xl container, dividers between):
  - Map thumbnail — 128×96px (desktop) or full-width 128px (mobile), rounded-lg
  - Ride name (headline) + date/time (caption-1 right)
  - Route description (subheadline, on-surface-variant)
  - Stats row: distance (`straighten` icon), duration (`schedule` icon)
  - Safety score — right-aligned; green `verified` icon for high scores (98, 99), orange `warning` for lower (85)
- Fixed bottom nav (History tab active)

**Colors:** Primary for high scores, tertiary for warning scores  
**Navigation:** Ride card tap → Ride Detail  
**States:** Empty state, loading skeleton  
**Assets:** Route map thumbnails

---

### Screen 6: Groups

**File:** `motorguard_groups/code.html`  
**Purpose:** Group discovery, active rides, and my groups list

**Components:**
- Fixed top bar
- Section header — "Active Rides" (large-title-mobile), subtitle, "Create" pill button (primary)
- **Active Group cards** (platter rounded-xl with map thumbnail):
  - Map image header (h-40), "Live" badge (primary/90 bg) or "Scheduled" badge (on-surface-variant/80)
  - Stacked avatar row (bottom-right of map)
  - Group name (headline), member count with `group` icon
  - Chevron right
- **"My Groups" section** — title-3 header, "See All" link
  - Inset grouped list (surface-container-lowest bg, divide-y):
    - 48×48px rounded-lg icon with colored bg (secondary, tertiary, neutral)
    - Group name (headline), member count + active count (footnote)
    - Chevron right
- **Browse/Explore card** — dashed border, `explore` icon, "Find more groups" CTA
- **FAB** — `group_add`, primary, fixed bottom-right, above nav
- Fixed bottom nav (Groups tab active)

**Colors:** Secondary for sport group icon, tertiary for adventure group icon  
**Navigation:** Group card → Group Detail; FAB/Create → Create Group  
**States:** Active (Live badge), Scheduled badge, Offline state

---

### Screen 7: SOS Alert

**File:** `motorguard_sos_alert/code.html`  
**Purpose:** Crash detection countdown with cancel option

**Components:**
- Full-screen modal (max-w-md, center, surface bg, rounded-[2rem] on desktop)
- Error tint overlay — `bg-error/10` absolute inset
- Header — `warning` icon (48px, error color, FILL=1), "Crash Detected" title-1, description body text
- **Countdown circle** — 192×192px, error-container bg, **pulsing ring animation** (pulse-ring keyframe with scale + box-shadow)
  - Large countdown number (72px, bold, error color)
  - SVG circular progress ring — two circles: background stroke (error/20) + animated stroke (error color, strokeDashoffset animates)
- Label — "Seconds until dispatch" (subheadline, uppercase, tracking-wider)
- **"Call Emergency Now"** button — rounded-full, h-56px, error bg, `phone_in_talk` icon
- **"I am Okay (Cancel)"** button — rounded-full, h-56px, surface-container-high bg
- Countdown from 10 → 0, progress ring depletes, auto-dispatch at 0

**Colors:** Error red throughout, error-container bg circle  
**Navigation:** Cancel → dismiss SOS; auto-dispatch at 0  
**States:** Counting (10→0), dispatched, cancelled  
**Animations:** pulse-ring (scale 0.8→1, box-shadow glow), progress ring strokeDashoffset

---

### Screen 8: Profile

**File:** `motorguard_profile/code.html`  
**Purpose:** User stats, settings, and account management

**Components:**
- Fixed top bar — avatar (left), "Profile" (center headline), settings icon (right, primary)
- **Profile header** — 96×96px avatar with ring-4 ring-primary/10, name (title-1), "Pro Rider" badge (primary, verified icon)
- **Stats bento grid** (3 columns, surface-container-lowest cards):
  - Rides: 124 (title-3, primary)
  - Miles: 2.4k (title-3, primary)
  - Safety: 98% (title-3, tertiary color)
- **Settings sections** (iOS-style inset grouped lists):
  - "Safety & Security" — Emergency Contacts (error icon), Ride Settings (primary icon)
  - "Hardware" — Device Connectivity (secondary icon) with "Connected" status
  - "General" — Account (neutral icon)
- **Sign Out** button — full-width, error color, footnote-style
- Fixed bottom nav (Profile tab active)

**Colors:** Primary for stats, tertiary for safety score, error for sign out  
**Navigation:** Settings items → sub-screens; Sign Out → Login  
**States:** Connected/disconnected device status

---

## Reusable Components

| Component | Description | Used In |
|-----------|-------------|---------|
| `TopAppBar` | Fixed header, avatar + brand + action icon | All screens |
| `BottomNav` | 5-tab nav bar, blur backdrop | All main screens |
| `PrimaryButton` | Full-width, h-14, rounded-xl/full, primary bg | Login, OTP, SOS |
| `GhostButton` | Text-only, primary color | Login |
| `OAuthButton` | Icon only, border, rounded-xl | Login |
| `PhoneInput` | Country picker + phone field | Login |
| `OTPInput` | 6-box grid with keyboard | OTP |
| `NumericKeypad` | Cupertino-style 3×4 keypad | OTP |
| `BentoCard` | Rounded-[24px] card with decorative bg | Home |
| `SOSCard` | Error-container, pulsing animation | Home |
| `WeatherCard` | #F2F2F7 bg, icon + temp | Home |
| `GroupPreviewCard` | Stacked avatars + name + count + join | Home |
| `HazardMarker` | 44×44px circle, absolute map position | Live Map |
| `BottomSheet` | Slide-up sheet, handle, action buttons | Live Map |
| `SegmentedControl` | Pill filter tabs | Ride History |
| `RideCard` | Map thumb + stats + safety score | Ride History |
| `ActiveGroupCard` | Map banner + live badge + avatars | Groups |
| `GroupListItem` | Icon + name + count + chevron | Groups |
| `FAB` | 56×56px circle, primary, fixed | Groups, Live Map |
| `CountdownTimer` | SVG ring + pulsing circle + number | SOS |
| `ProfileHeader` | Avatar + name + badge | Profile |
| `StatCard` | Single stat value + label | Profile |
| `SettingsSection` | Grouped list with colored icons | Profile |
| `AvatarStack` | Overlapping circular avatars (-space-x-2) | Home, Groups |

---

## Navigation Map

```
Splash
  └── unauthenticated → Login
        └── Send Code → OTP Verification
              └── Verify → Home Dashboard
                    ├── (bottom nav) Live Map
                    ├── (bottom nav) Ride History
                    ├── (bottom nav) Groups
                    │     └── Group card → Group Detail
                    └── (bottom nav) Profile
                          ├── Emergency Contacts
                          ├── Ride Settings
                          └── Sign Out → Login

  Home Dashboard
    ├── Start Ride → Active Ride → Finish → Ride Completed
    └── SOS Alert → SOS Countdown → Cancel / Dispatch
```
