# EROS-002 — revisão e spec de revisão da entrega

Data: **2026-10-15**. Resultado: **não aceita**. Implementação preservada, sem liberação dos cartões dependentes. Este documento complementa o [cartão](../tasks/EROS-002.md), o [contrato de execução](../AGENT_GUIDE.md) e a [entrega inicial](../deliveries/EROS-002.md).

## O que foi verificado nesta revisão

- Os dez testes Python passaram novamente em 1,783 s.
- O release usado coincide por SHA-256 com o binário instrumentado registrado na entrega.
- Fora do sandbox, com autorização da execução e somente fixtures próprias, a UI **Tauri/WebKit real** e o socket Unix funcionaram.
- W0 curto terminou `complete` em 41,68 s, com cleanup de zero sobreviventes. Parâmetros: 1 startup frio, 1 quente, idle de 5 s e 1 ciclo. **É triagem, não baseline de PERFORMANCE.md.**
- W1/W2 curtos falharam com `Native benchmark condition timed out`, após aproximadamente 17,58 s e 16,54 s; ambos ficaram na fase `ready` antes de concluir o cenário de terminal. Não foi erro de inicialização GTK. Foram observados até três processos controlados durante a montagem. A etapa exata precisa de diagnóstico adicional do driver.
- W3 curto falhou com `Service startup exceeded 10 seconds`. Uma reprodução independente mostrou que o serviço continuava vivo, aceitava `set_active_scope`, mas não entregava o snapshot grande: somente o JSON das 10.000 conversas tinha **2.147.891 bytes**, maior que `MAX_FRAME = 2.097.152`; a resposta de snapshot encerrou com **zero bytes**. A causa de capacidade está no caminho real de catálogo/IPC, não em falta de socket.
- Todos os cleanups das execuções desta revisão registraram zero sobreviventes. Nenhum provedor real foi iniciado ou chamado.

Evidências sintéticas, com prefixo pessoal do repositório mascarado:

- [W0 curto JSON](../deliveries/EROS-002-results/review-w0-short.json) e [relatório](../deliveries/EROS-002-results/review-w0-short.md).
- [W1–W3 curtos JSON](../deliveries/EROS-002-results/review-w1-w3-short.json) e [relatório](../deliveries/EROS-002-results/review-w1-w3-short.md).

Os builds/checks Rust/frontend da entrega original não foram todos repetidos nesta revisão. Os resultados acima são adicionais e não apagam as tentativas históricas bloqueadas no sandbox.

## Apontamentos acionáveis

### 1. P1 — o probe perde marcadores dentro de lotes grandes

Em [TerminalPane.svelte](../../../apps/desktop/src/lib/TerminalPane.svelte), linhas 161–162, o código trunca `(tail + decoded)` para 256 caracteres **antes** de procurar marcadores. Um lote que começa com `EROS_MARK_1000_END` e contém mais 1.024 caracteres de carga perde o marcador antes da busca, mesmo tendo sido recebido/renderizado. Isso pode provocar falso timeout exatamente sob pressão.

Correção requerida: buscar no lote completo combinado com o pequeno sufixo anterior, depois reter apenas o sufixo necessário para fronteiras de chunks. Manter memória e registro de marcadores limitados. Cobrir marcador no começo/meio/fim de lote de 32 KiB, marcador dividido entre lotes, vários marcadores e callbacks atrasados. Uma espera fixa para `stty -echo` não é handshake: garantir que texto ecoado do comando não satisfaça a medição.

### 2. P1 — W1/W2 não chegam ao cenário medido na UI nativa

O timeout foi reproduzido fora do sandbox. O driver em [benchmark.ts](../../../apps/desktop/src/lib/benchmark.ts) espera quantidade exata de probes após cada criação, mas só informa uma mensagem genérica. O relatório de falha em [run.py](../../../scripts/performance/run.py) descarta o último payload UI ao levantar a exceção; assim também perde métricas parciais já coletadas.

Correção requerida no harness: registrar subetapa, quantidade esperada/observada, prontidão e erro de UI sanitizado; preservar os resultados parciais no JSON/Markdown, identificando validade. Fazer estágios de 1 e 8 terminais independentes: falhar na montagem de 8 não pode apagar a prova de 1 nem impedir o estágio de pressão de 1 terminal.

Ponto a reproduzir como possível falha preexistente do produto: [LayoutTree.svelte](../../../apps/desktop/src/lib/LayoutTree.svelte) mantém `TerminalPane` sem chave de identidade, e o terminal cria assinatura/probe em `onMount`. Ao adicionar um terceiro painel, uma posição leaf pode receber outra conversa mantendo a instância e sua assinatura antiga. Além disso, o cleanup do probe usa `delete(conversation.id)` com props mutáveis, podendo remover o registro de outra instância. Confirmar identidades com fixture; não declarar a hipótese comprovada apenas pelo timeout. O registro/cleanup **da instrumentação** deve capturar identidade imutável e não apagar um probe substituto. Uma correção geral do lifecycle do produto deve ser registrada para EROS-005/006, sem ser escondida neste cartão.

### 3. P1 — interpretar capacidade do catálogo como achado, não bloqueio de ambiente

W3 não pode depender de um snapshot maior que o protocolo suporta e depois atribuir sua ausência à inicialização do serviço. [start_service](../../../scripts/performance/fixture.py) usa `snapshot` como readiness; o serviço grande pode estar vivo e acessível por controle enquanto a leitura falha por tamanho.

Correção requerida: distinguir liveness/readiness de disponibilidade do catálogo, registrar cenário, tamanho/limite e erro observável; apresentar a falha W3 como **resultado nativo de capacidade**. Uma falha real do produto é um resultado válido de investigação, não uma meta aprovada. Não elevar MAX_FRAME, reduzir silenciosamente os 10.000 chats ou implementar paginação de produção dentro de EROS-002. Vincular o achado a EROS-005, que trata catálogo/estado/paginação. Completar as medições viáveis de W3 e declarar precisamente as inviáveis.

### 4. P2 — clocks são registrados, mas saltos não invalidam a rodada

[run.py](../../../scripts/performance/run.py) guarda offsets epoch/monotonic antes/depois e calcula startup subtraindo relógios epoch, mas não rejeita/qualifica uma rodada com salto. A documentação exige descarte, e o render ainda agregaria tempos inválidos.

Correção requerida: política explícita de validade do clock, limite/precisão justificados e testes com clocks injetados. Salto invalida startup/comparação afetados; não deve descartar métricas monotônicas independentes sem motivo. Evitar latências negativas ou p95 aparentemente normal de amostras inválidas.

### 5. P1 — completar métodos essenciais antes de duas rodadas de baseline

A entrega identifica corretamente as lacunas, mas elas continuam pendentes: interrupção por Ctrl+C até produtor parado e entrada recuperada; troca de chat de fato; efeito de animações/minimização sobre idle; comparação PSS entre catálogos equivalentes; overhead da instrumentação UI. Criar oito painéis não mede troca de chats. Os produtores W2 atuais rodam em background (`&`), então enviar Ctrl+C ao shell não comprova interromper esses produtores: o teste de interrupção precisa de um produtor foreground e confirmação observável de término/entrada.

Correção requerida: implementar os métodos viáveis com instrumentação opt-in, sem exigir provedores reais nem antecipar configurações/otimizações de produção. Quando uma capacidade ainda não existir, produzir uma evidência negativa ou indisponibilidade específica, vinculada ao cartão que a implementará. Distribuir amostras de echo ao longo da pressão confirmada, com handshake de início de produção; não medir só um burst inicial que pode anteceder a carga. Não anunciar duas tentativas ou duas execuções curtas como baseline completo.

## Ordem para o próximo agente

1. Corrigir confiabilidade do probe, identidade/cleanup da instrumentação e diagnóstico/preservação das falhas.
2. Reproduzir os timeouts de W1/W2 por subetapa e separar defeito de driver de defeito de produto. Registrar o último no cartão pertinente, preservando a medição de estágios independentes que funcionam.
3. Classificar corretamente o limite real de catálogo e validar clocks.
4. Completar os métodos essenciais ainda viáveis e seus testes contratuais/comportamentais; nenhum mock em produção.
5. Executar triagem curta nativa. Só então rodar duas vezes a mesma configuração completa, mesmo binário/hardware, com os parâmetros de PERFORMANCE.md. Não repetir inferência; apenas doubles/shells próprios.
6. Interromper uma rodada nativa com fixture Eros viva para confirmar cleanup real. Medir/explicar overhead; resultados negativos e indisponibilidades não podem ser ocultados.
7. Atualizar a entrega, evidências e sua linha do Kanban para Em revisão. Não marcar Aceita. Não iniciar EROS-003/005/006 neste trabalho.

Use o mecanismo normal de aprovação de ferramenta para executar o comando controlado fora do sandbox quando necessário; esta revisão conseguiu fazê-lo. Não desative salvaguardas do sistema/app, não execute como root, não use display virtual como se fosse a sessão de referência e não encerre serviços pessoais.

## How to test da revisão

Da raiz do repositório, usando o release **instrumentado** já compilado e a sessão gráfica com execução permitida:

```bash
python3 -m unittest discover -s scripts/performance -p 'test_*.py' -v
python3 scripts/performance/run.py --workloads W0 --cold-samples 1 --warm-samples 1 --idle-seconds 5 --pressure-seconds 1 --cycles 1 --output /tmp/eros002-review-triage.json
python3 scripts/performance/run.py --workloads W1,W2,W3 --cold-samples 1 --warm-samples 1 --idle-seconds 5 --pressure-seconds 1 --cycles 1 --output /tmp/eros002-review-w1-w3.json
```

Resultados nesta revisão: dez testes passam; primeira triagem exit 0; segunda exit 2 com as falhas descritas. As fixtures são removidas pelo harness e os relatórios permanecem no destino escolhido. Após corrigir o medidor, repetir estes cenários para verificar os métodos; a falha de catálogo pode continuar como achado negativo de produto, corretamente identificado. Nenhum desses comandos curtos certifica as metas da primeira versão.

Para reproduzir a perda de marcador sem iniciar o app, em Node:

```js
const marker = 'EROS_MARK_1000_END';
const decoded = marker + 'x'.repeat(1024);
console.log(decoded.includes(marker)); // true
console.log([...decoded.slice(-256).matchAll(/EROS_MARK_\d+_END/g)].length); // 0
```

O How to test final da tarefa precisa incluir os métodos corrigidos, a execução completa, falha real, comparação de relatórios e cleanup; os passos acima são somente reprodução desta revisão.
