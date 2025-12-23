<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { settings } from "../stores/settings";
    import TagInput from "./TagInput.svelte";
    import { marked } from "marked";

    // Form state
    let title = $state("");
    let slug = $state("");
    let excerpt = $state("");
    let content = $state("");
    let tags = $state<string[]>([]);
    let keywords = $state("");

    // Settings state
    let repoPath = $state("");
    $effect(() => {
        const unsubscribe = settings.subscribe((s) => {
            repoPath = s.repoPath;
        });
        return unsubscribe;
    });

    // UI state
    let isLoading = $state(false);
    let message = $state<{ type: "success" | "error"; text: string } | null>(
        null,
    );
    let slugManuallyEdited = $state(false);

    // Rendered markdown preview
    let renderedContent = $derived(
        marked(content || "*Start writing your post...*"),
    );

    // Auto-generate slug from title
    $effect(() => {
        if (!slugManuallyEdited && title) {
            invoke<string>("slugify", { text: title }).then((result) => {
                slug = result;
            });
        }
    });

    function handleSlugInput(e: Event) {
        slugManuallyEdited = true;
        slug = (e.target as HTMLInputElement).value;
    }

    async function handleSubmit(e: Event) {
        e.preventDefault();

        // Prevent duplicate submissions
        if (isLoading) return;

        if (!title.trim()) {
            message = { type: "error", text: "Title is required" };
            return;
        }

        if (!slug.trim()) {
            message = { type: "error", text: "Slug is required" };
            return;
        }

        if (!excerpt.trim()) {
            message = { type: "error", text: "Excerpt is required" };
            return;
        }

        isLoading = true;
        message = null;

        try {
            const result = await invoke<string>("create_blog_post", {
                repoPath: repoPath,
                title: title.trim(),
                slug: slug.trim(),
                excerpt: excerpt.trim(),
                content: content.trim(),
                tags,
                keywords: keywords.trim(),
            });

            message = { type: "success", text: result };

            // Reset form
            title = "";
            slug = "";
            excerpt = "";
            content = "";
            tags = [];
            keywords = "";
            slugManuallyEdited = false;
        } catch (err) {
            message = { type: "error", text: String(err) };
        } finally {
            isLoading = false;
        }
    }

    function clearForm() {
        title = "";
        slug = "";
        excerpt = "";
        content = "";
        tags = [];
        keywords = "";
        slugManuallyEdited = false;
        message = null;
    }

    // Get today's date for preview
    function getLocalDate() {
        const now = new Date();
        const year = now.getFullYear();
        const month = String(now.getMonth() + 1).padStart(2, "0");
        const day = String(now.getDate()).padStart(2, "0");
        return `${year}-${month}-${day}`;
    }
    let today = $derived(getLocalDate());
</script>

<div class="post-form-container">
    <!-- Title Section -->
    <header class="title-section">
        <h2>Create New Post</h2>
        <div class="title-row">
            <div class="form-group title-input">
                <label for="title">Title</label>
                <input
                    id="title"
                    type="text"
                    bind:value={title}
                    placeholder="Enter post title..."
                    autocomplete="off"
                />
            </div>
            <div class="form-group slug-input">
                <label for="slug">
                    Slug
                    <span class="hint">(URL path)</span>
                </label>
                <div class="slug-input-wrapper">
                    <span class="slug-prefix">/blog/</span>
                    <input
                        id="slug"
                        type="text"
                        value={slug}
                        oninput={handleSlugInput}
                        placeholder="post-url-slug"
                        autocomplete="off"
                    />
                </div>
            </div>
        </div>
    </header>

    <!-- Editor and Preview Section -->
    <div class="editor-preview-section">
        <div class="editor-panel">
            <div class="panel-header">
                <h3>Content</h3>
                <span class="hint">Markdown supported</span>
            </div>
            <textarea
                class="content-editor"
                bind:value={content}
                placeholder="# Your Post Title&#10;&#10;Write your blog post content here using **Markdown**...&#10;&#10;## Section Heading&#10;&#10;- List item 1&#10;- List item 2&#10;&#10;> Quote or callout&#10;&#10;`inline code` or code blocks with triple backticks"
            ></textarea>
        </div>

        <div class="preview-panel">
            <div class="panel-header">
                <h3>Live Preview</h3>
                <span class="preview-date">{today}</span>
            </div>
            <div class="preview-content">
                <h1 class="preview-title">{title || "Post Title"}</h1>
                <div class="preview-tags">
                    {#each tags as tag}
                        <span class="preview-tag">{tag}</span>
                    {:else}
                        <span class="preview-tag-placeholder">No tags</span>
                    {/each}
                </div>
                <div class="markdown-body">
                    {@html renderedContent}
                </div>
            </div>
        </div>
    </div>

    <!-- Metadata Section -->
    <form class="metadata-section" onsubmit={handleSubmit}>
        <div class="metadata-row">
            <div class="form-group excerpt-input">
                <label for="excerpt">Excerpt</label>
                <textarea
                    id="excerpt"
                    bind:value={excerpt}
                    placeholder="Brief description for the blog listing..."
                    rows="2"
                ></textarea>
            </div>

            <div class="form-group tags-input">
                <label for="tags">Tags</label>
                <TagInput bind:tags />
            </div>

            <div class="form-group keywords-input">
                <label for="keywords">
                    Keywords
                    <span class="hint">(optional)</span>
                </label>
                <input
                    id="keywords"
                    type="text"
                    bind:value={keywords}
                    placeholder="Search keywords..."
                    autocomplete="off"
                />
            </div>
        </div>

        {#if message}
            <div
                class={message.type === "success"
                    ? "success-message"
                    : "error-message"}
            >
                {message.text}
            </div>
        {/if}

        <div class="form-actions">
            <button type="button" class="btn btn-secondary" onclick={clearForm}>
                Clear
            </button>
            <button type="submit" class="btn btn-primary" disabled={isLoading}>
                {#if isLoading}
                    Creating...
                {:else}
                    <svg
                        width="16"
                        height="16"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                    >
                        <line x1="12" y1="5" x2="12" y2="19" />
                        <line x1="5" y1="12" x2="19" y2="12" />
                    </svg>
                    Create Post
                {/if}
            </button>
        </div>
    </form>
</div>

<style>
    .post-form-container {
        display: flex;
        flex-direction: column;
        height: 100%;
        max-width: 1600px;
        margin: 0 auto;
        gap: var(--space-lg);
    }

    /* Title Section */
    .title-section {
        background: var(--color-bg-card);
        border: 1px solid var(--color-border);
        border-radius: var(--radius-xl);
        padding: var(--space-lg);
    }

    .title-section h2 {
        font-size: var(--font-size-xl);
        margin-bottom: var(--space-md);
        color: var(--color-text-primary);
    }

    .title-row {
        display: grid;
        grid-template-columns: 2fr 1fr;
        gap: var(--space-lg);
    }

    /* Editor and Preview Section */
    .editor-preview-section {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: var(--space-lg);
        flex: 1;
        min-height: 0;
    }

    .editor-panel,
    .preview-panel {
        background: var(--color-bg-card);
        border: 1px solid var(--color-border);
        border-radius: var(--radius-xl);
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    .panel-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: var(--space-md) var(--space-lg);
        border-bottom: 1px solid var(--color-border);
    }

    .panel-header h3 {
        font-size: var(--font-size-base);
        font-weight: 600;
        color: var(--color-text-primary);
    }

    .preview-date {
        font-size: var(--font-size-xs);
        color: var(--color-text-muted);
    }

    .content-editor {
        flex: 1;
        padding: var(--space-lg);
        background: var(--color-bg-secondary);
        border: none;
        resize: none;
        font-family: "Fira Code", "Consolas", monospace;
        font-size: var(--font-size-sm);
        line-height: 1.6;
        color: var(--color-text-primary);
    }

    .content-editor:focus {
        outline: none;
    }

    .content-editor::placeholder {
        color: var(--color-text-muted);
    }

    .preview-content {
        flex: 1;
        padding: var(--space-lg);
        overflow: auto;
    }

    .preview-title {
        font-size: var(--font-size-2xl);
        font-weight: 700;
        color: var(--color-text-primary);
        margin-bottom: var(--space-sm);
    }

    .preview-tags {
        display: flex;
        gap: var(--space-xs);
        flex-wrap: wrap;
        margin-bottom: var(--space-lg);
    }

    .preview-tag {
        font-size: var(--font-size-xs);
        padding: var(--space-xs) var(--space-sm);
        background: rgba(59, 130, 246, 0.15);
        color: var(--color-accent-light);
        border-radius: var(--radius-sm);
        font-weight: 500;
    }

    .preview-tag-placeholder {
        font-size: var(--font-size-xs);
        color: var(--color-text-muted);
        font-style: italic;
    }

    /* Markdown Body Styles */
    .markdown-body {
        color: var(--color-text-secondary);
        line-height: 1.7;
        font-size: var(--font-size-base);
    }

    .markdown-body :global(h1),
    .markdown-body :global(h2),
    .markdown-body :global(h3) {
        color: var(--color-text-primary);
        margin-top: var(--space-xl);
        margin-bottom: var(--space-sm);
        font-weight: 600;
    }

    .markdown-body :global(h1) {
        font-size: var(--font-size-2xl);
    }
    .markdown-body :global(h2) {
        font-size: var(--font-size-xl);
    }
    .markdown-body :global(h3) {
        font-size: var(--font-size-lg);
    }

    .markdown-body :global(p) {
        margin-bottom: var(--space-md);
    }

    .markdown-body :global(strong) {
        color: var(--color-text-primary);
        font-weight: 600;
    }

    .markdown-body :global(em) {
        font-style: italic;
    }

    .markdown-body :global(a) {
        color: var(--color-accent-light);
        text-decoration: underline;
    }

    .markdown-body :global(ul),
    .markdown-body :global(ol) {
        margin-bottom: var(--space-md);
        padding-left: var(--space-xl);
    }

    .markdown-body :global(li) {
        margin-bottom: var(--space-xs);
    }

    .markdown-body :global(blockquote) {
        border-left: 3px solid var(--color-accent);
        background: rgba(59, 130, 246, 0.1);
        padding: var(--space-md) var(--space-lg);
        margin: var(--space-md) 0;
        border-radius: 0 var(--radius-md) var(--radius-md) 0;
        font-style: italic;
    }

    .markdown-body :global(code) {
        background: var(--color-bg-primary);
        padding: 2px 6px;
        border-radius: var(--radius-sm);
        font-family: "Fira Code", monospace;
        font-size: 0.9em;
    }

    .markdown-body :global(pre) {
        background: var(--color-bg-primary);
        padding: var(--space-md);
        border-radius: var(--radius-md);
        overflow-x: auto;
        margin: var(--space-md) 0;
    }

    .markdown-body :global(pre code) {
        background: none;
        padding: 0;
    }

    /* Table styles */
    .markdown-body :global(table) {
        width: 100%;
        border-collapse: collapse;
        margin: var(--space-md) 0;
    }

    .markdown-body :global(th),
    .markdown-body :global(td) {
        padding: var(--space-sm) var(--space-md);
        border: 1px solid var(--color-border);
        text-align: left;
    }

    .markdown-body :global(th) {
        background: var(--color-bg-secondary);
        color: var(--color-text-primary);
        font-weight: 600;
    }

    .markdown-body :global(tr:nth-child(even)) {
        background: rgba(255, 255, 255, 0.02);
    }

    .markdown-body :global(hr) {
        border: none;
        border-top: 1px solid var(--color-border);
        margin: var(--space-lg) 0;
    }

    /* Metadata Section */
    .metadata-section {
        background: var(--color-bg-card);
        border: 1px solid var(--color-border);
        border-radius: var(--radius-xl);
        padding: var(--space-lg);
    }

    .metadata-row {
        display: grid;
        grid-template-columns: 2fr 1fr 1fr;
        gap: var(--space-lg);
        margin-bottom: var(--space-md);
    }

    /* Form Groups */
    .form-group {
        display: flex;
        flex-direction: column;
    }

    .form-group label {
        display: flex;
        align-items: center;
        gap: var(--space-xs);
        margin-bottom: var(--space-xs);
    }

    .hint {
        color: var(--color-text-muted);
        font-weight: 400;
        font-size: var(--font-size-xs);
    }

    .form-group input,
    .form-group textarea {
        width: 100%;
    }

    .slug-input-wrapper {
        display: flex;
        align-items: center;
    }

    .slug-prefix {
        padding: var(--space-sm) var(--space-md);
        background: var(--color-bg-primary);
        border: 1px solid var(--color-border);
        border-right: none;
        border-radius: var(--radius-md) 0 0 var(--radius-md);
        color: var(--color-text-muted);
        font-size: var(--font-size-sm);
    }

    .slug-input-wrapper input {
        border-radius: 0 var(--radius-md) var(--radius-md) 0;
        flex: 1;
    }

    .form-actions {
        display: flex;
        gap: var(--space-md);
        justify-content: flex-end;
    }

    @media (max-width: 1200px) {
        .metadata-row {
            grid-template-columns: 1fr 1fr;
        }
        .excerpt-input {
            grid-column: span 2;
        }
    }

    @media (max-width: 900px) {
        .editor-preview-section {
            grid-template-columns: 1fr;
        }
        .title-row {
            grid-template-columns: 1fr;
        }
        .metadata-row {
            grid-template-columns: 1fr;
        }
        .excerpt-input {
            grid-column: span 1;
        }
    }
</style>
