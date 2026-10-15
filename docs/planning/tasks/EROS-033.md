# EROS-033 — Exportação e backup consistente dos dados Eros

**Etapa:** B · **Prioridade:** P1 — workspace

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-004](EROS-004.md), [EROS-010](EROS-010.md), [EROS-012](EROS-012.md), [EROS-023](EROS-023.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Preservar configurações, perfil e catálogo sem copiar dados privados desnecessários.

## Pontos de entrada

- crates/eros-service/src/store.rs
- módulo backup
- apps/desktop/src-tauri/src/lib.rs
- settings Dados

## Escopo e decisões de implementação

Backup consistente SQLite por API própria ou método equivalente documentado; incluir avatar e manifesto com schema/app/version/checksums. Exportação portable separada de backup quando necessário. Mostrar conteúdo/escopo antes de salvar. Excluir tokens, logs/PTY/transcritos e arquivos dos projetos por default. Limitar tamanho, temp files e concorrência; arquivo final atômico e permissão privada.

## Critérios de aceite

- [ ] Backup aberto durante atividade contém snapshot íntegro.
- [ ] Manifesto deixa explícito o que não foi incluído.
- [ ] Falha/disco cheio não sobrescreve backup anterior ou deixa falso sucesso.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Escritas concorrentes, WAL ativo, checksums, avatar ausente, permissão negada e falha no final; abrir banco exportado e validar integridade.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Criar dados sintéticos, iniciar atividade double e fazer backup.
2. Inspecionar manifesto/preview: sem segredos/projetos.
3. Simular destino sem escrita e verificar mensagem e backup anterior intacto.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-033.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Trabalho em background limitado, cancelável entre etapas; sem cópia integral em cada clique de settings.

## Fora do escopo

Não copiar só .sqlite3 vivo, exportar auth.json ou anunciar backup dos históricos externos.

## Bases da pesquisa

[R22](../RESEARCH.md#r22), [R23](../RESEARCH.md#r23). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
