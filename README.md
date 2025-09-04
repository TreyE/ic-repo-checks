# ic-repo-checks

Checks for compliance with IdeaCrew repository standards and workflows.

Currently checks for:
* Dependabot Enabled
* A .github/dependabot.yml file
* Reporting to Yellr
* For private repositories, a .copilotignore file has been added.

## Action Configuration

<!-- BEGIN_ACTION_DOCS -->

# ic-repo-checks
Checks for compliance with IdeaCrew repository standards and workflows.

# inputs
| Title | Required | Type | Default| Description |
|-----|-----|-----|-----|-----|
| access_token | True | string |  | Access token used for more sensitive endpoints. |
| check_dependabot | False | boolean | `true` | Perform dependabot compliance checks. |
