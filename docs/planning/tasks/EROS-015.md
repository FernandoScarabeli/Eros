# EROS-015 — Diagnóstico local e logs seguros

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-003](EROS-003.md), [EROS-010](EROS-010.md), [EROS-011](EROS-011.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Permitir entender falhas sem expor credenciais ou coletar telemetria.

## Pontos de entrada

- eros-service
- apps/desktop/src-tauri/src/lib.rs
- settings Sobre/Diagnóstico
- scripts/

## Escopo e decisões de implementação

Logs estruturados com códigos, operation IDs, nível e rotação (proposta 5 arquivos de 2 MiB, ajustável dentro de teto). Não registrar prompts, argv/env completos, PTY, tokens nem transcritos. Diagnóstico mostra versões/saúde/paths mascarados e exporta relatório com preview e seleção. Error UI usa códigos recuperáveis e detalhes úteis. Comando de copiar suporte não inclui dados pessoais por default.

## Critérios de aceite

- [ ] Rotação funciona e não cresce indefinidamente.
- [ ] Erros do IPC/provedor são distinguíveis sem payload privado.
- [ ] Exportação tem preview; nenhum envio externo automático.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Segredos sentinela em erro/argv/paths, rotação, disco sem escrita e exportação; validar redaction por allowlist de campos.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Provocar falha de CLI double e abrir diagnóstico.
2. Gerar/exportar relatório e conferir sentinelas ausentes.
3. Repetir logs até limite e verificar rotação/continuidade do app.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-015.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Logging assíncrono ou limitado; sem fsync por tecla nem scanner contínuo do sistema.

## Fora do escopo

Não adicionar serviço de analytics ou botão que anexe logs automaticamente a issues.

## Bases da pesquisa

[R11](../RESEARCH.md#r11), [R12](../RESEARCH.md#r12), [R15](../RESEARCH.md#r15). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
