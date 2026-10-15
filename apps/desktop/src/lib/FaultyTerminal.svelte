<script lang="ts">
  import { onMount } from 'svelte';
  import { mountFaultyTerminal } from './faulty-terminal.js';

  let container: HTMLDivElement;
  let canvas: HTMLCanvasElement;

  onMount(() => {
    const motion = window.matchMedia('(prefers-reduced-motion: reduce)');
    let dispose: (() => void) | undefined;
    const mount = () => {
      dispose?.();
      const options = {
        tint: '#6366f1', glyphs: 'characters', clickEffect: 'glitch',
        scale: 2.4, digitSize: 2.15, timeScale: 0.15,
        brightness: 1.45, glow: 0.5, persistence: 0.5,
        mouseStrength: 0.15, mouseGlow: 0.6, scramble: 0,
        scanlineIntensity: 0.55, glitchAmount: 1, flickerAmount: 1,
        noiseAmp: 1, chromaticAberration: 1.6, curvature: 0.2,
        vignette: 0.25, dpr: 1,
        mouseReact: !motion.matches, pageLoadAnimation: !motion.matches,
      };
      dispose = mountFaultyTerminal(container, canvas, options);
    };
    mount();
    motion.addEventListener('change', mount);
    return () => {
      dispose?.();
      motion.removeEventListener('change', mount);
    };
  });
</script>

<div class="terminal-atmosphere" bind:this={container} aria-hidden="true">
  <canvas class="terminal-atmosphere-canvas" bind:this={canvas}></canvas>
</div>
