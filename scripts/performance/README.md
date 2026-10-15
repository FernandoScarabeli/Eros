# Medidor Eros — opt-in, versão de relatório 2

Linux, Python stdlib e dependências frontend já existentes. Executar da raiz.
Leia `docs/development.md` e a revisão/entrega EROS-002. Requer sessão gráfica
real, sockets Unix/PTYs e acesso a `/proc` dos próprios processos. Nenhuma conta,
login ou inferência real. Fixtures têm HOME, PATH, SQLite e runtime temporários.
Não usar um display virtual ou Chromium como prova de WebKit nativo.

## Build, testes e triagem antes de medir

```bash
npm --prefix apps/desktop ci
(cd apps/desktop && VITE_EROS_BENCHMARK=1 npm run tauri -- build --no-bundle -- --locked)
python3 -W error::ResourceWarning -m unittest discover -s scripts/performance -p 'test_*.py' -v
node apps/desktop/scripts/benchmark-probe.test.mjs
python3 scripts/performance/run.py --workloads W0,W1,W2,W3 --cold-samples 1 --warm-samples 1 --idle-seconds 5 --pressure-seconds 1 --cycles 1 --output /tmp/eros-triage.json
```

Esperado: 25 testes Python e dez testes do probe passam. O teste Node transpila o
arquivo TypeScript real com o TypeScript já instalado; não requer suporte
experimental a TS do binário Node. A triagem usa UI Tauri/WebKit real. Falhas
nativas do produto são resultados negativos; falhas do driver/ambiente impedem
certificar a rodada. **Triagem não é baseline**. Inspecione JSON/Markdown antes
de executar duas rodadas longas.

```bash
python3 scripts/performance/run.py --output /tmp/eros-baseline-1.json
python3 scripts/performance/run.py --output /tmp/eros-baseline-2.json --compare /tmp/eros-baseline-1.json
```

Não rebuildar entre rodadas. Defaults: 10 startups frios/20 quentes por catálogo,
90 segundos idle (30 estabilização + 60 coleta), produtores 60 s, 100 ciclos de
views, 100 trocas de chats. O release é executado diretamente: não é pacote
instalado. O monitor só existe quando solicitado por esses comandos.

Saída 0 = execução completa, sem certificação das metas. Saída 2 = resultados
negativos/incompletos/indisponíveis; `complete_with_product_failures` distingue
coleta concluída com falha do produto de `incomplete` por driver/ambiente. Saída
130 = interrupção. Verificar também status/classification de cada estágio.

## Estágios independentes

- W0: startup vazio/100 projetos; idle com um terminal e 100 projetos. Fixtures
  extras: vazio com um terminal, Home off/on equivalente com um shell retido,
  terminal minimizado com confirmação de estado pelo window manager.
- W1.1 e W1.8: fixtures independentes com 1/8 shells, 20 echoes, Unicode/ANSI,
  tela alternativa, resize por IPC e até 100 ciclos abrir/fechar.
- W1.chat-switch: oito shells reais, apresentados pelo fluxo existente de chats
  via CLI double **somente na fixture**. A metadata usa kind Codex para entrar na
  lista de chats; o executável temporário é bash, sem provedor/autenticação.
  Troca workspace real pelo botão do chat, espera assinatura e comprova entrada
  por saída de comando. Não mede CLI Codex/Claude real.
- W2.1 e W2.8: fixtures separadas; 1 MiB/s ou 8×256 KiB/s por 60 s. Marcador de
  início somente depois do primeiro payload; confirmar estado running/progresso
  fora do PTY antes de cada echo. Vinte amostras distribuídas na janela de carga,
  offsets reais registrados (duas na triagem curta). A etapa de um terminal
  também executa 20 interrupções de produtor foreground.
- W3: 100 projetos/10.000 chats persistidos, sem 10.000 processos. Liveness é
  comando pequeno. Snapshot/limite/tamanho do array serializado são observações
  separadas. EOF com controle vivo é falha de entrega/capacidade do catálogo,
  relacionada a EROS-005; não reduzir fixture nem aumentar MAX_FRAME. Busca e
  navegação só podem rodar se o snapshot for entregue. Ordenação por critério não
  existe; organização manual não equivale a sort. Comparação PSS service-only
  permanece separada da meta UI+serviço, que pode ficar indisponível.

Adicionar o terceiro painel é a reprodução planejada da hipótese de reuso de
TerminalPane. O medidor registra identidade imutável da sessão/painel/view,
identidades atuais do DOM e todas as instâncias vivas, inclusive uma assinatura
substituída no índice por sessão. Divergência de props sem remount é reportada
como product_failure com EROS-005/006. **Confirmar no relatório nativo**; leitura
de código e teste de registro não são confirmação integrada desse defeito.
LayoutTree, assinatura do produto e lifecycle geral não foram corrigidos aqui.

## Métodos e validade

CPU = delta(utime+stime)/SC_CLK_TCK/delta(monotonic) × 100; um core = 100%.
PSS/RSS, FDs reais e threads: `/proc`, no máximo 1 Hz. Classificação por raízes
possuídas, nunca nome/argv globais; identidade PID+starttime e proprietário
retido para auxiliares já observados/reparentados. Processos efêmeros entre
coletas podem escapar. Ausente/inacessível é null, nunca zero. p95 nearest rank
ceil(0,95*n), com mínimo/mediana/máximo e n; NaN/inf não entram na distribuição.

Startup: epoch do monitor versus performance.timeOrigin+now da UI. Correlacionar
epoch com monotonic antes/depois e durante startup; bracket máximo 1 ms
(incerteza até 0,5 ms por correlação), tolerância de variação de offset 5 ms.
A tolerância supera arredondamento de Date.now e erro do bracket, mantendo erro
pequeno frente a startup de 1,5/3 s. UI também verifica Date.now contra seu epoch
performance (5 ms). Isso é política conservadora, **não precisão nativa já
certificada**. Saltos/brackets imprecisos/latência negativa invalidam a amostra;
raw diagnóstico é preservado e p95 usa apenas amostras válidas. Comparações de
startup são suspensas se a validade do clock da rodada falhar. CPU/PSS/echo
monotônico independente continuam disponíveis. Saltos breves entre observações
podem escapar; não declarar detecção contínua nem precisão do compositor.

Echo: marcador armado antes do envio; comando printf dividido não contém o
marcador completo, mesmo com ECHO ativo. `stty -echo && printf ...` confirma
sucesso por saída observada, sem espera fixa. Busca em chunk completo + sufixo,
depois retém até 29 caracteres; até 64 tickets pendentes e 32 instâncias de
views. Callbacks antigos não confirmam tickets novos nem removem probe
substituto. Callback de parsing xterm + dois rAF é aproximação, não pixels nem
retorno IPC. Erro de scheduling/composição e precisão efetiva do timer ainda
exigem validação nativa; metas de renderização não ficam certificadas.

Ctrl+C: produtor foreground, primeiro payload confirmado, enviar 0x03, esperar
marcador de finally e depois executar comando de recuperação no shell. A latência
monotônica inclui confirmação de parada e entrada recuperada. A espera serial
adiciona callbacks/dois rAF nas duas confirmações; não equivale ao instante físico
do SIGINT. Estado agregado do produtor deve ser interrupted, não apenas IPC ok.

Bytes produzidos vêm de contadores fora do PTY; bytes entregues incluem prompts,
ANSI e transformação LF→CRLF. Fila observável: entregues menos parsing xterm
concluído; não é backlog interno IPC/WebKit/kernel. Assinaturas Rust/GPU internas
não são medidas. FDs/threads não as substituem.

Off/on é controle **de benchmark**, sem criar configuração de produto: off
suprime CSS motion, cursor blink, WebGL Home e timers TextType; on mantém o
comportamento atual. Home off/on tem mesma superfície e shell retido. Terminal
visível e Home não são cenários intercambiáveis. Minimização precisa confirmar
estado nativo e separa a janela correspondente no relatório.

Overhead: sampler tem calibração sem app e custo de coleta separado. Probe
registra scanWallMs/callbackWallMs por chunk; são trechos JS medidos com relógio
de parede, sujeitos à quantização, não CPU por função. Não incluem todo o custo
de driver, IPC, rAF/GPU ou compositor. Esses contadores não delimitam o overhead
total; medição pareada nativa da instrumentação continua necessária. Não chamar
esse custo de zero ou desprezível sem evidência.

## Opções, falha e interrupção

```bash
python3 scripts/performance/run.py --workloads W4 --output /tmp/eros-w4.json
python3 scripts/performance/run.py --workloads W5 --output /tmp/eros-w5.json
python3 scripts/performance/run.py --workloads W6 --output /tmp/eros-w6.json
```

W4 gera 50.000 arquivos, mas adapters/casos Git ainda indisponíveis. W5 oferece
frame parcial, operação inválida e reinício da fixture; demais casos indisponíveis
explicitamente. W6 é opt-in de 30 minutos, nunca padrão. Sem inferência real.

```bash
python3 scripts/performance/run.py --workloads W6 --output /tmp/eros-interrupted.json &
round_pid=$!
# Em sessão gráfica apta: esperar estágio long-duration, com fixture viva.
sleep 10
kill -TERM "$round_pid"
wait "$round_pid"
```

Esperado: 130, interrupted, cleanup.live_survivors=0 e relatórios preservados.
Se a inicialização já falhou, isso não testa interrupção Eros viva. Os testes
Python cobrem falha, SIGTERM com filho real e descendente adotado pelo subreaper;
não substituem essa rodada nativa. Não usar killall/pkill nem remover por glob.
Só o diretório criado é removido. Sobrevivência retém fixture com erro explícito;
SIGKILL/queda do host não permitem finally e não são certificados.

Para restaurar build normal:

```bash
(cd apps/desktop && npm run tauri -- build --no-bundle -- --locked)
```

Sem flags benchmark, sem monitor/timers/produtores; comandos adicionais retornam
desabilitado. Relatórios históricos v1 não são sobrescritos. Referências:
[Linux proc](https://www.kernel.org/doc/html/latest/filesystems/proc.html) e
[xterm flow control](https://xtermjs.org/docs/guides/flowcontrol/).
