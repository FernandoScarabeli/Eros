# EROS-034 — Restauração, retenção e limpeza com recuperação

**Etapa:** B · **Prioridade:** P1 — workspace

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-009](EROS-009.md), [EROS-033](EROS-033.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Recuperar dados com proteção da instalação atual e controlar crescimento local.

## Pontos de entrada

- backup/store
- settings Dados
- serviço lifecycle

## Escopo e decisões de implementação

Validar manifesto, checksums, schema e caminhos antes de aplicar; evitar zip-slip/symlinks/bombas com limites. Criar backup de segurança da base atual. Restauração é substituição explícita nesta etapa, sem merge complexo; sessões vivas exigem encerramento consciente antes. Descrever caminhos de projeto ausentes e ausência de processos recuperados. Limpeza scoped de caches/logs/arquivados com preview/retencão, sem apagar transcritos/projetos.

## Critérios de aceite

- [ ] Arquivo corrupto/incompatível não modifica instalação.
- [ ] Falha ao trocar base permite rollback testado.
- [ ] Restore nunca recria PTYs nem executa prompts; limpeza lista efeitos antes.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Backup válido/corrupto/futuro, traversal, limite de expansão, falha de troca e sessão ativa; testar recuperação do backup pré-restore.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Restaurar backup sintético numa instalação temporária e conferir perfil/chats.
2. Tentar arquivo corrompido: dados anteriores permanecem.
3. Executar limpeza com preview e verificar só itens selecionados; remover ambiente temporário.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-034.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Limitar streaming/expansão e verificar antes de alocar; manutenção não trava UI.

## Fora do escopo

Não migrar pastas de projeto, mesclar bancos arbitrários nem restaurar credenciais externas.

## Bases da pesquisa

[R22](../RESEARCH.md#r22), [R23](../RESEARCH.md#r23). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
