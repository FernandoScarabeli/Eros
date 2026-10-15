import { chromium } from 'playwright';
import assert from 'node:assert/strict';

const browser = await chromium.launch({ executablePath: process.env.EROS_CHROMIUM_EXECUTABLE || undefined, headless: true });
try {
  const page = await browser.newPage({ viewport: { width: 1200, height: 800 } });
  const errors = [];
  page.on('pageerror', error => errors.push(error.message));
  await page.addInitScript(() => {
    window.isTauri = true;
    window.__failStartup = location.search.includes("startupFailure");
    window.__focused = true;
    Object.defineProperty(document, 'hasFocus', { value: () => window.__focused });
    const conversations = [
      { id: 'standalone', project_id: null, name: 'Chat avulso' },
      { id: 'other', project_id: null, name: 'Outro avulso' },
      { id: 'project-chat', project_id: 'project', name: 'Chat do projeto' },
      { id: 'project-other', project_id: 'project', name: 'Outro do projeto' },
    ].map(c => ({ ...c, cwd: '/fixture', kind: 'codex', state: 'stopped', provider_thread_id: null, pinned: false, chat_status: 'normal', activity_revision: 0 }));
    const snapshot = {
      protocol: 4, projects: [{ id: 'project', name: 'Projeto de teste', path: '/fixture' }], conversations,
      workspaces: [], authenticated_providers: [], sidebar_orders: { projects: [], chats: [] },
      active_scope: 'conversation:standalone', chat_default_folder: '/fixture', chat_default_folder_unavailable: false,
    };
    let next = 1;
    const callbacks = new Map();
    const listeners = new Map();
    window.__calls = [];
    window.__TAURI_EVENT_PLUGIN_INTERNALS__ = { unregisterListener: () => {} };
    const emit = (event, payload) => {
      for (const [id, listener] of listeners) {
        if (listener.event === event) callbacks.get(listener.handler)?.({ event, id, payload });
      }
    };
    let activityId;
    window.__emitActivity = (id, status, emitNow = true) => {
      const conversation = conversations.find(c => c.id === id);
      conversation.chat_status = status;
      conversation.activity_revision++;
      const payload = { conversation_id: id, chat_status: status, activity_revision: conversation.activity_revision };
      if (emitNow) emit(`eros-activity-${activityId}`, payload);
      return payload;
    };
    window.__disconnectActivity = () => emit(`eros-activity-disconnected-${activityId}`, null);
    window.__emitRawActivity = payload => emit(`eros-activity-${activityId}`, payload);
    window.__focusActivity = focused => {
      window.__focused = focused;
      window.dispatchEvent(new Event(focused ? 'focus' : 'blur'));
    };
    window.__TAURI_INTERNALS__ = {
      transformCallback: callback => { const id = next++; callbacks.set(id, callback); return id; },
      unregisterCallback: id => callbacks.delete(id),
      invoke: async (cmd, args = {}) => {
        window.__calls.push({ cmd, args });
        if (cmd === 'snapshot') {
          if (window.__failStartup) throw 'O serviço local está desatualizado.';
          return structuredClone(snapshot);
        }
        if (cmd === 'plugin:event|listen') { const id = next++; listeners.set(id, args); return id; }
        if (cmd === 'plugin:event|unlisten') { listeners.delete(args.eventId); return; }
        if (cmd === 'subscribe_activity') {
          activityId = args.subscriptionId;
          for (const c of conversations) emit(`eros-activity-${activityId}`, { conversation_id: c.id, chat_status: c.chat_status, activity_revision: c.activity_revision });
          return;
        }
        if (cmd === 'unsubscribe_terminal') return;
        if (cmd === 'mark_conversation_read') {
          const c = conversations.find(c => c.id === args.conversationId);
          if (c.chat_status === 'completed_unread' && c.activity_revision === args.activityRevision) window.__emitActivity(c.id, 'normal');
          return { conversation_id: c.id, chat_status: c.chat_status, activity_revision: c.activity_revision };
        }
        if (cmd === 'set_active_scope') { snapshot.active_scope = args.scope; return; }
        if (cmd === 'save_workspace') {
          snapshot.workspaces = snapshot.workspaces.filter(w => w.scope !== args.workspace.scope);
          snapshot.workspaces.push(args.workspace);
          return args.workspace;
        }
        if (cmd === 'resize_terminal' || cmd === 'write_terminal') return;
        throw new Error(`Unexpected controlled command: ${cmd}`);
      },
    };
  });
  await page.goto(process.env.EROS_TEST_URL || 'http://127.0.0.1:1422');
  await page.getByRole('button', { name: 'Chat avulso, Normal', exact: true }).waitFor();
  const status = async (id, value) => page.evaluate(([id, value]) => window.__emitActivity(id, value), [id, value]);
  await status('other', 'working');
  await page.getByRole('button', { name: 'Outro avulso, Trabalhando', exact: true }).waitFor();
  await status('other', 'awaiting_decision');
  await page.getByRole('button', { name: 'Outro avulso, Aguardando decisão', exact: true }).waitFor();
  await page.getByRole('button', { name: 'Outro avulso, Aguardando decisão', exact: true }).click();
  assert.equal(await page.locator('.chat-status-dot.awaiting_decision').count(), 1, 'Opening a decision does not answer it');
  await status('other', 'working');
  await status('standalone', 'completed_unread');
  await page.getByRole('button', { name: 'Chat avulso, Concluído · ainda não visto', exact: true }).waitFor();
  await page.getByRole('button', { name: 'Chat avulso, Concluído · ainda não visto', exact: true }).click();
  await page.getByRole('button', { name: 'Chat avulso, Normal', exact: true }).waitFor();
  const stale = await status('standalone', 'completed_unread');
  await page.getByRole('button', { name: 'Chat avulso, Normal', exact: true }).waitFor();
  await page.evaluate(event => window.__emitRawActivity(event), stale);
  assert.equal(await page.getByRole('button', { name: 'Chat avulso, Normal', exact: true }).count(), 1, 'Old event cannot overwrite read acknowledgement');
  await page.evaluate(() => window.__focusActivity(false));
  await status('standalone', 'completed_unread');
  await page.getByRole('button', { name: 'Chat avulso, Concluído · ainda não visto', exact: true }).waitFor();
  await page.evaluate(() => window.__focusActivity(true));
  await page.getByRole('button', { name: 'Chat avulso, Normal', exact: true }).waitFor();

  await status('project-chat', 'completed_unread');
  await page.getByRole('button', { name: 'Projeto de teste', exact: true }).click();
  await page.getByRole('button', { name: 'Chat do projeto, Concluído · ainda não visto', exact: true }).waitFor();
  await page.getByRole('button', { name: 'Chat do projeto, Concluído · ainda não visto', exact: true }).click();
  await page.getByRole('button', { name: 'Chat do projeto, Normal', exact: true }).waitFor();
  await status('project-other', 'awaiting_decision');
  await status('other', 'working');
  await status('standalone', 'completed_unread');
  await page.emulateMedia({ reducedMotion: 'reduce' });
  assert.equal(await page.locator('.chat-status-dot.working').evaluate(el => getComputedStyle(el).animationName), 'none');
  const sizes = await page.locator('.chat-status-dot').evaluateAll(dots => dots.map(dot => ({ width: dot.getBoundingClientRect().width, height: dot.getBoundingClientRect().height })));
  assert.ok(sizes.every(size => size.width === 7 && size.height === 7), 'Status dots stay circular in both lists');
  await page.screenshot({ path: process.env.EROS_ACTIVITY_SCREENSHOT || '/tmp/eros-chat-activity.png' });

  const before = await page.evaluate(() => window.__calls.filter(c => c.cmd === 'subscribe_activity').length);
  await page.evaluate(() => { window.__disconnectActivity(); window.__emitActivity('other', 'awaiting_decision', false); });
  await page.waitForFunction(count => window.__calls.filter(c => c.cmd === 'subscribe_activity').length > count, before);
  await page.getByRole('button', { name: 'Outro avulso, Aguardando decisão', exact: true }).waitFor();
  await page.goto((process.env.EROS_TEST_URL || 'http://127.0.0.1:1422') + '?startupFailure');
  await page.getByRole('alert').filter({ hasText: 'O serviço local está desatualizado.' }).waitFor();
  assert.equal(await page.evaluate(() => window.__calls.filter(c => c.cmd === 'subscribe_activity').length), 0, 'Do not start indicators before the service is ready');
  await page.evaluate(() => { window.__failStartup = false; });
  await page.getByRole('button', { name: 'Tentar novamente', exact: true }).click();
  await page.waitForFunction(() => window.__calls.some(c => c.cmd === 'subscribe_activity'));
  assert.deepEqual(errors, []);
  console.log('PASS: controlled live status events in standalone/project chats, background completion, focus/read acknowledgement, stale event protection, decision retention, reduced motion and reconnect replay. Zero provider calls.');
} finally {
  await browser.close();
}
