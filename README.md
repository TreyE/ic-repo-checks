# ic-repo-checks

Checks for compliance with IdeaCrew repository standards and workflows.

Currently checks for:
* Dependabot Enabled
* A .github/dependabot.yml file
* Reporting to Yellr
* For private repositories, a .copilotignore file has been added.

## Action Configuration

# inputs
| Title | Required | Type | Default| Description |
|-----|-----|-----|-----|-----|
| access_token | True | string |  | Access token used for more sensitive endpoints. |
| check_dependabot | False | boolean | `true` | Perform dependabot compliance checks. |
| check_yellr | False | boolean | `true` | Check if the repository reports to Yellr. |
