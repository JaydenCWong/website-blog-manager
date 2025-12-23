<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { settings } from "../stores/settings";

    interface BibEntry {
        key: string;
        entry_type: string;
        author: string;
        year: string;
        title: string;
        url: string | null;
        booktitle: string | null;
        pages: string | null;
        publisher: string | null;
        journal: string | null;
    }

    interface SyncResult {
        entries_synced: number;
        message: string;
    }

    let entries = $state<BibEntry[]>([]);
    let isLoading = $state(false);
    let isSyncing = $state(false);
    let message = $state<{ type: "success" | "error"; text: string } | null>(
        null,
    );

    async function loadReferences() {
        if (!$settings.repoPath) return;

        isLoading = true;
        message = null;

        try {
            entries = await invoke<BibEntry[]>("read_bib_file", {
                repoPath: $settings.repoPath,
            });
        } catch (err) {
            message = { type: "error", text: String(err) };
        } finally {
            isLoading = false;
        }
    }

    async function syncReferences() {
        if (!$settings.repoPath) return;

        isSyncing = true;
        message = null;

        try {
            const result = await invoke<SyncResult>("sync_references", {
                repoPath: $settings.repoPath,
            });
            message = { type: "success", text: result.message };
        } catch (err) {
            message = { type: "error", text: String(err) };
        } finally {
            isSyncing = false;
        }
    }

    // Load on mount
    $effect(() => {
        if ($settings.repoPath) {
            loadReferences();
        }
    });
</script>

<div class="bib-manager">
    <header class="bib-header">
        <div>
            <h2>References</h2>
            <p class="subtitle">BibTeX entries from your website repository</p>
        </div>
        <div class="header-actions">
            <button
                class="btn btn-secondary"
                onclick={loadReferences}
                disabled={isLoading}
            >
                <svg
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                >
                    <polyline points="23 4 23 10 17 10" />
                    <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10" />
                </svg>
                Refresh
            </button>
            <button
                class="btn btn-success"
                onclick={syncReferences}
                disabled={isSyncing}
            >
                <svg
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                >
                    <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
                    <polyline points="22 4 12 14.01 9 11.01" />
                </svg>
                {isSyncing ? "Syncing..." : "Sync to TypeScript"}
            </button>
        </div>
    </header>

    {#if message}
        <div
            class={message.type === "success"
                ? "success-message"
                : "error-message"}
        >
            {message.text}
        </div>
    {/if}

    {#if isLoading}
        <div class="loading">Loading references...</div>
    {:else if entries.length === 0}
        <div class="empty-state">
            <div class="empty-icon">📚</div>
            <h3>No References Found</h3>
            <p>
                Add entries to <code>src/lib/references.bib</code> in your website
                repository.
            </p>
        </div>
    {:else}
        <div class="entries-grid">
            {#each entries as entry}
                <article class="entry-card">
                    <div class="entry-header">
                        <span class="entry-type">{entry.entry_type}</span>
                        <code class="entry-key">{entry.key}</code>
                    </div>
                    <h3 class="entry-title">{entry.title}</h3>
                    <p class="entry-author">{entry.author}</p>
                    <div class="entry-meta">
                        <span class="entry-year">{entry.year}</span>
                        {#if entry.booktitle}
                            <span class="entry-booktitle"
                                >{entry.booktitle}</span
                            >
                        {/if}
                        {#if entry.journal}
                            <span class="entry-journal">{entry.journal}</span>
                        {/if}
                    </div>
                    {#if entry.url}
                        <a
                            href={entry.url}
                            target="_blank"
                            rel="noopener"
                            class="entry-url"
                        >
                            <svg
                                width="12"
                                height="12"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                            >
                                <path
                                    d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"
                                />
                                <polyline points="15 3 21 3 21 9" />
                                <line x1="10" y1="14" x2="21" y2="3" />
                            </svg>
                            View Source
                        </a>
                    {/if}
                </article>
            {/each}
        </div>
    {/if}
</div>

<style>
    .bib-manager {
        max-width: 1200px;
        margin: 0 auto;
    }

    .bib-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        margin-bottom: var(--space-xl);
        flex-wrap: wrap;
        gap: var(--space-md);
    }

    .bib-header h2 {
        font-size: var(--font-size-xl);
        color: var(--color-text-primary);
        margin-bottom: var(--space-xs);
    }

    .subtitle {
        font-size: var(--font-size-sm);
        color: var(--color-text-muted);
    }

    .header-actions {
        display: flex;
        gap: var(--space-sm);
    }

    .loading {
        text-align: center;
        padding: var(--space-2xl);
        color: var(--color-text-muted);
    }

    .empty-state {
        text-align: center;
        padding: var(--space-2xl);
    }

    .empty-icon {
        font-size: 3rem;
        margin-bottom: var(--space-md);
    }

    .empty-state h3 {
        font-size: var(--font-size-lg);
        color: var(--color-text-primary);
        margin-bottom: var(--space-sm);
    }

    .empty-state p {
        color: var(--color-text-secondary);
    }

    .empty-state code {
        background: var(--color-bg-secondary);
        padding: 2px 6px;
        border-radius: var(--radius-sm);
        font-size: var(--font-size-sm);
    }

    .entries-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
        gap: var(--space-lg);
    }

    .entry-card {
        background: var(--color-bg-card);
        border: 1px solid var(--color-border);
        border-radius: var(--radius-xl);
        padding: var(--space-lg);
        transition: all var(--transition-fast);
    }

    .entry-card:hover {
        border-color: var(--color-border-hover);
    }

    .entry-header {
        display: flex;
        align-items: center;
        gap: var(--space-sm);
        margin-bottom: var(--space-sm);
    }

    .entry-type {
        font-size: var(--font-size-xs);
        padding: var(--space-xs) var(--space-sm);
        background: rgba(59, 130, 246, 0.15);
        color: var(--color-accent-light);
        border-radius: var(--radius-sm);
        text-transform: uppercase;
        font-weight: 600;
    }

    .entry-key {
        font-size: var(--font-size-xs);
        color: var(--color-text-muted);
    }

    .entry-title {
        font-size: var(--font-size-base);
        font-weight: 600;
        color: var(--color-text-primary);
        margin-bottom: var(--space-xs);
        line-height: 1.4;
    }

    .entry-author {
        font-size: var(--font-size-sm);
        color: var(--color-text-secondary);
        margin-bottom: var(--space-sm);
    }

    .entry-meta {
        display: flex;
        flex-wrap: wrap;
        gap: var(--space-sm);
        margin-bottom: var(--space-sm);
        font-size: var(--font-size-xs);
        color: var(--color-text-muted);
    }

    .entry-url {
        display: inline-flex;
        align-items: center;
        gap: var(--space-xs);
        font-size: var(--font-size-xs);
        color: var(--color-accent);
    }

    .entry-url:hover {
        text-decoration: underline;
    }
</style>
