# EROS-017 — Conexão Codex com autenticação oficial

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-016](EROS-016.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Conectar a instalação do Codex de forma verificável e reutilizar a autenticação oficial.

## Pontos de entrada

- provider adapter Codex
- apps/desktop/src-tauri/src/lib.rs
- providers settings

## Escopo e decisões de implementação

Implementar status, login, cancelamento e desconexão pelo fluxo oficial compatível com a versão instalada. Preferir CLI existente; app-server somente se necessário para metadados e sem criar thread/turno. Login acontece por ação do usuário em navegador/terminal confiável. Não manipular auth.json. Distinguir remover conexão Eros de logout do CLI compartilhado e informar seus efeitos.

## Critérios de aceite

- [ ] Login concluído só após confirmação oficial; fechar navegador não vira sucesso.
- [ ] Cancelamento/expiração/versão incompatível são recuperáveis.
- [ ] Nenhuma inferência ou segredo entra em SQLite/log/DOM de diagnóstico.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Doubles para sucesso, cancelamento, expiração, sessão existente e logout; schema e erro do provider versionados. Real login somente com usuário presente.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Em fixture, executar conectar/cancelar/reconectar.
2. Com instalação real e ação do usuário, conectar e confirmar status sem enviar prompt.
3. Remover conexão Eros e verificar que não foi feito logout global silencioso.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-017.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Login/probe não bloqueia startup nem cria processo residente extra quando não necessário.

## Fora do escopo

Não implementar fluxo OAuth próprio, API key gate, token sharing experimental ou inferência de teste automática.

## Bases da pesquisa

[R04](../RESEARCH.md#r04), [R05](../RESEARCH.md#r05), [R06](../RESEARCH.md#r06). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
