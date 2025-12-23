<script lang="ts">
    import { settings } from "../stores/settings";

    let repoPath = $state($settings.repoPath);
    let isSaved = $state(false);

    function handleSubmit(e: Event) {
        e.preventDefault();
        settings.setRepoPath(repoPath.trim());
        isSaved = true;
        setTimeout(() => {
            isSaved = false;
        }, 2000);
    }

    function useDefaultPath() {
        repoPath = "F:\\website";
    }
</script>

<div class="settings">
    <h2>Settings</h2>

    <form onsubmit={handleSubmit}>
        <div class="setting-group">
            <label for="repoPath">Website Repository Path</label>
            <p class="setting-description">
                The absolute path to your website repository. This is where blog
                posts and references will be created.
            </p>
            <div class="path-input-wrapper">
                <input
                    id="repoPath"
                    type="text"
                    bind:value={repoPath}
                    placeholder="e.g., F:\website or /Users/you/website"
                />
                <button
                    type="button"
                    class="btn btn-secondary"
                    onclick={useDefaultPath}
                >
                    Use Default
                </button>
            </div>
        </div>

        <div class="form-actions">
            <button type="submit" class="btn btn-primary">
                {isSaved ? "✓ Saved!" : "Save Settings"}
            </button>
        </div>
    </form>

    <hr />

    <div class="info-section">
        <h3>About</h3>
        <p>
            Blog Manager is a desktop app for creating blog posts and syncing
            BibTeX references. Built with <a
                href="https://tauri.app"
                target="_blank"
                rel="noopener">Tauri</a
            >
            and
            <a href="https://svelte.dev" target="_blank" rel="noopener"
                >Svelte</a
            >.
        </p>

        <h4>Expected Repository Structure</h4>
        <pre><code
                >website/
├── src/
│   ├── lib/
│   │   ├── references.bib    ← BibTeX source
│   │   └── data/
│   │       └── references.ts ← Generated TypeScript
│   └── routes/
│       └── blog/
│           ├── +page.svelte  ← Blog listing
│           └── {slug}/       ← Post directories
│               └── +page.md</code
            ></pre>
    </div>
</div>

<style>
    .settings {
        max-width: 600px;
        margin: 0 auto;
    }

    .settings h2 {
        font-size: var(--font-size-xl);
        color: var(--color-text-primary);
        margin-bottom: var(--space-xl);
    }

    .setting-group {
        margin-bottom: var(--space-xl);
    }

    .setting-group label {
        font-size: var(--font-size-base);
        font-weight: 600;
        color: var(--color-text-primary);
        margin-bottom: var(--space-xs);
    }

    .setting-description {
        font-size: var(--font-size-sm);
        color: var(--color-text-muted);
        margin-bottom: var(--space-md);
    }

    .path-input-wrapper {
        display: flex;
        gap: var(--space-sm);
    }

    .path-input-wrapper input {
        flex: 1;
    }

    .form-actions {
        margin-top: var(--space-xl);
    }

    hr {
        border: none;
        border-top: 1px solid var(--color-border);
        margin: var(--space-2xl) 0;
    }

    .info-section h3 {
        font-size: var(--font-size-lg);
        color: var(--color-text-primary);
        margin-bottom: var(--space-md);
    }

    .info-section p {
        font-size: var(--font-size-sm);
        color: var(--color-text-secondary);
        line-height: 1.6;
        margin-bottom: var(--space-xl);
    }

    .info-section h4 {
        font-size: var(--font-size-sm);
        color: var(--color-text-secondary);
        margin-bottom: var(--space-sm);
        font-weight: 500;
    }

    .info-section pre {
        background: var(--color-bg-secondary);
        border: 1px solid var(--color-border);
        border-radius: var(--radius-md);
        padding: var(--space-md);
        overflow: auto;
    }

    .info-section code {
        font-size: var(--font-size-xs);
        color: var(--color-text-secondary);
    }
</style>
