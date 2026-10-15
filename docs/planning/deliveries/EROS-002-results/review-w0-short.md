# Revisão — triagem curta nativa

Não constitui baseline nem certificação das metas. Caminho pessoal do repositório foi mascarado no JSON.

# Eros — medição opt-in

Status: **complete**. Build release direto; pacote instalado não medido.

## Metodologia e limites

Linux /proc: CPU = delta(utime+stime)/delta(monotonic) × 100; 100% = um core. PSS/RSS/FDs/threads agregados por árvore de raízes possuídas, no máximo 1 Hz. Processos curtos entre amostras podem ser perdidos; primeiro delta CPU e grupo ausente são null. Não são coletados argv/env/conteúdo pessoal. p95: nearest rank ceil(0,95×n), distribuição min/mediana/p95/max.

Startup: relógio epoch da UI (performance.timeOrigin+now) correlacionado com time.time do monitor; cache de disco normal. Pronto = controle habilitado + dois rAF, aproximação de workspace interativo. Echo = escrita solicitada → callback xterm + dois rAF; não é tempo de retorno IPC e não prova pixels apresentados. Erro de composição/scheduling não limitado; duas frames acrescentam tipicamente 0–2 períodos de refresh. Sem medição GPU/pixels, metas de renderização permanecem não verificadas.

Fila xterm: bytes recebidos menos bytes cujo callback de parsing terminou; não mede buffers internos WebKit/IPC/kernel. Bytes PTY incluem prompts/controle e transformação LF→CRLF; produção real vem de contador separado do produtor. Assinaturas Rust e filas internas: não medidas. Animações off/on e minimização: pendentes. Overhead do sampler é medido por process_time e monotonic ao redor da coleta; custo da instrumentação UI requer rodada pareada sem probes.

## Metas propostas (sem alteração)

| Medida | Meta | Resultado / limitação |
| --- | --- | --- |
| Startup quente instalado / 20 amostras | ≤ 1500 ms | p95 576.219; diagnóstico, não verificado contra meta completa |
| Startup frio / 10 amostras | ≤ 3000 ms | p95 1156.965; diagnóstico, não verificado contra meta completa |
| CPU idle, estabilização 30 s + janela 60 s | ≤ 1% core | não medida / não verificado |
| CPU minimizada | ≤ 0,5% core | não medida / não verificado |
| PSS UI+WebKit+serviço | ≤ 250 MiB | não medida / não verificado |
| Catálogo / aumento PSS | ≤ 75 MiB | não medida / não verificado |
| Navegação catálogo p95 | ≤ 100 ms | não medida / não verificado |
| Troca de chat p95 | ≤ 150 ms | não medida / não verificado |
| Echo renderizado idle / carga | ≤ 50 / 100 ms | não medida / não verificado |
| Ctrl+C p95 | ≤ 500 ms | não medida / não verificado |
| Leaks 100 ciclos | ≤ max(10%,20 MiB) | não medida / não verificado |
| Watchers / jobs | limitados; sem varredura periódica | não medida / não verificado |

## Workloads

- W0: complete; duração 41.684 s; amostras 45; cleanup 0 sobreviventes.
  Recursos derivados (UI + serviço): {"idle_cpu_percent_one_core_mean": null, "idle_samples_after_30s": 0, "peak_app_pss_kib": 292464, "view_cycles_growth": {"pss_kib": null, "fds": null, "threads": null}, "background_cpu": null, "rust_subscriptions": null}

Calibração sem aplicativo: 5 amostras; CPU da coleta 0.053137 s; parede da coleta 0.053159 s em 5.054 s. Inclui monitor apenas; não é desempenho Eros.

## Resumo medido

| Métrica | n | min | mediana | p95 | max |
| --- | --- | --- | --- | --- | --- |
| W0.startupEmpty.coldMs | 1 | 1156.9651 | 1156.9651 | 1156.9651 | 1156.9651 |
| W0.startupEmpty.warmMs | 1 | 576.219 | 576.219 | 576.219 | 576.219 |
| W0.startup100Projects.coldMs | 1 | 634.7444 | 634.7444 | 634.7444 | 634.7444 |
| W0.startup100Projects.warmMs | 1 | 606.4548 | 606.4548 | 606.4548 | 606.4548 |
| W0.service.cpu_percent | 43 | 0.0 | 0.0 | 0.0 | 1.9563 |
| W0.service.pss_kib | 45 | 11070 | 11652.0 | 11825 | 11883 |
| W0.service.rss_kib | 45 | 39616 | 40136.0 | 40136 | 40136 |
| W0.service.fds | 45 | 8 | 13.0 | 13 | 13 |
| W0.service.threads | 45 | 1 | 4.0 | 4 | 4 |
| W0.ui_webkit.cpu_percent | 35 | 0.0 | 0.0 | 17.4902 | 43.782 |
| W0.ui_webkit.pss_kib | 44 | 13599 | 236832.0 | 250851 | 281362 |
| W0.ui_webkit.rss_kib | 44 | 44920 | 563084.0 | 578912 | 618120 |
| W0.ui_webkit.fds | 44 | 13 | 160.0 | 160 | 160 |
| W0.ui_webkit.threads | 44 | 7 | 93.0 | 109 | 111 |
| W0.controlled.cpu_percent | 34 | 0.0 | 0.0 | 0.0 | 0.0 |
| W0.controlled.pss_kib | 35 | 1025 | 1025.0 | 1026 | 1026 |
| W0.controlled.rss_kib | 35 | 4564 | 4564.0 | 4564 | 4564 |
| W0.controlled.fds | 35 | 4 | 4.0 | 4 | 4 |
| W0.controlled.threads | 35 | 1 | 1.0 | 1 | 1 |
| W0.monitor.cpu_percent | 44 | 1.9426 | 2.9151 | 46.7477 | 62.3225 |
| W0.monitor.pss_kib | 45 | 14643 | 14866.0 | 14902 | 14906 |
| W0.monitor.rss_kib | 45 | 25616 | 26284.0 | 26336 | 26340 |
| W0.monitor.fds | 45 | 5 | 8.0 | 8 | 8 |
| W0.monitor.threads | 45 | 1 | 1.0 | 1 | 1 |

## Comparação entre rodadas

Use --compare rodada-anterior.json no mesmo build/configuração/hardware.

Ambiente e amostras brutas: JSON adjacente. Valores ausentes são null, nunca zero.
