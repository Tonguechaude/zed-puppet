# zed-puppet

Puppet language extension for the Zed editor that provides support for:

 * Syntax highlighting
 * Puppet icon for .pp files (Needs to be enabled via "Icon theme selector")
 * LSP support with puppet-editor-services lsp. Check below for more info.

## How to install?

Current stable version can be installed through the Zed Editor -> Extensions.

## Using Puppet LSP (Puppet Editor Services)

The Puppet LSP integration is done via [Puppet Editor Services](https://github.com/puppetlabs/puppet-editor-services).

__NOTE: This extension does not install the LSP server!__

For Zed to detect the `puppet-languageserver` executable, it needs to be in your __PATH__.
To confirm it's there, run `puppet-languageserver -v` and you should see something similar:

```
$ puppet-languageserver -v
2.0.4
```

It is possible to pass puppet config settings to the server. To do that, open Zed Settings and under the `lsp` config
section add the following:

```
"puppet-languageserver": {
    "binary": {
        "arguments": [
            "--stdio",
            "--puppet-settings=--environment,production"
        ]
    }
}
```

```
"lsp": {
  "puppet-languageserver": {
    "binary": {
      "path": "/opt/puppet-editor-services/puppet-languageserver",
      "arguments": [
        "--stdio",
        "--no-cache",
        "--puppet-settings=--environment,production"
      ]
    },
    "settings": {
      "puppet": {
        "modulePath": "./modules:./site-modules",
        "environment": "development"
      }
    }
  },
}
```

The `puppet-settings` expects comma separated list of cli flags. Full list of config options can be found [here](https://www.puppet.com/docs/puppet/7/configuration.html).

## Additional info

The tree sitter parser is based on [tree-sitter-puppet](https://github.com/tree-sitter-grammars/tree-sitter-puppet), licensed under the MIT license.

## Development

To setup a local dev environment, you will need to have installed.
