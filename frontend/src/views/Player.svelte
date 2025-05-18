<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { type Song } from "../types";
  let currentSong = $state("Nothing to Play");
  let duration = $state(0);
  let progress = $state(0);
  let playerStatus = $state<"playing" | "paused" | "stopped">("stopped");

  $effect(() => {
    let disposed = false;
    let unlisteners: UnlistenFn[] = [];

    (async () => {
      try {
        let progressListener = await listen<number>(
          "player-progress",
          ({ payload }) => {
            progress = payload;
          },
        );
        unlisteners.push(progressListener);

        let appendedListener = await listen<Song>(
          "player-appended",
          ({ payload }) => {
            if (payload.duration) duration = payload.duration;
          },
        );
        unlisteners.push(appendedListener);

        let endedListener = await listen<null>("player-ended", () => {
          progress = duration;
          playerStatus = "stopped";
        });
        unlisteners.push(endedListener);


        if (disposed) {
          unlisteners.forEach((l) => l());
          unlisteners = [];
        }
      } catch (e) {
        console.log(e);
      }
    })();

    return () => {
      disposed = true;
      unlisteners.forEach((l) => l());
      unlisteners = [];
    };
  });

  $effect(() => {
    (async () => {
      await invoke("update_status", { status: playerStatus });
    })()
  })

  async function addToQueue() {
    currentSong = "Lady Butterfly";
    playerStatus = "playing";
    await invoke("add_to_queue", { songName: currentSong });
  }
  async function playPauseSong() {
    if (playerStatus === "playing") {
      playerStatus = "paused";
    } else {
      playerStatus = "playing";
    }
  }
</script>

<main>
  <h1>{currentSong}</h1>
  <progress value={progress} max={duration}></progress>
  <button onclick={addToQueue}>Add to Queue</button>
  <button onclick={playPauseSong}>{ playerStatus === "playing" ? "Pause" : "Play" }</button>
</main>

<style lang="scss">
  main {
    display: grid;
    place-items: center;
    gap: 2rem;
  }
  progress:indeterminate::-moz-progress-bar {
    color: red;
  }
  progress:indeterminate::-webkit-progress-bar {
    color: red;
  }
  progress:indeterminate::-webkit-progress-value {
    color: red;
  }
</style>
