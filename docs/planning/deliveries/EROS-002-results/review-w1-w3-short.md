# Revisão — triagem curta nativa

Não constitui baseline nem certificação das metas. Caminho pessoal do repositório foi mascarado no JSON.

# Eros — medição opt-in

Status: **incomplete**. Build release direto; pacote instalado não medido.

## Metodologia e limites

Linux /proc: CPU = delta(utime+stime)/delta(monotonic) × 100; 100% = um core. PSS/RSS/FDs/threads agregados por árvore de raízes possuídas, no máximo 1 Hz. Processos curtos entre amostras podem ser perdidos; primeiro delta CPU e grupo ausente são null. Não são coletados argv/env/conteúdo pessoal. p95: nearest rank ceil(0,95×n), distribuição min/mediana/p95/max.

Startup: relógio epoch da UI (performance.timeOrigin+now) correlacionado com time.time do monitor; cache de disco normal. Pronto = controle habilitado + dois rAF, aproximação de workspace interativo. Echo = escrita solicitada → callback xterm + dois rAF; não é tempo de retorno IPC e não prova pixels apresentados. Erro de composição/scheduling não limitado; duas frames acrescentam tipicamente 0–2 períodos de refresh. Sem medição GPU/pixels, metas de renderização permanecem não verificadas.

Fila xterm: bytes recebidos menos bytes cujo callback de parsing terminou; não mede buffers internos WebKit/IPC/kernel. Bytes PTY incluem prompts/controle e transformação LF→CRLF; produção real vem de contador separado do produtor. Assinaturas Rust e filas internas: não medidas. Animações off/on e minimização: pendentes. Overhead do sampler é medido por process_time e monotonic ao redor da coleta; custo da instrumentação UI requer rodada pareada sem probes.

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
| Troca de chat p95 | ≤ 150 ms | não medida / não verificado |
| Echo renderizado idle / carga | ≤ 50 / 100 ms | não medida / não verificado |
| Ctrl+C p95 | ≤ 500 ms | não medida / não verificado |
| Leaks 100 ciclos | ≤ max(10%,20 MiB) | não medida / não verificado |
| Watchers / jobs | limitados; sem varredura periódica | não medida / não verificado |

## Workloads

- W1: blocked; duração 17.576 s; amostras 18; cleanup 0 sobreviventes.
  Error: Native benchmark condition timed out
- W2: blocked; duração 16.539 s; amostras 17; cleanup 0 sobreviventes.
  Error: Native benchmark condition timed out
- W3: blocked; duração 10.072 s; amostras 0; cleanup 0 sobreviventes.
  Service startup exceeded 10 seconds

Calibração sem aplicativo: 5 amostras; CPU da coleta 0.054569 s; parede da coleta 0.054593 s em 5.056 s. Inclui monitor apenas; não é desempenho Eros.

## Resumo medido

| Métrica | n | min | mediana | p95 | max |
| --- | --- | --- | --- | --- | --- |
| W1.service.cpu_percent | 17 | 0.0 | 0.0 | 0.9709 | 0.9709 |
| W1.service.pss_kib | 18 | 11427 | 11728.0 | 11779 | 11779 |
| W1.service.rss_kib | 18 | 39596 | 40276.0 | 40276 | 40276 |
| W1.service.fds | 18 | 8 | 21.0 | 23 | 23 |
| W1.service.threads | 18 | 1 | 8.0 | 10 | 10 |
| W1.ui_webkit.cpu_percent | 16 | 0.0 | 0.9712 | 61.1667 | 61.1667 |
| W1.ui_webkit.pss_kib | 18 | 16218 | 289679.0 | 292370 | 292370 |
| W1.ui_webkit.rss_kib | 18 | 51252 | 627734.0 | 630512 | 630512 |
| W1.ui_webkit.fds | 18 | 13 | 166.0 | 167 | 167 |
| W1.ui_webkit.threads | 18 | 7 | 111.5 | 112 | 112 |
| W1.controlled.cpu_percent | 15 | 0.0 | 0.0 | 0.0 | 0.0 |
| W1.controlled.pss_kib | 17 | 1011 | 2668.0 | 2669 | 2669 |
| W1.controlled.rss_kib | 17 | 4532 | 13604.0 | 13604 | 13604 |
| W1.controlled.fds | 17 | 4 | 12.0 | 12 | 12 |
| W1.controlled.threads | 17 | 1 | 3.0 | 3 | 3 |
| W1.monitor.cpu_percent | 17 | 0.978 | 2.9129 | 3.8841 | 3.8841 |
| W1.monitor.pss_kib | 18 | 14588 | 14634.0 | 14642 | 14642 |
| W1.monitor.rss_kib | 18 | 25460 | 25528.0 | 25536 | 25536 |
| W1.monitor.fds | 18 | 5 | 5.0 | 5 | 5 |
| W1.monitor.threads | 18 | 1 | 1.0 | 1 | 1 |
| W2.service.cpu_percent | 16 | 0.0 | 0.0 | 0.9703 | 0.9703 |
| W2.service.pss_kib | 17 | 11326 | 11522.0 | 11751 | 11751 |
| W2.service.rss_kib | 17 | 39544 | 40064.0 | 40064 | 40064 |
| W2.service.fds | 17 | 8 | 21.0 | 21 | 21 |
| W2.service.threads | 17 | 1 | 8.0 | 8 | 8 |
| W2.ui_webkit.cpu_percent | 15 | 0.0 | 0.9712 | 27.1696 | 27.1696 |
| W2.ui_webkit.pss_kib | 17 | 16020 | 288915.0 | 290037 | 290037 |
| W2.ui_webkit.rss_kib | 17 | 50800 | 623772.0 | 624704 | 624704 |
| W2.ui_webkit.fds | 17 | 13 | 166.0 | 167 | 167 |
| W2.ui_webkit.threads | 17 | 7 | 112.0 | 112 | 112 |
| W2.controlled.cpu_percent | 14 | 0.0 | 0.0 | 0.0 | 0.0 |
| W2.controlled.pss_kib | 16 | 1024 | 2664.0 | 2664 | 2664 |
| W2.controlled.rss_kib | 16 | 4560 | 13568.0 | 13568 | 13568 |
| W2.controlled.fds | 16 | 4 | 12.0 | 12 | 12 |
| W2.controlled.threads | 16 | 1 | 3.0 | 3 | 3 |
| W2.monitor.cpu_percent | 16 | 1.9419 | 2.9137 | 3.8858 | 3.8858 |
| W2.monitor.pss_kib | 17 | 14666 | 14674.0 | 14684 | 14684 |
| W2.monitor.rss_kib | 17 | 25540 | 25564.0 | 25576 | 25576 |
| W2.monitor.fds | 17 | 5 | 5.0 | 5 | 5 |
| W2.monitor.threads | 17 | 1 | 1.0 | 1 | 1 |

## Comparação entre rodadas

Use --compare rodada-anterior.json no mesmo build/configuração/hardware.

Ambiente e amostras brutas: JSON adjacente. Valores ausentes são null, nunca zero.
