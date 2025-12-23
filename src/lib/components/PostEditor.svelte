<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { settings } from "../stores/settings";
    import TagInput from "./TagInput.svelte";
    import { marked } from "marked";

    interface PostSummary {
        slug: string;
        title: string;
        date: string;
        excerpt: string;
        tags: string[];
    }

    interface PostContent {
        slug: string;
        title: string;
        date: string;
        excerpt: string;
        tags: string[];
        content: string;
    }

    // Settings state
    let repoPath = $state("");
    $effect(() => {
        const unsubscribe = settings.subscribe((s) => {
            repoPath = s.repoPath;
        });
        return unsubscribe;
    });

    // Posts list state
    let posts = $state<PostSummary[]>([]);
    let isLoadingPosts = $state(false);

    // Editor state
    let selectedPost = $state<PostContent | null>(null);
    let isLoadingPost = $state(false);
    let isSaving = $state(false);
    let message = $state<{ type: "success" | "error"; text: string } | null>(
        null,
    );

    // Edit form state
    let editTitle = $state("");
    let editExcerpt = $state("");
    let editContent = $state("");
    let editTags = $state<string[]>([]);

    // Rendered markdown preview
    let renderedContent = $derived(
        marked(editContent || "*Select a post to edit...*"),
    );

    async function loadPosts() {
        if (!repoPath) return;

        isLoadingPosts = true;
        message = null;

        try {
            posts = await invoke<PostSummary[]>("get_existing_posts", {
                repoPath,
            });
        } catch (err) {
            message = { type: "error", text: String(err) };
        } finally {
            isLoadingPosts = false;
        }
    }

    async function selectPost(slug: string) {
        isLoadingPost = true;
        message = null;

        try {
            const post = await invoke<PostContent>("read_post", {
                repoPath,
                slug,
            });
            selectedPost = post;
            editTitle = post.title;
            editExcerpt = post.excerpt;
            editContent = post.content;
            editTags = [...post.tags];
        } catch (err) {
            message = { type: "error", text: String(err) };
        } finally {
            isLoadingPost = false;
        }
    }

    async function savePost() {
        if (!selectedPost) return;
        if (isSaving) return;

        isSaving = true;
        message = null;

        try {
            const result = await invoke<string>("update_post", {
                repoPath,
                slug: selectedPost.slug,
                title: editTitle.trim(),
                excerpt: editExcerpt.trim(),
                content: editContent.trim(),
                tags: editTags,
            });
            message = { type: "success", text: result };

            // Update local state
            selectedPost.title = editTitle;
            selectedPost.excerpt = editExcerpt;
            selectedPost.content = editContent;
            selectedPost.tags = editTags;
        } catch (err) {
            message = { type: "error", text: String(err) };
        } finally {
            isSaving = false;
        }
    }

    function clearSelection() {
        selectedPost = null;
        editTitle = "";
        editExcerpt = "";
        editContent = "";
        editTags = [];
        message = null;
    }

    // Load posts on mount
    $effect(() => {
        if (repoPath) {
            loadPosts();
        }
    });

    // Date formatting
    function getLocalDate() {
        const now = new Date();
        const year = now.getFullYear();
        const month = String(now.getMonth() + 1).padStart(2, "0");
        const day = String(now.getDate()).padStart(2, "0");
        return `${year}-${month}-${day}`;
    }
    let today = $derived(getLocalDate());
</script>

<div class="editor-container">
    <!-- Posts List Sidebar -->
    <aside class="posts-sidebar">
        <div class="sidebar-header">
            <h2>Posts</h2>
            <button
                class="btn btn-secondary"
                onclick={loadPosts}
                disabled={isLoadingPosts}
                title="Refresh posts"
            >
                <svg
                    width="14"
                    height="14"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                >
                    <polyline points="23 4 23 10 17 10" />
                    <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10" />
                </svg>
            </button>
        </div>

        <div class="posts-list">
            {#if isLoadingPosts}
                <div class="loading">Loading posts...</div>
            {:else if posts.length === 0}
                <div class="empty">No posts found</div>
            {:else}
                {#each posts as post}
                    <button
                        class="post-item"
                        class:active={selectedPost?.slug === post.slug}
                        onclick={() => selectPost(post.slug)}
                    >
                        <span class="post-title">{post.title || post.slug}</span
                        >
                        <span class="post-slug">/{post.slug}</span>
                    </button>
                {/each}
            {/if}
        </div>
    </aside>

    <!-- Editor Main Area -->
    <main class="editor-main">
        {#if !selectedPost}
            <div class="no-selection">
                <div class="no-selection-icon">📝</div>
                <h3>Select a Post to Edit</h3>
                <p>Choose a post from the sidebar to start editing.</p>
            </div>
        {:else if isLoadingPost}
            <div class="loading-post">Loading post content...</div>
        {:else}
            <!-- Title Section -->
            <header class="edit-header">
                <div class="edit-title-row">
                    <div class="form-group">
                        <label for="edit-title">Title</label>
                        <input
                            id="edit-title"
                            type="text"
                            bind:value={editTitle}
                            placeholder="Post title..."
                        />
                    </div>
                    <div class="post-info">
                        <span class="post-date"
                            >{selectedPost.date || today}</span
                        >
                        <code class="post-path">/blog/{selectedPost.slug}</code>
                    </div>
                </div>
            </header>

            <!-- Editor and Preview -->
            <div class="edit-panels">
                <div class="editor-panel">
                    <div class="panel-header">
                        <h3>Content</h3>
                        <span class="hint">Markdown</span>
                    </div>
                    <textarea
                        class="content-editor"
                        bind:value={editContent}
                        placeholder="Write your post content..."
                    ></textarea>
                </div>

                <div class="preview-panel">
                    <div class="panel-header">
                        <h3>Preview</h3>
                    </div>
                    <div class="preview-content">
                        <h1 class="preview-title">{editTitle}</h1>
                        <div class="preview-tags">
                            {#each editTags as tag}
                                <span class="preview-tag">{tag}</span>
                            {:else}
                                <span class="preview-tag-placeholder"
                                    >No tags</span
                                >
                            {/each}
                        </div>
                        <div class="markdown-body">
                            {@html renderedContent}
                        </div>
                    </div>
                </div>
            </div>

            <!-- Metadata Section -->
            <div class="edit-metadata">
                <div class="metadata-row">
                    <div class="form-group excerpt-input">
                        <label for="edit-excerpt">Excerpt</label>
                        <textarea
                            id="edit-excerpt"
                            bind:value={editExcerpt}
                            placeholder="Brief description..."
                            rows="2"
                        ></textarea>
                    </div>
                    <div class="form-group tags-input">
                        <label for="edit-tags">Tags</label>
                        <TagInput bind:tags={editTags} />
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
                    <button class="btn btn-secondary" onclick={clearSelection}>
                        Cancel
                    </button>
                    <button
                        class="btn btn-primary"
                        onclick={savePost}
                        disabled={isSaving}
                    >
                        {#if isSaving}
                            Saving...
                        {:else}
                            <svg
                                width="16"
                                height="16"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                            >
                                <path
                                    d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"
                                />
                                <polyline points="17 21 17 13 7 13 7 21" />
                                <polyline points="7 3 7 8 15 8" />
                            </svg>
                            Save Changes
                        {/if}
                    </button>
                </div>
            </div>
        {/if}
    </main>
</div>

<style>
    .editor-container {
        display: grid;
        grid-template-columns: 280px 1fr;
        height: 100%;
        gap: var(--space-lg);
    }

    /* Sidebar */
    .posts-sidebar {
        background: var(--color-bg-card);
        border: 1px solid var(--color-border);
        border-radius: var(--radius-xl);
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    .sidebar-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: var(--space-md) var(--space-lg);
        border-bottom: 1px solid var(--color-border);
    }

    .sidebar-header h2 {
        font-size: var(--font-size-base);
        font-weight: 600;
        color: var(--color-text-primary);
    }

    .posts-list {
        flex: 1;
        overflow-y: auto;
        padding: var(--space-sm);
    }

    .post-item {
        display: block;
        width: 100%;
        padding: var(--space-sm) var(--space-md);
        text-align: left;
        border-radius: var(--radius-md);
        margin-bottom: var(--space-xs);
        transition: all var(--transition-fast);
    }

    .post-item:hover {
        background: var(--color-bg-hover);
    }

    .post-item.active {
        background: rgba(59, 130, 246, 0.15);
    }

    .post-title {
        display: block;
        font-weight: 500;
        color: var(--color-text-primary);
        font-size: var(--font-size-sm);
        text-transform: capitalize;
    }

    .post-slug {
        display: block;
        font-size: var(--font-size-xs);
        color: var(--color-text-muted);
    }

    .loading,
    .empty {
        padding: var(--space-lg);
        text-align: center;
        color: var(--color-text-muted);
        font-size: var(--font-size-sm);
    }

    /* Main Editor Area */
    .editor-main {
        display: flex;
        flex-direction: column;
        gap: var(--space-lg);
        min-height: 0;
    }

    .no-selection,
    .loading-post {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100%;
        text-align: center;
    }

    .no-selection-icon {
        font-size: 3rem;
        margin-bottom: var(--space-md);
    }

    .no-selection h3 {
        font-size: var(--font-size-lg);
        color: var(--color-text-primary);
        margin-bottom: var(--space-sm);
    }

    .no-selection p {
        color: var(--color-text-secondary);
    }

    /* Edit Header */
    .edit-header {
        background: var(--color-bg-card);
        border: 1px solid var(--color-border);
        border-radius: var(--radius-xl);
        padding: var(--space-lg);
    }

    .edit-title-row {
        display: flex;
        align-items: flex-end;
        gap: var(--space-lg);
    }

    .edit-title-row .form-group {
        flex: 1;
    }

    .post-info {
        display: flex;
        align-items: center;
        gap: var(--space-md);
        padding-bottom: var(--space-sm);
    }

    .post-date {
        font-size: var(--font-size-sm);
        color: var(--color-text-muted);
    }

    .post-path {
        font-size: var(--font-size-sm);
        color: var(--color-accent-light);
        background: var(--color-bg-secondary);
        padding: 2px 8px;
        border-radius: var(--radius-sm);
    }

    /* Edit Panels */
    .edit-panels {
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

    .hint {
        font-size: var(--font-size-xs);
        color: var(--color-text-muted);
    }

    .content-editor {
        flex: 1;
        padding: var(--space-lg);
        background: var(--color-bg-secondary);
        border: none;
        font-family: "Fira Code", "Consolas", monospace;
        font-size: var(--font-size-sm);
        line-height: 1.6;
        color: var(--color-text-primary);
    }

    .content-editor:focus {
        outline: none;
    }

    .preview-content {
        flex: 1;
        padding: var(--space-lg);
        overflow: auto;
    }

    .preview-title {
        font-size: var(--font-size-xl);
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

    .markdown-body {
        color: var(--color-text-secondary);
        line-height: 1.7;
        font-size: var(--font-size-sm);
    }

    .markdown-body :global(h1),
    .markdown-body :global(h2),
    .markdown-body :global(h3) {
        color: var(--color-text-primary);
        margin-top: var(--space-lg);
        margin-bottom: var(--space-sm);
        font-weight: 600;
    }

    .markdown-body :global(p) {
        margin-bottom: var(--space-md);
    }

    .markdown-body :global(strong) {
        color: var(--color-text-primary);
    }

    .markdown-body :global(code) {
        background: var(--color-bg-primary);
        padding: 2px 6px;
        border-radius: var(--radius-sm);
        font-size: 0.9em;
    }

    /* Metadata Section */
    .edit-metadata {
        background: var(--color-bg-card);
        border: 1px solid var(--color-border);
        border-radius: var(--radius-xl);
        padding: var(--space-lg);
    }

    .metadata-row {
        display: grid;
        grid-template-columns: 2fr 1fr;
        gap: var(--space-lg);
        margin-bottom: var(--space-md);
    }

    .form-group {
        display: flex;
        flex-direction: column;
    }

    .form-group label {
        margin-bottom: var(--space-xs);
    }

    .form-group input,
    .form-group textarea {
        width: 100%;
    }

    .form-actions {
        display: flex;
        gap: var(--space-md);
        justify-content: flex-end;
    }

    @media (max-width: 1200px) {
        .edit-panels {
            grid-template-columns: 1fr;
        }
    }

    @media (max-width: 900px) {
        .editor-container {
            grid-template-columns: 1fr;
        }
        .posts-sidebar {
            max-height: 200px;
        }
        .metadata-row {
            grid-template-columns: 1fr;
        }
    }
</style>
