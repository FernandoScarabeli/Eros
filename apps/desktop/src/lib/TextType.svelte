<script lang="ts">
  import { onMount } from 'svelte';

  type Props = {
    text: string | string[];
    typingSpeed?: number;
    pauseDuration?: number;
    showCursor?: boolean;
    cursorCharacter?: string;
    deletingSpeed?: number;
    variableSpeedEnabled?: boolean;
    variableSpeedMin?: number;
    variableSpeedMax?: number;
    cursorBlinkDuration?: number;
  };

  let {
    text,
    typingSpeed = 45,
    pauseDuration = 2500,
    showCursor = true,
    cursorCharacter = '|',
    deletingSpeed = 25,
    variableSpeedEnabled = false,
    variableSpeedMin = 60,
    variableSpeedMax = 120,
    cursorBlinkDuration = 0.5,
  }: Props = $props();

  let displayedText = $state('');
  let currentPhraseIndex = $state(0);

  onMount(() => {
    const phrases = Array.isArray(text) ? text : [text];
    if (!phrases.length) return;

    currentPhraseIndex = Math.floor(Math.random() * phrases.length);
    const motionPreference = window.matchMedia('(prefers-reduced-motion: reduce)');
    if (motionPreference.matches || window.__EROS_BENCHMARK__?.animations === 'off') {
      displayedText = phrases[currentPhraseIndex];
      return;
    }

    let isDeleting = false;
    let timeout = 0;
    const getTypingDelay = () => {
      if (!variableSpeedEnabled) return typingSpeed;
      const min = Math.min(variableSpeedMin, variableSpeedMax);
      const max = Math.max(variableSpeedMin, variableSpeedMax);
      return min + Math.random() * (max - min);
    };

    const typeNextCharacter = () => {
      const phrase = phrases[currentPhraseIndex];
      if (!isDeleting) {
        if (displayedText.length < phrase.length) {
          displayedText = phrase.slice(0, displayedText.length + 1);
          timeout = window.setTimeout(typeNextCharacter, getTypingDelay());
        } else {
          isDeleting = true;
          timeout = window.setTimeout(typeNextCharacter, pauseDuration);
        }
      } else if (displayedText.length > 0) {
        displayedText = displayedText.slice(0, -1);
        timeout = window.setTimeout(typeNextCharacter, deletingSpeed);
      } else {
        isDeleting = false;
        currentPhraseIndex = (currentPhraseIndex + 1) % phrases.length;
        timeout = window.setTimeout(typeNextCharacter, getTypingDelay());
      }
    };

    timeout = window.setTimeout(typeNextCharacter, getTypingDelay());
    return () => window.clearTimeout(timeout);
  });
</script>

<span class="text-type">
  <span class="text-type__content">{displayedText}</span><span
    class="text-type__cursor"
    class:text-type__cursor--hidden={!showCursor}
    style={`--cursor-blink-duration: ${cursorBlinkDuration}s`}
    aria-hidden="true">{cursorCharacter}</span
  >
</span>
