<script lang="ts">
  import { onMount } from 'svelte';
  import { Terminal } from '@xterm/xterm';
  import { FitAddon } from '@xterm/addon-fit';
  import { WebLinksAddon } from '@xterm/addon-web-links';
  import {
    writeToTerminal,
    resizeTerminal,
    onTerminalOutput,
    onTerminalClosed,
    onTerminalError,
  } from '$lib/api/terminal';
  import { markUnread, updateSessionStatus } from '$lib/stores/terminals';
  import { settings } from '$lib/stores/settings';
  import type { TerminalSession } from '$lib/types/terminal';

  interface Props {
    session: TerminalSession;
    active: boolean;
  }

  let { session, active }: Props = $props();

  let containerEl: HTMLDivElement;
  let terminal: Terminal | null = $state(null);
  let fitAddon: FitAddon | null = $state(null);
  let unlistenFns: (() => void)[] = [];
  let resizeObserver: ResizeObserver | null = null;
  let initialized = $state(false);

  onMount(() => {
    initTerminal();

    // Cleanup on unmount
    return () => {
      cleanup();
    };
  });

  async function initTerminal() {
    // Initialize terminal with font size from settings
    const fontSize = $settings.terminalFontSize;
    const term = new Terminal({
      cursorBlink: true,
      fontSize,
      fontFamily: 'Menlo, Monaco, "Courier New", monospace',
      theme: {
        background: '#0d0d0d',
        foreground: '#d4d4d4',
        cursor: '#d4d4d4',
        cursorAccent: '#0d0d0d',
        black: '#000000',
        red: '#cd3131',
        green: '#0dbc79',
        yellow: '#e5e510',
        blue: '#2472c8',
        magenta: '#bc3fbc',
        cyan: '#11a8cd',
        white: '#e5e5e5',
        brightBlack: '#666666',
        brightRed: '#f14c4c',
        brightGreen: '#23d18b',
        brightYellow: '#f5f543',
        brightBlue: '#3b8eea',
        brightMagenta: '#d670d6',
        brightCyan: '#29b8db',
        brightWhite: '#e5e5e5',
      },
    });

    // Add addons
    const fit = new FitAddon();
    term.loadAddon(fit);
    term.loadAddon(new WebLinksAddon());

    // Open terminal
    term.open(containerEl);

    // Store references
    terminal = term;
    fitAddon = fit;

    // Initial fit
    requestAnimationFrame(() => {
      fit.fit();
    });

    // Handle input - send to backend
    term.onData((data) => {
      writeToTerminal(session.id, data).catch((e) => {
        console.error('Failed to write to terminal:', e);
      });
    });

    // Listen for output from backend
    const unlistenOutput = await onTerminalOutput(session.id, (data) => {
      term.write(data);
      markUnread(session.id);
    });
    unlistenFns.push(unlistenOutput);

    // Listen for session close
    const unlistenClose = await onTerminalClosed(session.id, () => {
      updateSessionStatus(session.id, 'closed');
      term.write('\r\n\x1b[33m[Session closed]\x1b[0m\r\n');
    });
    unlistenFns.push(unlistenClose);

    // Listen for errors
    const unlistenError = await onTerminalError(session.id, (error) => {
      updateSessionStatus(session.id, 'error');
      term.write(`\r\n\x1b[31m[Error: ${error}]\x1b[0m\r\n`);
    });
    unlistenFns.push(unlistenError);

    // Setup resize observer
    resizeObserver = new ResizeObserver(() => {
      if (active && term && fit) {
        fit.fit();
        resizeTerminal(session.id, term.cols, term.rows).catch((e) => {
          console.error('Failed to resize terminal:', e);
        });
      }
    });
    resizeObserver.observe(containerEl);

    initialized = true;

    // Initial resize after a short delay
    setTimeout(() => {
      if (fit && term) {
        fit.fit();
        resizeTerminal(session.id, term.cols, term.rows).catch(() => {});
      }
    }, 100);
  }

  function cleanup() {
    // Cleanup event listeners
    unlistenFns.forEach((fn) => fn());
    unlistenFns = [];

    // Cleanup resize observer
    resizeObserver?.disconnect();
    resizeObserver = null;

    // Dispose terminal
    terminal?.dispose();
    terminal = null;
    fitAddon = null;
  }

  // Focus and fit when becoming active
  $effect(() => {
    if (active && initialized && terminal && fitAddon) {
      terminal.focus();
      fitAddon.fit();
    }
  });
</script>

<div
  class="terminal-pane"
  class:active
  bind:this={containerEl}
></div>

<style>
  .terminal-pane {
    flex: 1;
    display: none;
    background-color: #0d0d0d;
    padding: 4px;
    overflow: hidden;
  }

  .terminal-pane.active {
    display: block;
  }

  .terminal-pane :global(.xterm) {
    height: 100%;
    padding: 4px;
  }

  .terminal-pane :global(.xterm-viewport) {
    overflow-y: auto;
  }
</style>
