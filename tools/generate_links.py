from pathlib import Path
import re
import toml

index = []

getToml = re.compile(r'\+\+\+(.*)\+\+\+', re.M | re.S)

for path in Path("content").rglob('*.md'):
    file = path.open("r")

    contents = file.read()

    match = getToml.match(contents)

    if not match:
        continue

    tomlToLoad = match.group(1)

    parsedToml = toml.loads(tomlToLoad)

    if not "template" in parsedToml or parsedToml["template"] != "object.html":
        continue

    stringPath = path.relative_to("content").as_posix()

    title = parsedToml["title"]
    title = title.replace("/", "", 1)
    title = title.replace("/", "_")

    with open(f"templates/shortcodes/{title}.md", "w") as file:
        file.write('{% import "shortcodes.html" as sc %}\n')
        file.write('{{ ' + f'sc::render_link(type="/{title}", destination="@/{stringPath}", proc=proc | default(value=false), var=var | default(value=false))' + ' }}')
