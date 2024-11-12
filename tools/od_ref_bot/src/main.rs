use std::collections::HashMap;

use poise::serenity_prelude::{self as serenity, Colour, CreateEmbed};
use regex::Regex;
use toml::Table;

mod content;

struct Data {
    titles_to_path: HashMap<String, &'static str>,
    path_to_parsed: HashMap<String, Table>,
    path_to_text: HashMap<&'static str, &'static str>,
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

    let page = get_page(search_for, ctx.data()).unwrap_or("Not found.");

    match format_embed(page, ctx.data()) {
        Some(embed) => ctx.send(poise::CreateReply::default().embed(embed)).await?,
        None => {
            ctx.send(
                poise::CreateReply::default()
                    .embed(CreateEmbed::default().description("An error occured.")),
            )
            .await?
        }
    };

    Ok(())
}

fn clean_query(query: String) -> String {
    query.trim().to_lowercase().to_string()
}

fn get_page(query: String, data: &Data) -> Option<&str> {
    let mut path_find = query.replace(" ", "/");

    if path_find.chars().nth(0) == Some('/') {
        path_find = path_find[1..].to_string();
    }

    match data
        .path_to_text
        .get_key_value(format!("objects/{}/_index.md", path_find).as_str())
    {
        Some(string) => return Some(*string.0),
        None => (),
    }

    if path_find.contains("/") {
        let components: Vec<&str> = path_find.split("/").collect();

        let mut var = components.clone();
        var.insert(components.len() - 1, "var");

        let var_string = var.join("/");

        match data.path_to_text.get_key_value(format!("objects/{}.md", var_string).as_str()) {
            Some(string) => return Some(*string.0),
            None => (),
        }

        let proc_string = var_string.replace("var", "proc");
        match data.path_to_text.get_key_value(format!("objects/{}.md", proc_string).as_str()) {
            Some(string) => return Some(*string.0),
            None => (),
        }
    }

    match data.titles_to_path.get(&query) {
        Some(string) => return Some(*string),
        None => (),
    }

    for thing in data.path_to_text.iter() {
        if thing.0.contains(&path_find) {
            return Some(*thing.0);
        }
    }

    None
}

fn format_embed(page: &str, data: &Data) -> Option<serenity::CreateEmbed> {
    let body_regex = Regex::new(r"(?s)\+\+\+(.*)\+\+\+(.*)").unwrap();

    let parsed = data.path_to_parsed.get(page)?;

    let body = body_regex
        .captures(data.path_to_text.get(page)?)?
        .get(2)?
        .as_str();

    let mut title = parsed.get("title")?.as_str()?.to_string();

    let mut components: Vec<&str> = page.split("/").collect();

    let proc = components.contains(&"proc");
    if proc || components.contains(&"var") {
        components.truncate(components.len() - 2);
        components.push("_index.md");

        let parent_parsed = data.path_to_parsed.get(&components.join("/"))?;

        title = format!("{} ({} {})", title, parent_parsed.get("title")?.as_str()?, if proc { "proc" } else { "var" })
    };

    let embed = serenity::CreateEmbed::default()
        .title(title)
        .url(get_url(page, parsed))
        .color(Colour::from_rgb(246, 114, 128))
        .description(format_body(body));

    let extra = match parsed.get("extra") {
        Some(table) => table.as_table()?,
        None => return Some(embed),
    };

    let embed = match extra.get("usage") {
        Some(val) => embed.field("Usage", val.as_str()?, false),
        None => embed,
    };

    let embed = match extra.get("return") {
        Some(val) => {
            if val.is_str() {
                embed.field("Return", val.as_str()?, false)
            } else {
                let table = val.as_table()?;

                let embed = match table.get("type") {
                    Some(val) => embed.field("Return Type", val.as_str()?, false),
                    None => embed,
                };

                let embed = match table.get("description") {
                    Some(val) => embed.field("Return Description", val.as_str()?, false),
                    None => embed,
                };

                embed
            }
        }
        None => embed,
    };

    let embed = match extra.get("default_value") {
        Some(val) => {
            let mut string_val = val.as_str()?;

            if string_val.len() == 0 {
                string_val = "\"\"";
            }

            embed.field("Default Value", string_val, true)
        }
        None => embed,
    };

    let embed = match extra.get("permitted_values") {
        Some(val) => embed.field("Permitted Values", val.as_str()?, true),
        None => embed,
    };

    let embed = match extra.get("type") {
        Some(val) => embed.field("Type", val.as_str()?, true),
        None => embed,
    };

    Some(embed)
}

fn format_body(body: &str) -> String {
    let mut replaced_body = body.to_string();

    let link_finder_regex =
        Regex::new(r##"\{\{ *?([a-zA-Z]*)\(((var|proc)="([a-zA-Z]*?)")? ?\).*?\}\}"##).unwrap();

    for capture in link_finder_regex.captures_iter(body) {
        let original = capture.get(0).unwrap().as_str();
        let type_string = capture.get(1).unwrap().as_str().replace("_", "/");

        let mut formatted = type_string.to_string();

        match capture.get(3) {
            Some(val) => {
                formatted.push_str("/");
                formatted.push_str(val.as_str())
            }
            None => (),
        }

        match capture.get(4) {
            Some(val) => {
                formatted.push_str("/");
                formatted.push_str(val.as_str())
            }
            None => (),
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

    let tag_cleaner_regex = Regex::new(r"\{\{.*?}}").unwrap();

    replaced_body = tag_cleaner_regex
        .replace_all(&replaced_body, "")
        .to_string();
    replaced_body.replace("```dm", "```js")
}

fn get_url(path: &str, data: &Table) -> String {
    let mut path = path.replace(".md", "");
    path = path.replace("_index", "");

    match data.get("slug") {
        Some(slug) => {
            let mut components: Vec<&str> = path.split("/").collect();
            components.pop();
            components.push(slug.as_str().unwrap());

            path = components.join("/");
        }
        None => (),
    };

    format!("https://ref.opendre.am/{path}")
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let records = content::get_all();
    let mut path_to_parsed = HashMap::new();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![odref()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    titles_to_path: generate_titles_to_page(&records, &mut path_to_parsed).unwrap(),
                    path_to_parsed,
                    path_to_text: records,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}

fn generate_titles_to_page(
    records: &HashMap<&'static str, &'static str>,
    path_to_parsed: &mut HashMap<String, Table>,
) -> Result<HashMap<String, &'static str>, Box<dyn std::error::Error>> {
    let mut title_map = HashMap::new();

    let frontmatter_regex = Regex::new(r"(?s)\+\+\+(.*)\+\+\+")?;

    for record in records.iter() {
        let frontmatter = match frontmatter_regex.captures(record.1) {
            Some(front) => match front.get(1) {
                Some(capture) => capture.as_str(),
                None => continue,
            },
            None => continue,
        };

        let parsed = toml::from_str::<Table>(frontmatter)?;

        {
            let title = match parsed.get("title") {
                Some(title) => match title.as_str() {
                    Some(title) => title,
                    None => continue,
                },
                None => continue,
            };

            title_map.insert(title.to_string(), *record.0);
        }

        path_to_parsed.insert(record.0.to_string(), parsed);
    }

    Ok(title_map)
}
