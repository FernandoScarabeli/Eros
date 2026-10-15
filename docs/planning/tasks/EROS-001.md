# EROS-001 — Build reproduzível, CI e documentação coerente

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** Nenhuma.

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Um checkout limpo deve compilar e executar os checks sem depender de arquivos privados ou instruções históricas incorretas.

## Pontos de entrada

- README.md
- README.pt-BR.md
- CONTRIBUTING.md
- docs/frontend-verification.md
- .github/workflows/repository.yml
- Cargo.toml
- apps/desktop/package.json

## Escopo e decisões de implementação

Documentar Rust 1.90, Node compatível com Vite 8 e dependências Ubuntu, incluindo libdbus-1-dev e librsvg2-dev. Fixar uma versão suportada de Node. Ampliar CI para Rust, contratos e frontend com lockfiles; acrescentar smokes controlados em job adequado. Separar build, testes controlados e aceitação nativa. Preservar registros antigos como históricos e corrigir instruções atuais.

## Critérios de aceite

- [ ] Checkout limpo instala dependências e compila; CI não usa credenciais nem legado.
- [ ] Comandos documentados correspondem aos manifests reais; versões incompatíveis falham com explicação.
- [ ] Nenhuma funcionalidade passa a ser anunciada como aceita apenas porque compilou.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Rodar foundation, testes Rust, contratos, check/build e smokes com doubles. Validar workflow em runner local/CI disponível, distinguindo inspeção YAML de execução hospedada. Registrar warnings preexistentes.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Preparar checkout temporário e seguir README do zero.
2. Abrir um shell no desktop e confirmar teclado/resize.
3. Repetir sem Rust ou headers em ambiente descartável: instrução deve permitir diagnosticar; limpar somente o checkout de teste.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-001.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Não fazer tuning ainda; preservar baseline para 002. Instalação de ferramentas é atividade de desenvolvimento, não dependência runtime.

## Fora do escopo

Não atualizar todo o dependency tree, publicar release ou instalar pacotes com sudo automaticamente.

## Bases da pesquisa

[R12](../RESEARCH.md#r12), [R27](../RESEARCH.md#r27). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
