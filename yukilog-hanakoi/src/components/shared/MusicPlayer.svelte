<script lang="ts">
  // ================================
  // 背景音乐播放器
  // 自动循环播放，带淡入淡出效果
  // 播放进度通过 sessionStorage 在页面间保持
  // ================================
  import { onMount } from 'svelte';

  const musicUrl = '/music/时空漫步.mp3';
  const FADE_IN_DURATION = 800;
  const MAX_VOLUME = 0.4;
  const STORAGE_KEY = 'yukilog-music';

  let audio: HTMLAudioElement | undefined = $state();

  onMount(() => {
    if (!audio) return;

    let started = false;

    function restoreProgress() {
      try {
        const saved = sessionStorage.getItem(STORAGE_KEY);
        if (saved) {
          const { time, timestamp } = JSON.parse(saved);
          const elapsed = (Date.now() - timestamp) / 1000;
          const resumeTime = time + elapsed;
          if (resumeTime > 0 && isFinite(resumeTime)) {
            if (audio!.duration && resumeTime < audio!.duration) {
              audio!.currentTime = resumeTime;
            } else {
              audio!.addEventListener('loadedmetadata', () => {
                if (resumeTime < audio!.duration) {
                  audio!.currentTime = resumeTime;
                }
              }, { once: true });
            }
          }
        }
      } catch {}
    }

    function saveProgress() {
      try {
        if (!audio!.paused) {
          sessionStorage.setItem(STORAGE_KEY, JSON.stringify({
            time: audio!.currentTime,
            timestamp: Date.now(),
          }));
        }
      } catch {}
    }

    function fadeIn() {
      const startTime = Date.now();
      const interval = setInterval(() => {
        const elapsed = Date.now() - startTime;
        const progress = Math.min(elapsed / FADE_IN_DURATION, 1);
        const eased = progress * (2 - progress); // ease-out
        audio!.volume = eased * MAX_VOLUME;
        if (progress >= 1) clearInterval(interval);
      }, 30);
    }

    function tryPlay() {
      if (started || !audio!.paused) {
        started = true;
        return;
      }
      audio!.volume = 0;
      restoreProgress();
      const p = audio!.play();
      if (p) {
        p.then(() => {
          started = true;
          fadeIn();
          cleanup();
        }).catch(() => {});
      }
    }

    function cleanup() {
      events.forEach(e => document.removeEventListener(e, tryPlay));
    }

    const events = ['click', 'touchstart', 'scroll', 'keydown', 'mousemove', 'pointerdown'];
    tryPlay();
    events.forEach(e => document.addEventListener(e, tryPlay, { passive: true }));

    // 重试机制
    let retryCount = 0;
    const retryInterval = setInterval(() => {
      if (started || retryCount > 20) {
        clearInterval(retryInterval);
        return;
      }
      tryPlay();
      retryCount++;
    }, 250);

    // 定期保存进度
    const progressTimer = setInterval(saveProgress, 2000);
    window.addEventListener('beforeunload', saveProgress);

    return () => {
      cleanup();
      clearInterval(retryInterval);
      clearInterval(progressTimer);
      window.removeEventListener('beforeunload', saveProgress);
    };
  });
</script>

<audio bind:this={audio} src={musicUrl} loop preload="auto"
  style="position:absolute;width:0;height:0;overflow:hidden"></audio>
