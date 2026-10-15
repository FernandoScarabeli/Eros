# EROS-036 — Fronteiras locais: CSP, comandos, caminhos e dados não confiáveis

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-003](EROS-003.md), [EROS-011](EROS-011.md), [EROS-015](EROS-015.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Preparar o app para distribuição com permissões delimitadas e validação no serviço.

## Pontos de entrada

- apps/desktop/src-tauri/tauri.conf.json
- apps/desktop/src-tauri/capabilities/default.json
- apps/desktop/src-tauri/build.rs
- crates/eros-service/src/server.rs
- filesystem/assets
- UI de links/conteúdo

## Escopo e decisões de implementação

Configurar CSP de produção e dev separadas, fontes/assets locais e IPC necessário, sem liberar tudo para resolver erro. Inventariar comandos e restringir capabilities por janela; handlers validam IDs, caminhos/tamanhos e origem conforme superfície. Socket privado, owner/permissões/peer conforme suporte, sem listener público. Links externos allowlist de esquemas e abertura por ação; markdown/texto sanitizados. Incluir avatar e futuros arquivos na política de assets.

## Critérios de aceite

- [ ] Build prod funciona com CSP ativa; recurso não permitido é bloqueado com diagnóstico.
- [ ] Traversal/symlink/asset ID malicioso não abre arquivo externo ao escopo.
- [ ] Terminal/conteúdo do repo não executa script nem aciona shell/clipboard automaticamente.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Fixtures de XSS, links javascript/file arbitrários, path traversal, socket inesperado, frame oversized e comando indevido; smoke desktop nativo com CSP.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Executar fixture de conteúdo malicioso e conferir texto inerte.
2. Tentar abrir caminho/asset fora do escopo: erro sem vazamento.
3. Usar shell normal, avatar e UI buildada para confirmar funcionalidades legítimas.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-036.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Validação bounded; sem scanner de antivírus próprio nem serialização extra de todo PTY.

## Fora do escopo

Não prometer sandbox para processos Rust/provedores, adicionar root service ou expor IPC pela rede.

## Bases da pesquisa

[R11](../RESEARCH.md#r11), [R26](../RESEARCH.md#r26), [R29](../RESEARCH.md#r29). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
