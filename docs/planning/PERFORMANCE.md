# Desempenho como requisito de produto

Status: **metas propostas, ainda não medidas nem alcançadas**. EROS-002 cria o baseline e valida a viabilidade no hardware de referência. Qualquer revisão de meta precisa registrar medição, causa e decisão; não se aumenta o limite só para aprovar uma tarefa.

## Princípios

1. O Eros deve gastar recursos principalmente quando existe trabalho útil. Sem polling de modelo, varreduras recorrentes de todo disco, autenticação consultada a cada snapshot ou WebGL obrigatório no início.
2. Interface, serviço Eros e processos dos provedores são medidos separadamente. WebKit e seus processos auxiliares pertencem ao custo da UI. Um shell ocioso controlado mede a base; inferência real não serve como benchmark determinístico.
3. Memória limitada em **todas** as camadas: PTY, IPC, filas do bridge, fila de escrita do xterm, catálogo, caches, watchers, logs e jobs. Limitar só o histórico de 512 KiB não basta.
4. Criar ou destruir views deve liberar assinaturas, timers, observadores e recursos de GPU. Views ocultas não devem renderizar continuamente; sessões podem continuar ativas no serviço.
5. Não introduzir microserviços, containers, servidor Node residente, cache distribuído, banco vetorial ou um novo runtime só para cumprir tarefas deste plano.

Base: [xterm flow control](https://xtermjs.org/docs/guides/flowcontrol/), [Tauri process model](https://v2.tauri.app/concept/process-model/), [Tokio blocking](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html), [Linux proc](https://www.kernel.org/doc/html/latest/filesystems/proc.html), [notify](https://docs.rs/notify/latest/notify/). Números abaixo são decisões do Eros, não benchmarks dessas fontes.

## Metas da primeira versão

| Medida | Cenário e definição | Meta inicial |
| --- | --- | --- |
| Startup quente | Release instalado; processo UI até workspace interativo; serviço vivo; 20 amostras | p95 ≤ 1,5 s |
| Startup frio | Serviço e UI encerrados; cache de disco normal, sem simular drop_caches; 10 amostras | p95 ≤ 3 s |
| CPU ociosa | Release, animações off, 1 shell controlado, sem jobs; após 30 s de estabilização, janela de 60 s | UI + serviço ≤ 1% de **um core** em média |
| CPU em background | Mesmo cenário, janela minimizada | UI + serviço ≤ 0,5% de um core em média |
| Memória base | Soma de PSS do serviço, UI e WebKit; 1 shell controlado contabilizado à parte | ≤ 250 MiB de PSS |
| Catálogo grande | 100 projetos + 10.000 chats sintéticos, sem 10.000 processos | Aumento sobre base ≤ 75 MiB; navegação p95 ≤ 100 ms |
| Troca de chat | 8 sessões shell controladas, terminal visível pronto para entrada | p95 ≤ 150 ms |
| Digitação | Marca enviada → echo renderizado, terminal controlado sem rede | p95 ≤ 50 ms ocioso; ≤ 100 ms sob carga |
| Interrupção | Ctrl+C durante saída intensa, até produtor parar e UI voltar a aceitar entrada | p95 ≤ 500 ms |
| Leaks | 100 ciclos abrir/trocar/fechar views, 30 s finais de estabilização | Crescimento PSS ≤ 10% ou 20 MiB, o maior; sem crescimento contínuo de FDs/assinaturas |
| Watchers/jobs | Projeto de 50.000 arquivos, ignorados pesados, usuário ocioso | Sem scan completo periódico; assinaturas/concorrência limitadas e observáveis |

PSS reduz dupla contagem de páginas compartilhadas; RSS continua registrado para diagnóstico. CPU normalizada: tempo de CPU consumido / tempo de parede × 100, somando os processos relevantes; 100% corresponde a um core. Não dividir pelo número de cores para fazer o consumo parecer menor. p95 deve informar número de amostras, distribuição e método; 10 amostras são uma triagem, não certificação estatística.

Repositórios reais de 50.000 arquivos e CLIs reais podem variar; use também fixtures reproduzíveis. Se uma API WebKit impedir medir echo com precisão, documente a aproximação e seu erro; não substitua por tempo até o IPC retornar.

## Workloads controlados

- **W0 / startup e idle:** base vazia e base de 100 projetos; animação off/on separadamente; sem inferência.
- **W1 / terminal normal:** UTF-8 com acentos/emojis, ANSI, aplicação de tela cheia e resize; 1 e 8 shells.
- **W2 / pressão:** produtor determinístico de 1 MiB/s por 60 s em 1 terminal; 8 terminais de 256 KiB/s por 60 s. Registrar bytes produzidos/entregues, desconexões, atraso e pico de fila.
- **W3 / navegação:** 10.000 chats e 100 projetos persistidos em fixture; busca, ordenação, mudança de escopo e 100 trocas.
- **W4 / arquivos e Git:** 50.000 arquivos com `node_modules`, `target` e `.git` excluídos das varreduras de arquivos; diffs de texto pequenos, 1 MiB, binários, conflitos e renames. O observador de Git pode observar metadados Git selecionados.
- **W5 / falha:** subscriber lento, IPC incompleto, provedor travado, arquivo removido, disco sem escrita, serviço reiniciado e versão incompatível. Tudo em dados temporários.
- **W6 / longa duração:** 30 min de trabalho controlado com abrir/fechar, retorno a idle e crescimento de FD/threads/PSS; executar em marcos, não em cada edição de CSS.

Não usar `yes` sem limite em automação. Produtores precisam de duração/volume máximo e cleanup de grupo de processos; o teste manual de Ctrl+C pode usar um produtor limitado. Nunca despejar PTY/logs pessoais no relatório.

## Limites de implementação

- Conservar os limites existentes até avaliação: 24 sessões vivas, 16 painéis, frame IPC 2 MiB, histórico por sessão 512 KiB e lotes até 32 KiB. São limites de implementação, não suporte certificado a 24 agentes pesados.
- EROS-003 define teto de conexões/jobs e reserva para controle. EROS-006 define watermarks e créditos de terminal com valores medidos. Meta inicial para backlog renderizável: até 512 KiB por terminal visível, além de buffers documentados de transporte; qualquer teto diferente precisa ser medido.
- Consumidor lento recebe falha/resync explícito. Não perder bytes silenciosamente e depois afirmar continuidade perfeita. Ring buffer de bytes não recria necessariamente uma tela TUI; lacunas/estado parcial precisam ficar claros.
- Diretórios listados sob demanda, busca cancelável, no máximo uma busca ativa por view e limite de resultados. Ignorar saídas/dependências por padrão; sem seguir symlinks arbitrariamente.
- Catálogo grande paginado ou carregado por escopo; renderizar somente itens necessários. Não tornar todos os terminais montados uma estratégia de cache.
- Efeitos visuais off por padrão; opt-in pode ter 30 FPS e DPR limitado, pausando oculto/minimizado. Respeitar reduced motion e não impedir o app se WebGL falhar.
- Dependências pesadas de editor/diff entram por import lazy. Medir tamanho inicial, parse e tempo de abertura; não instalar dois editores para a mesma finalidade.
- Logs com rotação; nada de auto-VACUUM, backup completo ou verificação recursiva na renderização de uma tela. Manutenção explícita ou agendada com limites.

## Evidência e regressão

EROS-002 entrega comando/script, fixture, esquema de relatório e hardware/OS/WebKit/versões registrados. Relatórios públicos contêm somente dados sintéticos, identificadores de ambiente necessários e medidas agregadas. Cache de dados pessoais, argv e env completos não devem ser coletados.

Compare baseline e mudança no mesmo ambiente/build, com provedor excluído e incluído de forma separada. Uma regressão sustentada maior que 10% em cenário relevante exige investigação; diferenças próximas do ruído exigem repetição limitada. Não inserir assert de milissegundos em runner compartilhado como se fosse estável: CI testa limites determinísticos de filas/leaks, e o hardware de referência valida números de experiência.

Cada tarefa que toca UI, processos, catálogo, Git, arquivos ou armazenamento registra impacto pertinente. EROS-038 consolida o resultado: sem p95 ou PSS medidos, status **não verificado**, não "otimizado porque usa Rust".
