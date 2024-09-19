const fs = require('fs');
const { Taplo } = require("@taplo/lib")

const frontMatcher = new RegExp('(\\+\\+\\+)(.*)(\\+\\+\\+.*)', "s")

const taploConfig = {
    options: {
        alignEntries: false,
        alignComments: false
    }
}

const main = async() => {
    const taplo = await Taplo.initialize();

    var files = fs.readdirSync("content", { recursive: true }).filter(fn => fn.endsWith(".md"))
    files.forEach(file => {
        processFile(file, taplo)
    });

}

const processFile = async(file, taplo) => {
    fs.readFile(`content/${file}`, 'utf8', (err, data) => {
        const match = data.match(frontMatcher)
        if(!match) return
    
        const frontmatter = match[2]
        const newFrontmatter = taplo.format(frontmatter, taploConfig)
    
        const newFile = data.replace(frontMatcher, newFrontmatter)
        fs.writeFileSync(`content/${file}`, `${match[1]}${newFile}${match[3]}`)
    })
}

main()