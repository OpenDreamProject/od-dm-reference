use std::collections::HashMap;

use poise::serenity_prelude::{self as serenity, Colour, CreateEmbed};
use regex::Regex;
use serde::Deserialize;
use tantivy::{
    collector::TopDocs,
    doc,
    query::QueryParser,
    schema::{Field, Schema, Value, STORED, TEXT},
    Document, Index, IndexReader, IndexWriter, TantivyDocument,
};
use tempfile::TempDir;

mod content;

struct Data {
    titles_to_path: HashMap<String, &'static str>,
    path_to_parsed: HashMap<String, PageFrontmatter>,
    path_to_text: HashMap<&'static str, &'static str>,
    reader: IndexReader,
    index: Index,
    default_fields: Vec<Field>,
} // User data, which is stored and accessible in all command invocations

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Get an entry from the OpenDream DM Reference
#[poise::command(slash_command)]
async fn odref(
    ctx: Context<'_>,
    #[description = "The ref entry to look for"] search_for: String,
) -> Result<(), Error> {
    let search_for = clean_query(search_for);

    let page = get_page(&search_for, ctx.data()).unwrap_or("Not found.");

    match format_embed(page, ctx.data()) {
        Some(embed) => ctx.send(poise::CreateReply::default().embed(embed)).await?,
        None => {
            log::debug!("Unable to locate a page for {}.", &search_for);

            ctx.send(
                poise::CreateReply::default()
                    .embed(CreateEmbed::default().description("Could not locate a page.")),
            )
            .await?
        }
    };

    Ok(())
}

/// Prepare the query to be checked against the pages
fn clean_query(query: String) -> String {
    query.trim().to_lowercase().to_string()
}

/// Retrieves the path of a valid page from a query.
///
/// Searches for exact path names, then page titles (from the frontmatter TOML)
/// then searching through all path names for matches
fn get_page<'a>(query: &'a String, data: &'a Data) -> Option<&'a str> {
    let mut path_find = query.replace(' ', "/");

    if path_find.starts_with('/') {
        path_find = path_find[1..].to_string();
    }

    if let Some(string) = data
        .path_to_text
        .get_key_value(format!("objects/{}/_index.md", path_find).as_str())
    {
        return Some(*string.0);
    }

    if path_find.contains('/') {
        let components: Vec<&str> = path_find.split('/').collect();

        let mut var = components.clone();
        var.insert(components.len() - 1, "var");

        let var_string = var.join("/");

        if let Some(string) = data
            .path_to_text
            .get_key_value(format!("objects/{}.md", var_string).as_str())
        {
            return Some(*string.0);
        }

        let proc_string = var_string.replace("var", "proc");
        if let Some(string) = data
            .path_to_text
            .get_key_value(format!("objects/{}.md", proc_string).as_str())
        {
            return Some(*string.0);
        }
    }

    if let Some(string) = data.titles_to_path.get(query) {
        return Some(*string);
    }

    let searcher = data.reader.searcher();
    let query_parser = QueryParser::for_index(&data.index, data.default_fields.clone());

    if let Ok(query) = query_parser.parse_query(query) {
        if let Ok(res) = searcher.search(&query, &TopDocs::with_limit(1)) {
            if let Some(doc_tuple) = res.first() {
                let doc: TantivyDocument = searcher.doc(doc_tuple.1).unwrap();

                for field in doc.iter_fields_and_values() {
                    if let Some(path) = data.path_to_text.get_key_value(field.1.as_str().unwrap()) {
                        return Some(*path.0);
                    }
                }
            }
        }
    };

    for thing in data.path_to_text.iter() {
        if thing.0.contains(&path_find) {
            return Some(*thing.0);
        }
    }

    None
}

/// Returns a formatted embed based on the available information in the page.
///
/// Retains most Markdown, as this (mostly) renders fine in Discord embeds.
/// Scrubs all Tera shortcodes, apart from the ones we can render into direct links here.
/// Pulls some extra information from the frontmatter, and puts it in fields.
fn format_embed(page: &str, data: &Data) -> Option<serenity::CreateEmbed> {
    let body_regex = Regex::new(r"(?s)\+\+\+(.*)\+\+\+(.*)").unwrap();

    let parsed = data.path_to_parsed.get(page)?;

    let body = body_regex
        .captures(data.path_to_text.get(page)?)?
        .get(2)?
        .as_str();

    let mut title = parsed.title.clone()?;

    let mut components: Vec<&str> = page.split('/').collect();

    let proc = components.contains(&"proc");
    if proc || components.contains(&"var") {
        components.truncate(components.len() - 2);
        components.push("_index.md");

        let parent_parsed = data.path_to_parsed.get(&components.join("/"))?;

        title = format!(
            "{} ({} {})",
            &title,
            parent_parsed.title.clone()?,
            if proc { "proc" } else { "var" }
        )
    };

    let mut embed = serenity::CreateEmbed::default()
        .title(&title)
        .url(get_url(page, parsed))
        .color(Colour::from_rgb(246, 114, 128))
        .description(format_body(body, data));

    let extra = parsed.extra.as_ref();

    if extra.is_none() {
        return Some(embed);
    }

    let extra = extra.unwrap();

    if let Some(formats) = &extra.format {
        for format in formats.iter().enumerate() {
            let val = format.1;
            let mut format_string = format!("```js\n{}(\n", &parsed.title.clone()?);

            for array_value in val.iter() {
                format_string = format!("{}{}\n", format_string, &array_value.get_arg_as_string());
            }

            format_string.push_str(")```");

            embed = embed.field(
                format!("Format {}", format.0 + 1).as_str(),
                format_string,
                false,
            );
        }
    } else if let Some(val) = &extra.args {
        let mut args_string = format!("```js\n{}(\n", &parsed.title.clone()?);

        for array_value in val.iter() {
            args_string = format!("{}{}\n", args_string, &array_value.get_arg_as_string());
        }

        args_string.push_str(")```");

        embed = embed.field("Arguments", args_string, false);
    }

    if let Some(val) = &extra.usage {
        embed = embed.field("Usage", val.as_str(), false)
    };

    if let Some(val) = &extra._return {
        match val {
            PageReturnOrString::PageReturn(page) => {
                let mut return_string = String::new();

                if let Some(ReturnTypeOrBool::String(string)) = &page._type {
                    return_string.push_str(string.as_str());
                };

                if let Some(val) = &page.description {
                    return_string = if !return_string.is_empty() {
                        format!("{}: {}", return_string, val.as_str())
                    } else {
                        val.as_str().to_string()
                    }
                };

                if !return_string.is_empty() {
                    embed = embed.field("Return", return_string, false);
                }
            }

            PageReturnOrString::String(string) => {
                embed = embed.field("Return", string.as_str(), false)
            }
        }
    };

    if let Some(val) = &extra.default_value {
        let mut string_val = val.as_str();

        if string_val.is_empty() {
            string_val = "\"\"";
        }

        embed = embed.field("Default Value", string_val, true)
    };

    if let Some(val) = &extra.permitted_values {
        embed = embed.field("Permitted Values", val.as_str(), true)
    };

    if let Some(val) = &extra._type {
        embed = embed.field("Type", val.as_str(), true)
    };

    Some(embed)
}

/// Converts the Tera-formatted markdown into more Discord compatible markdown.
fn format_body(body: &str, data: &Data) -> String {
    let mut replaced_body = body.to_string();

    let link_finder_regex =
        Regex::new(r##"\{\{ *?([a-zA-Z_]*)\(((var|proc)="([a-zA-Z]*?)")? ?\).*?\}\}"##).unwrap();

    for capture in link_finder_regex.captures_iter(body) {
        let original = capture.get(0).unwrap().as_str();
        let type_string = capture.get(1).unwrap().as_str().replace('_', "/");

        let mut formatted = type_string.to_string();

        if let Some(val) = capture.get(3) {
            formatted.push('/');
            formatted.push_str(val.as_str())
        }

        if let Some(val) = capture.get(4) {
            formatted.push('/');
            formatted.push_str(val.as_str())
        }

        replaced_body = replaced_body.replace(
            original,
            format!(
                "[/{link}](https://ref.opendre.am/objects/{link})",
                link = formatted
            )
            .as_str(),
        )
    }

    let mut new_body = replaced_body.clone();

    let internal_link_regex = Regex::new(r"\[(.*?)]\(@/(.*?)\)").unwrap();
    for capture in internal_link_regex.captures_iter(&replaced_body) {
        let original = capture.get(0).unwrap().as_str();

        let display = capture.get(1).unwrap().as_str();
        let path = capture.get(2).unwrap().as_str();

        let url = get_url(path, data.path_to_parsed.get(path).unwrap());

        new_body = new_body.replace(original, format!("[{}]({})", display, url).as_str());
    }

    let tag_cleaner_regex = Regex::new(r"\{\{.*?}}|\{%.*?%}").unwrap();

    new_body = tag_cleaner_regex.replace_all(&new_body, "").to_string();
    new_body = new_body.replace("```dm", "```js");
    new_body.replace("\n\n\n", "\n")
}

/// Converts the internal Zola page structure into something we can link to.
/// Respects the slug set in the frontmatter.
fn get_url(path: &str, data: &PageFrontmatter) -> String {
    let mut path = path.replace(".md", "");
    path = path.replace("_index", "");

    if let Some(slug) = &data.slug {
        let mut components: Vec<&str> = path.split('/').collect();
        components.pop();
        components.push(slug.as_str());

        path = components.join("/");
    };

    format!("https://ref.opendre.am/{path}")
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let records = content::get_all();
    let mut path_to_parsed = HashMap::new();

    let search_index_path = TempDir::new().unwrap();

    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT);
    schema_builder.add_text_field("path", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT);

    let schema = schema_builder.build();

    let index = Index::create_in_dir(&search_index_path, schema.clone()).unwrap();

    let mut index_writer: IndexWriter = index.writer(15_000_000).unwrap();

    let titles =
        generate_titles_to_page(&records, &mut path_to_parsed, &schema, &mut index_writer).unwrap();

    let reader = index
        .reader_builder()
        .reload_policy(tantivy::ReloadPolicy::OnCommitWithDelay)
        .try_into()
        .unwrap();

    let default_fields = vec![
        schema.get_field("title").unwrap(),
        schema.get_field("path").unwrap(),
        schema.get_field("body").unwrap(),
    ];

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![odref()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    titles_to_path: titles,
                    path_to_parsed,
                    path_to_text: records,
                    index,
                    reader,
                    default_fields,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}

/// Parses the frontmatter of our pages, creating a HashMap for the title -> path
/// (but this is not correctly implemented - same name titles clobber
/// eachother non-deterministically), and path -> frontmatter.
fn generate_titles_to_page(
    records: &HashMap<&'static str, &'static str>,
    path_to_parsed: &mut HashMap<String, PageFrontmatter>,
    schema: &Schema,
    index_writer: &mut IndexWriter,
) -> Result<HashMap<String, &'static str>, Box<dyn std::error::Error>> {
    let mut title_map = HashMap::new();

    let frontmatter_regex = Regex::new(r"(?s)\+\+\+(.*)\+\+\+")?;

    let title_field = schema.get_field("title").unwrap();
    let path_field = schema.get_field("path").unwrap();
    let body_field = schema.get_field("body").unwrap();

    for record in records.iter() {
        let frontmatter = match frontmatter_regex.captures(record.1) {
            Some(front) => match front.get(1) {
                Some(capture) => capture.as_str(),
                None => continue,
            },
            None => continue,
        };

        let parsed = toml::from_str::<PageFrontmatter>(frontmatter)?;

        let title = parsed.title.clone().unwrap_or("Empty Title".to_string());
        let path = record.0.to_string();

        index_writer
            .add_document(doc!(
                title_field => title.clone(),
                path_field => path.clone(),
                body_field => record.1.to_string(),
            ))
            .unwrap();

        title_map.insert(title, *record.0);
        path_to_parsed.insert(path, parsed);
    }

    index_writer.commit().unwrap();

    Ok(title_map)
}

#[derive(Deserialize)]
struct PageFrontmatter {
    title: Option<String>,
    slug: Option<String>,
    extra: Option<PageExtra>,
}

#[derive(Deserialize, Clone, Debug)]
struct PageExtra {
    /// For operator pages
    usage: Option<String>,

    /// For proc/ pages
    #[serde(rename = "return")]
    _return: Option<PageReturnOrString>, // and operator pages
    format: Option<Vec<Vec<PageArgs>>>,
    args: Option<Vec<PageArgs>>,

    /// For var/ pages
    default_value: Option<String>,
    permitted_values: Option<String>,
    #[serde(rename = "type")]
    _type: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
enum PageReturnOrString {
    String(String),
    PageReturn(PageReturn),
}

#[derive(Deserialize, Clone, Debug)]

struct PageReturn {
    #[serde(rename = "type")]
    _type: Option<ReturnTypeOrBool>,
    description: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
enum ReturnTypeOrBool {
    String(String),

    #[allow(dead_code)]
    Bool(bool),
}

#[derive(Deserialize, Clone, Debug)]
struct PageArgs {
    name: String,
    description: Option<String>,
    #[serde(rename = "type")]
    _type: Option<String>,
    default_value: Option<String>,
}

impl PageArgs {
    fn get_arg_as_string(&self) -> String {
        let mut string = format!("\t{}", &self.name);

        if let Some(val) = &self._type {
            string = format!("{} as {}", string, val)
        }

        if let Some(val) = &self.default_value {
            string = format!("{} = {}", string, val);
        }

        if let Some(val) = &self.description {
            string = format!("{} // {}", string, val);
        }

        string
    }
}
