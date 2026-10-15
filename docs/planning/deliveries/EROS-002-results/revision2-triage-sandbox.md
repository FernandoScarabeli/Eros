# Eros — medição opt-in

Status: **incomplete**. Build release direto; pacote instalado não medido.

## Metodologia e limites

Linux /proc: CPU = delta(utime+stime)/delta(monotonic) × 100; 100% = um core. PSS/RSS/FDs/threads agregados por árvore de raízes possuídas, no máximo 1 Hz. Processos curtos entre amostras podem ser perdidos; primeiro delta CPU e grupo ausente são null. Não são coletados argv/env/conteúdo pessoal. p95: nearest rank ceil(0,95×n), distribuição min/mediana/p95/max.

Startup: relógio epoch da UI (performance.timeOrigin+now) correlacionado com time.time do monitor; cache de disco normal. Pronto = controle habilitado + dois rAF, aproximação de workspace interativo. Echo = escrita solicitada → callback xterm + dois rAF; não é tempo de retorno IPC e não prova pixels apresentados. Erro de composição/scheduling não limitado; duas frames acrescentam tipicamente 0–2 períodos de refresh. Sem medição GPU/pixels, metas de renderização permanecem não verificadas.

Fila xterm: bytes recebidos menos bytes cujo callback de parsing terminou; não mede buffers internos WebKit/IPC/kernel. Bytes PTY incluem prompts/controle e transformação LF→CRLF; produção real vem de contador separado do produtor. Assinaturas Rust e filas internas: não medidas. Animações off/on usam Home equivalente com shell retido; minimização exige confirmação nativa. Overhead do sampler é medido por process_time e monotonic ao redor da coleta; custo JS de scan/callback é registrado por chunk; não limita custo total de rAF/composição/driver. Comparação pareada integral permanece pendente.

## Metas propostas (sem alteração)

| Medida | Meta | Resultado / limitação |
| --- | --- | --- |
| Startup quente instalado / 20 amostras | ≤ 1500 ms | não medida / não verificado |
| Startup frio / 10 amostras | ≤ 3000 ms | não medida / não verificado |
| CPU idle, estabilização 30 s + janela 60 s | ≤ 1% core | não medida / não verificado |
| CPU minimizada | ≤ 0,5% core | não medida / não verificado |
| PSS UI+WebKit+serviço | ≤ 250 MiB | não medida / não verificado |
| Catálogo / aumento PSS | ≤ 75 MiB | não medida / não verificado |
| Navegação catálogo p95 | ≤ 100 ms | não medida / não verificado |
| Troca de chat p95 (shells via fixture de chat) | ≤ 150 ms | não medida / não verificado |
| Echo renderizado idle / carga | ≤ 50 / 100 ms | não medida / não verificado |
| Ctrl+C p95 | ≤ 500 ms | não medida / não verificado |
| Leaks 100 ciclos | ≤ max(10%,20 MiB) | não medida / não verificado |
| Watchers / jobs | limitados; sem varredura periódica | não medida / não verificado |

## Workloads

- W0: unavailable; duração 0.000 s; amostras 0; cleanup 0 sobreviventes.
  Clock: comparação de startup válida=None; amostras rejeitadas=0; tolerância de salto 5 ms, bracket máximo 1 ms. Métricas monotônicas independentes preservadas.
  Recursos derivados (UI + serviço): {"idle_cpu_percent_one_core_mean": null, "idle_samples_after_30s": 0, "peak_app_pss_kib": null, "view_cycles_growth": {"pss_kib": null, "fds": null, "threads": null}, "background_cpu": null, "rust_subscriptions": null, "catalog_service_only_pss": {"base_kib": null, "large_kib": null, "delta_kib": null, "scope": "service-only; UI-inclusive catalog goal remains unmeasured if snapshot fails"}}
  Unix socket bind denied: errno 1
- W0.empty-terminal: unavailable; duração 0.000 s; amostras 0; cleanup 0 sobreviventes.
  Clock: comparação de startup válida=None; amostras rejeitadas=0; tolerância de salto 5 ms, bracket máximo 1 ms. Métricas monotônicas independentes preservadas.
  Recursos derivados (UI + serviço): {"idle_cpu_percent_one_core_mean": null, "idle_samples_after_30s": 0, "peak_app_pss_kib": null, "view_cycles_growth": {"pss_kib": null, "fds": null, "threads": null}, "background_cpu": null, "rust_subscriptions": null, "catalog_service_only_pss": {"base_kib": null, "large_kib": null, "delta_kib": null, "scope": "service-only; UI-inclusive catalog goal remains unmeasured if snapshot fails"}}
  Unix socket bind denied: errno 1
- W0.home-off: unavailable; duração 0.000 s; amostras 0; cleanup 0 sobreviventes.
  Clock: comparação de startup válida=None; amostras rejeitadas=0; tolerância de salto 5 ms, bracket máximo 1 ms. Métricas monotônicas independentes preservadas.
  Recursos derivados (UI + serviço): {"idle_cpu_percent_one_core_mean": null, "idle_samples_after_30s": 0, "peak_app_pss_kib": null, "view_cycles_growth": {"pss_kib": null, "fds": null, "threads": null}, "background_cpu": null, "rust_subscriptions": null, "catalog_service_only_pss": {"base_kib": null, "large_kib": null, "delta_kib": null, "scope": "service-only; UI-inclusive catalog goal remains unmeasured if snapshot fails"}}
  Unix socket bind denied: errno 1
- W0.home-on: unavailable; duração 0.000 s; amostras 0; cleanup 0 sobreviventes.
  Clock: comparação de startup válida=None; amostras rejeitadas=0; tolerância de salto 5 ms, bracket máximo 1 ms. Métricas monotônicas independentes preservadas.
  Recursos derivados (UI + serviço): {"idle_cpu_percent_one_core_mean": null, "idle_samples_after_30s": 0, "peak_app_pss_kib": null, "view_cycles_growth": {"pss_kib": null, "fds": null, "threads": null}, "background_cpu": null, "rust_subscriptions": null, "catalog_service_only_pss": {"base_kib": null, "large_kib": null, "delta_kib": null, "scope": "service-only; UI-inclusive catalog goal remains unmeasured if snapshot fails"}}
  Unix socket bind denied: errno 1
- W0.minimized: unavailable; duração 0.000 s; amostras 0; cleanup 0 sobreviventes.
  Clock: comparação de startup válida=None; amostras rejeitadas=0; tolerância de salto 5 ms, bracket máximo 1 ms. Métricas monotônicas independentes preservadas.
  Recursos derivados (UI + serviço): {"idle_cpu_percent_one_core_mean": null, "idle_samples_after_30s": 0, "peak_app_pss_kib": null, "view_cycles_growth": {"pss_kib": null, "fds": null, "threads": null}, "background_cpu": null, "rust_subscriptions": null, "catalog_service_only_pss": {"base_kib": null, "large_kib": null, "delta_kib": null, "scope": "service-only; UI-inclusive catalog goal remains unmeasured if snapshot fails"}}
  Unix socket bind denied: errno 1
- W1.1: unavailable; duração 0.000 s; amostras 0; cleanup 0 sobreviventes.
  Clock: comparação de startup válida=None; amostras rejeitadas=0; tolerância de salto 5 ms, bracket máximo 1 ms. Métricas monotônicas independentes preservadas.
  Recursos derivados (UI + serviço): {"idle_cpu_percent_one_core_mean": null, "idle_samples_after_30s": 0, "peak_app_pss_kib": null, "view_cycles_growth": {"pss_kib": null, "fds": null, "threads": null}, "background_cpu": null, "rust_subscriptions": null, "catalog_service_only_pss": {"base_kib": null, "large_kib": null, "delta_kib": null, "scope": "service-only; UI-inclusive catalog goal remains unmeasured if snapshot fails"}}
  Unix socket bind denied: errno 1
- W1.8: unavailable; duração 0.000 s; amostras 0; cleanup 0 sobreviventes.
  Clock: comparação de startup válida=None; amostras rejeitadas=0; tolerância de salto 5 ms, bracket máximo 1 ms. Métricas monotônicas independentes preservadas.
  Recursos derivados (UI + serviço): {"idle_cpu_percent_one_core_mean": null, "idle_samples_after_30s": 0, "peak_app_pss_kib": null, "view_cycles_growth": {"pss_kib": null, "fds": null, "threads": null}, "background_cpu": null, "rust_subscriptions": null, "catalog_service_only_pss": {"base_kib": null, "large_kib": null, "delta_kib": null, "scope": "service-only; UI-inclusive catalog goal remains unmeasured if snapshot fails"}}
  Unix socket bind denied: errno 1
- W1.chat-switch: unavailable; duração 0.000 s; amostras 0; cleanup 0 sobreviventes.
  Clock: comparação de startup válida=None; amostras rejeitadas=0; tolerância de salto 5 ms, bracket máximo 1 ms. Métricas monotônicas independentes preservadas.
  Recursos derivados (UI + serviço): {"idle_cpu_percent_one_core_mean": null, "idle_samples_after_30s": 0, "peak_app_pss_kib": null, "view_cycles_growth": {"pss_kib": null, "fds": null, "threads": null}, "background_cpu": null, "rust_subscriptions": null, "catalog_service_only_pss": {"base_kib": null, "large_kib": null, "delta_kib": null, "scope": "service-only; UI-inclusive catalog goal remains unmeasured if snapshot fails"}}
  Unix socket bind denied: errno 1
- W2.1: unavailable; duração 0.000 s; amostras 0; cleanup 0 sobreviventes.
  Clock: comparação de startup válida=None; amostras rejeitadas=0; tolerância de salto 5 ms, bracket máximo 1 ms. Métricas monotônicas independentes preservadas.
  Recursos derivados (UI + serviço): {"idle_cpu_percent_one_core_mean": null, "idle_samples_after_30s": 0, "peak_app_pss_kib": null, "view_cycles_growth": {"pss_kib": null, "fds": null, "threads": null}, "background_cpu": null, "rust_subscriptions": null, "catalog_service_only_pss": {"base_kib": null, "large_kib": null, "delta_kib": null, "scope": "service-only; UI-inclusive catalog goal remains unmeasured if snapshot fails"}}
  Unix socket bind denied: errno 1
- W2.8: unavailable; duração 0.000 s; amostras 0; cleanup 0 sobreviventes.
  Clock: comparação de startup válida=None; amostras rejeitadas=0; tolerância de salto 5 ms, bracket máximo 1 ms. Métricas monotônicas independentes preservadas.
  Recursos derivados (UI + serviço): {"idle_cpu_percent_one_core_mean": null, "idle_samples_after_30s": 0, "peak_app_pss_kib": null, "view_cycles_growth": {"pss_kib": null, "fds": null, "threads": null}, "background_cpu": null, "rust_subscriptions": null, "catalog_service_only_pss": {"base_kib": null, "large_kib": null, "delta_kib": null, "scope": "service-only; UI-inclusive catalog goal remains unmeasured if snapshot fails"}}
  Unix socket bind denied: errno 1
- W3: unavailable; duração 0.000 s; amostras 0; cleanup 0 sobreviventes.
  Clock: comparação de startup válida=None; amostras rejeitadas=0; tolerância de salto 5 ms, bracket máximo 1 ms. Métricas monotônicas independentes preservadas.
  Recursos derivados (UI + serviço): {"idle_cpu_percent_one_core_mean": null, "idle_samples_after_30s": 0, "peak_app_pss_kib": null, "view_cycles_growth": {"pss_kib": null, "fds": null, "threads": null}, "background_cpu": null, "rust_subscriptions": null, "catalog_service_only_pss": {"base_kib": null, "large_kib": null, "delta_kib": null, "scope": "service-only; UI-inclusive catalog goal remains unmeasured if snapshot fails"}}
  Unix socket bind denied: errno 1

Calibração sem aplicativo: 5 amostras; CPU da coleta 0.003710 s; parede da coleta 0.003727 s em 5.004 s. Inclui monitor apenas; não é desempenho Eros.

## Resumo medido

| Métrica | n | min | mediana | p95 | max |
| --- | --- | --- | --- | --- | --- |

## Comparação entre rodadas

Use --compare rodada-anterior.json no mesmo build/configuração/hardware.

Ambiente e amostras brutas: JSON adjacente. Valores ausentes são null, nunca zero.
