import { mount } from 'svelte';
import App from './App.svelte';
import './styles.css';

async function start() {
  const benchmark = import.meta.env.VITE_EROS_BENCHMARK === '1'
    ? await import('./lib/benchmark') : null;
  const config = benchmark ? await benchmark.configureBenchmark() : null;
  mount(App, { target: document.getElementById('app')! });
  if (benchmark && config) void benchmark.runBenchmark(config);
}
void start();
