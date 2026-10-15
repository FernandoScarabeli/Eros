# EROS-011 — Central de configurações pesquisável e acessível

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-005](EROS-005.md), [EROS-010](EROS-010.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Reunir configurações em um lugar claro, sem telas ou controles que só simulam funcionalidade.

## Pontos de entrada

- apps/desktop/src/App.svelte
- novos componentes de settings em apps/desktop/src/lib/
- apps/desktop/src/styles.css

## Escopo e decisões de implementação

Criar navegação Geral, Meu perfil, Aparência, Terminal, IA, Git/GitHub, Notificações, Dados e Sobre/Diagnóstico. Busca por nome/descrição, indicação de origem e reset. Montar conteúdo sob demanda. Integrar os campos já suportados; seções dependentes ganham estado informativo sem toggle falso. Tratar erro/save pendente e foco/teclado; entrada pelo menu do perfil e Ctrl+, quando livre.

## Critérios de aceite

- [ ] Busca chega ao campo certo e não perde edição ao mudar seção.
- [ ] Salvar/refazer/reset refletem estado confirmado pelo serviço.
- [ ] Teclado/Escape/foco e escala de texto funcionam; janela pequena não corta ações.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Smokes com save lento/falhando, busca, reset, foco e navegação; inspeção visual nos tamanhos do guia.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Abrir Configurações e procurar pasta padrão.
2. Alterar, fechar/reabrir e resetar; conferir valor aplicado.
3. Simular erro e usar somente teclado para recuperar sem perder contexto.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-011.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Seções ocultas não montam editor, scanner ou CLI de login; medir abertura com W3.

## Fora do escopo

Não implementar todos os domínios neste cartão nem adicionar campos cosméticos sem efeito.

## Bases da pesquisa

[R01](../RESEARCH.md#r01), [R24](../RESEARCH.md#r24). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
