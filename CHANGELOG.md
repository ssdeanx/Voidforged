# Changelog

## v0.3.0 — GPU particles, shaders, combat polish

### Added
- **GPU particle effects** (bevy_hanabi 0.15) — impact bursts, glow auras, dash trails
- **Custom WGSL shader** (`GlowMaterial`) — time-pulsing emissive glow for telegraph/VFX
- **`EffectsLibrary` resource** — pre-built effect handles built at load time
- **`SpawnImpactEvent`** — event-driven VFX pipeline (cross-crate)
- **Impact bursts** on projectile hit (player + enemy)
- **Screen shake** — camera trauma on player damage, decays over time
- **Floating damage numbers** — 3D text with lifetime, yellow for crits
- **Enemy telegraphing** — 0.3-0.6s windup pause before attack
- **Lifesteal** — heal % of damage dealt
- **Health pickups** — 15-40% drop chance, heal 25 HP
- **Gold pickups** — 10-50% drop chance, +10 gold
- **Equipment system** — 4 slots (weapon/offhand/armor/accessory), applied on run start
- **Expanded CombatStats** — all 12 fields wired into gameplay
- **Crit rolls** — `crit_chance` rolled per hit, `crit_multiplier` applied
- **Armor calculation** — `armor / (armor + 100)` formula, min 1 damage
- **Dash cooldown reduction** — stat now applied
- **HUD** — health bar + HP text, XP bar, wave counter, level, dash CD, damage numbers
- **Dead code removed** — `projectile.rs` deleted

### Changed
- **Player movement** — velocity lerps for acceleration (10x/sec), weightier feel
- **Primary attack** — hold-to-fire on cooldown (Hades-style), mouse-aimed
- **Secondary attack** — right mouse, 3-projectile spread, 0.8s CD
- **Cast ability** — Q key, piercing projectile, 3s CD
- **Dash** — i-frames, dash attack, dash trail VFX ready
- **Loot drops** — XP gems + health + gold with per-variant chances
- **`GameAssets`** — added health/gold pickup mesh + material

### Fixed
- Weapon cooldown now decrements every frame (was stuck at 1 attack per life)
- `cursor_to_world` state-gated to Playing
- Pickup collection uses real player entity (not `Entity::from_raw(0)`)

### Technical
- bevy_hanabi 0.15.1 for GPU particles
- Custom assets/shaders/glow.wgsl for emissive material
- All 8 crates compiling with zero warnings
- Event-driven cross-crate VFX (no circular deps)

## v0.2.0 — Skill-based combat + game loop

### Added
- Mouse aiming, click-to-attack, dash/dodge system
- Enemy melee/ranged attacks, projectile movement, collision
- AttackCooldown component, XP gem collection
- Wave announcements, pause toggle
- Game loop: MainMenu → Playing → GameOver → restart
- Per-run resource reset

### Fixed
- Entity handle placeholders, weapon spawn, projectile velocity
- Deprecated Bevy API migration, 21 warnings eliminated

## v0.1.0 — Initial scaffold

8-crate workspace, core ECS types, isometric camera, 5-variant enemy AI,
wave spawning, XP leveling, networking stubs, server stub.
