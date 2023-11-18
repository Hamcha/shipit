# shipit - Stop using `git clone` for your deployments

`shipit` is a tool for committing changes to JSON/YAML files from CI environments to supported Git providers.

## Why?

GitOps-enabled environments use git repositories to store deployment manifests. It's common to have the CI update those repositories but the traditional "git clone/commit/push" approach can lead to a lot of time wasted downloading unneeded info. `shipit` leverages REST APIs provided by certain Git providers (such as GitHub, GitLab, and more—see the list below) to make the process super snappy!

## Support table

### Git Providers

- [Gitea / Forgejo](#gitea--forgejo)
- [GitLab](#gitlab) (both self-managed and gitlab.com)
- (planned) Azure DevOps
- (planned) BitBucket (only BitBucket cloud ie. bitbucket.org)
- (planned) GitHub (both GitHub.com and GitHub Enterprise Server)

### Templaters

- [JSON](#json)
- [YAML](#yaml)
- [Nix](#nix)

## Usage

```
Usage: shipit.exe [OPTIONS]

Options:
  -p, --provider <provider>  Provider info (as JSON) [env: SHIPIT_PROVIDER=]
  -c, --changeset <changes>  Changes to apply (as JSON) [env: SHIPIT_CHANGES=]
  -a, --author <author>      Commit author (as 'name <email>') [env: SHIPIT_AUTHOR=] [default: shipit]
  -b, --branch <branch>      Branch to commit to [env: SHIPIT_BRANCH=] [default: main]
  -m, --message <message>    Commit message [env: SHIPIT_MESSAGE=] [default: "Update deployment"]
  -h, --help                 Print help
  -V, --version              Print version
```

To use shipit you will need to specify a provider and a changeset.

A provider is a Git forge with supported APIs. A changeset is a list of file with the fields that need to be updated. You can specify multiple files in a changeset, each with their own templater (e.g. you can modify both a YAML and JSON file in a single call). shipit will *usually* perform all changes in a single commits but some APIs might only allow one file per commit so make sure to check the description for your provider.

Both provider and changeset are specified as JSON, either via command line or environment variable, like in the following table:

| Parameter      | Environment variable | CLI parameter   | Format                                                     |
| -------------- | -------------------- | --------------- | ---------------------------------------------------------- |
| Provider       | SHIPIT_PROVIDER      | -p, --provider  | `{"provider":<provider id>, ...<provider args>}`           |
| Changeset      | SHIPIT_CHANGES       | -c, --changeset | `[{"templater":<templater id>, ...<templater args>}, ...]` |
| Branch         | SHIPIT_BRANCH        | -b, --branch    | `main`                                                     |
| Commit author  | SHIPIT_AUTHOR        | -a, --author    | `Shipit deploy <gitops@example.tld>`                       |
| Commit message | SHIPIT_MESSAGE       | -m, --message   | `Updated image tag`                                        |

Every file specified in the changeset **MUST** exist.

### Using providers

#### Gitea / Forgejo

To use Gitea/Forgejo you will need to create an application, go to `<your-instance-url>/user/settings/applications` and generate a new token. Make sure to give it "repository: Read and Write" permissions, then use the following format for the provider parameter:

```json
{
  "provider": "gitea",
  "api_url": "https://<your-instance-url>/api/v1",
  "project_id":"<username>/<repo_name>",
  "token": "<username>:<token>"
}
```

#### GitLab

You will need to create an access token, either a project or account (project is probably better). You can find them at: `<your-instance-url>/<username>/<project-id>/-/settings/access_tokens`

The required scopes are `api, write_repository` and the role should be Maintainer, then use the following as your provider parameter:

```json
{
  "provider": "gitea",
  "api_url": "https://<your-instance-url>/api/v4",
  "project_id":"<username>/<repo_name>",
  "token": "<token>"
}
```

You can omit `api_url` if you are using Gitlab.com

### Using Templaters

Templaters are change operations that operate on a specific type of file, unlike providers you can specify as many templaters you want in a single call to shipit, for example:

```json

[
  { "templater": "json", "file": "cdk.json", "changes": { "image/tag": "2.0@abcdef" } },
  { "templater": "yaml", "file": "kustomize.yml", "changes": { "image.tag": "2.0@abcdef" } },
]
```

#### JSON

The JSON templater replaces values within a JSON file. Keys are provided as strings using `/` for separation, you can specify array indexes for array (e.g. `nested/field/10/tag` becomes `nested.field[10].tag`).

Your changeset should look like this:

```json
[
  {
    "templater":"json",
    "file":"/path/to/file.json",
    "changes":{
      "root-field":"new value",
      "field/nested/0": "nested value"
    }
  }
]
```

The fields **MUST** exist in the file or the change will fail.

#### YAML

The YAML templater replaces values within a YAML file. Keys are provided as strings using `.` for separation, you can specify array indexes for array (e.g. `nested.field.10.tag` becomes `nested.field[10].tag`).

Your changeset should look like this:

```json
[
  {
    "templater":"yaml",
    "file":"/path/to/file.json",
    "changes":{
      "root-field":"new value",
      "field.nested.0": "nested value"
    }
  }
]
```

The fields **MUST** exist in the file or the change will fail.

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
