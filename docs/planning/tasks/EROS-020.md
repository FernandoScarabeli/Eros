# EROS-020 — Modelo por sessão e retomada explícita de chats encerrados

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-008](EROS-008.md), [EROS-019](EROS-019.md), [EROS-023](EROS-023.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Permitir escolher configuração suportada e continuar uma conversa identificada sem criar trabalho duplicado.

## Pontos de entrada

- crates/eros-core/src/lib.rs e src/bin/contracts.rs
- provider adapters
- crates/eros-service/src/runtime.rs
- crates/eros-service/src/store.rs
- chat UI

## Escopo e decisões de implementação

Persistir provedor/modelo/reasoning suportado e identidade de sessão verificada. Validar escolhas por versão/capability; se catálogo oficial indisponível, configuração manual validada ou opção default do CLI, nunca lista falsa. Retomar usa ID exato, lock por conversa, action ID e confirmação de estado terminal. Sessão viva reconecta; sessão encerrada só retoma por ação explícita. Não enviar última mensagem novamente.

## Critérios de aceite

- [ ] Duplo clique em Retomar cria no máximo um processo.
- [ ] Chat mantém identidade; ID ausente/inválido explica alternativa sem trocar pelo --last.
- [ ] Modelo salvo afeta novas sessões/retomadas conforme suporte, sem reiniciar chat ativo.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Doubles verificam argv/ID/modelo, dupla ação, timeout incerto, identidade ausente e conversa já viva; migração de chats sem campos novos.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Criar chat double, encerrar processo e Retomar.
2. Confirmar ID exato e ausência de prompt duplicado.
3. Testar modelo inválido e duplo clique; processo único e erro claro.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-020.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Retomada segue locks por chat e não segura criação global durante operação lenta.

## Fora do escopo

Não importar todos os históricos privados, usar --last ou reiniciar automaticamente após crash/reboot.

## Bases da pesquisa

[R03](../RESEARCH.md#r03), [R04](../RESEARCH.md#r04), [R08](../RESEARCH.md#r08). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
