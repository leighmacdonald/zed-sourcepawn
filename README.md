# zed-sourcepawn

A [zed](https://zed.dev) extension for [sourcepawn](https://github.com/alliedmodders/sourcepawn) & [SourceMod](https://sourcemod.net)

The vast majority of the functionality is provided by the following libs, this extension simply provides the plumbing required:

  - [sourcepawn-studio](https://github.com/SourcePawn-Studios/sourcepawn-studio) LSP
  - [tree-sitter-sourcepawn](https://github.com/nilshelmig/tree-sitter-sourcepawn) Syntax highlighting


  ## Configuration 

```json
{
  "lsp": {
    "sourcepawn-studio": {
      "initialization_options": {
        "eventsGameName": "Team Fortress 2",
       	"includeDirectories": [
         	"/path/to/addons/sourcemod/scripting/include"
       	]
      }
    }
  }
}
  ```
