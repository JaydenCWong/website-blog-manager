<script lang="ts" context="module">
    function generateTemplate(
        title: string,
        slug: string,
        excerpt: string,
        tags: string[],
        date: string,
    ): string {
        const tagsStr = tags.map((t) => `"${t}"`).join(", ");
        return `<script context="module">
  export const metadata = {
    title: "${title || "Post Title"}",
    date: "${date}",
    excerpt: "${excerpt || "..."}",
    tags: [${tagsStr}]
  };
<\/script>

# ${title || "Post Title"}

<!-- Write your content here -->`;
    }
</script>

<script lang="ts">
    interface Props {
        title: string;
        slug: string;
        excerpt: string;
        tags: string[];
    }

    let { title, slug, excerpt, tags }: Props = $props();

    let today = new Date().toISOString().split("T")[0];
</script>

<div class="preview">
    <h2>Live Preview</h2>

    <div class="preview-content">
        <div class="preview-meta">
            <span class="preview-date">{today}</span>
            <div class="preview-tags">
                {#each tags as tag}
                    <span class="preview-tag">{tag}</span>
                {:else}
                    <span class="preview-tag-placeholder">No tags</span>
                {/each}
            </div>
        </div>

        <h3 class="preview-title">
            {title || "Post Title"}
        </h3>

        <p class="preview-excerpt">
            {excerpt || "Your excerpt will appear here..."}
        </p>

        <div class="preview-url">
            <span class="url-label">URL:</span>
            <code>/blog/{slug || "post-slug"}</code>
        </div>

        <hr />

        <div class="template-preview">
            <h4>Generated Template</h4>
            <pre><code
                    >{generateTemplate(title, slug, excerpt, tags, today)}</code
                ></pre>
        </div>
    </div>
</div>

<style>
    .preview {
        height: 100%;
        display: flex;
        flex-direction: column;
    }

    .preview h2 {
        font-size: var(--font-size-xl);
        margin-bottom: var(--space-lg);
        color: var(--color-text-primary);
    }

    .preview-content {
        flex: 1;
        overflow: auto;
    }

    .preview-meta {
        display: flex;
        align-items: center;
        gap: var(--space-md);
        margin-bottom: var(--space-md);
        flex-wrap: wrap;
    }

    .preview-date {
        font-size: var(--font-size-sm);
        color: var(--color-text-muted);
    }

    .preview-tags {
        display: flex;
        gap: var(--space-xs);
        flex-wrap: wrap;
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

    .preview-title {
        font-size: var(--font-size-2xl);
        font-weight: 700;
        color: var(--color-text-primary);
        margin-bottom: var(--space-sm);
    }

    .preview-excerpt {
        color: var(--color-text-secondary);
        line-height: 1.6;
        margin-bottom: var(--space-lg);
    }

    .preview-url {
        display: flex;
        align-items: center;
        gap: var(--space-sm);
        margin-bottom: var(--space-lg);
    }

    .url-label {
        font-size: var(--font-size-sm);
        color: var(--color-text-muted);
    }

    .preview-url code {
        padding: var(--space-xs) var(--space-sm);
        background: var(--color-bg-secondary);
        border-radius: var(--radius-sm);
        font-size: var(--font-size-sm);
        color: var(--color-accent-light);
    }

    hr {
        border: none;
        border-top: 1px solid var(--color-border);
        margin: var(--space-lg) 0;
    }

    .template-preview h4 {
        font-size: var(--font-size-sm);
        color: var(--color-text-secondary);
        margin-bottom: var(--space-sm);
        font-weight: 500;
    }

    .template-preview pre {
        background: var(--color-bg-primary);
        border: 1px solid var(--color-border);
        border-radius: var(--radius-md);
        padding: var(--space-md);
        overflow: auto;
        max-height: 300px;
    }

    .template-preview code {
        font-size: var(--font-size-xs);
        color: var(--color-text-secondary);
        white-space: pre-wrap;
        word-break: break-word;
    }
</style>
