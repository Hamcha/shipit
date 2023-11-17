# shipit - Stop cloning in your CI!

`shipit` is a tool for committing changes to JSON/YAML files from CI environments to supported Git providers.

## Why?

GitOps-enabled environments use git repositories to store deployment manifests. It's common to have the CI update those repositories but the traditional "git clone/commit/push" approach can lead to a lot of time wasted downloading unneeded info. `shipit` leverages REST APIs provided by certain Git providers (such as GitHub, GitLab, and more—see the list below) to make the process super snappy!

## Support table

### Git Providers

- Gitea / Forgejo
- GitLab (both self-managed and gitlab.com)
- (planned) Azure DevOps
- (planned) BitBucket (only BitBucket cloud ie. bitbucket.org)
- (planned) GitHub (both GitHub.com and GitHub Enterprise Server)

### Templaters

- JSON (generic)
- YAML (generic)
- Nix

## Usage

### Using Templaters

#### Nix

The Nix templater replaces attributes within a Nix file. Arrays and other field types are not supported.

For each templater entry, specify a `file` (path from repo root) and a dictionary of changes (field path = new value).

Values will by default be wrapped with "" to indicate strings, if you want to specify a raw identifier (like a number or identifier), prefix the value with `r##`.

```json
{
  "templater": "nix",
  "file": "path/to/file.nix",
  "changes": {
    "path.to.field": "new_string_value",
    "path.to.other.field": "r##my_dynamic value"
  }
}
```
