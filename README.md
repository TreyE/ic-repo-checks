# ic-repo-checks

Checks for compliance with IdeaCrew repository standards and workflows.

Currently checks for:
* Dependabot Enabled
* A .github/dependabot.yml file
* Reporting to Yellr (configurable)
* For private repositories, a .copilotignore file has been added.
* Bundler Audit (configurable)
* Default Branch Protections (configurable)

## Action Configuration

# Inputs

| Title | Required | Type | Default| Description |
|-----|-----|-----|-----|-----|
| access_token | True | string |  | Access token used for more sensitive endpoints. |
| check_dependabot | False | boolean | `true` | Perform dependabot compliance checks. |
| check_yellr | False | boolean | `true` | Check if the repository reports to Yellr. |
| check_bundler_audit | False | boolean | `true` | Check if the repository has bundler-audit. |
| check_default_branch_protected | False | boolean | `true` | Check if the default repository branch is protected. |
