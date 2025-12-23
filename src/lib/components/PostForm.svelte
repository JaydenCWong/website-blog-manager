<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { settings } from "../stores/settings";
    import TagInput from "./TagInput.svelte";
    import LivePreview from "./LivePreview.svelte";

    // Form state
    let title = $state("");
    let slug = $state("");
    let excerpt = $state("");
    let tags = $state<string[]>([]);
    let keywords = $state("");

    // UI state
    let isLoading = $state(false);
    let message = $state<{ type: "success" | "error"; text: string } | null>(
        null,
    );
    let slugManuallyEdited = $state(false);

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
                repoPath: $settings.repoPath,
                title: title.trim(),
                slug: slug.trim(),
                excerpt: excerpt.trim(),
                tags,
                keywords: keywords.trim(),
            });

            message = { type: "success", text: result };

            // Reset form
            title = "";
            slug = "";
            excerpt = "";
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
        tags = [];
        keywords = "";
        slugManuallyEdited = false;
        message = null;
    }
</script>

<div class="post-form-container">
    <div class="form-panel">
        <h2>Create New Post</h2>

        <form onsubmit={handleSubmit}>
            <div class="form-group">
                <label for="title">Title</label>
                <input
                    id="title"
                    type="text"
                    bind:value={title}
                    placeholder="Enter post title..."
                    autocomplete="off"
                />
            </div>

            <div class="form-group">
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

            <div class="form-group">
                <label for="excerpt">Excerpt</label>
                <textarea
                    id="excerpt"
                    bind:value={excerpt}
                    placeholder="Brief description for the blog listing..."
                    rows="3"
                ></textarea>
            </div>

            <div class="form-group">
                <label>Tags</label>
                <TagInput bind:tags />
            </div>

            <div class="form-group">
                <label for="keywords">
                    Search Keywords
                    <span class="hint">(optional, for search)</span>
                </label>
                <input
                    id="keywords"
                    type="text"
                    bind:value={keywords}
                    placeholder="Additional keywords for search..."
                    autocomplete="off"
                />
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
                <button
                    type="button"
                    class="btn btn-secondary"
                    onclick={clearForm}
                >
                    Clear
                </button>
                <button
                    type="submit"
                    class="btn btn-primary"
                    disabled={isLoading}
                >
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

    <div class="preview-panel">
        <LivePreview {title} {slug} {excerpt} {tags} />
    </div>
</div>

<style>
    .post-form-container {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: var(--space-xl);
        height: 100%;
        max-width: 1400px;
        margin: 0 auto;
    }

    .form-panel,
    .preview-panel {
        background: var(--color-bg-card);
        border: 1px solid var(--color-border);
        border-radius: var(--radius-xl);
        padding: var(--space-xl);
        overflow: auto;
    }

    .form-panel h2 {
        font-size: var(--font-size-xl);
        margin-bottom: var(--space-lg);
        color: var(--color-text-primary);
    }

    .form-group {
        margin-bottom: var(--space-lg);
    }

    .form-group label {
        display: flex;
        align-items: center;
        gap: var(--space-xs);
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

    .form-group textarea {
        resize: vertical;
        min-height: 80px;
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
        margin-top: var(--space-xl);
    }

    @media (max-width: 900px) {
        .post-form-container {
            grid-template-columns: 1fr;
        }
    }
</style>
