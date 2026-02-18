# Fauna — Game Platform Design Document

## Vision

An in-browser game platform where creators build and deploy games, and players discover and play them. Long-term goal: a Steam/PlayStation-like experience that runs entirely in the browser.

## Core Components

### 1. Game Engine (Rust → WASM → wgpu)

- Written in Rust, compiled to WASM via `wasm-bindgen` / `wasm-pack`
- Rendering via `wgpu` (abstracts over WebGPU and WebGL2 backends)
- Everything is 3D — 2D is a special case with a locked orthographic camera and z=0. One renderer, one physics system, one set of tools. Creators pick "2D mode" or "3D mode" which just constrains the camera and defaults.
- Engine handles: rendering, physics, audio, input, asset loading
- Fullscreen support via the browser Fullscreen API

### 2. Scripting Layer

Python is the initial scripting language (via RustPython compiled to WASM) chosen for accessibility — anyone can pick it up.

**Key design constraint:** The scripting layer is behind an abstraction (Rust trait) so the backend is swappable. The engine defines a scripting interface; RustPython is just one implementation. Lua, JavaScript, or other languages can be added later without touching the engine core.

**Scripting interface responsibilities:**
- Execute scripts, call functions by name
- Pass data between engine and scripts (entities, events, input state)
- Expose engine APIs to scripts (spawn entity, play sound, move object, etc.)

**Boundary between engine and script:**
- Engine (Rust): rendering, physics, audio, input, asset management — anything performance-critical
- Scripts (Python): game logic, entity behavior, UI flow, event handling — high-level orchestration

**Python-in-WASM tradeoffs to design around:**
- Binary size: RustPython adds several MB — use code splitting, lazy-load the runtime
- Performance: interpreted Python is slow for hot loops — keep tight loops in Rust, scripts orchestrate
- Stdlib: not everything works in WASM — document what's available, sandbox appropriately

### 3. Web Platform

Standard web application providing:

- **User roles:** creators (build/deploy games) and players (discover/play games)
- **Auth & profiles:** accounts, creator pages, player libraries
- **Game deployment pipeline:** creator uploads WASM bundle → platform hosts and serves it
- **Game pages:** embedded player (iframe or direct WASM mount), metadata, ratings
- **Discovery:** browse, search, categories, featured games

### 4. Marketplace (Future)

- Asset store: textures, sprites, 3D models, sound effects, music
- Creator-to-creator resource trading
- Payment processing, licensing, moderation

## Asset Compatibility

Support standard formats so creators can use existing resources:

- **Images/Textures:** PNG, JPEG
- **2D maps:** Tiled (.tmx), LDtk
- **2D animation:** Aseprite, Spine
- **3D models (future):** glTF
- **Audio:** OGG, WAV, MP3

## Architecture

```
Creator's Python Script
        │
  Scripting Interface (abstract trait — language-agnostic)
        │
  RustPython Backend (swappable implementation)
        │
  Engine Core (Rust, compiled to WASM)
        │
  WebGL2 / Web Audio / Input APIs / Asset Loaders
        │
  Browser
```

## Phased Approach

**Phase 1 — Foundation**
- 3D engine in Rust/WASM with wgpu (2D = locked camera mode)
- Scripting interface + RustPython backend
- Basic web platform: auth, game upload, game hosting/playing

**Phase 2 — Creator Tools**
- In-browser editor / IDE for game development
- Asset import pipeline with format support
- Debugging and preview tools

**Phase 3 — Platform Growth**
- Marketplace for assets and resources
- Social features, ratings, collections

**Phase 4 — Ecosystem**
- Additional scripting language backends
- Community tools, modding support
- Monetization options for creators
