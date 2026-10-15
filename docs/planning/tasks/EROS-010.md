# EROS-010 — Configurações tipadas, precedência e persistência

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-004](EROS-004.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Criar uma base única para configurações reais, validada no serviço.

## Pontos de entrada

- crates/eros-core/src/lib.rs e src/bin/contracts.rs
- crates/eros-service/src/store.rs
- crates/eros-service/src/server.rs
- apps/desktop/src/lib/api.ts

## Escopo e decisões de implementação

Definir schema/versionamento e defaults para perfil, aparência, terminal, notificações, conexões, Git, dados e comportamento de fechamento. Implementar read/update/reset com validação por chave e origem efetiva default/pessoal/projeto. Overrides de projeto só em allowlist; caminhos de executáveis e política de segurança ficam pessoais. Atualização transacional com revisão evita overwrite de telas concorrentes.

## Critérios de aceite

- [ ] Valores inválidos não entram no banco; reset restaura default da chave.
- [ ] Origem efetiva é consultável e mudanças sobrevivem à reabertura.
- [ ] Configuração não implementada não aparece como funcional; projeto não pode reduzir proteção global.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Testes de defaults, migração, reset, precedência, revisão antiga e tipos inválidos; contratos gerados e integração IPC.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Alterar chave suportada via fixture e reabrir.
2. Salvar valor fora do intervalo: erro claro e valor anterior preservado.
3. Aplicar override permitido e remover: volta ao pessoal.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-010.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Ler conjunto necessário de uma vez; sem consulta ao SQLite por render de campo.

## Fora do escopo

Não armazenar credenciais em settings nem persistir JSON arbitrário vindo da UI sem validação.

## Bases da pesquisa

[R01](../RESEARCH.md#r01), [R23](../RESEARCH.md#r23), [R34](../RESEARCH.md#r34). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
