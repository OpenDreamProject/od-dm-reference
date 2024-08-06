# OpenDream DreamMaker Reference

This project serves the DreamMaker reference, as implemented by [OpenDream](https://github.com/OpenDreamProject/OpenDream). We use [Zola](https://www.getzola.org/) to build our website.

## Contributing to the Reference

DreamMaker objects are kept under `content/objects`, with the `_index.html` pages under a directory providing that section's page. Procs and vars belonging to that object are kept under the respective `proc` and `var` sections.

## Locally previewing the site

All you need is Zola[^1]. The instructions to install Zola are [here](https://www.getzola.org/documentation/getting-started/installation/). You can then run `zola serve` to preview the site as you edit the Markdown files. You will notice that the navigation bar is incomplete - that is to optimize serve times locally - you can read more [here](https://github.com/cmss13-devs/cmss13/issues/6606).

[^1]: As long as you are not altering site CSS.

If you are altering site CSS, we currently use [TailwindCSS](https://tailwindcss.com/) (for now!), and you can edit class names to include Tailwind tags. Simply use `npm install` and `npm run watch`, which will run both `zola serve` (to update site contents) and `tailwind -w` (to update CSS).

## Valid shortcodes

We can use shortcodes to render specific HTML within our Markdown files. They're invoked like this:
```md
{{ some_shortcode(some_keyword_argument="some value!") }}
```

Currently, we have 4 callout shortcodes, which all take a `description` argument:
- never_implemented
- od_only
- parity
- unimplemented

To read up on how to make shortcodes, the Zola documentation is [here](https://www.getzola.org/documentation/content/shortcodes/). All of our shortcodes live in [templates/shortcode](templates/shortcodes/).

## Autodocumentation

Most of the global procs, and objects procs and vars are automatically documented using the [OpenDreamDocumentationTool](https://github.com/harryob/OpenDream/tree/oddt/OpenDreamDocumentationTool), which runs periodically to update this reference from the [DMStandard](https://github.com/OpenDreamProject/OpenDream/tree/master/DMCompiler/DMStandard). These are all automatically updated:
- Global procs
- New objects
- Object vars
- Object procs

And these frontmatter fields are automatically populated, and should not be changed manually:

- Procs
  - Name (`title`)
  - Arguments (`args`): `name` and `type` are derived from DMStandard, but the `description` field is set manually
  - Return type (`return_type`)
  - Override status (`is_override`)
  - Unimplemented status (`od_unimplemented`)

- Vars
  - Name (`title`)
  - Default value (`default_value`)
  - Type (`type`)
  - Override status (`is_override`)
  - Unimplemented status (`od_unimplemented`)

### Not Autodocced

"Procs" that are implemented as defines in DMStandard are not automatically populated. For example, ASSERT and REGEX_QUOTE. Furthermore, many procs are not present in DMStandard, such as istype() and prob() - these can be found [here](https://github.com/OpenDreamProject/OpenDream/blob/master/DMCompiler/Compiler/DM/DMParser.cs#L2460) in the OpenDream source code.

## [extra] Documentation

Zola allows us to put arbitrary TOML under the [extra] section, which we do for the various page types. This includes the fields that editors can set, not the ones pulled from DMStandard (above).

- Objects:
  - `parent_type` - This allows us to construct the parent types for any object - such as /mob going to /atom/movable, then /atom, then /datum.
- Procs:
  - `return_type_desc` - This allows us to add extra information about the return type - for example, what specific return types mean. It can also be used to document the return type if not pulled from DMStandard.
  - `[[args.description]]` - We can document individual arguments to procs with the `description` field. These edits will not be removed by the Autodocumentation tool.
- Vars:
  - `default_value_desc` - This allows us to add extra information about the default value that this variable is set to.


## Available markdown formatting

Zola uses [CommonMark](https://commonmark.org/) and the [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark#pulldown-cmark) parser.

### Syntax highlighting

We support DreamMaker syntax highlighting, with the following:

`````md
```dm
var/my_variable
```
`````

This is based on our [.sublime-syntax](syntaxes/dreammaker.sublime-syntax) file. This can be updated as necessary to keep up with the language.