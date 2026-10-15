<script lang="ts">
  import ArrowDown from '@lucide/svelte/icons/arrow-down';
  import ArrowLeft from '@lucide/svelte/icons/arrow-left';
  import ArrowRight from '@lucide/svelte/icons/arrow-right';
  import ArrowUp from '@lucide/svelte/icons/arrow-up';
  import Ellipsis from '@lucide/svelte/icons/ellipsis';
  import EllipsisVertical from '@lucide/svelte/icons/ellipsis-vertical';
  import Bot from '@lucide/svelte/icons/bot';
  import Brain from '@lucide/svelte/icons/brain';
  import Network from '@lucide/svelte/icons/network';
  import Columns3 from '@lucide/svelte/icons/columns-3';
  import ChevronRight from '@lucide/svelte/icons/chevron-right';
  import Clock from '@lucide/svelte/icons/clock';
  import Folder from '@lucide/svelte/icons/folder';
  import FolderOpen from '@lucide/svelte/icons/folder-open';
  import Gauge from '@lucide/svelte/icons/gauge';
  import GitBranch from '@lucide/svelte/icons/git-branch';
  import Columns2 from '@lucide/svelte/icons/columns-2';
  import Rows2 from '@lucide/svelte/icons/rows-2';
  import Maximize from '@lucide/svelte/icons/maximize';
  import MessageSquare from '@lucide/svelte/icons/message-square';
  import PanelLeftClose from '@lucide/svelte/icons/panel-left-close';
  import PanelRightClose from '@lucide/svelte/icons/panel-right-close';
  import Plus from '@lucide/svelte/icons/plus';
  import Pin from '@lucide/svelte/icons/pin';
  import PinOff from '@lucide/svelte/icons/pin-off';
  import Search from '@lucide/svelte/icons/search';
  import Shell from '@lucide/svelte/icons/shell';
  import Settings from '@lucide/svelte/icons/settings';
  import Shapes from '@lucide/svelte/icons/shapes';
  import Smartphone from '@lucide/svelte/icons/smartphone';
  import Terminal from '@lucide/svelte/icons/terminal';
  import Trash2 from '@lucide/svelte/icons/trash-2';
  import UserRound from '@lucide/svelte/icons/user-round';
  import X from '@lucide/svelte/icons/x';

  type IconName =
    | 'arrow-left'
    | 'arrow-right'
    | 'arrow-down'
    | 'arrow-up'
    | 'bot'
    | 'brain'
    | 'network'
    | 'kanban'
    | 'chat'
    | 'chevron'
    | 'clock'
    | 'folder'
    | 'folder-open'
    | 'ellipsis'
    | 'ellipsis-vertical'
    | 'gauge'
    | 'git-branch'
    | 'split-horizontal'
    | 'split-vertical'
    | 'maximize'
    | 'panel-left-close'
    | 'panel-right-close'
    | 'plus'
    | 'pin'
    | 'pin-off'
    | 'search'
    | 'shell'
    | 'settings'
    | 'shapes'
    | 'smartphone'
    | 'terminal'
    | 'trash'
    | 'user'
    | 'x';

  type Props = {
    name: IconName;
    size?: number;
    className?: string;
    strokeWidth?: number;
  };

  let { name, size = 16, className = '', strokeWidth = 1.5 }: Props = $props();

  const icons = {
    'arrow-left': ArrowLeft,
    'arrow-right': ArrowRight,
    'arrow-down': ArrowDown,
    'arrow-up': ArrowUp,
    bot: Bot,
    brain: Brain,
    network: Network,
    kanban: Columns3,
    chat: MessageSquare,
    chevron: ChevronRight,
    clock: Clock,
    ellipsis: Ellipsis,
    'ellipsis-vertical': EllipsisVertical,
    folder: Folder,
    'folder-open': FolderOpen,
    gauge: Gauge,
    'git-branch': GitBranch,
    'split-horizontal': Columns2,
    'split-vertical': Rows2,
    maximize: Maximize,
    'panel-left-close': PanelLeftClose,
    'panel-right-close': PanelRightClose,
    plus: Plus,
    pin: Pin,
    'pin-off': PinOff,
    search: Search,
    shell: Shell,
    settings: Settings,
    shapes: Shapes,
    smartphone: Smartphone,
    terminal: Terminal,
    trash: Trash2,
    user: UserRound,
    x: X,
  };
  const Icon = $derived(icons[name]);
</script>

<Icon
  class={`lucide-icon ${className}`}
  size={size}
  strokeWidth={strokeWidth}
  aria-hidden="true"
  style={`width:${size}px;height:${size}px`}
/>
