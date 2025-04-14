# CHANGELOG

All significant changes made to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/fr/1.1.0/)
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)

from zed-puppet v0.2.0.

--------------------------------

## 0.2.0, 04/08/2025

### Upgrade

- bump zed_extension_api to v0.6.0

### Fixes

- fix language_servers name in extension.toml and remove it from `languages/puppet/config.toml`
- fix highlight.scm @conditional
- fix indents.scm @indent.begin @indent.end
- fix brackets.scm for '()'

### Features

- add support for workspace configuration
