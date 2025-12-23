use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostMetadata {
    pub title: String,
    pub slug: String,
    pub excerpt: String,
    pub content: String,
    pub tags: Vec<String>,
    pub date: String,
    pub keywords: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExistingPost {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub excerpt: String,
    pub tags: Vec<String>,
}

/// Generate the post template (Svelte + Markdown)
fn generate_post_template(metadata: &PostMetadata) -> String {
    let tags_str = metadata.tags.iter()
        .map(|t| format!("\"{}\"", t))
        .collect::<Vec<_>>()
        .join(", ");
    
    // Use provided content or default placeholder
    let post_content = if metadata.content.is_empty() {
        format!("# {}\n\n<!-- Write your content here -->", metadata.title)
    } else {
        metadata.content.clone()
    };

    format!(
        r#"<script context="module">
    export const metadata = {{
        title: "{}",
        date: "{}",
        excerpt: "{}",
        tags: [{}]
    }};
</script>

<script lang="ts">
    import Citation from '$lib/components/Citation.svelte';
    import References from '$lib/components/References.svelte';
    
    function formatDate(dateString: string): string {{
        return new Date(dateString).toLocaleDateString('en-US', {{
            year: 'numeric', month: 'long', day: 'numeric'
        }});
    }}
</script>

<svelte:head>
    <title>{{metadata.title}} | Mr. Wong's Blog</title>
    <meta name="description" content={{metadata.excerpt}} />
</svelte:head>

<article class="blog-post">
    <div class="container">
        <header class="post-header">
            <a href="/blog" class="back-link">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M19 12H5M12 19l-7-7 7-7" />
                </svg>
                Back to Blog
            </a>
            <div class="post-meta">
                <time datetime={{metadata.date}}>{{formatDate(metadata.date)}}</time>
                <div class="post-tags">
                    {{#each metadata.tags as tag}}
                        <span class="tag">{{tag}}</span>
                    {{/each}}
                </div>
            </div>
            <h1 class="post-title">{{metadata.title}}</h1>
        </header>
        
        <div class="post-content">

{}

        </div>
        
        <footer class="post-footer">
            <a href="/blog" class="back-link">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M19 12H5M12 19l-7-7 7-7" />
                </svg>
                Back to Blog
            </a>
        </footer>
    </div>
</article>

<style>
    .blog-post {{ min-height: calc(100vh - 72px); padding: var(--space-4xl) 0; background: var(--color-bg-primary); }}
    .blog-post .container {{ max-width: 800px; }}
    .back-link {{ display: inline-flex; align-items: center; gap: var(--space-sm); color: var(--color-text-secondary); font-size: var(--font-size-sm); font-weight: 500; margin-bottom: var(--space-xl); transition: color var(--transition-fast); }}
    .back-link:hover {{ color: var(--color-accent); }}
    .post-header {{ margin-bottom: var(--space-2xl); }}
    .post-meta {{ display: flex; align-items: center; gap: var(--space-md); margin-bottom: var(--space-lg); flex-wrap: wrap; }}
    .post-meta time {{ font-size: var(--font-size-sm); color: var(--color-text-muted); }}
    .post-tags {{ display: flex; gap: var(--space-xs); flex-wrap: wrap; }}
    .tag {{ font-size: var(--font-size-xs); padding: var(--space-xs) var(--space-sm); background: rgba(59, 130, 246, 0.15); color: var(--color-accent-light); border-radius: var(--radius-sm); font-weight: 500; }}
    .post-title {{ font-size: var(--font-size-4xl); font-weight: 700; line-height: 1.2; color: var(--color-text-primary); }}
    .post-content {{ color: var(--color-text-secondary); line-height: 1.8; font-size: var(--font-size-lg); }}
    .post-content :global(h1), .post-content :global(h2), .post-content :global(h3) {{ color: var(--color-text-primary); margin-top: var(--space-2xl); margin-bottom: var(--space-md); font-weight: 600; }}
    .post-content :global(h1) {{ font-size: var(--font-size-3xl); }}
    .post-content :global(h2) {{ font-size: var(--font-size-2xl); }}
    .post-content :global(h3) {{ font-size: var(--font-size-xl); }}
    .post-content :global(p) {{ margin-bottom: var(--space-lg); }}
    .post-content :global(a) {{ color: var(--color-accent-light); text-decoration: underline; }}
    .post-content :global(ul), .post-content :global(ol) {{ margin-bottom: var(--space-lg); padding-left: var(--space-xl); }}
    .post-content :global(li) {{ margin-bottom: var(--space-sm); }}
    .post-content :global(blockquote) {{ border-left: 3px solid var(--color-accent); background: rgba(59, 130, 246, 0.1); padding: var(--space-md) var(--space-lg); margin: var(--space-lg) 0; border-radius: 0 var(--radius-lg) var(--radius-lg) 0; font-style: italic; }}
    .post-content :global(strong) {{ color: var(--color-text-primary); font-weight: 600; }}
    .post-content :global(em) {{ font-style: italic; }}
    .post-content :global(hr) {{ border: none; border-top: 1px solid var(--color-border); margin: var(--space-2xl) 0; }}
    .post-footer {{ margin-top: var(--space-4xl); padding-top: var(--space-xl); border-top: 1px solid var(--color-border); }}
</style>
"#,
        metadata.title,
        metadata.date,
        metadata.excerpt,
        tags_str,
        post_content
    )
}

/// Generate the listing entry to add to +page.svelte
fn generate_listing_entry(metadata: &PostMetadata) -> String {
    let tags_str = metadata.tags.iter()
        .map(|t| format!("\"{}\"", t))
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        r#"        {{
            slug: "{}",
            title: "{}",
            date: "{}",
            excerpt: "{}",
            tags: [{}],
            content: "{}",
        }},"#,
        metadata.slug,
        metadata.title,
        metadata.date,
        metadata.excerpt,
        tags_str,
        metadata.keywords
    )
}

#[tauri::command]
pub fn create_blog_post(
    repo_path: String,
    title: String,
    slug: String,
    excerpt: String,
    content: String,
    tags: Vec<String>,
    keywords: String,
) -> Result<String, String> {
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    
    let metadata = PostMetadata {
        title: title.clone(),
        slug: slug.clone(),
        excerpt,
        content,
        tags,
        date,
        keywords,
    };

    // Create post directory
    let post_dir = Path::new(&repo_path)
        .join("src")
        .join("routes")
        .join("blog")
        .join(&slug);

    if post_dir.exists() {
        return Err(format!("Post directory already exists: {}", slug));
    }

    fs::create_dir_all(&post_dir)
        .map_err(|e| format!("Failed to create directory: {}", e))?;

    // Write post file
    let post_content = generate_post_template(&metadata);
    let post_file = post_dir.join("+page.md");
    fs::write(&post_file, post_content)
        .map_err(|e| format!("Failed to write post file: {}", e))?;

    // Update blog listing
    let listing_path = Path::new(&repo_path)
        .join("src")
        .join("routes")
        .join("blog")
        .join("+page.svelte");

    let listing_content = fs::read_to_string(&listing_path)
        .map_err(|e| format!("Failed to read blog listing: {}", e))?;

    let new_entry = generate_listing_entry(&metadata);
    let insert_marker = "const posts = [";
    
    if let Some(pos) = listing_content.find(insert_marker) {
        let insert_pos = pos + insert_marker.len();
        let new_content = format!(
            "{}\n{}{}",
            &listing_content[..insert_pos],
            new_entry,
            &listing_content[insert_pos..]
        );
        fs::write(&listing_path, new_content)
            .map_err(|e| format!("Failed to update blog listing: {}", e))?;
    } else {
        return Err("Could not find posts array in blog listing".to_string());
    }

    Ok(format!("Created post: {}", slug))
}

#[tauri::command]
pub fn get_existing_posts(repo_path: String) -> Result<Vec<ExistingPost>, String> {
    let blog_dir = Path::new(&repo_path)
        .join("src")
        .join("routes")
        .join("blog");

    if !blog_dir.exists() {
        return Err("Blog directory not found".to_string());
    }

    let mut posts = Vec::new();
    
    for entry in fs::read_dir(&blog_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        
        if path.is_dir() {
            let slug = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();
            
            // Skip if not a post directory
            if slug.is_empty() || slug.starts_with('.') {
                continue;
            }

            posts.push(ExistingPost {
                slug: slug.clone(),
                title: slug.replace('-', " "),
                date: String::new(),
                excerpt: String::new(),
                tags: vec![],
            });
        }
    }

    Ok(posts)
}

#[tauri::command]
pub fn slugify(text: String) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostContent {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub excerpt: String,
    pub tags: Vec<String>,
    pub content: String,
}

/// Read a post's content from the file system
#[tauri::command]
pub fn read_post(repo_path: String, slug: String) -> Result<PostContent, String> {
    let post_file = Path::new(&repo_path)
        .join("src")
        .join("routes")
        .join("blog")
        .join(&slug)
        .join("+page.md");

    if !post_file.exists() {
        return Err(format!("Post not found: {}", slug));
    }

    let file_content = fs::read_to_string(&post_file)
        .map_err(|e| format!("Failed to read post: {}", e))?;

    // Parse metadata from the file
    let mut title = slug.replace('-', " ");
    let mut date = String::new();
    let mut excerpt = String::new();
    let mut tags = Vec::new();
    let mut content = String::new();

    // Extract metadata from script block
    if let Some(start) = file_content.find("export const metadata = {") {
        if let Some(end) = file_content[start..].find("};") {
            let metadata_block = &file_content[start..start + end + 2];
            
            // Extract title
            if let Some(title_start) = metadata_block.find("title: \"") {
                let title_rest = &metadata_block[title_start + 8..];
                if let Some(title_end) = title_rest.find("\"") {
                    title = title_rest[..title_end].to_string();
                }
            }
            
            // Extract date
            if let Some(date_start) = metadata_block.find("date: \"") {
                let date_rest = &metadata_block[date_start + 7..];
                if let Some(date_end) = date_rest.find("\"") {
                    date = date_rest[..date_end].to_string();
                }
            }
            
            // Extract excerpt
            if let Some(excerpt_start) = metadata_block.find("excerpt: \"") {
                let excerpt_rest = &metadata_block[excerpt_start + 10..];
                if let Some(excerpt_end) = excerpt_rest.find("\"") {
                    excerpt = excerpt_rest[..excerpt_end].to_string();
                }
            }
            
            // Extract tags
            if let Some(tags_start) = metadata_block.find("tags: [") {
                let tags_rest = &metadata_block[tags_start + 7..];
                if let Some(tags_end) = tags_rest.find("]") {
                    let tags_str = &tags_rest[..tags_end];
                    tags = tags_str
                        .split(',')
                        .map(|t| t.trim().trim_matches('"').to_string())
                        .filter(|t| !t.is_empty())
                        .collect();
                }
            }
        }
    }

    // Extract content from post-content div
    if let Some(content_start) = file_content.find("<div class=\"post-content\">") {
        let after_div = &file_content[content_start + 26..];
        if let Some(content_end) = after_div.find("</div>") {
            content = after_div[..content_end].trim().to_string();
        }
    }

    Ok(PostContent {
        slug,
        title,
        date,
        excerpt,
        tags,
        content,
    })
}

/// Update an existing post
#[tauri::command]
pub fn update_post(
    repo_path: String,
    slug: String,
    title: String,
    excerpt: String,
    content: String,
    tags: Vec<String>,
) -> Result<String, String> {
    let post_file = Path::new(&repo_path)
        .join("src")
        .join("routes")
        .join("blog")
        .join(&slug)
        .join("+page.md");

    if !post_file.exists() {
        return Err(format!("Post not found: {}", slug));
    }

    // Read existing content to preserve date
    let existing = read_post(repo_path.clone(), slug.clone())?;
    
    let metadata = PostMetadata {
        title,
        slug: slug.clone(),
        excerpt,
        content,
        tags,
        date: existing.date,
        keywords: String::new(),
    };

    let post_content = generate_post_template(&metadata);
    
    fs::write(&post_file, post_content)
        .map_err(|e| format!("Failed to write post: {}", e))?;

    Ok(format!("Updated post: {}", slug))
}

