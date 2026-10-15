# EROS-012 — Meu perfil local com nome e foto

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-010](EROS-010.md), [EROS-011](EROS-011.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Permitir personalização pessoal sem cadastro ou serviço de nuvem.

## Pontos de entrada

- settings UI
- sidebar
- crates/eros-core/src/lib.rs e src/bin/contracts.rs
- crates/eros-service/src/store.rs
- bridge/assets locais

## Escopo e decisões de implementação

Nome de exibição de 1–80 caracteres e avatar local PNG/JPEG/WebP, até 5 MiB na importação. Validar conteúdo/dimensões, reencodar para miniatura até 256×256 e descartar metadados; guardar em diretório privado do app com ID estável. Substituir/remover sem arquivos órfãos. Iniciais como fallback. Nome Eros não altera identidade de commit Git.

## Critérios de aceite

- [ ] Nome/foto aparecem na sidebar e persistem.
- [ ] Arquivo inválido, imagem excessiva ou pasta sem escrita não destroem avatar anterior.
- [ ] Sem URL remota, token, SVG executável ou dependência do arquivo original.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Imagens pequenas, enormes em dimensões, formato falso, remoção/substituição e reabertura; teste da rota de asset sem path traversal.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Escolher imagem de teste e nome; reabrir.
2. Apagar imagem original: avatar permanece.
3. Importar arquivo inválido e depois remover foto: erro/fallback corretos.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-012.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Decodificar/reencodar fora da UI, limitar dimensões antes de alocação grande e servir miniatura cacheável.

## Fora do escopo

Não adicionar login Eros, sincronização ou usar nome pessoal para reconfigurar Git automaticamente.

## Bases da pesquisa

[R12](../RESEARCH.md#r12), [R26](../RESEARCH.md#r26), [R29](../RESEARCH.md#r29). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
