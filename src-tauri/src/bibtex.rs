use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BibEntry {
    pub key: String,
    pub entry_type: String,
    pub author: String,
    pub year: String,
    pub title: String,
    pub url: Option<String>,
    pub booktitle: Option<String>,
    pub pages: Option<String>,
    pub publisher: Option<String>,
    pub journal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    pub author: String,
    pub year: i32,
    pub title: String,
    pub short_cite: String,
    pub booktitle: Option<String>,
    pub pages: Option<String>,
    pub publisher: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncResult {
    pub entries_synced: usize,
    pub message: String,
}

/// Parse a BibTeX file and extract entries
fn parse_bibtex(content: &str) -> Vec<BibEntry> {
    let mut entries = Vec::new();
    
    // Match @type{key, ... }
    let entry_re = Regex::new(r"@(\w+)\s*\{\s*([^,]+),([^@]*)").unwrap();
    let field_re = Regex::new(r#"(\w+)\s*=\s*[{"]((?:[^{}"]|(?:\{[^}]*\}))*)[}"]"#).unwrap();

    for cap in entry_re.captures_iter(content) {
        let entry_type = cap[1].to_lowercase();
        let key = cap[2].trim().to_string();
        let fields_str = &cap[3];

        let mut author = String::new();
        let mut year = String::new();
        let mut title = String::new();
        let mut url = None;
        let mut booktitle = None;
        let mut pages = None;
        let mut publisher = None;
        let mut journal = None;

        for field_cap in field_re.captures_iter(fields_str) {
            let field_name = field_cap[1].to_lowercase();
            let field_value = field_cap[2].trim().to_string();

            match field_name.as_str() {
                "author" => author = field_value,
                "year" => year = field_value,
                "title" => title = field_value,
                "url" => url = Some(field_value),
                "booktitle" => booktitle = Some(field_value),
                "pages" => pages = Some(field_value.replace("--", "-")),
                "publisher" => publisher = Some(field_value),
                "journal" => journal = Some(field_value),
                _ => {}
            }
        }

        entries.push(BibEntry {
            key,
            entry_type,
            author,
            year,
            title,
            url,
            booktitle,
            pages,
            publisher,
            journal,
        });
    }

    entries
}

/// Convert BibEntry to Reference for TypeScript output
fn entry_to_reference(entry: &BibEntry) -> (String, Reference) {
    // Convert key like "thurston2009" or "razbuten_2025_12_17" to simpler form
    let ts_key = entry.key
        .replace('-', "")
        .split('_')
        .take(2)
        .collect::<Vec<_>>()
        .join("");

    let author_last = entry.author
        .split(',')
        .next()
        .unwrap_or(&entry.author)
        .split_whitespace()
        .last()
        .unwrap_or("Unknown");

    let year: i32 = entry.year.parse().unwrap_or(0);
    let short_cite = format!("{}, {}", author_last, entry.year);

    let publisher = entry.publisher.clone().or_else(|| entry.journal.clone());

    (ts_key, Reference {
        author: entry.author.clone(),
        year,
        title: entry.title.clone(),
        short_cite,
        booktitle: entry.booktitle.clone(),
        pages: entry.pages.clone(),
        publisher,
        url: entry.url.clone(),
    })
}

/// Generate the TypeScript references file content
fn generate_references_ts(references: &[(String, Reference)]) -> String {
    let mut content = String::from(
r#"// Auto-generated from references.bib
// Run: npm run blog sync
// Do NOT edit manually - edit references.bib instead

export interface Reference {
    author: string;
    year: number;
    title: string;
    shortCite: string;
    booktitle?: string;
    pages?: string;
    publisher?: string;
    url?: string;
}

export const references: Record<string, Reference> = {
"#);

    for (key, ref_data) in references {
        content.push_str(&format!("    {}: {{\n", key));
        content.push_str(&format!("        author: \"{}\",\n", ref_data.author));
        content.push_str(&format!("        year: {},\n", ref_data.year));
        content.push_str(&format!("        title: \"{}\",\n", ref_data.title));
        content.push_str(&format!("        shortCite: \"{}\"", ref_data.short_cite));
        
        if let Some(booktitle) = &ref_data.booktitle {
            content.push_str(&format!(",\n        booktitle: \"{}\"", booktitle));
        }
        if let Some(pages) = &ref_data.pages {
            content.push_str(&format!(",\n        pages: \"{}\"", pages));
        }
        if let Some(publisher) = &ref_data.publisher {
            content.push_str(&format!(",\n        publisher: \"{}\"", publisher));
        }
        if let Some(url) = &ref_data.url {
            content.push_str(&format!(",\n        url: \"{}\"", url));
        }
        
        content.push_str("\n    },\n");
    }

    content.push_str("};\n");
    content
}

#[tauri::command]
pub fn read_bib_file(repo_path: String) -> Result<Vec<BibEntry>, String> {
    let bib_path = Path::new(&repo_path)
        .join("src")
        .join("lib")
        .join("references.bib");

    if !bib_path.exists() {
        return Err("references.bib not found".to_string());
    }

    let content = fs::read_to_string(&bib_path)
        .map_err(|e| format!("Failed to read bib file: {}", e))?;

    Ok(parse_bibtex(&content))
}

#[tauri::command]
pub fn sync_references(repo_path: String) -> Result<SyncResult, String> {
    let bib_path = Path::new(&repo_path)
        .join("src")
        .join("lib")
        .join("references.bib");

    let refs_path = Path::new(&repo_path)
        .join("src")
        .join("lib")
        .join("data")
        .join("references.ts");

    if !bib_path.exists() {
        return Err("references.bib not found".to_string());
    }

    let bib_content = fs::read_to_string(&bib_path)
        .map_err(|e| format!("Failed to read bib file: {}", e))?;

    let entries = parse_bibtex(&bib_content);
    let references: Vec<_> = entries.iter()
        .map(entry_to_reference)
        .collect();

    let ts_content = generate_references_ts(&references);

    // Ensure data directory exists
    if let Some(parent) = refs_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create data directory: {}", e))?;
    }

    fs::write(&refs_path, ts_content)
        .map_err(|e| format!("Failed to write references.ts: {}", e))?;

    Ok(SyncResult {
        entries_synced: entries.len(),
        message: format!("Synced {} references", entries.len()),
    })
}
