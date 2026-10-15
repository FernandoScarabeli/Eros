# Pesquisa de produto, engenharia e referências

Consulta: **2026-10-15**. Escopo: Eros pessoal em um PC Ubuntu, com possíveis clientes mobile futuros. Método: documentação oficial de produtos/protocolos e leitura do código local. Não foram usados posts promocionais, benchmarks de stacks diferentes ou promessas de fornecedores como prova de desempenho do Eros. Nenhuma inferência de IA, login ou operação Git externa foi realizada nesta pesquisa.

## Síntese e decisões

O produto deve reduzir a fricção entre **conectar ferramentas → abrir projeto → trabalhar com agentes → entender mudanças → revisar → retomar depois**. Para isso, a primeira versão precisa de menos superfícies e mais confiabilidade: perfil local, configurações aplicadas, conexões reais, sessões recuperáveis, Git útil, atenção e backup.

As referências de VS Code e Zed mostram padrões relevantes de configurações por escopo, revisão Git, histórico e capacidades diferentes entre agentes. Isso fundamenta nossa organização, não a decisão de copiar uma IDE inteira. [R01](#r01) [R02](#r02) [R03](#r03)

| Decisão do Eros | Motivo e tradeoff | Execução |
| --- | --- | --- |
| Manter Rust/Tauri/Svelte/SQLite | Evita reescrita; arquitetura já separa UI de processos. Stack não é garantia de leveza | 001–008 |
| Terminal original como superfície principal | Preserva comportamento dos CLIs; controles estruturados somente quando suportados | 016–020, 042 |
| Perfil local sem conta Eros | Atende nome/foto no PC, sem servidor de identidade | 010–012 |
| Configurações com precedência clara | Default → pessoal → projeto apenas nas chaves permitidas → override de nova sessão; segurança não reduzida pelo projeto | 010–014, 022 |
| Conectar via fluxos oficiais dos CLIs | Reutiliza ferramentas e credenciais existentes; logout pode afetar outras ferramentas, o que deve ser explicado | 016–019, 030 |
| Git local separado de GitHub | Repositório e revisão funcionam offline; serviço remoto é opcional | 025–031, 046 |
| Arquivos + editor externo primeiro | Entrega acesso ao código sem carregar uma IDE residente; editor interno fica para etapa posterior | 024, 035, 047 |
| Desempenho medido e visual econômico | Sem WebGL obrigatório; filas com limites; baseline antes de otimizar | 002–007, 038 |
| Atualização `.deb` conservadora | Primeiro pacote instalado/testado e descoberta manual de versão; não supor que plugin de updater atualiza `.deb` | 037 |
| Mobile apenas depois | Não expor hoje uma API remota; futuramente cliente pareado e revogável do PC | 045 |

## Inventário do código e lacunas

Leitura inicial: aproximadamente 10 mil linhas de código-fonte, scripts e estilos; conferência focada adicional nesta pesquisa. Não houve nova execução dos testes funcionais do app. Resultados antigos em `docs/chat-activity.md` são evidências históricas, não um teste executado hoje.

| Área | Evidência no repositório | Lacuna concreta |
| --- | --- | --- |
| Contratos | [eros-core](../../crates/eros-core/src/lib.rs), gerador `src/bin/contracts.rs` | Novos domínios precisam de contratos gerados e compatibilidade IPC |
| Serviço e PTY | [runtime.rs](../../crates/eros-service/src/runtime.rs), [server.rs](../../crates/eros-service/src/server.rs) | Lifetime, reinício, filas, conexão e pressão precisam de aceitação e limites completos |
| Persistência | [store.rs](../../crates/eros-service/src/store.rs) | Migrações ad hoc; `snapshot()` também limpa/migra; falta política versionada de backup/recovery |
| Provedores | `Runtime::authenticated_providers`, `provider_hooks.rs` | Login Codex detectado por texto e executado no snapshot; não há central de conexões e paridade Claude |
| Atividade | `activity.rs`, `api.ts`, `ChatStatusDot.svelte` | Boa separação de revisão de atividade; hooks são observadores e dependem de suporte/trust |
| Terminal UI | [TerminalPane.svelte](../../apps/desktop/src/lib/TerminalPane.svelte) | `terminal.write` não acompanha conclusão de renderização; limites do backend não limitam toda a fila no frontend |
| Navegação | [App.svelte](../../apps/desktop/src/App.svelte) | Muitas responsabilidades; gravação otimista/debounce e snapshots podem disputar estado; medir e testar antes de refatorar |
| Visual | `styles.css`, `faulty-terminal.js` | Fundo animado mantém trabalho contínuo enquanto visível; precisa de modo econômico/default e medição |
| Segurança desktop | `tauri.conf.json`, capabilities | CSP está `null`; validar comandos, caminhos, assets locais, links e permissões antes de distribuir |
| Verificação | `scripts/`, `apps/desktop/scripts/`, workflow | Smokes controlados existem; CI atual só verifica fundação, sem gate completo de Rust/frontend |
| Produto básico | Sidebar, chats, configurações limitadas | Sem perfil editável, central completa, Git, arquivos, backup, onboarding e gestão completa de chats/projetos |

As lacunas são constatações da leitura e hipóteses de risco que precisam de reprodução; não foram todas demonstradas como bugs. Exemplo: a falta de timeout no subprocesso de status pode atrasar o snapshot se o CLI travar; EROS-003/016 precisam testar isso com um double travado.

## Achados por tema

### Conexões e capacidades dos agentes

Codex documenta autenticação e retomada; seu app-server oferece integração estruturada. Claude documenta autenticação, flags de sessão e hooks. Isso abre caminhos oficiais, mas não prova que um canal de controle independente pode dirigir com segurança a mesma TUI já aberta. **EROS-042 exige provar a topologia antes da coordenação.** Não criar uma segunda execução escondida nem migrar os chats automaticamente para API. [R04](#r04) [R05](#r05) [R06](#r06) [R07](#r07) [R08](#r08) [R09](#r09)

A central deve distinguir instalado, autenticado, acessível, versão compatível e hooks ativos. Suporte a modelo, resume, uso e aprovações é por adapter e por versão. Modelos não serão uma lista eterna hardcoded. A interface mostra indisponibilidade quando a capacidade não existir. [R03](#r03)

Credenciais continuam sob responsabilidade do provedor/GitHub CLI. Eros guarda referência e estado, não cópia de tokens. Login não é teste de inferência; conexão válida não garante créditos ou sucesso de um pedido. Logout de um CLI compartilhado tem efeito fora do Eros e precisa ficar explícito. [R05](#r05) [R20](#r20)

### Terminal, concorrência e otimização

O xterm descreve como produtores rápidos podem acumular dados e afetar interatividade. Portanto, o limite de histórico do Rust não resolve a fila de escrita da UI. Precisamos de créditos/ack ou mecanismo equivalente medido, com watermarks, limites em bytes, prioridade de controle e tratamento explícito de consumidor lento. [R10](#r10)

Tauri mantém o trabalho privilegiado no core; Tokio ressalta que trabalho bloqueante iniciado não é cancelado simplesmente abortando uma task. Por isso, os cartões exigem deadlines e término/recolhimento real de subprocessos. Limitar apenas o número de sessões não limita conexões, threads IPC, verificações de login ou jobs Git. [R12](#r12) [R13](#r13)

Watchers ajudam a evitar polling, mas podem perder eventos e atingir limites do SO. Eros precisa de debounce, invalidação, uma ação Atualizar e recuperação após overflow, sem um scan periódico global como solução. [R14](#r14)

Métricas Linux permitem separar consumo por processos. Não declarar "baixo consumo" usando só o RSS do binário Rust e ignorando WebKit/provedores. Metas propostas, workloads e critérios estão em [PERFORMANCE.md](PERFORMANCE.md). [R15](#r15)

### Git, arquivos e confiança

Git fornece formato de status próprio para máquinas e representação NUL para caminhos difíceis. Usaremos argumentos separados, parsers com fixtures e operações serializadas por repositório; não analisar saída colorida ou montar shell strings. Leitura de diff desativa helpers externos/textconv para não executar configuração arbitrária ao visualizar um arquivo. [R16](#r16) [R17](#r17)

Worktrees ajudam a isolar alterações, mas compartilham partes do repositório. Criação e remoção precisam checar estado, branch ocupada, alterações e commits; não são sandbox nem justificam `--force`. Na primeira versão, stage é por arquivo e pull é fast-forward explícito; hunk staging, rebase automatizado e descarte em lote ficam fora. [R18](#r18) [R19](#r19) [R33](#r33)

GitHub entra por adapter opcional de `gh`. Um detalhe importante da documentação: `gh pr create --dry-run` pode enviar mudanças. Logo, o preview de PR deve ser construído localmente, sem usar dry-run como garantia de ausência de efeitos. [R20](#r20) [R21](#r21)

### Configurações, dados e distribuição

Configurações por escopo ajudam a manter defaults pessoais e overrides de projeto claros. Execução de binários e segurança permanecem pessoais; abrir um projeto não pode alterar silenciosamente quem executa comandos. Central completa significa comportamento aplicado e persistido, com busca e reset, não um catálogo de toggles sem efeito. [R01](#r01)

SQLite WAL possui estado além do arquivo principal. Backup usará um mecanismo consistente, manifesto de versão e verificação; não copiar apenas um `.sqlite3` aberto. Restaurar deve validar antes, proteger a base atual e explicar que PTYs/transcritos dos provedores/arquivos dos projetos não fazem parte do backup padrão do Eros. [R22](#r22) [R23](#r23)

O pacote Ubuntu deve ser testado numa instalação limpa. O updater Tauri é uma referência para artefatos assinados, não uma promessa de auto-update do `.deb`. Planejamos primeira distribuição por pacote e verificação explícita de versão; instalar/reiniciar com sessões ativas exige fluxo próprio. [R27](#r27) [R28](#r28)

### UX limpa, privacidade e extensão futura

Diálogos precisam de foco, teclado e retorno ao gatilho; acessibilidade faz parte dos critérios básicos. Notificações partem de eventos reais, com deduplicação e preferências, sem conteúdo de prompts no lockscreen por padrão. [R24](#r24) [R25](#r25)

CSP e capabilities reduzem exposição da WebView, mas não substituem validação no Rust. A própria biblioteca de terminal orienta a tratar seu entorno e acesso a privilégios como uma fronteira sensível. Não inserir HTML vindo de agentes, abrir links automaticamente ou liberar clipboard por sequências de controle sem ação explícita. [R11](#r11) [R26](#r26) [R29](#r29)

MCP e mobile são extensões posteriores com modelo de confiança próprio. Não executar automaticamente um MCP indicado por um repositório. Tailscale Serve ilustra um acesso privado possível, mas cria dependência externa e não é a decisão tomada para o Eros. A task mobile entrega apenas ADR, threat model e critérios de pareamento; nenhuma porta será aberta nesta fase. [R30](#r30) [R31](#r31)

## Priorização: o que entra e o que espera

| Prioridade | Incluído | Por que |
| --- | --- | --- |
| Essencial | Estabilidade, performance, configurações, perfil, conexão Codex/Claude, gestão local, Git, atenção, backup, instalação | Fecha o ciclo de trabalho individual |
| Opcional na primeira versão | GitHub/PR | Útil, mas não deve bloquear Git local para quem não usa GitHub |
| Após a primeira versão | Editor interno, perfis de agente, conhecimento, busca local ampliada, MCP/uso | Precisa de base e capacidade verificadas; evitar custo residente |
| Depois de prova técnica | Coordenação e rotinas | Exigem controle estruturado, idempotência, limites e recuperação |
| Futuro, projeto apenas | Mobile | Um cliente do PC, não uma segunda plataforma de execução |

Não há prazo estimado sem baseline e execução das primeiras tarefas. O objetivo é qualidade verificável, não quantidade de botões.

## Referências primárias consultadas

Todos os links foram consultados nesta pesquisa; capacidades mudam. O agente deve rever a documentação do adapter na versão instalada antes de implementar. Cada cartão aponta os IDs relevantes; os critérios de produto e números de desempenho são nossas decisões, não exigências dos autores abaixo.

### R01

[VS Code — user/workspace settings](https://code.visualstudio.com/docs/configure/settings). Base para precedência, busca e distinção de configurações pessoais/projeto. Não adotar todas as camadas do VS Code.

### R02

[VS Code — source control](https://code.visualstudio.com/docs/sourcecontrol/overview). Base de fluxo revisão → stage → commit e distinção Git/GitHub.

### R03

[Zed — Agent Panel](https://zed.dev/docs/ai/agent-panel). Histórico, threads e isolamento; explicita que recursos variam entre integrações de agentes. Referência de produto, sem comparação de performance.

### R04

[Codex — CLI reference](https://learn.chatgpt.com/docs/cli/reference). Comandos oficiais, retomada explícita por sessão e flags; validar disponibilidade por versão.

### R05

[Codex — authentication](https://learn.chatgpt.com/docs/auth). Fluxos e armazenamento de credenciais pertencem ao provedor; o Eros não deve copiar tokens.

### R06

[Codex — app-server](https://learn.chatgpt.com/docs/app-server). Protocolo estruturado, geração de schemas por versão e integração profunda. Referência de investigação; não implementar transporte experimental de rede como default.

### R07

[Claude Code — authentication](https://code.claude.com/docs/en/authentication). Base de login oficial; suporte real depende da instalação, conta e políticas aplicáveis.

### R08

[Claude Code — CLI reference](https://code.claude.com/docs/en/cli-reference). Retomada, modelo e configurações de sessão; não assumir paridade com outro CLI.

### R09

[Claude Code — hooks](https://code.claude.com/docs/en/hooks). Eventos oficiais e seus limites. Observação de atividade não vira automaticamente canal de controle/aprovação.

### R10

[xterm.js — flow control](https://xtermjs.org/docs/guides/flowcontrol/). Fila de escrita, watermarks e callbacks; fundamenta controle de pressão ponta a ponta.

### R11

[xterm.js — security](https://xtermjs.org/docs/guides/security/). Saída e acesso ao terminal exigem fronteiras de confiança no aplicativo que o hospeda.

### R12

[Tauri — process model](https://v2.tauri.app/concept/process-model/). Separação entre core e WebView; manter trabalho privilegiado e bloqueante fora da UI.

### R13

[Tokio — spawn_blocking](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html). Limites, concorrência e cancelamento de trabalho bloqueante; referência, não obrigação de converter o serviço inteiro para async.

### R14

[notify — documentation/known problems](https://docs.rs/notify/latest/notify/). Limites de watchers e perda de eventos; usar invalidação e recuperação explícitas.

### R15

[Linux kernel — proc filesystem](https://www.kernel.org/doc/html/latest/filesystems/proc.html). Métricas por processo, PSS/RSS e `smaps_rollup`; evitar coleta de ambientes e comandos pessoais.

### R16

[Git — status](https://git-scm.com/docs/git-status). Porcelain, caminhos e custo de refresh/untracked. Base para parser de status e invalidação sem polling global.

### R17

[Git — diff](https://git-scm.com/docs/git-diff). Revisões, formatos e flags que desativam helpers; limitar tamanho e tratamento de binários.

### R18

[Git — worktree](https://git-scm.com/docs/git-worktree). Inventário, branches, locks e remoção. Base para gestão explícita de isolamento.

### R19

[Git — pull](https://git-scm.com/docs/git-pull). Integração de alterações remotas; política Eros inicial propõe fast-forward e erro claro para divergência.

### R20

[GitHub CLI — auth login](https://cli.github.com/manual/gh_auth_login) e [auth status](https://cli.github.com/manual/gh_auth_status). Fluxos oficiais e efeitos do armazenamento compartilhado. Proibir saída de tokens em diagnósticos.

### R21

[GitHub CLI — PR create](https://cli.github.com/manual/gh_pr_create). Criação de PR e efeitos possíveis do dry-run. Preview Eros será local, sem efeito externo.

### R22

[SQLite — online backup](https://sqlite.org/backup.html). Snapshot consistente do banco aberto; fundamenta backup em vez de cópia ingênua de arquivo.

### R23

[SQLite — WAL](https://sqlite.org/wal.html). Persistência, checkpoints e arquivos auxiliares. Não separar WAL ativo do estado a preservar.

### R24

[W3C APG — modal dialog](https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/). Foco, navegação por teclado e semântica. Referência de requisitos de acessibilidade.

### R25

[Tauri — notifications](https://v2.tauri.app/plugin/notification/). Plugin nativo disponível; aceitação depende do desktop Linux e do lifetime de quem envia.

### R26

[Tauri — capabilities](https://v2.tauri.app/security/capabilities/). Permissões da WebView e limites da proteção; comandos Rust continuam responsáveis pela validação.

### R27

[Tauri — Debian packaging](https://v2.tauri.app/distribute/debian/). Pacote, desktop entry e dependências de runtime. Testar no Ubuntu escolhido.

### R28

[Tauri — updater](https://v2.tauri.app/plugin/updater/). Assinaturas e distribuição de atualizações; investigar suporte do formato escolhido antes de adicionar atualização automática.

### R29

[Tauri — CSP](https://v2.tauri.app/security/csp/). Proteção precisa ser configurada; verificar produção e desenvolvimento sem liberar tudo como correção.

### R30

[MCP — security best practices](https://modelcontextprotocol.io/specification/latest/basic/security_best_practices). Fronteiras de autenticação e riscos de servidores/conexões; uso futuro opcional, não parser genérico de segredos.

### R31

[Tailscale — Serve](https://tailscale.com/docs/features/tailscale-serve). Exemplo de acesso privado ao serviço local. Opção a avaliar no mobile, sem adotar conta externa obrigatória.

### R32

[Codex — hooks](https://learn.chatgpt.com/docs/hooks). Eventos e trust oficial; não alterar/burlar políticas do usuário para fazer indicadores funcionarem.

### R33

[Git — restore](https://git-scm.com/docs/git-restore). Diferenças entre índice/working tree e semântica em conflitos. Descartar mudanças exige escopo e confirmação concretos.

### R34

[Claude Code — settings](https://code.claude.com/docs/en/settings). Precedência e configuração por sessão; preservar regras existentes e políticas gerenciadas.

## Limitações e questões deliberadamente abertas

- Editor interno: o guia público do CodeMirror apareceu na busca, mas falhou ao abrir nesta sessão. Não foi usado como prova de escolha/performance; EROS-035 exige consultar documentação acessível e comparar opções antes de adotar uma.
- Versões exatas mínimas de CLIs, Ubuntu suportado e viabilidade dos tetos de PSS/latência serão registrados por 001/002/016/021/037; não inferidos das páginas atuais.
- Não se comprovou controle simultâneo de TUI e protocolo estruturado na mesma sessão para ambos os provedores. Esta é uma dependência real de 043/044.
- Orçamento monetário estrito não é universal entre provedores; EROS-049 separa estimativa, quota e limite realmente imposto.
- Mobile, plugins e modelos locais são possibilidades, não dependências da primeira versão. Um modelo local pode consumir mais recursos que o Eros inteiro; não é requisito para um produto leve.
