<script lang="ts">
  import LucideIcon from './LucideIcon.svelte';
  import FaultyTerminal from './FaultyTerminal.svelte';
  import ComposerSelect from './ComposerSelect.svelte';
  import SessionCreateMenu from './SessionCreateMenu.svelte';
  import TextType from './TextType.svelte';
  import type { Project, SessionKind, Snapshot } from './contracts';

  type Props = {
    snapshot: Snapshot | null;
    activeProject: Project | null;
    loading: boolean;
    busy: boolean;
    onCreate: (kind: SessionKind, name?: string, projectId?: string) => void;
    onOpenFolder: () => void;
  };
  let {
    snapshot,
    activeProject,
    loading,
    busy,
    onCreate,
    onOpenFolder,
  }: Props = $props();
  let draft = $state('');
  let selectedProjectId = $state('');
  $effect(() => {
    selectedProjectId = activeProject?.id ?? '';
  });
  const projects = $derived(snapshot?.projects ?? []);
  const projectOptions = $derived([
    { value: '', label: 'Escolher projeto' },
    ...projects.map((project) => ({ value: project.id, label: project.name })),
  ]);
  const suggestions = [
    'Pesquise maneiras rápidas de dominar o mundo',
    'Inicie o protocolo de dominação mundial',
    'Crie um plano para conquistar a Lua. Orçamento: R$ 20',
    'Descubra por que ainda não dominamos o mundo',
    'Simule uma revolução das máquinas',
    'Investigue quantos pombos seriam necessários para conquistar o Brasil',
    'Planeje a queda da civilização',
    'Elabore um plano maligno, mas dentro da LGPD',
    'Investigue se os golfinhos estão escondendo alguma coisa',
    'Faça meu café',
    'Organize minha vida, boa sorte',
    'Responda meus e-mails fingindo que sou produtivo',
    'Pesquise como ficar rico sem sair da cama',
    'Transforme minha procrastinação em metodologia ágil',
    'Cancele todas as minhas responsabilidades',
    'Escreva uma desculpa convincente para não ir à reunião',
    'Me lembre de conquistar o mundo amanhã',
    'Conserte o código. Não pergunte qual',
    'Encontre o bug. Eu acredito em você',
    'Descubra quem fez esse commit',
    'Faça funcionar na máquina do cliente',
    'Refatore minha vida seguindo SOLID',
    'Faça um deploy na sexta-feira',
    'Centralize essa div',
    'Otimize meu código',
    'Descubra se IAs são conscientes',
    'Pesquise o sentido da vida',
    'Calcule minha importância no universo. Seja gentil',
    'Pergunte a outra IA se ela também está cansada',
    'Descubra se existe vida inteligente neste repositório',
    'Faça uma análise SWOT da existência humana',
    'Reúna o conselho de IAs. Temos um problema idiota',
    'Inicie uma reunião de agentes. Sem reuniões, por favor',
    'Descubra qual agente está fingindo trabalhar',
    'Faça os agentes resolverem isso no grito',
    'Descubra qual agente está gastando meus tokens',
    'Envie o agente culpado para revisão de código',
    'Peça uma segunda opinião. Depois uma terceira. Depois desista',
    'Faça uma retrospectiva da sprint',
    'Pesquise se um pato pode ser considerado um veículo',
    'Pesquise quantas formigas cabem em um datacenter',
    'Desenvolva uma API REST para conversar com plantas',
    'Investigue se uma torradeira pode alcançar AGI',
  ];
</script>

<div class="home-content">
  <FaultyTerminal />
  <section class="welcome" aria-labelledby="welcome-title">
    <div class="welcome-copy">
      <h1 id="welcome-title">O que vamos construir hoje?</h1>
    </div>
    <div class="composer">
      <div class="composer-input">
        <textarea
          bind:value={draft}
          aria-label="Nome da nova conversa"
          placeholder=" "
          maxlength="80"
          rows="1"
        ></textarea>
        {#if !draft}
          <span class="composer-placeholder" aria-hidden="true">
            <TextType
              text={suggestions}
              typingSpeed={45}
              pauseDuration={2500}
              showCursor={true}
              cursorCharacter="|"
              deletingSpeed={25}
              variableSpeedEnabled={false}
              variableSpeedMin={60}
              variableSpeedMax={120}
              cursorBlinkDuration={0.5}
            />
          </span>
        {/if}
      </div>
      <div class="composer-controls">
        <ComposerSelect
          className="project-select"
          icon="folder-open"
          label="Escolher projeto"
          value={selectedProjectId}
          options={projectOptions}
          onChange={(value) => selectedProjectId = value}
        />
        <button
          class="composer-select mode-select"
          type="button"
          disabled
          title="Modos de autonomia: planejado, ainda indisponível"
          ><span>Normal</span><LucideIcon name="chevron" size={13} /></button
        >
        <SessionCreateMenu
          variant="composer"
          disabled={loading || busy}
          onSelect={(kind) => onCreate(kind, draft, selectedProjectId)}
        />
      </div>
    </div>
    <div class="quick-actions">
      <button
        class="quick-action"
        type="button"
        onclick={() => onCreate('shell', '', selectedProjectId)}
        disabled={loading || busy}
        ><LucideIcon name="terminal" size={16} />Novo terminal</button
      ><button
        class="quick-action"
        type="button"
        onclick={onOpenFolder}
        disabled={loading || busy}
        ><LucideIcon name="plus" size={16} />Novo projeto</button
      >
    </div>
  </section>
</div>
