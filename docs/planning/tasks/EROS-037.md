# EROS-037 — Pacote Ubuntu, versão e atualização conservadora

**Etapa:** C · **Prioridade:** P0 — gate de lançamento

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-001](EROS-001.md), [EROS-009](EROS-009.md), [EROS-015](EROS-015.md), [EROS-036](EROS-036.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Instalar o Eros como aplicativo real, sem exigir Rust/Node no PC de uso.

## Pontos de entrada

- apps/desktop/src-tauri/tauri.conf.json
- build/distribution
- docs/publishing.md
- UI Sobre

## Escopo e decisões de implementação

Definir Ubuntu suportado com evidência (testar 24.04 LTS e ambiente 26.04 disponível sem presumir paridade). Gerar .deb release, desktop entry, ícone, dependências runtime e instruções de instalação/remoção preservando dados. Sobre mostra versão/build e link de releases configurável. Checagem de atualização manual/opcional com timeout; sem URL fictícia. Documentar assinatura/checksums e política de compatibilidade do serviço; não substituir binário em sessões ativas.

## Critérios de aceite

- [ ] Instalação limpa abre shell real sem toolchain de desenvolvimento.
- [ ] Atualizar pacote preserva dados e informa serviço incompatível sem matar agentes.
- [ ] Sem endpoint real, atualização aparece indisponível honestamente; fluxo manual continua documentado.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Build e instalação em VM/container apropriado com desktop para prova nativa; upgrade/downgrade incompatível e uninstall/reinstall com dados temporários.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Instalar .deb no Ubuntu alvo e abrir pelo launcher.
2. Criar dados, atualizar pacote e verificar perfil/chats preservados.
3. Remover aplicativo e confirmar que dados pessoais não foram apagados silenciosamente.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-037.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Medir release instalado com W0; tamanho de pacote/runtime separado; não bundlar Node/Python por conveniência.

## Fora do escopo

Não publicar release, criar chave de assinatura em repositório ou assumir auto-update .deb pelo plugin.

## Bases da pesquisa

[R27](../RESEARCH.md#r27), [R28](../RESEARCH.md#r28). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
