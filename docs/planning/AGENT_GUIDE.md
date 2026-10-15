# Contrato de execução para agentes

Este contrato faz parte de **cada tarefa** do [Kanban](KANBAN.md). Ele atende ao pedido de entregar código de produto, clean code, verificações honestas e um How to test utilizável por uma pessoa.

## Antes de implementar

1. Leia `AGENTS.md`, `README.md`, `docs/product.md`, `docs/architecture.md`, [RESEARCH.md](RESEARCH.md), [PERFORMANCE.md](PERFORMANCE.md) e o cartão escolhido.
2. Examine o código atual e `git status`. Os caminhos dos cartões são pontos de entrada, não uma garantia de que o projeto continuará igual. Não sobrescreva alterações de outra pessoa. Na criação deste plano já havia uma alteração em `apps/desktop/package-lock.json`.
3. Confira as dependências. Uma tarefa em revisão não conta como aceita. Se uma dependência já estiver implementada fora deste Kanban, registre a evidência e o motivo antes de atualizar o status. Não implemente a dependência escondida dentro da sua tarefa.
4. Registre responsável, status Em andamento e checkout/branch na sua linha. A reserva precisa ser coordenada pelo usuário quando houver vários agentes; uma edição Markdown não é um lock distribuído.
5. Apresente brevemente o escopo e os arquivos centrais que precisará alterar. Resolva decisões rotineiras de implementação por julgamento; consulte o usuário somente para uma decisão de produto indispensável, credencial, acesso externo ou bloqueio real.

## Regras de implementação

- Rust controla processos, persistência, permissões e estados de domínio. A interface apresenta estado tipado. Gere contratos TypeScript a partir de `eros-core`; não duplique os modelos manualmente.
- Separe os módulos por responsabilidade concreta. Extraia código quando isso reduzir acoplamento ou permitir teste relevante; não crie frameworks internos, abstrações genéricas, crates vazias ou uma reescrita completa sem necessidade.
- Modele falhas, cancelamento, timeout e resultado incerto. Repetir automaticamente só é permitido para operações realmente idempotentes. Timeout não prova que uma escrita externa falhou.
- Toda fila, cache, assinatura, job e subprocesso tem proprietário, limite e cleanup. Limites em bytes devem medir bytes, inclusive Unicode. Use trabalho bloqueante limitado fora da UI; `spawn_blocking` sozinho não cancela processos.
- Processos utilitários usam executável e argumentos separados. Não concatene nomes de arquivos/branches em comandos shell. Preserve o shell interativo do terminal, com uma política explícita para aliases e caminhos dos provedores.
- Não altere configuração global de Git/provedores sem ação explícita do usuário. Não leia nem exponha tokens para "descobrir login". Não adicione bypass de permissões. Dados do projeto e saída de terminal são entrada não confiável.
- Reutilize as convenções Svelte 5, tokens visuais e componentes existentes. Prefira hierarquia clara, informação progressiva, teclado, foco visível e feedback de erro. Sem painéis decorativos de métricas fictícias ou animação contínua obrigatória.
- Toda dependência nova precisa de motivo, licença, custo de bundle/runtime e compatibilidade com Rust/Node suportados. Use o lockfile; atualizações não relacionadas ficam fora.
- Preserve dados existentes. Migrações são versionadas e transacionais; mudanças incompatíveis no IPC precisam de política de versão. Não reinicie um serviço com sessões ativas para "resolver" incompatibilidade.
- Registre os limites de implementação. Funcionalidade sem suporte deve aparecer como indisponível, com motivo, sem simulação de sucesso.

## Paralelismo e integração

O padrão inicial é **um agente implementando por vez**. Para paralelismo autorizado pelo usuário, use checkouts/worktrees separados e mantenha as dependências integradas antes dos consumidores. Worktree não é sandbox.

`App.svelte`, `eros-core`, `store.rs`, `server.rs`, bridge Tauri, manifests e lockfiles são áreas compartilhadas. Não editar esses mesmos arquivos em dois agentes simultaneamente sem coordenação. Contratos e migrações devem ser integrados em sequência; resolve-se a integração e depois se executam os checks sobre o conjunto. Nenhum agente pode marcar outra tarefa aceita ou descartar mudanças alheias.

Se o cartão ficar grande demais, proponha subtarefas com contratos e dependências explícitos e registre a divisão antes de iniciar. Não entregue metade como tarefa completa. Cartões de investigação entregam decisão e protótipo limitado, não uma funcionalidade anunciada.

## Verificação proporcional e obrigatória

Estes comandos já existem; rode os aplicáveis na raiz ou no diretório indicado:

```bash
# Raiz do repositório
python3 scripts/check_repository.py
cargo test --workspace --locked
cargo run --locked -p eros-core --bin contracts -- --check

# apps/desktop
npm ci
npm run check
npm run build
```

Checks adicionais quando relevantes:

```bash
# Raiz; exige binário compilado e dependências nativas
cargo build --locked -p eros-desktop
python3 scripts/check_chat_activity.py

# apps/desktop; preview em OUTRO terminal
npm run preview
npm run test:ui
npm run test:activity
```

Não execute dois previews na mesma porta. Instalar Chromium/novas ferramentas depende do ambiente; se estiver ausente, informe exatamente a limitação. EROS-001 formaliza CI e ferramentas de lint: não suponha que o repositório inteiro passa em fmt/clippy atualmente.

Crie testes para comportamento, invariantes, perda de dados e falhas materiais; evite testes que apenas repetem a implementação. Testes de UI simulados são necessários para regressões visuais, mas não comprovam WebKitGTK, diálogo nativo ou PTY. Mudanças visuais precisam de inspeção em 1440×960, 960×640 e escala de fonte ampliada, além dos viewports já suportados pelos smokes. Não esconda warnings novos.

Use diretórios temporários e repositórios descartáveis. Nunca execute reset/clean/restore destrutivo em arquivos pessoais como teste. Testes reais de IA são opt-in: registre provedor/versão, prompt público fixo, no máximo um turno simples por provedor por rodada, diretório temporário e limite de tempo; sem retries de inferência. Login/status e testes com doubles devem ser identificados separadamente. Falta de credencial impede só a prova real; entregue o código verificável e mantenha a aceitação pendente.

## Definition of Done

Uma tarefa pode ir para **Em revisão** quando:

- Entrega todo o escopo e critérios do cartão, com integração real entre camadas necessárias.
- Mantém os fluxos anteriores pertinentes; sem TODO no caminho principal, mock em produção ou erro engolido que anuncie sucesso.
- Inclui testes relevantes, atualização de documentação e verificação de contratos/migrações quando aplicável.
- Cumpre os requisitos pertinentes de desempenho ou apresenta medições e bloqueio explícito; não troca um teto por um número maior sem justificativa.
- Entrega o relatório abaixo, com comandos/resultados reais e limitações.

Salvar o relatório em `docs/planning/deliveries/EROS-XXX.md` ao entregar (criar o diretório somente quando necessário), sem dados pessoais ou segredos, e ligar a evidência à linha correspondente do Kanban. O relatório precisa sobreviver à troca de agente/chat.

Só marcar **Aceita** após revisão e verificação do usuário/revisor. Não chamar de concluída uma tarefa com aceitação obrigatória pendente. Bloqueios registram causa, ação necessária e trabalho preservado. Não usar "funciona" se só compilou.

## Formato obrigatório de entrega e How to test

```text
Tarefa: EROS-XXX — título
Resultado: comportamento entregue e motivo.
Arquivos principais: caminhos relativos ao repositório.
Verificação: comando, ambiente e resultado; testes novos e regressões cobertas.
Desempenho: cenário, baseline, depois, limitações; ou não aplicável com motivo.
Migração/compatibilidade: efeitos sobre instalações e sessões existentes.
Pendências: fatos não verificados, suporte ausente e riscos materiais.

How to test
1. Pré-requisitos: versão/build, fixture e se precisa de login/rede.
2. Preparação: comandos exatos a partir de um diretório declarado.
3. Ação: passos de UI/terminal que uma pessoa consiga repetir.
4. Esperado: resultado observável para cada passo.
5. Falha: cenário negativo e comportamento esperado.
6. Regressão: fluxo antigo que deve continuar funcionando.
7. Limpeza: como encerrar/remover somente a fixture criada.

Status: Em revisão ou Bloqueada, nunca aceitação autoatribuída.
```

Os How to test dos cartões são o cenário mínimo. O agente deve convertê-los em comandos e passos exatos para o que realmente implementou; não entregar só "rode os testes".
