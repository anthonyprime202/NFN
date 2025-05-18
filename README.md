# 🎧 NFN - Not For Normies (WIP)

**NFN** is a fully offline/streamable personal music player built with a hybrid Rust + web tech stack. It aims to provide a clean, fast, and modern music experience — without relying on mainstream bloated platforms. Think of it as your own open-source Spotify with full control.

## 🚀 Tech Stack

| Layer          | Technology                         |
|----------------|-------------------------------------|
| **Frontend**   | Svelte + Vite + TypeScript + SCSS  |
| **Native Shell** | Tauri v2 (desktop + mobile capable) |
| **Backend**    | Rust (Axum for APIs)               |
| **Audio**      | Rodio                              |
| **Database**   | PostgreSQL                         |
| **Cloud**      | Supabase (optional - sync, auth)   |
| **Streaming**  | yt-dlp                             |

## 🔑 Key Features

- 🎵 Play **any song** from local or online sources
- 🗂️ **Create & manage playlists**
- 📶 **Stream songs** online (via yt-dlp) and **cache** them for later
- 🔁 **Offline support**
- 🎧 **Multi-device sync** (like Spotify Connect - using Supabase Realtime)
- 🕐 **Track playback progress** and analyze listening habits
- 🎨 Beautiful full-screen **Player View**
- 🔀 **Smart shuffle** based on current genre
- 📈 **Stats** like total time listened or genre breakdown by month
- 💾 Persistent metadata: artist, album, artwork, etc.
- 🧑‍🎤 Artist pages, album pages (future)
- 🎙️ Optional synced lyrics
- 🌈 Optional visualizer

## 🧠 App Architecture

- Frontend talks to backend via **Tauri commands**
- Backend stores and manages state in **AppState**
- Audio is handled by **Rodio** and controlled via an internal **player loop**
- State transitions (play/pause/stop) are tracked using an enum, not booleans
- All error handling is cleanly done using a hybrid **AppError + anyhow** system
- Events are emitted from backend to UI using **Tauri events**

## ⚠️ Developer Philosophy

- No Tailwind. SCSS + Svelte is the preferred styling system.
- Semantic HTML, accessibility, and performance matter.
- Internal state is precise and modeled using proper enums and types.
- No “just get it working” mentality — clean patterns or bust.

## 🧪 Todo / Upcoming

* [ ] Music library scanner
* [ ] Smart shuffle & genre detection
* [ ] Artist / Album pages
* [ ] Mobile app support (via Tauri v2)
* [ ] Lyrics syncing
* [ ] Playback visualizer
* [ ] Caching strategy + Supabase sync
* [ ] Real analytics / time-tracking

## ❤️ Special Notes

This project is not just about tech — it's personal.
Built from the ground up by someone who **loves music**, **hates bloat**, and **cares about experience**.
This isn’t for normies — it’s for people who want to own their music experience.

## 📜 License

MIT — Do whatever you want.
Just don’t turn it into another ad-ridden mess.

