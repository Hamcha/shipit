# shipit - Stop cloning in your CI!

`shipit` is a tool for committing changes to JSON/YAML files from CI environments to supported Git providers.

## Why?

GitOps-enabled environments use git repositories to store deployment manifests. It's common to have the CI update those repositories but the traditional "git clone/commit/push" approach can lead to a lot of time wasted downloading unneeded info. `shipit` leverages REST APIs provided by certain Git providers (such as GitHub, GitLab, and more—see the list below) to make the process super snappy!

## Support table

### Git Providers

- `gitea` Gitea / Forgejo
- `gitlab` GitLab (both self-managed and gitlab.com)
- (planned) Azure DevOps
- (planned) BitBucket (only BitBucket cloud ie. bitbucket.org)
- (planned) GitHub (both GitHub.com and GitHub Enterprise Server)
### Templaters

- `json` JSON (generic)
- `yaml` YAML (generic)

