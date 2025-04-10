using System.Globalization;
using System.Reflection;
using System.Text.RegularExpressions;
using DMCompiler;
using DMCompiler.Compiler.DM;
using DMCompiler.Compiler.DM.AST;
using DMCompiler.Compiler.DMPreprocessor;
using Tomlyn;
using Tomlyn.Model;
using Tomlyn.Syntax;

namespace OpenDreamDocumentationTool;

internal class DMDocProcParameter(string name, string? type, string? value) {
    public readonly string Name = name;
    public readonly string? Type = type;
    public readonly string? Value = value;
}

internal class DMDocProc(string name, List<DMDocProcParameter> parameters, string? returnType, bool isUnimplemented, bool isOverride) {
    public readonly string Name = name;
    public readonly List<DMDocProcParameter> Parameters = parameters;
    public readonly string? ReturnType = returnType;
    public readonly bool IsUnimplemented = isUnimplemented;
    public readonly bool IsOverride = isOverride;
}

internal class DMDocObject {
    public readonly List<DMDocProc> Procs = [];
    public readonly List<DMDocVar> Vars = [];

    public string? GetParentType() {
        return Vars.Where(var => var.Name == "parent_type").Select(var => var.Value).FirstOrDefault();
    }
}

internal class DMDocVar(string name, string? value, bool isOverride, string? type, bool? isUnimplemented) {
    public readonly string Name = name;
    public readonly string? Value = value;
    public readonly string? Type = type;
    public readonly bool IsOverride = isOverride;
    public readonly bool? IsUnimplemented = isUnimplemented;
}

public static partial class Program {
    private static readonly Dictionary<string, DMDocObject> Objects = new();
    private static readonly Dictionary<string, DMDocProc> Procs = new();

    public static void Main(string[] args) {
        var docPath = "od-dm-reference";
        string? dmStandardDirectory = null;
        foreach (var arg in args) {
            if (arg.StartsWith("--standard")) {
                if (!arg.Contains('=')) {
                    throw new Exception("Standard option must point to a path, eg, --standard=/path/to/standard");
                }

                dmStandardDirectory = arg.Split("=").Last();
            }

            if (!arg.StartsWith("--documentation")) continue;
            if (!arg.Contains('=')) {
                throw new Exception("Documentation option must point to a path, eg --documentation=/path/to/repo");
            }

            docPath = arg.Split("=").Last();
        }
        
        DMCompiler.DMCompiler compiler = new()
        {
            Settings = new DMCompilerSettings()
        };

        DMPreprocessor preprocessor = new(compiler, true);

        if (dmStandardDirectory == null) {
            var compilerDirectory = Path.GetDirectoryName(Assembly.GetExecutingAssembly().Location) ?? string.Empty;
            dmStandardDirectory = Path.Join(compilerDirectory, "DMStandard");
        }

        preprocessor.IncludeFile(dmStandardDirectory, "_Standard.dm", true);
        preprocessor.IncludeFile(dmStandardDirectory, "DefaultPragmaConfig.dm", false);

        var dmLexer = new DMLexer(null!, preprocessor);
        var dmParser = new DMParser(compiler, dmLexer);

        var astFile = dmParser.File();

        // Finds existing pages, to allow for nested pages (eg, /atom -> /atom/movable)
        Dictionary<string, string> pathToFile = new Dictionary<string, string>();
        foreach (var file in Directory.EnumerateFiles(docPath, "_index.md", SearchOption.AllDirectories)) {
            var fileContents = File.ReadAllText(file);
            var frontMatch = FrontmatterRegex().Match(fileContents);

            if (!frontMatch.Success) {
                continue;
            }

            var frontmatter = frontMatch.Groups[1].Value;
            var doc = Toml.ToModel(frontmatter);

            var pageTitle = (string)doc["title"];

            if (file.Contains("language/proc")) {
                pathToFile["globalProcs"] = file;
                continue;
            }

            if (!pageTitle.StartsWith('/')) {
                continue;
            }

            pathToFile[pageTitle] = file;
        }

        ParseAstStatements(astFile.BlockInner.Statements);

        var globalProcs = pathToFile["globalProcs"];
        foreach (var pair in Procs) {
            var proc = pair.Value;

            var newProcPage = globalProcs.Replace("_index.md", $"{proc.Name.ToLower()}.md");
            ProcessPage(newProcPage, proc, ProcessProc);
        }

        foreach (var pair in Objects) {
            var obj = pair.Value;
            if (!pathToFile.TryGetValue(pair.Key, out var indexPage)) {
                var parent = Path.Combine(docPath, $"content/objects/{pair.Key.Replace("/", "").ToLower()}");
                Directory.CreateDirectory(parent);
                indexPage = $"{parent}/_index.md";
                using var file = new StreamWriter(indexPage);
                file.Write($"""
                           +++
                           title = "{pair.Key}"
                           template = "object.html"

                           [extra]
                           parent_type = "{obj.GetParentType()}"
                           +++
                           """);
            }

            if (obj.Procs.Count > 0) {
                var procIndex = indexPage.Replace("_index.md", "proc/_index.md");

                if (!File.Exists(procIndex)) {
                    Directory.CreateDirectory(indexPage.Replace("_index.md", "proc/"));
                    using var file = new StreamWriter(procIndex);
                    file.Write("""
                               +++
                               title = "proc"
                               template = "proc_list.html"

                               page_template = "proc.html"
                               +++
                               """);
                }
            }

            foreach(var proc in obj.Procs) {
                var newPage = indexPage.Replace("_index.md", GetValidPageName($"proc/{proc.Name.ToLower()}.md"));
                ProcessPage(newPage, proc, ProcessProc);
            }

            if (obj.Vars.Count > 0) {
                var varIndex = indexPage.Replace("_index.md", "var/_index.md");
                if (!File.Exists(varIndex)) {
                    Directory.CreateDirectory(indexPage.Replace("_index.md", "var/"));
                    using var file = new StreamWriter(varIndex);
                    file.Write("""
                               +++
                               title = "var"
                               template = "var_list.html"

                               page_template = "var.html"
                               +++
                               """);
                }
            }

            foreach (var var in obj.Vars) {
                var newPage = indexPage.Replace("_index.md", GetValidPageName($"var/{var.Name.ToLower()}.md"));
                ProcessPage(newPage, var, ProcessVar);
            }
        }
    }

    private static void ProcessPage(string path, object dmObj, Func<TomlTable, object, string, string> handler) {
        if (File.Exists(path)) {
            var existingContents = File.ReadAllText(path);
            var frontmatter = GetFrontmatter(existingContents);
            if (frontmatter != null) {
                var frontmatterModel = Toml.ToModel(frontmatter);
                var newFrontmatter = handler(frontmatterModel, dmObj, path);
                if (frontmatter == newFrontmatter) {
                    return;
                }

                var content = newFrontmatter;
                WriteToExistingFile(content, path);
                return;
            }

            var newContent = handler(new TomlTable(), dmObj, path);
            WriteToNewFile(newContent, path);
            return;
        }

        var newFileContent = handler(new TomlTable(), dmObj, path);
        WriteToNewFile(newFileContent, path);
    }

    private static void WriteToExistingFile(string contents, string path) {
        var fileContents = File.ReadAllText(path);
        var replaced = FrontmatterRegex().Replace(fileContents, $"+++{contents}+++");

        using var file = new StreamWriter(path);
        file.Write(replaced);
    }

    private static void WriteToNewFile(string contents, string path) {
        var newPageContents = $"+++\n{contents}+++";

        using var file = new StreamWriter(path);
        file.Write(newPageContents);
    }

    private static string ProcessVar(TomlTable existingContents, object toProcess, string path) {
        var var = (DMDocVar)toProcess;

        if (!existingContents.ContainsKey("title")) {
            SetTomlValue(existingContents, "title", var.Name);
        }

        if (path.Split('/').Last().Replace(".md", "") != var.Name) {
            SetTomlValue(existingContents, "slug", var.Name);
        }

        var extras = new TomlTable();

        if (existingContents.TryGetValue("extra", out var existingExtras))
        {
            var extraTable = (TomlTable)existingExtras;
            foreach (var extra in extraTable)
            {
                extras[extra.Key] = extra.Value;
            }

            extras.PropertiesMetadata = extraTable.PropertiesMetadata;
        }
        
        if (var.Value != null) SetTomlValue(extras, "default_value", var.Value);
        else extras.Remove("default_value");
        
        if (var.Type != null && var.Type != "anything") SetTomlValue(extras, "type", var.Type);
        if (var.IsUnimplemented != null) SetTomlValue(extras, "od_unimplemented", var.IsUnimplemented);

        extras["is_override"] = var.IsOverride;

        SetTomlValue(extras, "is_override", var.IsOverride);

        switch (var.IsUnimplemented)
        {
            case false when extras.ContainsKey("od_unimplemented"):
                extras.Remove("od_unimplemented");
                break;
            case true:
                extras["od_unimplemented"] = var.IsUnimplemented;
                break;
        }

        existingContents.Remove("extras");

        existingContents["extra"] = extras;

        return Toml.FromModel(existingContents);
    }

    private static string ProcessProc(TomlTable existingContents, object toProcess, string path) {
        var proc = (DMDocProc)toProcess;

        if (!existingContents.ContainsKey("title")) {
            existingContents["title"] = proc.Name;
        }

        if (path.Split('/').Last().Replace(".md", "") != proc.Name) {
            SetTomlValue(existingContents, "slug", proc.Name);
        }

        existingContents.TryGetValue("extra", out var potentialExtras);
        var extras = (TomlTable?)potentialExtras ?? new TomlTable();
        
        if (proc.ReturnType != null)
        {
            var newReturns = new TomlTable(false);
            if(extras.TryGetValue("return", out var potentialReturns))
            {
                var existingReturns = (TomlTable)potentialReturns;
                foreach(var entry in existingReturns)
                {
                    newReturns[entry.Key] = entry.Value;
                }

                newReturns.PropertiesMetadata = existingReturns.PropertiesMetadata;
            }
            SetTomlValue(newReturns, "type", proc.ReturnType);
            extras["return"] = newReturns;
        }

        if (proc.Parameters.Count > 0) {
            extras.TryGetValue("args", out var potentialArgs);
            var tomlArgs = new TomlTableArray();
            var existingArgs = (TomlTableArray?)potentialArgs;

            Dictionary<string, TomlTable?> nameToArg = new Dictionary<string, TomlTable?>();
            if (existingArgs != null)
                foreach (var param in existingArgs) {
                    var name = (string)param["name"];
                    nameToArg[name] = param;
                }

            foreach (var parameter in proc.Parameters) {
                var tomlParameter = new TomlTable(false);

                if (nameToArg.TryGetValue(parameter.Name, out var param))
                {
                    foreach (var existing in param!)
                    {
                        tomlParameter[existing.Key] = existing.Value;
                    }

                    tomlParameter.PropertiesMetadata = param.PropertiesMetadata;
                }

                SetTomlValue(tomlParameter, "name", parameter.Name, "AUTOGEN STATIC");
                if (parameter.Type != null) SetTomlValue(tomlParameter, "type", parameter.Type);
                if (parameter.Value != null) SetTomlValue(tomlParameter, "default_value", parameter.Value);

                tomlArgs.Add(tomlParameter);
            }

            extras["args"] = tomlArgs;
        }

        switch (proc.IsOverride)
        {
            case true:
                SetTomlValue(extras, "is_override", proc.IsOverride);
                break;
            default:
                extras.Remove("is_override");
                break;
        }

        switch (proc.IsUnimplemented)
        {
            case false when extras.ContainsKey("od_unimplemented") && !IsTomlSkip(extras, "od_unimplemented"):
                extras.Remove("od_unimplemented");
                break;
            case true:
                SetTomlValue(extras, "od_unimplemented", proc.IsUnimplemented);
                break;
        }

        existingContents["extra"] = extras;

        return Toml.FromModel(existingContents);
    }

    private static void ParseAstStatements(DMASTStatement[] astStatements) {
        foreach (var statement in astStatements) {
            switch (statement) {
                case DMASTObjectDefinition objectDefinition:
                    Objects[objectDefinition.Path.PathString] = new DMDocObject();
                    if (objectDefinition.InnerBlock != null)
                        ParseAstStatements(objectDefinition.InnerBlock!.Statements);
                    break;

                case DMASTProcDefinition procDefinition:
                    List<DMDocProcParameter> parsedParameters = [];
                    foreach (var parameter in procDefinition.Parameters) {
                        parsedParameters.Add(
                            new DMDocProcParameter(parameter.Name, parameter.ObjectType?.PathString, GetValueFromDmastExpression(parameter.Value)));
                    }

                    var unimplemented = false;
                    if (procDefinition.Body?.SetStatements != null)
                        foreach (var potentialStatement in procDefinition.Body?.SetStatements!) {
                            if (potentialStatement is DMASTProcStatementSet { Attribute: "opendream_unimplemented" }) {
                                unimplemented = true;
                            }
                        }

                    DMDocProc newProc = new(procDefinition.Name,
                        parsedParameters,
                        CleanReturnTypes(procDefinition.ReturnTypes?.ToString().Trim('"')) ?? procDefinition.ReturnTypes?.TypePath?.PathString,
                        unimplemented,
                        procDefinition.IsOverride
                        );

                    if (procDefinition.ObjectPath.PathString != "/") {
                        Objects[procDefinition.ObjectPath.PathString].Procs.Add(newProc);
                        continue;
                    }

                    Procs[procDefinition.Name] = newProc;
                    break;

                case DMASTObjectVarDefinition varDefinition:
                    if (varDefinition.ObjectPath.PathString == "/") break;
                    var varDefinitionObj = Objects[varDefinition.ObjectPath.PathString];

                    var type = varDefinition.ValType.ToString().Trim('"');
                    if (type == "noconstfold")
                    {
                        type = null;
                    }
                    
                    varDefinitionObj.Vars.Add(new DMDocVar(varDefinition.Name,
                        GetValueFromDmastExpression(varDefinition.Value),
                        false,
                        varDefinition.Type?.PathString ?? type,
                        varDefinition.ValType.IsUnimplemented
                        ));
                    break;

                case DMASTObjectVarOverride varOverride:
                    var varOverrideObj = Objects[varOverride.ObjectPath.PathString];

                    varOverrideObj.Vars.Add(new DMDocVar(varOverride.VarName,
                        GetValueFromDmastExpression(varOverride.Value),
                        true,
                        null,
                        null
                        ));
                    break;
            }
        }
    }

    private static void SetTomlValue(TomlTable toml, string key, object? value, string comment = "AUTOGEN FIELD") {
        if (IsTomlSkip(toml, key)) {
            return;
        }

        toml[key] = value!;
        toml.PropertiesMetadata ??= new TomlPropertiesMetadata();
        toml.PropertiesMetadata.SetProperty(key,
            new TomlPropertyMetadata {
                TrailingTrivia = [
                    new TomlSyntaxTriviaMetadata(TokenKind.Whitespaces, " "),
                    new TomlSyntaxTriviaMetadata(TokenKind.Comment, $"# {comment}")
                ]
            });
    }

    private static bool IsTomlSkip(TomlTable toml, string key) {
        if (toml.PropertiesMetadata?.TryGetProperty(key, out var metadata) != true) return false;

        if (metadata?.TrailingTrivia == null) return false;

        foreach (var meta in metadata.TrailingTrivia) {
            if (meta.Text != null && meta.Text.Contains("AUTOGEN SKIP")) {
                return true;
            }
        }

        return false;
    }

    private static string? CleanReturnTypes(string? types)
    {
        if (types == null) return null;
        var clean = types.Split(", ").Where(part => !part.Contains("path")).ToList();
        return string.Join(", ", clean);
    }

    /// <summary>
    /// Returns a Zola-safe path. Currently only removes "index" as a path, as this upsets Zola's rendering.
    /// </summary>
    /// <param name="attemptedName">Path we want to use.</param>
    /// <returns>string, a Zola-safe path.</returns>
    private static string GetValidPageName(string attemptedName) {
        return attemptedName.Replace("/index", "/page_index");
    }

    /// <summary>
    /// Quick and dirty way of grabbing the string value of a constant DMAST expression
    /// </summary>
    /// <param name="expression">The constant DMAST expression we want to check.</param>
    /// <returns>string, the value of the expression.</returns>
    private static string? GetValueFromDmastExpression(DMASTExpression? expression) {
        return expression switch {
            DMASTConstantPath path => path.Value.Path.PathString,
            DMASTConstantFloat constantFloat => constantFloat.Value.ToString(CultureInfo.CurrentCulture),
            DMASTConstantInteger constantInteger => constantInteger.Value.ToString(),
            DMASTConstantNull => "",
            DMASTConstantString constantString => constantString.Value,
            _ => null
        };
    }

    private static string? GetFrontmatter(string pageToCheck) {
        var frontMatch = FrontmatterRegex().Match(pageToCheck);
        return frontMatch.Success ? frontMatch.Groups[1].Value : null;
    }

    [GeneratedRegex(@"\+\+\+(.*)\+\+\+", RegexOptions.Singleline)]
    private static partial Regex FrontmatterRegex();
}
