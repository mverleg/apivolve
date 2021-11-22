# Design notes

## Evolution files

Evolution files describe the changes made to the api. They should be grouped into releases that follow semver, after which they should not change.

### Requirements

* Include types, validations and communication protocols.
* No changes after release (comments and descriptions can change).
* Fixed version after release, that is easy to find from the source.
* Strict validation of changes, flag all backwards incompatible changes.
* Before release, making changes should be easy, but have full validation.
* Breaking changes must bump major version. New endpoints must bump minor version.
* When merging VCS branches, avoid unnecessary manual merging.
* When merging VCS branches, avoid getting invalid results without merge conflict.
* Creating a release with every merge to main branch should be easy.
* Nice to have, probably hard: creating a release on feature branch should not conflict.
* Nice to have: preserve VCS blame when releasing.


