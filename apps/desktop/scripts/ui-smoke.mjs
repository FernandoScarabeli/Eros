import { chromium } from 'playwright';
import assert from 'node:assert/strict';
(async () => {
  const browser = await chromium.launch({
    headless: true,
    executablePath: process.env.EROS_CHROMIUM_EXECUTABLE || undefined,
  });
  try {
    const page = await browser.newPage({
      viewport: { width: 1600, height: 1000 },
    });
    page.setDefaultTimeout(30000);
    const errors = [];
    page.on('pageerror', (e) => errors.push(e.message));
    await page.addInitScript(() => {
      window.isTauri = true;
      const projects = [
        { id: 'eros', name: 'EROS', path: '/test/eros' },
        { id: 'work', name: 'Trabalho', path: '/test/work' },
      ];
      const conversations = ['Frontend', 'Backend', 'Banco de dados'].map(
        (name, i) => ({
          id: 'p' + i,
          project_id: 'eros',
          name,
          cwd: '/test/eros',
          kind: 'codex',
          state: 'stopped',
          chat_status: ['normal', 'working', 'awaiting_decision'][i],
          provider_thread_id: null,
        }),
      );
      conversations.push(
        ...[
          'Autenticação do app',
          'Arquitetura da API',
          'Ideias e exploração',
        ].map((name, i) => ({
          id: 'c' + i,
          project_id: null,
          name,
          cwd: '/test/chats',
          kind: 'codex',
          state: 'stopped',
          chat_status: i === 0 ? 'completed_unread' : 'normal',
          provider_thread_id: null,
        })),
      );
      const snapshot = {
        protocol: 4,
        projects,
        conversations,
        workspaces: [],
        authenticated_providers: [
          { id: 'codex', label: 'Codex' },
          { id: 'claude', label: 'Claude' },
        ],
        active_scope: 'eros',
      };
      window.__calls = [];
      window.__TAURI_EVENT_PLUGIN_INTERNALS__ = { unregisterListener: () => {} };
      window.__TAURI_INTERNALS__ = {
        invoke: async (cmd, args) => {
          window.__calls.push({ cmd, args });
          if (cmd === 'snapshot') return structuredClone(snapshot);
          if (cmd === 'set_active_scope') {
            snapshot.active_scope = args.scope;
            return;
          }
          if (cmd === 'save_workspace') {
            snapshot.workspaces = snapshot.workspaces.filter(
              (w) => w.scope !== args.workspace.scope,
            );
            snapshot.workspaces.push(args.workspace);
            return args.workspace;
          }
          if (cmd === 'create_conversation') {
            const c = {
              id: args.id,
              project_id: args.projectId,
              name: args.name,
              cwd: args.cwd,
              kind: args.kind,
              state: args.kind === 'shell' ? 'running' : 'stopped',
              provider_thread_id: null,
            };
            snapshot.conversations.push(c);
            return c;
          }
          if (cmd === 'stop_conversation') {
            snapshot.conversations = snapshot.conversations.filter((c) => c.id !== args.conversationId);
            return;
          }
          if (cmd === 'plugin:dialog|open') return '/test/new-project';
          if (cmd === 'add_project') {
            const p = { id: 'new', name: 'Novo projeto', path: args.path };
            snapshot.projects.push(p);
            return p;
          }
          if (cmd === 'subscribe_activity') return;
          if (cmd === 'mark_conversation_read') {
            const c = snapshot.conversations.find(c => c.id === args.conversationId);
            c.chat_status = 'normal';
            c.activity_revision = (c.activity_revision ?? 0) + 1;
            return { conversation_id: c.id, chat_status: c.chat_status, activity_revision: c.activity_revision };
          }
          if (cmd === 'plugin:event|listen') return 1;
          if (
            cmd === 'plugin:event|unlisten' ||
            cmd === 'subscribe_terminal' ||
            cmd === 'unsubscribe_terminal' ||
            cmd === 'resize_terminal'
          )
            return;
          throw Error('Unexpected command ' + cmd);
        },
        transformCallback: () => 1,
        unregisterCallback: () => {},
      };
    });
    await page.goto(process.env.EROS_TEST_URL || 'http://127.0.0.1:1422');
    await page.evaluate(() => document.fonts.ready);
    await page
      .getByRole('img', {
        name: 'Codex conectado; dados de uso indisponíveis',
      })
      .waitFor();
    await page
      .getByText('Claude', { exact: true })
      .waitFor({ timeout: 7000 });
    await page.locator('.terminal-atmosphere canvas').waitFor();
    await page.locator('.text-type__cursor').waitFor();
    await page.waitForFunction(
      () => (document.querySelector('.text-type__content')?.textContent?.length ?? 0) > 0,
    );
    assert.equal(await page.locator('.text-type__cursor').textContent(), '|');
    await page.waitForTimeout(250);
    assert.equal(
      await page.locator('.terminal-atmosphere canvas').evaluate((canvas) => {
        return Boolean(canvas.getContext('webgl2'));
      }),
      true,
      'Desktop and web preview must use the same WebGL Faulty Terminal renderer',
    );
    assert.equal(
      await page.getByRole('button', { name: /Uso restante/ }).count(),
      0,
    );
    const iconIssues = await page
      .locator('.lucide-icon')
      .evaluateAll((icons) =>
        icons
          .filter((icon) => icon.getClientRects().length > 0)
          .filter((icon) => {
            const bounds = icon.getBoundingClientRect();
            return (
              icon.tagName.toLowerCase() !== 'svg' ||
              !bounds.width ||
              !bounds.height
            );
          })
          .map((icon) => icon.outerHTML),
      );
    assert.deepEqual(
      iconIssues,
      [],
      'Every UI icon must render as a visible Lucide SVG',
    );

    await page
      .getByRole('textbox', { name: 'Nome da nova conversa' })
      .fill('Construir autenticação');
    await page.getByRole('button', { name: 'Escolher projeto', exact: true }).click();
    await page.getByRole('listbox', { name: 'Escolher projeto' }).getByRole('option', { name: 'Trabalho', exact: true }).click();
    await page.getByRole('button', { name: 'Iniciar sessão', exact: true }).click();
    await page.getByRole('menu', { name: 'Iniciar com', exact: true }).waitFor();
    assert.equal(await page.getByRole('dialog').count(), 0);
    await page.keyboard.press('Escape');
    assert.equal(await page.getByRole('dialog').count(), 0);
    assert.equal(
      await page
        .getByRole('textbox', { name: 'Nome da nova conversa' })
        .inputValue(),
      'Construir autenticação',
    );
    await page
      .locator('.quick-actions')
      .getByRole('button', { name: 'Novo terminal', exact: true })
      .click();
    await page.getByRole('group', { name: 'Saída do terminal' }).waitFor();
    assert.equal(await page.getByRole('dialog').count(), 0);
    await page.getByRole('tab', { name: 'Início', exact: true }).waitFor();
    const calls = await page.evaluate(() => window.__calls);
    const create = calls.find((c) => c.cmd === 'create_conversation');
    assert.equal(create.args.kind, 'shell');
    assert.equal(create.args.projectId, 'work');
    assert.equal(await page.locator('.shell-tabs').count(), 0);
    assert.equal(await page.locator('.project-conversation').filter({ hasText: 'Terminal' }).count(), 0);
    await page.getByRole('button', { name: 'Encerrar terminal e seus processos', exact: true }).click();
    await page.waitForFunction(() => window.__calls.some((c) => c.cmd === 'stop_conversation'));
    await page.waitForFunction(() => !document.querySelector('[aria-label="Saída do terminal"]'));
    assert.equal(await page.locator('.shell-tabs').count(), 0);
    await page.getByRole('tab', { name: 'Início', exact: true }).click();
    await page
      .locator('.quick-actions')
      .getByRole('button', { name: 'Novo projeto', exact: true })
      .click();
    await page
      .getByRole('heading', { name: 'Novo projeto', exact: true })
      .waitFor();
    await page
      .getByRole('button', { name: 'Arquitetura da API, Normal', exact: true })
      .click();
    await page.getByRole('group', { name: 'Saída do terminal' }).waitFor();
    assert(
      (await page.evaluate(() => window.__calls)).some(
        (call) =>
          call.cmd === 'set_active_scope' && call.args.scope === 'conversation:c1',
      ),
    );
    assert.equal(errors.length, 0, errors.join('\n'));
    await page.setViewportSize({ width: 960, height: 640 });
    await page.getByRole('tab', { name: 'Início', exact: true }).click();
    assert.equal(
      await page.evaluate(
        () => document.documentElement.scrollWidth > innerWidth,
      ),
      false,
    );
    await page.setViewportSize({ width: 390, height: 844 });
    await page.reload();
    await page.evaluate(() => document.fonts.ready);
    await page
      .getByRole('button', { name: 'Recolher navegação', exact: true })
      .waitFor({ state: 'hidden' });
    await page
      .getByRole('button', { name: 'Alternar navegação', exact: true })
      .click();
    await page
      .getByRole('button', { name: 'Fechar navegação', exact: true })
      .click({ position: { x: 350, y: 100 } });
    assert.equal(
      await page.evaluate(
        () => document.documentElement.scrollWidth > innerWidth,
      ),
      false,
    );
    console.log(
      'PASS: controlled provider rotation, project selection, draft retention/menu Escape, shell creation, explicit shell stop, shell exclusion from chat history, absence of terminal strip, folder selection, 960px/390px overflow and mobile menu; no real provider calls.',
    );
    console.log('Errors:', errors);
  } finally {
    await browser.close();
  }
})();
