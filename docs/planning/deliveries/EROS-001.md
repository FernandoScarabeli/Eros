# EROS-001 — Build reproduzível, CI e documentação coerente

Status: **Aceita após revisão**, para o escopo de build, documentação e checks
controlados. Entrega inicial: Em revisão. Responsável pela implementação: Codex;
checkout principal, branch `main`. Revisão posterior: agente principal deste
planejamento. Data: 2026-10-15. Aceitação nativa gráfica, provedores reais e
execução hospedada do workflow não estão incluídas neste aceite.

## Resultado e arquivos

Documentação atual centralizada em [development.md](../../development.md): Ubuntu
24.04 LTS x86_64, Rust 1.90.0 e Node 22.22.1; pacotes nativos incluem DBus e
librsvg; instalação com `npm ci` e Cargo `--locked`; desenvolvimento, build,
smokes, diagnóstico e aceitação manual separados. O `--locked` dos comandos Tauri
é encaminhado ao Cargo após um segundo separador `--`.

Arquivos desta tarefa:

- `.nvmrc`: versão definida de Node.
- `apps/desktop/package.json`, `apps/desktop/package-lock.json` e
  `apps/desktop/.npmrc`: engines correspondente e rejeição de Node incompatível.
  Nenhuma dependência foi atualizada; apenas metadados do pacote raiz no lockfile.
- `.github/workflows/repository.yml`: fundação preservada; jobs separados para
  Rust/contratos/build/smoke nativo controlado e frontend/check/build/smokes de
  navegador. Instalações nativas e Chromium explícitas; timeout e cleanup de preview.
- `README.md`, `README.pt-BR.md`, `CONTRIBUTING.md`, `docs/architecture.md` e
  `docs/frontend-verification.md`: instruções atuais corrigidas e referências ao
  guia. Resultados históricos preservados como evidência de sua própria rodada.
- Este relatório e somente a linha EROS-001 de `docs/planning/KANBAN.md`.

Estado prévio preservado: alterações em `AGENTS.md`, `ROADMAP.md`, remoção de 45
linhas de metadados libc no lockfile npm e diretório `docs/planning/` não rastreado.
A tarefa não reverteu nem se apropriou dessas mudanças. O resumo inicial do Kanban
não foi recalculado, conforme a instrução de alterar somente a linha EROS-001.

## Verificação realmente executada

Ambiente local: Ubuntu 26.04.1 LTS x86_64, Rust 1.90.0, Node 22.22.1, npm 9.2.0,
Python 3.14. Headers detectados por pkg-config: WebKitGTK 2.52.6, DBus 1.16.2,
librsvg 2.61.3. O alvo Ubuntu 24.04 está configurado no workflow, mas não foi
executado neste ambiente. Não se instalou pacote via sudo.

| Comando | Resultado real |
| --- | --- |
| `python3 scripts/check_repository.py` | Passou; arquivos obrigatórios, links locais e exclusão de caminhos privados/legado |
| `cargo test --workspace --locked` | Passou: 14 testes (1 core + 13 service), nenhum ignorado/falho |
| `cargo run --locked -p eros-core --bin contracts -- --check` | Passou, sem alteração dos contratos |
| `npm ci` em `apps/desktop` | Passou: 58 pacotes instalados; lockfile preservado |
| `npm run check` | Passou: zero erros, nove warnings preexistentes em três arquivos |
| `npm run build` | Passou: 217 módulos, JS 499,78 kB / gzip 138,70 kB; sem tuning |
| `cargo build --locked -p eros-desktop` | Passou, binário debug |
| `npm run tauri -- dev --help` e `npm run tauri -- build --help` | Passaram; conferidos argumentos e encaminhamento ao runner |
| `npm run tauri -- build --debug --no-bundle -- --locked` | Passou, incluindo beforeBuildCommand e compilação desktop; sem pacote/publicação |
| `python3 scripts/check_chat_activity.py` | Falhou: serviço controlado não iniciou dentro do limite; fixture removida pelo finally |
| Diagnóstico isolado de bind Unix e socket TCP em Python | Ambos negados com `PermissionError: Operation not permitted`; limitação do sandbox |
| `npx playwright install chromium` | Falhou ao criar cache padrão fora das raízes graváveis |
| `PLAYWRIGHT_BROWSERS_PATH=/tmp/eros001-browsers npx playwright install chromium` | Falhou: DNS `ENOTFOUND cdn.playwright.dev`; alternativa com cache gravável tentada |
| `npm run preview` | Falhou: `listen EPERM 127.0.0.1:1422` |
| `npm run test:ui` e `npm run test:activity` | Ambos falharam antes dos cenários: executável Chromium ausente |
| `npm ci --offline` em fixture com engine incompatível | Falhou como esperado, exit 1 e `EBADENGINE` |
| `env PATH=/nonexistent /bin/sh -c 'cargo --version'` | Falhou como esperado: `cargo: not found`; sem desinstalar Rust |
| YAML via PyYAML e todos os blocos run via `bash -n` | Passaram; três jobs presentes e sintaxe shell válida |
| `git diff --check` | Passou |

Também foi criada uma cópia pública descartável em `/tmp/eros001-clean-*` usando
`git ls-files --cached --others --exclude-standard`, copiando somente esses
arquivos e inicializando Git. Sem legado, diretórios privados, `target` ou
`node_modules` anteriores. Nela passaram novamente fundação, `npm ci`, check/build,
`cargo test --workspace --locked`, contratos e build nativo, com target novo.
Caches de pacotes e ferramentas do host continuaram disponíveis: isto demonstra
independência de arquivos locais privados, não instalação num SO vazio nem
execução Ubuntu 24.04. A cópia descartável foi removida após a verificação.

Warnings mantidos: `TerminalPane.svelte` (dois de semântica de clique/teclado),
`LayoutTree.svelte` (tabindex, eventos em elemento não interativo e `splitElement`
sem `$state`) e `App.svelte` (três roles dialog em section e label sem controle).
O Vite também informa ausência de configuração Svelte e uso dos defaults.
Nenhum warning Rust foi emitido nas compilações verificadas. fmt/clippy não foram
executados nem promovidos a gates; o guia já advertia sobre violações existentes.

## Limitações e pendências

- Workflow GitHub Actions **configurado e inspecionado**, não disparado nem
  acompanhado remotamente. Não houve ação externa, login ou inferência.
- Smokes controlados precisam ser repetidos no runner sem as restrições locais;
  nenhum resultado deles foi declarado aprovado nesta rodada.
- Ubuntu 24.04 limpo, build release, pacote .deb, diálogo nativo, teclado/resize
  gráfico e reconnect nativo não foram aceitos. Build debug não comprova isso.
- Provedores reais continuam opt-in e fora desta tarefa; não há versão de CLI
  certificada por este build. Nenhuma credencial é necessária aos checks comuns.
- Sem mudanças de runtime, contratos, migrações ou sessões existentes. Não se
  iniciou serviço pessoal. Impacto de desempenho: não aplicável ao runtime;
  baseline/latência/PSS continuam para EROS-002, sem alegação de otimização.

## How to test

### Preparação e checkout público

Após integrar os arquivos desta entrega, em Bash a partir da raiz do repositório:

```bash
checkout=$(mktemp -d /tmp/eros001-review.XXXXXX)
git clone --no-local . "$checkout/source"
cd "$checkout/source"
```

O clone só contém commits: antes da integração, use uma cópia pública dos arquivos
revisados, nunca copie `legado/`, caches ou dados pessoais. Prepare Ubuntu 24.04,
Rust e Node seguindo [o guia](../../development.md#prepare-ubuntu). Rede é necessária
para downloads de dependências/Chromium; login de provedor não é necessário.

### Comandos e ações controladas

Na raiz do checkout temporário:

```bash
python3 scripts/check_repository.py
cargo test --workspace --locked
cargo run --locked -p eros-core --bin contracts -- --check
cd apps/desktop
npm ci
npm run check
npm run build
npm run tauri -- build --debug --no-bundle -- --locked
cd ../..
cargo build --locked -p eros-desktop
python3 scripts/check_chat_activity.py
```

Esperado: cada comando termina com exit 0; contratos sem drift; check sem erros
(os nove warnings acima continuam visíveis); smoke imprime PASS e zero chamadas
a provedores. Falhas são blockers para o check correspondente, não sucesso parcial.

Depois, em `apps/desktop`, instale Chromium e mantenha o preview no terminal A:

```bash
npx playwright install chromium
npm run preview
```

No terminal B, também em `apps/desktop`:

```bash
npm run test:ui
npm run test:activity
```

Esperado: ambos passam, cobrindo navegação/fluxos anteriores e atividade sintética,
sem exceções de navegador. Encerre somente esse preview com Ctrl+C no terminal A.

### Ação e regressão nativa (pendente de execução gráfica)

Siga os comandos completos da seção
[native acceptance](../../development.md#development-and-native-acceptance):
fixture com shell sem arquivos de startup, diretórios Eros isolados, serviço próprio
com PID capturado e `npm run tauri -- dev -- --locked`. Escolha **New terminal**,
execute `printf 'eros-shell-ok\n'`, redimensione a janela e digite novamente.
Esperado: echo/saída e teclado funcionam após resize; fechar o painel shell encerra
seu processo. Não selecione Codex/Claude. Esta ação precisa de desktop gráfico e
não foi realizada aqui. Aceitação de provedor/reconnect é uma rodada distinta.

### Falha e diagnóstico seguro

Sem alterar o Node do host, crie uma fixture negativa na raiz do checkout:

```bash
negative=$(mktemp -d /tmp/eros001-engine.XXXXXX)
cp apps/desktop/package.json apps/desktop/package-lock.json apps/desktop/.npmrc "$negative/"
python3 - "$negative/package.json" <<'PY'
import json, sys
from pathlib import Path
p = Path(sys.argv[1])
data = json.loads(p.read_text())
data['engines']['node'] = '0.0.1'
p.write_text(json.dumps(data))
PY
(cd "$negative" && npm ci --offline)
```

Esperado: exit diferente de zero com `EBADENGINE`, antes de instalar dependências.
Não altere o manifesto real para permitir uma versão incompatível.
Para simular Rust ausente sem desinstalá-lo:

```bash
env PATH=/nonexistent /bin/sh -c 'cargo --version'
```

Esperado: `cargo: not found`; a preparação explica a instalação do toolchain.
Para headers ausentes, use apenas VM descartável Ubuntu 24.04 sem os pacotes
nativos: o build deve falhar indicando biblioteca via pkg-config, e a preparação
permite instalar o pacote faltante. Esse cenário de VM não foi executado aqui;
não remova headers da máquina pessoal para reproduzi-lo.

### Limpeza

Feche os painéis shell e o app; encerre somente o PID de serviço da fixture,
conforme a checagem de executável e limpeza do guia. Não mate serviços Eros de
outros diretórios. Após parar os processos da fixture, saia do checkout:

```bash
cd /tmp
case "$negative" in /tmp/eros001-engine.*) rm -r -- "$negative" ;; esac
case "$checkout" in /tmp/eros001-review.*) rm -r -- "$checkout" ;; esac
```

Não use `git clean`, reset, remoção de caches globais ou diretórios pessoais.

## Revisão posterior — 2026-10-15

A revisão conferiu os diffs, os manifests, o guia, o workflow e as evidências
da entrega. Reexecutou fundação e validação de whitespace com sucesso. Os
resultados de Rust/build/checkout limpo da entrega inicial permanecem atribuídos
àquela rodada; não foram todos repetidos pelo revisor.

Os bloqueios dos smokes locais foram resolvidos usando execução autorizada fora
do sandbox, sem alterar as permissões do aplicativo ou iniciar serviço pessoal:

| Verificação adicional | Resultado |
| --- | --- |
| `python3 scripts/check_chat_activity.py` | PASS: PTYs, IPC, SQLite e hooks reais com doubles Codex/Claude; zero chamadas aos provedores |
| `PLAYWRIGHT_BROWSERS_PATH=/tmp/eros-review-browsers npx playwright install chromium` | Passou; browser de testes instalado em diretório temporário, sem instalar pacotes do sistema |
| Bloco de browser smokes extraído do workflow, executado com o cache acima | `test:ui` e `test:activity` passaram; UI sem exceções de navegador, zero chamadas aos provedores |
| YAML e `bash -n` de todos os blocos `run` | Passaram após o ajuste de cleanup |
| Falha simulada antes do primeiro smoke, mantendo o trap do workflow | Exit 23 esperado; preview encerrado e porta 1422 livre |

A revisão ajustou o workflow para iniciar o Vite diretamente com Node. Assim o
PID capturado pertence ao servidor, em vez de depender do encerramento do npm
para parar um filho. O trap tolera processo já encerrado e o readiness check
detecta saída antecipada. O bloco completo passou e a falha simulada confirmou
cleanup. Não houve alteração de código de produção.

Para reproduzir o teste de navegador desta rodada, use o fluxo documentado de
preview em um terminal e smokes em outro, com
`PLAYWRIGHT_BROWSERS_PATH=/tmp/eros-review-browsers` definido nos comandos de
instalação e teste. O workflow usa seu cache padrão no runner; não depende desse
caminho temporário. A inspeção YAML e a execução local do bloco **não** comprovam
uma execução hospedada em Ubuntu 24.04.

Aceite restrito à EROS-001: build/checks documentados e smokes controlados
verificados; não certifica instalação gráfica, reconexão com provedores reais,
pacote release ou desempenho. Essas provas continuam nos cartões próprios.
EROS-002 e EROS-004 ficam prontas conforme suas dependências, priorizando EROS-002.
