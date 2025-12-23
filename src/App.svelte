<script lang="ts">
  import PostForm from "./lib/components/PostForm.svelte";
  import BibManager from "./lib/components/BibManager.svelte";
  import Settings from "./lib/components/Settings.svelte";
  import { settings } from "./lib/stores/settings";

  let activeTab = $state<"posts" | "references" | "settings">("posts");

  // Check if repo path is configured
  let needsSetup = $derived(!$settings.repoPath);
</script>

<div class="app">
  <header class="app-header">
    <div class="logo">
      <svg
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
      >
        <path
          d="M12 20h9M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"
        />
      </svg>
      <h1>Blog Manager</h1>
    </div>

    <nav class="tabs">
      <button
        class="tab"
        class:active={activeTab === "posts"}
        onclick={() => (activeTab = "posts")}
      >
        <svg
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <path
            d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"
          />
          <polyline points="14 2 14 8 20 8" />
          <line x1="16" y1="13" x2="8" y2="13" />
          <line x1="16" y1="17" x2="8" y2="17" />
          <polyline points="10 9 9 9 8 9" />
        </svg>
        New Post
      </button>
      <button
        class="tab"
        class:active={activeTab === "references"}
        onclick={() => (activeTab = "references")}
      >
        <svg
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20" />
          <path
            d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"
          />
        </svg>
        References
      </button>
      <button
        class="tab"
        class:active={activeTab === "settings"}
        onclick={() => (activeTab = "settings")}
      >
        <svg
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <circle cx="12" cy="12" r="3" />
          <path
            d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"
          />
        </svg>
        Settings
      </button>
    </nav>
  </header>

  <main class="app-main">
    {#if needsSetup}
      <div class="setup-notice">
        <div class="setup-icon">⚙️</div>
        <h2>Welcome to Blog Manager</h2>
        <p>
          To get started, configure your website repository path in Settings.
        </p>
        <button
          class="btn btn-primary"
          onclick={() => (activeTab = "settings")}
        >
          Go to Settings
        </button>
      </div>
    {:else if activeTab === "posts"}
      <PostForm />
    {:else if activeTab === "references"}
      <BibManager />
    {:else if activeTab === "settings"}
      <Settings />
    {/if}
  </main>
</div>

<style>
  .app {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .app-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-md) var(--space-xl);
    background: var(--color-bg-secondary);
    border-bottom: 1px solid var(--color-border);
  }

  .logo {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    color: var(--color-text-primary);
  }

  .logo h1 {
    font-size: var(--font-size-lg);
    font-weight: 600;
  }

  .logo svg {
    color: var(--color-accent);
  }

  .tabs {
    display: flex;
    gap: var(--space-xs);
  }

  .tab {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
    padding: var(--space-sm) var(--space-md);
    font-size: var(--font-size-sm);
    font-weight: 500;
    color: var(--color-text-secondary);
    border-radius: var(--radius-lg);
    transition: all var(--transition-fast);
  }

  .tab:hover {
    color: var(--color-text-primary);
    background: var(--color-bg-hover);
  }

  .tab.active {
    color: var(--color-accent-light);
    background: rgba(59, 130, 246, 0.1);
  }

  .app-main {
    flex: 1;
    overflow: auto;
    padding: var(--space-xl);
  }

  .setup-notice {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    text-align: center;
    gap: var(--space-md);
  }

  .setup-icon {
    font-size: 3rem;
  }

  .setup-notice h2 {
    font-size: var(--font-size-2xl);
    color: var(--color-text-primary);
  }

  .setup-notice p {
    color: var(--color-text-secondary);
    max-width: 400px;
  }
</style>
