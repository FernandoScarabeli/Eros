# Retomar o Eros em outro PC

Estado salvo em **2026-10-15**. O próximo agente deve ler `AGENTS.md`,
[o plano](README.md), [o Kanban](KANBAN.md) e
[o contrato de execução](AGENT_GUIDE.md).

## Estado das tarefas

- **EROS-001: Aceita no escopo de build/documentação/checks controlados.**
  CI hospedado, instalação/aceitação gráfica e provedores reais não estão
  certificados por esse aceite. Consulte [a entrega](deliveries/EROS-001.md).
- **EROS-002: Em revisão e incompleta.** A implementação e a segunda revisão
  do medidor foram preservadas, com 25 testes Python e dez testes do probe
  relatados como passando. O baseline nativo completo permanece pendente.
- **EROS-004: Pronta**, mas a sequência recomendada prioriza concluir a medição.
- EROS-003/005/006 continuam sem liberação enquanto dependerem de EROS-002.

Fonte de verdade da atualização mais recente:
[Revisão 2 da entrega EROS-002](deliveries/EROS-002.md#revisão-2--correções-do-medidor-e-bloqueio-de-execução).
A [revisão anterior](reviews/EROS-002.md) contém achados e evidências históricas;
não atribuir seus resultados ao código corrigido sem nova execução.

## Próximo trabalho

1. Preparar o ambiente conforme [development.md](../development.md).
2. Conferir as correções e executar testes unitários/probe e checks pertinentes.
3. Compilar o release instrumentado e fazer a triagem nativa curta em sessão
   gráfica com sockets/PTYs permitidos.
4. Confirmar/refutar a hipótese de identidade dos painéis, verificar os novos
   métodos e preservar falhas reais do produto como resultados negativos.
5. Após triagem confiável, executar duas rodadas completas comparáveis,
   medir/delimitar overhead e interromper uma rodada com fixture Eros viva.
6. Atualizar a entrega para revisão, sem marcar Aceita por conta própria.

Comandos iniciais, da raiz do repositório:

```bash
npm --prefix apps/desktop ci
python3 -W error::ResourceWarning -m unittest discover -s scripts/performance -p 'test_*.py' -v
node apps/desktop/scripts/benchmark-probe.test.mjs
npm --prefix apps/desktop run check
(cd apps/desktop && VITE_EROS_BENCHMARK=1 npm run tauri -- build --no-bundle -- --locked)
python3 scripts/performance/run.py --workloads W0,W1,W2,W3 --cold-samples 1 --warm-samples 1 --idle-seconds 5 --pressure-seconds 1 --cycles 1 --output /tmp/eros-home-triage.json
```

Triagem curta não certifica metas. Só seguir para as rodadas completas após
interpretar o resultado. Veja [o README do medidor](../../scripts/performance/README.md)
para parâmetros, classificação de falhas, saída e cleanup. Restaurar o build
normal conforme esse README ao terminar as medições.

## Limites preservados

Na última sessão de implementação, a política rejeitou execução fora do sandbox
antes de lançar a fixture. Esse bloqueio é específico daquela sessão e não
prova falha de GTK no código novo. Não contornar política de aprovação.

Na revisão nativa histórica, o catálogo de 10.000 chats excedeu o limite IPC;
essa é uma falha de capacidade relacionada à EROS-005. A hipótese de reutilização
de painel ainda requer confirmação com o diagnóstico novo. Não aumentar o
limite, reduzir a fixture ou corrigir o runtime dentro da tarefa de medição.

Nenhum teste real de inferência está autorizado por este handoff. Usar somente
fixtures, shells e doubles locais; não ler transcritos/credenciais ou parar
serviços pessoais. Não transferir `node_modules`, `target`, caches, dados de
runtime ou autenticações para o outro PC. Relatórios públicos têm somente
dados sintéticos/agregados e caminhos pessoais mascarados.
