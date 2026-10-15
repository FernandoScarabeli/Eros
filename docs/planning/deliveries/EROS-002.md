# EROS-002 — Baseline e ferramenta de medição de desempenho

Atualização vigente: [Revisão 2 — correções e bloqueio de execução](#revisão-2--correções-do-medidor-e-bloqueio-de-execução). A entrega inicial abaixo e seus artefatos foram preservados como histórico.

**Status: Em revisão, com escopo incompleto e baseline bloqueado. Não apta a aceite.**
Revisão posterior disponível em [reviews/EROS-002.md](../reviews/EROS-002.md):
W0 curto funcionou fora do sandbox; W1/W2 tiveram timeout nativo e W3 excedeu
o limite IPC com 10.000 chats sintéticos. As tentativas bloqueadas descritas
abaixo permanecem como registro da implementação inicial; não são o diagnóstico
mais recente nem baseline completo. Correções do medidor e medições pendentes.

A pedido do usuário, a linha vai para Em revisão para inspeção do trabalho
preservado. Isso não atende ainda à Definition of Done: faltam duas rodadas
nativas bem-sucedidas e os cenários/métricas listados abaixo. EROS-001 permanece
Aceita; dependentes de EROS-002 continuam sem liberação.

## Resultado e implementação

Harness opt-in, sem dependências novas, com fixture sintética descartável,
monitor Linux, driver da UI Tauri/WebKit e relatórios JSON/Markdown. Nenhuma
inferência, autenticação real, leitura de transcritos ou projeto pessoal. Não
houve otimização/correção do runtime, alteração de contratos persistidos ou
migração. Alterações preexistentes de EROS-001/documentação foram preservadas.

Arquivos desta tarefa:

- `scripts/performance/run.py`: CLI, metadados/hashes, W0–W6, relatório e comparação.
- `scripts/performance/metrics.py`: CPU por core, PSS/RSS, FDs/threads, classificação e percentis.
- `scripts/performance/fixture.py`: HOME/PATH/dados/runtime isolados, IPC limitado, SQLite sintético, subreaper Linux, identidade PID+starttime e cleanup.
- `scripts/performance/producer.py`: produtor determinístico com limites de volume, taxa e deadline independente; contador agregado fora do PTY.
- `scripts/performance/failures.py`: falhas IPC limitadas e reinício somente da fixture.
- `scripts/performance/test_performance.py`: dez testes de cálculo, classificação, ausência de métricas, crescimento, produtor, falha, adoção de órfão e SIGTERM.
- `scripts/performance/README.md`: preparação, comandos, métodos, opções e limites.
- `apps/desktop/src/lib/benchmark.ts`: driver sintético da UI real e sondas agregadas.
- `apps/desktop/src/main.ts`: import condicionado a `VITE_EROS_BENCHMARK=1`.
- `apps/desktop/src/App.svelte`: atributo de prontidão somente com benchmark habilitado.
- `apps/desktop/src/lib/TerminalPane.svelte`: contagem opt-in de bytes/fila e marcadores sintéticos após callback xterm + dois rAF.
- `apps/desktop/src-tauri/src/lib.rs`: dois comandos de instrumentação, desativados por padrão; relatório limitado a 64 KiB no runtime indicado pela fixture.
- Esta entrega, resultados adjacentes e linha/resumo de `docs/planning/KANBAN.md`.

O build normal não contém o chunk do driver; verificado em `dist/assets`. Não
há monitor/timer de benchmark no fluxo normal. A instrumentação não adiciona
telemetria ou exportação de conteúdo. O custo da instrumentação UI ainda precisa
de medição pareada; não foi declarado desprezível.

## Verificação realmente executada

Comandos a partir da raiz, exceto o build Tauri indicado:

| Comando | Resultado nesta sessão |
| --- | --- |
| `python3 scripts/check_repository.py` | Passou; links/documentos verificados |
| `python3 -m unittest discover -s scripts/performance -p 'test_*.py' -v` | 10 testes passaram; 1,783 s na última execução |
| `cargo test --workspace --locked` | Passou: 14 testes, além dos alvos/doc-tests sem testes |
| `cargo run --locked -p eros-core --bin contracts -- --check` | Passou, sem regeneração |
| `npm --prefix apps/desktop run check` | Zero erros, nove warnings em três arquivos; avisos preexistentes |
| `npm --prefix apps/desktop run build` | Passou, build normal sem chunk benchmark; warning de chunk >500 kB |
| Em `apps/desktop`: `npm run tauri -- build --no-bundle -- --locked` | Release direto compilou; primeiro build 1m27s; nenhum pacote instalado |
| Em `apps/desktop`: `VITE_EROS_BENCHMARK=1 npm run tauri -- build --no-bundle -- --locked` | Release instrumentado compilou; último build 51,79 s |
| `python3 scripts/check_chat_activity.py` | Falhou ao iniciar serviço controlado; socket Unix bloqueado pelo ambiente |
| `python3 scripts/performance/run.py --output docs/planning/deliveries/EROS-002-results/round-1.json` | Código 2; W0–W3 bloqueados antes de iniciar serviço |
| `python3 scripts/performance/run.py --output docs/planning/deliveries/EROS-002-results/round-2.json --compare docs/planning/deliveries/EROS-002-results/round-1.json` | Código 2; mesma configuração/binário/ambiente, mesmos bloqueios |
| `python3 scripts/performance/run.py --workloads W4 --output /tmp/eros002-w4.json` | Código 2: 50.000 arquivos gerados; adapter indisponível explicitamente; cleanup sem sobreviventes |
| Release UI lançado diretamente com `Fixture.spawn`, sem serviço pessoal | Encerrou com código 101; cleanup sem sobreviventes |
| Harness interrompido com SIGTERM após 1 s | Código 130 e relatório `interrupted`; aconteceu na calibração, antes de iniciar fixture Eros |

O teste de SIGTERM **com fixture viva** foi executado nos testes automáticos,
com filho real `/bin/sleep`. Outro teste valida descendente em sessão própria
adotado pelo subreaper após a raiz já ter saído; não ficou processo vivo ou
zumbi desse teste. Isso valida o lifecycle da ferramenta, mas ainda falta
interromper uma rodada Eros nativa durante W2/W6. Não confundir as duas provas.

`npm ci` não foi repetido: nenhuma dependência/manifest/lock foi alterada nesta
tarefa e os pacotes já estavam instalados. Smokes browser não foram executados:
o ambiente não permite listeners; Chromium com bridge simulada tampouco
comprovaria WebKit. A validação gráfica do driver novo permanece pendente.

## Ambiente, metodologia e resultados

Commit observado: `8cdb0172a2962e173fd367740e05d38f6a2f9cbc`, branch `main`,
checkout sujo com alterações anteriores e desta tarefa. JSON registra lista de
alterações, hash do diff, hash de cada fonte da ferramenta/instrumentação e do
binário, sem salvar o conteúdo do diff. Binário das duas tentativas:
`5feb0941a3bee791f8b6e1edaca1f93fca824d4b382ca21bc436a3407e5e86bf`.

- Intel Core Ultra 9 285K; 24 CPUs lógicas; MemTotal 32.024.312 KiB.
- Ubuntu 26.04.1 LTS, kernel 7.0.0-38-generic; **fora do alvo Ubuntu 24.04**.
- WebKitGTK 2.52.6; Rust 1.90.0; Cargo 1.90.0; Node 22.22.1; npm 9.2.0; Python 3.14.4.
- Tauri CLI 2.12.2/API 2.12.3; xterm 6.0.0; versões instaladas adicionais no JSON.
- Release executado diretamente, sem instalação de `.deb` ou medição de pacote.

O sandbox retorna `PermissionError: [Errno 1] Operation not permitted` ao criar
socket Unix. O serviço da fixture encerra com código 1. GTK também não inicializa
nesta sessão; a UI release encerra com 101. Não houve tentativa de contornar o
isolamento ou encerrar serviços pessoais.

As duas **tentativas** são comparáveis, mas **não são duas rodadas reais de
baseline**. W0–W3 não chegaram ao caminho ponta a ponta. Todas as métricas do
Eros permanecem não medidas; nenhuma meta de `PERFORMANCE.md` foi alterada ou
marcada atingida. Evidências:

- [Tentativa 1 JSON](EROS-002-results/round-1.json), [relatório](EROS-002-results/round-1.md).
- [Tentativa 2 JSON](EROS-002-results/round-2.json), [comparação](EROS-002-results/round-2.md).
- [Interrupção durante calibração JSON](EROS-002-results/interrupted.json), [relatório](EROS-002-results/interrupted.md).

Única medição de desempenho obtida: **overhead do monitor sem aplicativo**,
cinco amostras a aproximadamente 1 Hz por tentativa:

| Medida | Tentativa 1 | Tentativa 2 |
| --- | --- | --- |
| Janela de calibração | 5,004685 s | 5,004713 s |
| CPU acumulada dentro das coletas | 0,004043 s | 0,004077 s |
| Custo de CPU da coleta / janela | 0,0808% de um core | 0,0815% de um core |
| Parede acumulada dentro das coletas | 0,004060 s | 0,004098 s |
| PSS do monitor | 14.179–14.207 KiB | 14.170–14.198 KiB |
| RSS do monitor | 24.912–24.916 KiB | 24.888–24.892 KiB |

Cinco amostras são triagem. O custo acima cobre a coleta `/proc`; o monitor
completo também executa IPC, leitura de arquivos e geração do relatório. Estes
custos são separados do Eros quando houver processos vivos; a calibração não
mede o overhead da sonda xterm nem o de um WebKit sob pressão.

CPU usa `Δ(utime+stime)/SC_CLK_TCK/Δmonotonic × 100`, sem dividir pelos 24 cores.
PSS/RSS, FDs reais e threads são amostrados no máximo uma vez por segundo.
Classificação parte das raízes possuídas; WebKit pertence à UI, descendentes do
serviço são shells/utilitários controlados, monitor fica separado. Processos
curtos entre coletas podem não ser contabilizados; não é perfil CPU completo.
Métrica inacessível é `null`; zero observado de CPU pode decorrer da resolução
em ticks do kernel, não prova consumo nulo.

p95: nearest rank `ceil(0,95*n)`, com n, mínimo, mediana e máximo. Startup usa
relógio epoch da UI correlacionado com epoch/monotonic do monitor; offsets antes
e depois permitem detectar salto, que exige descartar a rodada. Prontidão usa
estado da aplicação e dois rAF. Echo aproxima callback de parsing xterm + dois
rAF, **não retorno IPC nem comprovação de pixels**. Não há limite medido para o
erro de composição/scheduling; dois rAF normalmente acrescentam até dois
períodos de refresh. Bytes PTY incluem prompts/ANSI e transformação LF→CRLF;
produção é contada separadamente. Fila xterm observável não mede backlog interno
do kernel/IPC/WebKit. Método completo no [README da ferramenta](../../../scripts/performance/README.md).
Referências: [Linux proc](https://www.kernel.org/doc/html/latest/filesystems/proc.html)
e [xterm flow control](https://xtermjs.org/docs/guides/flowcontrol/).

Formato JSON v1: `environment`, `monitorCalibration`, `workloads`, `summary`,
`status`; cada workload tem status, duração, amostras e cleanup. Amostras contêm
fase, monotonic e quatro grupos com processos/CPU/PSS/RSS/FDs/threads. Valores
não medidos são null; distribuições sem observação têm n=0 e estatísticas null.
O Markdown coloca metas propostas ao lado dos resultados/limitações e compara
p95 somente quando binário, fontes, configuração e ambiente coincidem.

## Gargalos, cobertura e pendências

Nenhum gargalo de desempenho nativo foi quantificado. O bloqueio observado é de
execução no ambiente, não evidência de regressão do produto. A fila de escrita
xterm sem controle ponta a ponta e o subprocesso de status dentro de snapshot
continuam hipóteses sustentadas pela leitura do código, não conclusões destas
rodadas. Relacionam-se a EROS-003/006/016; lifecycle/renderização e catálogo a
EROS-005/007/023; validação final a EROS-038.

| Cenário | Ferramenta disponível | Aceitação ainda pendente |
| --- | --- | --- |
| W0 | Startup frio/quente com base vazia/100 projetos; shell e janela idle; 10/20 amostras configuradas | Execução nativa; off/on de animações; minimização; pacote instalado; correlação de clocks validada |
| W1 | Echo aproximado com 1/8 shells; ANSI/Unicode/tela alternativa; resize IPC; 100 ciclos de views | WebKit ponta a ponta; resize gráfico/TUI inspecionado; troca entre oito chats pronta para entrada |
| W2 | Produtores limitados 1 MiB/s e 8×256 KiB/s por 60 s; contadores de produção e entrega; fila xterm e echo sob carga | Execução nativa; Ctrl+C até produtor parado/entrada recuperada; desconexões, atraso de transporte e filas internas não medidos |
| W3 | Fixture persistida 100 projetos/10.000 chats parados; busca e 100 mudanças reais de escopo | Execução nativa; ordenação e 100 trocas de chat; delta PSS entre cenários comparáveis |
| W4 opcional | Corpus 50.000 arquivos temporários | Arquivos/Git indisponíveis; corpus de diffs/binários/conflitos/renames ainda não completo; não implementados adapters fora de escopo |
| W5 opcional | Frame parcial, operação inválida, reinício da fixture | Execução nativa; subscriber lento, provedor travado, arquivo removido, disco sem escrita, cliente incompatível ainda indisponíveis |
| W6 opcional | Janela sintética de 30 minutos, ciclos e estabilização final | Não executado; crescimento sustentado ainda não verificado |

Assinaturas Rust, GPU e filas internas permanecem não medidas, nunca zero.
FDs/threads não substituem essas métricas. O aceite exige completar as lacunas
viáveis do driver **e** executar duas rodadas no mesmo Ubuntu gráfico compatível.
Não basta apenas repetir o script atual para considerar todos os critérios
atendidos. Esta entrega não libera cartões dependentes.

## How to test

1. Na raiz do repositório, preparar o ambiente conforme `docs/development.md`;
   usar Ubuntu gráfico com Unix sockets/PTYs autorizados. Nenhum login é necessário.

   ```bash
   npm --prefix apps/desktop ci
   (cd apps/desktop && VITE_EROS_BENCHMARK=1 npm run tauri -- build --no-bundle -- --locked)
   python3 -m unittest discover -s scripts/performance -p 'test_*.py' -v
   ```

   Esperado: release instrumentado e dez testes passando. O harness cria/removerá
   HOME, projetos, SQLite e runtime próprios; não selecionar pasta pessoal.

2. Rodar duas vezes o mesmo binário/configuração sem rebuild intermediário:

   ```bash
   python3 scripts/performance/run.py --output /tmp/eros-round-1.json
   python3 scripts/performance/run.py --output /tmp/eros-round-2.json --compare /tmp/eros-round-1.json
   ```

   Esperado em ambiente apto: janelas nativas automáticas, workloads executados,
   JSON/Markdown adjacentes, custos separados e cleanup sem sobreviventes. Ainda
   revisar as métricas pendentes acima; saída 0 não certifica metas. Nesta sessão
   ambos retornam 2, status incomplete, W0–W3 blocked e métricas não medidas.

3. Falhas e opções, sem mudar o runtime do produto:

   ```bash
   python3 scripts/performance/run.py --workloads W4 --output /tmp/eros-w4.json
   python3 scripts/performance/run.py --workloads W5 --output /tmp/eros-w5.json
   python3 scripts/performance/run.py --workloads W6 --output /tmp/eros-w6.json
   ```

   W4 deve gerar/remover corpus e reportar adapter unavailable, código 2. W5
   deve registrar os casos suportados e as falhas indisponíveis, código 2
   (partial); neste sandbox será blocked. W6 é opt-in e dura pelo menos 30 min;
   não executar para cada edição. Não houve W6 real nesta entrega.

4. Interromper no ambiente gráfico apto, quando a fixture estiver rodando:

   ```bash
   python3 scripts/performance/run.py --workloads W6 --output /tmp/eros-interrupted.json &
   round_pid=$!
   sleep 10
   kill -TERM "$round_pid"
   wait "$round_pid"
   ```

   Esperado: código 130, status interrupted, fixture com live_survivors=0 e
   relatórios preservados. Ctrl+C segue o mesmo finally. No sandbox bloqueado,
   a rodada pode já ter encerrado antes de 10 s; use os testes automatizados de
   SIGTERM/falha para a prova executável de cleanup, sem afirmar medição Eros.

5. Regressão e restauração do build normal:

   ```bash
   cargo test --workspace --locked
   cargo run --locked -p eros-core --bin contracts -- --check
   npm --prefix apps/desktop run check
   (cd apps/desktop && npm run tauri -- build --no-bundle -- --locked)
   python3 scripts/check_repository.py
   ```

   Esperado: checks pertinentes passam e build normal não importa o driver.
   Em ambiente nativo apto, repetir `python3 scripts/check_chat_activity.py` para
   regressão PTY/IPC; neste ambiente sua inicialização está bloqueada.

6. Limpeza: o harness encerra apenas seus filhos/descendentes adotados,
   confere PID+starttime, recolhe processos e remove apenas o mkdtemp criado.
   Sobrevivente detectado provoca erro e retenção da fixture para inspeção.
   SIGKILL/queda do sistema não permitem finally e não são certificados.
   Não usar killall/pkill Eros nem apagar diretórios por glob. Os JSON/Markdown
   em `/tmp/eros-round-*.json` e arquivos adjacentes são artefatos escolhidos
   pelo operador; removê-los individualmente quando não forem mais necessários.


## Revisão 2 — correções do medidor e bloqueio de execução

Data: 2026-10-15. **Em revisão, incompleta. Não aceita.** As correções abaixo
foram implementadas e verificadas localmente. A nova validação Tauri/WebKit,
as duas rodadas completas e a interrupção com fixture Eros viva **não puderam
ser realizadas nesta sessão**. Não chamar esta revisão de baseline completo.

### Bloqueio atual e evidências preservadas

A tentativa de usar a aprovação normal de `exec_command` com
`sandbox_permissions=require_escalated` foi rejeitada antes da execução:
`sandbox_approval: false` na política Granular desta sessão. A ação solicitada
era somente criar/remover um socket Unix temporário, necessário às fixtures.
Não foi apresentada confirmação ao usuário pela ferramenta. Não houve contorno
por outro prefixo, mudança de salvaguardas ou execução de serviços pessoais.

O teste dentro do sandbox reproduziu `PermissionError: [Errno 1] Operation not
permitted` no bind Unix. A triagem atual identifica essa indisponibilidade
antes de lançar o serviço. Isso difere da revisão histórica, que conseguiu
execução nativa aprovada. O fato histórico não concede acesso técnico nesta
sessão e não prova a validação do código novo.

Evidência nova, distinta de todos os JSON históricos:

- [Triagem revisão 2 JSON](EROS-002-results/revision2-triage-sandbox.json).
- [Triagem revisão 2 Markdown](EROS-002-results/revision2-triage-sandbox.md).

Ela retornou 2: onze estágios isolados W0/variantes, W1.1/W1.8/chat-switch,
W2.1/W2.8 e W3, todos `environment_unavailable`, com `cleanup.live_survivors=0`.
Nenhuma UI/processo Eros foi lançado pelo novo harness, nenhum valor nativo de
startup/echo/CPU/PSS foi inventado e nenhuma meta mudou. A sequência de
verificação para no gate de triagem; **não foram executadas duas rodadas longas
que repetiriam o bloqueio**. A comparação nova de baseline permanece pendente.

O usuário também forneceu na conversa uma captura do aviso de encerramento de
WebKitWebProcess. A captura não contém identidade/correlação com uma fixture;
não foi atribuída ao Eros ou a esta revisão. A imagem não foi copiada para os
artefatos públicos. Nenhum relatório de crash/transcrito pessoal foi lido.

### Correções e arquivos

- `apps/desktop/src/lib/benchmark-probe.ts` (novo): busca no chunk completo mais
  sufixo, depois retenção de até 29 caracteres; tickets somente para marcadores
  armados, máximo 64. Matches inteiramente antigos no sufixo são ignorados.
  Callback atrasado verifica o ticket e a instância ativos; não confirma ticket
  rearmado, não muda timestamp já confirmado e não atua após dispose.
- `TerminalPane.svelte`: identidade imutável de sessão/painel/view para a
  instrumentação. Registro por sessão e auditoria independente por instância,
  limitada a 32 views; uma instância deslocada do índice continua auditável.
  Cleanup condicional não remove um substituto. `$effect` registra props que
  mudaram sem remount. **Não alterou assinatura/onMount/roteamento de entrada do
  produto, nem adicionou key ao LayoutTree.**
- `benchmark.ts`: passos/condições nomeados, esperado/observado/pronto, sessões
  no DOM, instâncias e alerta sintético sanitizado. Relatórios serializados
  para evitar disputa pelo mesmo arquivo temporário. Falha preserva medições
  parciais e separa driver_failure de product_failure baseado em evidência de
  identidade. Bash recebe printf dividido: comando ecoado não contém o marcador
  completo. `stty -echo && printf ...` é handshake observável, sem sleep fixo.
- `run.py`: fixtures independentes para W1.1/W1.8 e W2.1/W2.8; falhar em oito não
  apaga um ou impede W2.1. Payload UI parcial é preservado no JSON/Markdown.
  Resultados nativos negativos podem ser `complete_with_product_failures`;
  driver/ambiente ainda geram incomplete. v2 é separado dos artefatos v1.
- `fixture.py`: prontidão por `set_active_scope`, separada de snapshot. Frame
  vazio, incompleto e acima de limite registram operação/tamanho/limite.
  Fixture W3 mantém 100/10.000 registros e calcula o tamanho serializado do
  array como diagnóstico inferior ao snapshot completo. EOF com controle vivo
  e tamanho maior que MAX_FRAME é catalog_capacity/product_failure, EROS-005.
  MAX_FRAME, persistência e protocolo do produto não foram alterados.
- `clocks.py` (novo), `metrics.py`, `run.py`: correlação injetável, bracket de até
  1 ms, tolerância de offset 5 ms, rejeição de salto/NaN/inf/tempo negativo;
  valores inválidos não entram em p95. Registro de raw delta e motivo permite
  auditoria. Comparação de startup afetada é suspensa, mantendo comparações de
  métricas monotônicas independentes. Owners observados de WebKit são retidos
  por PID+starttime enquanto vivos; cache de processos mortos é descartado.
- `producer.py`: primeiro payload antes do marcador de início, progresso/estado
  agregado fora do PTY, deadline/volume limitados, finally e contador também em
  SIGINT. W2 distribui 20 echoes na janela, confirmando carga ativa antes de cada
  um; offsets reais são guardados. Interrupção usa foreground, 0x03, confirmação
  de finally e comando subsequente que prova recuperação da entrada do shell.
- `benchmark.ts`, `HomeScreen.svelte`, `TextType.svelte`, `TerminalPane.svelte`:
  variantes **somente de benchmark** de motion off/on; off suprime WebGL Home,
  CSS motion, TextType timer e cursor blink. Não criou configuração de produto.
  Home off/on tem mesmo catálogo e shell retido; terminal visível é separado.
- `src-tauri/src/lib.rs`: leitura opt-in limitada dos estados dos produtores e
  minimização/restauração com confirmação nativa, worker bloqueante limitado a
  um segundo. Comandos retornam desabilitado sem flag benchmark.
- `fixture.py`, `run.py`: conexões SQLite explicitamente fechadas; cleanup também
  recolhe descendentes criados por handlers TERM, espera adoção/reaping com
  limite e protege sua finalização contra segunda interrupção. Erro de cleanup
  fica explícito, sem afirmar zero sobreviventes desconhecidos.
- Testes novos em `apps/desktop/scripts/benchmark-probe.test.mjs` e
  `scripts/performance/test_revision.py`, além do caso novo de lifecycle em
  `test_performance.py`; README do medidor atualizado. Nenhum manifesto,
  lockfile, migração, configuração global ou fonte do runtime do serviço foi
  alterado nesta revisão. Alterações anteriores foram preservadas.

### Métodos implementados, ainda dependentes de validação nativa

**Ctrl+C:** vinte amostras por estágio W2.1. Primeiro payload e início do produtor
foreground confirmados; enviar Ctrl+C, observar finally, depois enviar comando
no shell e confirmar a saída. O fim da produção mais recuperação da entrada
comprova mais que retorno IPC. A latência usa apenas performance.now e inclui
as esperas seriais por parsing/dois rAF; não mede o instante físico do SIGINT ou
pixels. O teste Python confirma lifecycle do produtor por SIGINT real; não é
prova de teclado/PTY/WebKit integrado.

**Troca de chats:** fixture-only CLI double temporário executa bash real. O tipo
metadata Codex permite usar a lista/fluxo de chat existentes; nenhum CLI real é
invocado. O driver troca cem vezes o workspace pelo botão de chat, aguarda
assinatura do alvo e confirma entrada por comando. Não equivale a oito providers
reais. Essa fixture nova ainda não foi exercitada ponta a ponta.

**Idle:** terminal vazio/100 projetos, Home off/on equivalentes e janela
minimizada confirmada pelo window manager. Em cada janela longa: 30 s de
estabilização + 60 s de amostras. Não comparar diretamente Home e terminal como
se fossem a mesma superfície. A variante off é controle de benchmark, não uma
funcionalidade econômica entregue ao usuário.

**PSS:** catálogo pequeno/grande registra serviço isolado em fases equivalentes
sem UI. Se snapshot grande falhar, delta UI+WebKit+serviço fica indisponível,
com motivo: a superfície grande não consegue montar. O delta service-only não
substitui a meta de 75 MiB. A comparação Home off/on pode usar as mesmas fases
idle/estabilização e o mesmo catálogo, a partir das amostras JSON.

**Clocks:** bracket máximo de 1 ms dá incerteza de até 0,5 ms por correlação; o
limite de offset 5 ms supera arredondamento nominal de Date.now/bracket e é
pequeno frente a startup de 1,5/3 s. UI verifica seu epoch performance contra
Date.now com tolerância igual. Isto é política de validade, não certificação da
precisão WebKit: granulação efetiva/composição permanecem pendentes. Saltos
muito breves entre observações podem escapar. Echo/Ctrl+C usam relógio local
monotônico e não são invalidados por uma diferença epoch sem relação causal.

**Overhead UI:** contadores scanWallMs/callbackWallMs por chunk delimitam apenas
trechos JS observados de parede, sujeitos à quantização. Não são CPU por função
nem incluem todo o custo do driver/IPC/rAF/compositor. **Overhead total nativo
pareado ainda não medido; não foi declarado zero nem limitado por esses
contadores parciais.** A única medição nova disponível é a calibração do monitor,
registrada no JSON e resumida abaixo. Nenhuma medição do Eros foi produzida.

### Verificação nova e resultados reais

| Comando | Resultado |
| --- | --- |
| `python3 -W error::ResourceWarning -m unittest discover -s scripts/performance -p 'test_*.py' -v` | 25 testes passaram; última execução 2,285 s, sem ResourceWarning |
| `node apps/desktop/scripts/benchmark-probe.test.mjs` | Dez testes passaram, incluindo chunk 32 KiB em três posições, todas as fronteiras, múltiplos marcadores, callbacks atrasados, echo de comando, ticket rearmado, budgets e instância substituta |
| `cargo test --workspace --locked` | 14 testes passaram, além de alvos/doc-tests sem testes |
| `cargo run --locked -p eros-core --bin contracts -- --check` | Passou, sem regenerar contratos |
| `npm --prefix apps/desktop run check` | Zero erros e nove warnings preexistentes em três arquivos |
| Em `apps/desktop`: `VITE_EROS_BENCHMARK=1 npm run tauri -- build --no-bundle -- --locked` | Release instrumentado compilou; último build 53,18 s |
| `npm --prefix apps/desktop run build` | Build normal passou; warning de chunk >500 kB, sem chunk do driver |
| `python3 scripts/check_repository.py` | Passou; links/documentos locais válidos |
| `python3 scripts/check_chat_activity.py` | Não passou: inicialização do serviço controlado bloqueada por socket; não é regressão nativa comprovada |
| Triagem curta completa, comando abaixo | Código 2; 11 estágios environment_unavailable; zero sobreviventes; nenhum processo Eros iniciado |
| Aprovação normal de execução fora do sandbox | Rejeitada pela política da sessão antes da execução; sandbox_approval=false |

Os testes Python novos validam cálculo de correlação, saltos nos dois sentidos,
bracket impreciso, latência negativa/não finita, filtragem de amostra inválida,
comparação de startup suspensa preservando echo/CPU, limite exato do frame com
newline, EOF/frame parcial, liveness independente, payload parcial e fixture
100/10.000. Casos de cleanup incluem SIGTERM com filho vivo, filho com sessão
própria, filho adotado após saída da raiz e filho criado no próprio handler TERM.

Não houve `npm ci` nesta revisão: dependências/manifests não mudaram e os pacotes
já estavam instalados. Smokes de browser não foram repetidos por falta de
listeners; tampouco seriam prova nativa. Não houve nova inspeção gráfica,
interrupção Eros viva, W6 real ou duas rodadas completas. A integridade/cleanup
da ferramenta foi testada; os métodos UI novos continuam sem aceite integrado.

Binário da triagem: `d24df1980cc38580f2e83486ed014a10a2fac283e263e056d54d04b41d7307d1`.
Metadados completos e hashes de fontes estão no JSON. É release direto, não
pacote instalado. Mesmo hardware/ambiente observado: Core Ultra 9 285K, 24 CPUs
lógicas, Ubuntu 26.04.1/kernel 7.0.0-38-generic, WebKitGTK 2.52.6; fora do alvo
Ubuntu 24.04. Não comparar números desta triagem com a triagem histórica como
se fossem duas rodadas do mesmo build/configuração.


Calibração nova do monitor: 5.004390 s, cinco amostras; CPU de coleta 0.003710 s (0.0741% de um core), parede de coleta 0.003727 s. Somente ferramenta sem aplicativo; não é baseline do Eros.

### Achados do produto e reprodução

1. **Catálogo/IPC — EROS-005, confirmado na revisão nativa histórica.** Seu
   relatório registrou array de conversas de 2.147.891 bytes, limite 2.097.152,
   snapshot encerrado com zero bytes e controle ainda responsivo. Para reproduzir
   com o código atual em sessão apta: executar W3, manter 100/10.000, inspecionar
   `catalog`, `ipc`, `liveness_after_snapshot`, `classification` e fases de PSS.
   Esperado, se persistir: product_failure/catalog_capacity, sem aumentar limite
   nem diminuir a fixture. O teste isolado novo confirma preservação/tamanho dos
   dados, mas não substitui nova reprodução nativa. Não houve paginação.
2. **Reutilização de TerminalPane — EROS-005/006, hipótese ainda não confirmada
   nativamente nesta revisão.** W1.8/W2.8 criam painéis pelo fluxo real; no terceiro,
   a estrutura recursiva pode manter uma leaf com outra conversa. Confirmar
   `diagnostic.probes[].bindingMismatch`, `view`/`panel`/`session` imutáveis e
   `renderedSessions`. A auditoria independente evita perder essa instância
   quando outra view substitui seu índice por sessão. Não declarar esse defeito
   comprovado somente porque houve timeout na revisão antiga. LayoutTree e o
   lifecycle geral permanecem para seus cartões, não foram corrigidos aqui.
3. **Timeouts W1/W2 antigos:** os marcadores descartados e cleanup do registro
   eram defeitos do medidor e têm testes de regressão agora. Ainda não foi
   comprovado que todas as falhas nativas antigas desaparecem. Se surgir
   driver_failure, revisar failedPhase/condition/diagnostic e preservar os
   resultados do estágio independente que funcionou.

Nenhum gargalo nativo novo foi quantificado. Metas permanecem propostas; falhas
confirmadas devem ser resultados negativos, nunca uma meta relaxada. EROS-003,
EROS-005 e EROS-006 não foram iniciadas nem tiveram seus fontes alterados.

### How to test — revisão 2

Da raiz, em sessão Ubuntu gráfica que **autorize** execução nativa controlada:

```bash
npm --prefix apps/desktop ci
(cd apps/desktop && VITE_EROS_BENCHMARK=1 npm run tauri -- build --no-bundle -- --locked)
python3 -W error::ResourceWarning -m unittest discover -s scripts/performance -p 'test_*.py' -v
node apps/desktop/scripts/benchmark-probe.test.mjs
python3 scripts/performance/run.py --workloads W0,W1,W2,W3 --cold-samples 1 --warm-samples 1 --idle-seconds 5 --pressure-seconds 1 --cycles 1 --output /tmp/eros-revision2-triage.json
```

Esperado: 25 + dez testes passam; triagem nativa com fases/condições e fixtures
independentes. Falha de oito não elimina dados de um. Em W3, uma falha real de
capacidade é resultado válido, com controle vivo/tamanho/limite, e não sucesso.
Se o ambiente estiver indisponível ou o driver ainda falhar, parar o gate e
corrigir/verificar antes das duas rodadas longas. Não executar scripts via
mecanismo alternativo para evitar a política de aprovação.

Depois de triagem confiável, mesmo binário/configuração/hardware, sem rebuild:

```bash
python3 scripts/performance/run.py --output /tmp/eros-revision2-baseline-1.json
python3 scripts/performance/run.py --output /tmp/eros-revision2-baseline-2.json --compare /tmp/eros-revision2-baseline-1.json
```

Esperado: parâmetros completos do README/PERFORMANCE, min/mediana/p95/max/n,
status negativos preservados, clocks e limites explícitos. Comparação de startup
inválida não deve produzir variação de p95; métricas monotônicas independentes
podem continuar comparadas. `complete_with_product_failures` ainda retorna 2;
não é meta aprovada. Essa dupla **não foi executada** aqui.

Interrupção, somente após confirmar fixture real viva em long-duration:

```bash
python3 scripts/performance/run.py --workloads W6 --output /tmp/eros-revision2-interrupted.json &
round_pid=$!
sleep 10
kill -TERM "$round_pid"
wait "$round_pid"
```

Esperado: 130, interrupted e live_survivors=0. Se a rodada já saiu antes de 10 s,
não há teste de Eros vivo. SIGINT/Ctrl+C usa o mesmo cleanup. O foreground Ctrl+C
interno de W2 é medição diferente da interrupção do monitor. No cenário negativo
W3 ou em timeout de UI, a fixture deve ser recolhida e o payload parcial salvo.
Não usar killall/pkill nem apagar diretórios pessoais; o harness remove somente
seu mkdtemp. Em cleanup_failure, inspeção do diretório retido é necessária e o
resultado não pode anunciar zero sobreviventes. SIGKILL/queda do host não
permitem finally e não são certificados.

Regressão/restauração:

```bash
cargo test --workspace --locked
cargo run --locked -p eros-core --bin contracts -- --check
npm --prefix apps/desktop run check
(cd apps/desktop && npm run tauri -- build --no-bundle -- --locked)
python3 scripts/check_repository.py
```

Para concluir o cartão ainda falta uma sessão com execução nativa aprovada,
confirmar/refutar a hipótese de identidade, validar os novos métodos ponta a
ponta, medir/delimitar o overhead total UI, duas rodadas completas comparáveis e
interrupção Eros viva. **Status final continua Em revisão, sem aceite ou
liberação dos dependentes.**
