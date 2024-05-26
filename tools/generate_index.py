from pathlib import Path
import re
import toml

index = []

getToml = re.compile(r'\+\+\+(.*)\+\+\+', re.M | re.S)
getTitle = re.compile(r'title = \"(.*)\"')

for path in Path("content").rglob('*.md'):
    file = path.open("r")

    contents = file.read()

    match = getToml.match(contents)

    if not match:
        continue

    tomlToLoad = match.group(1)

    print(path)

    parsedToml = toml.loads(tomlToLoad)

    stringPath = path.relative_to("content").as_posix()

    stringParts = stringPath.split("/")

    title = parsedToml["title"]

    parentPart = stringParts[len(stringParts) - 2]
    print(parentPart)
    if parentPart == "proc" or parentPart == "var":
        title += f" ({stringParts[len(stringParts) - 3]} {parentPart})"

    url = stringPath.replace(".md", "").replace("/_index", "")

    index.append({ "path": stringPath, "title": title, "url": url })

file = open("tools/stork_input.toml", "r")
tomlContent = file.read()
parsed = toml.loads(tomlContent)
parsed["input"]["files"] = index

with open("tools/stork_input.toml", "w") as file:
    toml.dump(parsed, file)

print(index)