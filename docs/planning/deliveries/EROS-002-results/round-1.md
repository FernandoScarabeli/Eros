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

- W0: blocked; duração 0.021 s; amostras 0; cleanup 0 sobreviventes.
  Isolated service exited (1); Unix socket/PTY permissions required
- W1: blocked; duração 0.021 s; amostras 0; cleanup 0 sobreviventes.
  Isolated service exited (1); Unix socket/PTY permissions required
- W2: blocked; duração 0.021 s; amostras 0; cleanup 0 sobreviventes.
  Isolated service exited (1); Unix socket/PTY permissions required
- W3: blocked; duração 0.021 s; amostras 0; cleanup 0 sobreviventes.
  Isolated service exited (1); Unix socket/PTY permissions required

Calibração sem aplicativo: 5 amostras; CPU da coleta 0.004043 s; parede da coleta 0.004060 s em 5.005 s. Inclui monitor apenas; não é desempenho Eros.

## Resumo medido

| Métrica | n | min | mediana | p95 | max |
| --- | --- | --- | --- | --- | --- |

## Comparação entre rodadas

Use --compare rodada-anterior.json no mesmo build/configuração/hardware.

Ambiente e amostras brutas: JSON adjacente. Valores ausentes são null, nunca zero.
