# BuildOps 4.0 - Installation & Setup Guide

## Prerequisites

### Required Software

1. **Rust** (1.77 or later)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Node.js** (20 or later)
   ```bash
   # Using nvm
   nvm install 20
   nvm use 20
   ```

3. **pnpm** (9 or later)
   ```bash
   npm install -g pnpm
   ```

4. **Xcode Command Line Tools** (macOS)
   ```bash
   xcode-select --install
   ```

## Installation Steps

### 1. Clone Repository (if applicable)

```bash
cd /Users/billyribeiro/Documents/projects/build-ops
```

### 2. Install Frontend Dependencies

```bash
pnpm install
```

This will install:
- SvelteKit 2.50+
- Svelte 5.49+
- TailwindCSS 4.1+
- Tauri API 2.2+
- Iconify Svelte 4.2+
- All Tauri plugins

### 3. Build Rust Backend

```bash
cd src-tauri
cargo build
cd ..
```

First build will take 5-10 minutes as it compiles all dependencies.

### 4. Run Development Server

```bash
pnpm tauri dev
```

This will:
- Start Vite dev server on `http://localhost:5173`
- Compile Rust backend
- Launch macOS application window
- Initialize SQLite database
- Run all migrations

## Database Initialization

On first run, the app will automatically:

1. Create database at: `~/Library/Application Support/com.billyribeiro.buildops40/buildops40.db`
2. Run all 13 migrations
3. Create default user capacity profile
4. Populate settings table

## Verify Installation

### Check Database

```bash
sqlite3 ~/Library/Application\ Support/com.billyribeiro.buildops40/buildops40.db

.tables
# Should show: programs, modules, day_plans, day_attempts, user_capacity_profiles, 
#              day_sessions, time_recommendations, focus_metrics_daily, etc.

SELECT * FROM user_capacity_profiles;
# Should show default profile

.quit
```

### Check Frontend

1. App window should open
2. Navigate to any route
3. Open browser DevTools (Cmd+Option+I)
4. Check console for errors (should be none)

### Check Backend

```bash
# In src-tauri directory
cargo test
```

## Configuration

### Update Capacity Profile

The default profile is:
- **default_daily_minutes:** 180 (3 hours)
- **weekly_study_days:** 5
- **preferred_start_time:** "18:00"
- **max_deep_days_per_week:** 2
- **break_pattern:** "50/10"
- **timezone:** "UTC"

To customize, you can either:

1. **Via UI** (when settings page is built):
   - Navigate to `/settings/capacity`
   - Update values
   - Save

2. **Via Database**:
   ```bash
   sqlite3 ~/Library/Application\ Support/com.billyribeiro.buildops40/buildops40.db
   
   UPDATE user_capacity_profiles 
   SET default_daily_minutes = 240,
       weekly_study_days = 6,
       preferred_start_time = '19:00',
       max_deep_days_per_week = 3
   WHERE user_id = 'default';
   
   .quit
   ```

## Building for Production

### Development Build

```bash
pnpm tauri build --debug
```

Output: `src-tauri/target/debug/bundle/`

### Production Build

```bash
pnpm tauri build
```

Output: `src-tauri/target/release/bundle/`

This creates:
- **DMG:** `BuildOps 40.dmg` (drag-to-install)
- **App Bundle:** `BuildOps 40.app`

### Install Production Build

```bash
open src-tauri/target/release/bundle/dmg/BuildOps\ 40.dmg
```

Drag `BuildOps 40.app` to `/Applications`

## Troubleshooting

### "Command not found: tauri"

```bash
pnpm install -g @tauri-apps/cli
```

### "Cannot find module '@sveltejs/adapter-static'"

```bash
pnpm install -D @sveltejs/adapter-static
```

### "Failed to initialize database"

Check permissions:
```bash
ls -la ~/Library/Application\ Support/com.billyribeiro.buildops40/
```

If directory doesn't exist:
```bash
mkdir -p ~/Library/Application\ Support/com.billyribeiro.buildops40/
```

### "Rust compilation failed"

Update Rust:
```bash
rustup update
```

Clear cargo cache:
```bash
cd src-tauri
cargo clean
cargo build
```

### "Port 5173 already in use"

Kill existing process:
```bash
lsof -ti:5173 | xargs kill -9
```

Or change port in `vite.config.ts`:
```typescript
export default defineConfig({
  server: {
    port: 5174
  }
});
```

### TypeScript Errors

The TypeScript errors you see in the IDE are expected during initial setup. They will resolve after:

```bash
pnpm install
pnpm run prepare
```

This runs `svelte-kit sync` which generates type definitions.

## Development Workflow

### Hot Reload

Both frontend and backend support hot reload:

- **Frontend:** Vite HMR (instant)
- **Backend:** Cargo watch (2-3 seconds)

### Database Migrations

To add a new migration:

1. Create file: `src-tauri/migrations/014_your_migration.sql`
2. Add SQL statements
3. Update `src-tauri/src/db/mod.rs` to include new migration
4. Restart app

### Adding New Commands

1. Create command in `src-tauri/src/commands/your_module.rs`
2. Add to `src-tauri/src/commands/mod.rs`
3. Register in `src-tauri/src/lib.rs` invoke_handler
4. Create TypeScript wrapper in `src/lib/commands/index.ts`
5. Use in components

## Environment Variables

Create `.env` file:

```bash
# Development
TAURI_PUBLIC_URL=http://localhost:5173

# Production
# TAURI_PUBLIC_URL=/
```

## Logging

### Frontend Logs

Open DevTools Console (Cmd+Option+I)

### Backend Logs

Rust logs appear in terminal where you ran `pnpm tauri dev`

To increase verbosity:

```bash
RUST_LOG=debug pnpm tauri dev
```

## Next Steps

1. ✅ Verify installation
2. ✅ Create a test program
3. ✅ Create a test day plan
4. ✅ Navigate to `/day/[id]`
5. ✅ Click "Plan My Day"
6. ✅ Start a session
7. ✅ Complete the workflow

## Support

For issues:
1. Check `TIME_ENGINE_IMPLEMENTATION.md` for architecture details
2. Review error messages in console
3. Check database state with SQLite
4. Verify all dependencies installed

---

**Installation Complete! 🎉**

You now have a fully functional BuildOps 4.0 application with comprehensive time management capabilities.
