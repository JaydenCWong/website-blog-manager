<script lang="ts">
    interface Props {
        tags: string[];
    }

    let { tags = $bindable([]) }: Props = $props();

    let inputValue = $state("");

    // Common blog tags for suggestions
    const suggestions = [
        "teaching",
        "learning",
        "programming",
        "math",
        "study-tips",
        "problem-solving",
        "general",
        "welcome",
    ];

    let filteredSuggestions = $derived(
        inputValue.length > 0
            ? suggestions.filter(
                  (s) =>
                      s.includes(inputValue.toLowerCase()) && !tags.includes(s),
              )
            : [],
    );

    function addTag(tag: string) {
        const normalizedTag = tag.toLowerCase().trim();
        if (normalizedTag && !tags.includes(normalizedTag)) {
            tags = [...tags, normalizedTag];
        }
        inputValue = "";
    }

    function removeTag(tag: string) {
        tags = tags.filter((t) => t !== tag);
    }

    function handleKeydown(e: KeyboardEvent) {
        if (e.key === "Enter" && inputValue.trim()) {
            e.preventDefault();
            addTag(inputValue);
        } else if (e.key === "Backspace" && !inputValue && tags.length > 0) {
            removeTag(tags[tags.length - 1]);
        }
    }
</script>

<div class="tag-input">
    <div class="tags-container">
        {#each tags as tag}
            <span class="tag">
                {tag}
                <button
                    type="button"
                    class="remove-tag"
                    onclick={() => removeTag(tag)}
                    title="Remove tag"
                >
                    <svg
                        width="12"
                        height="12"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                    >
                        <path d="M18 6L6 18M6 6l12 12" />
                    </svg>
                </button>
            </span>
        {/each}
        <input
            type="text"
            bind:value={inputValue}
            onkeydown={handleKeydown}
            placeholder={tags.length === 0 ? "Add tags..." : ""}
        />
    </div>

    {#if filteredSuggestions.length > 0}
        <div class="suggestions">
            {#each filteredSuggestions as suggestion}
                <button
                    type="button"
                    class="suggestion"
                    onclick={() => addTag(suggestion)}
                >
                    {suggestion}
                </button>
            {/each}
        </div>
    {/if}
</div>

<style>
    .tag-input {
        position: relative;
    }

    .tags-container {
        display: flex;
        flex-wrap: wrap;
        gap: var(--space-xs);
        padding: var(--space-sm);
        background: var(--color-bg-secondary);
        border: 1px solid var(--color-border);
        border-radius: var(--radius-md);
        min-height: 42px;
    }

    .tags-container:focus-within {
        border-color: var(--color-accent);
        box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.15);
    }

    .tag {
        display: inline-flex;
        align-items: center;
        gap: var(--space-xs);
        padding: var(--space-xs) var(--space-sm);
        background: rgba(59, 130, 246, 0.15);
        color: var(--color-accent-light);
        border-radius: var(--radius-sm);
        font-size: var(--font-size-sm);
        font-weight: 500;
    }

    .remove-tag {
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 2px;
        color: var(--color-accent-light);
        opacity: 0.7;
        transition: opacity var(--transition-fast);
    }

    .remove-tag:hover {
        opacity: 1;
    }

    .tags-container input {
        flex: 1;
        min-width: 100px;
        border: none;
        background: none;
        padding: var(--space-xs);
        font-size: var(--font-size-sm);
        color: var(--color-text-primary);
    }

    .tags-container input:focus {
        outline: none;
        box-shadow: none;
    }

    .suggestions {
        position: absolute;
        top: 100%;
        left: 0;
        right: 0;
        margin-top: var(--space-xs);
        background: var(--color-bg-card);
        border: 1px solid var(--color-border);
        border-radius: var(--radius-md);
        padding: var(--space-xs);
        z-index: 10;
        display: flex;
        flex-wrap: wrap;
        gap: var(--space-xs);
    }

    .suggestion {
        padding: var(--space-xs) var(--space-sm);
        background: var(--color-bg-secondary);
        border: 1px solid var(--color-border);
        border-radius: var(--radius-sm);
        font-size: var(--font-size-xs);
        color: var(--color-text-secondary);
        transition: all var(--transition-fast);
    }

    .suggestion:hover {
        background: var(--color-bg-hover);
        color: var(--color-text-primary);
        border-color: var(--color-border-hover);
    }
</style>
