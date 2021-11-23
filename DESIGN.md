# Design notes

## Evolution files

Evolution files describe the changes made to the api. They should be grouped into releases that follow semver, after which they should not change.

### Requirements

* Include types, validations and communication protocols.
* Include documentation and examples (which may be used for test generation).
* No changes after release (comments and descriptions can change).
* Fixed version after release, that is easy to find from the source.
* Strict validation of changes, flag all backwards incompatible changes.
* Before release, making changes should be easy, but have full validation.
* Pending changes and identical released changes should behave the same.
* Breaking changes must bump major version. New endpoints must bump minor version.
* When merging VCS branches, avoid unnecessary manual merging.
* When merging VCS branches, avoid getting invalid results without merge conflict.
* Creating a release with every merge to main branch should be easy.
* Nice to have, probably hard: creating a release on feature branch should not conflict.
* Nice to have: preserve VCS blame when releasing.

### Solution

This is a draft, not set in stone:

* An evolution file contains a group of functionally related changes.
* Pending changes live at the top level of the apivolve directory.
* When making a release:
  1. The pending changes are scanned to see major, minor or patch should be bumped.
  2. A directory named after the version is created (v1.2.3) in evolution directory.
  3. Unless filtered by CLI arguments, all evolutions are moved to the directory.
  4. A special file is created containing 1) the previous version and its hash and 2) the new evolution hash. (This file also helps create a merge conflict when two branches release the same version.)
  5. Hash does not include comments, descriptions or valid examples.
* This can be done on some schedule, of after each change.

Questions remaining:

* Should pending changes be in a separate directory, or top level?
* How is the order of changes in a version determined? Or must it always be idempotent unless dependencies are specified explicitly? 
* How to deal with multiple evolution directories?
* Is there a way to get merge conflicts for conflicting changes? E.g. evolution filename is endpoint or object name?

## Goals

Apivolve works best when there is one party that has the latest version, and any number of parties or data sources on various older versions.

### Difficulties

Apivolve's goals make some scenarios harder:

* Broadcasting: must use the oldest version in use, or send separate messages to each (which should be easy with apivolve, but is not broadcasting).
* Having a number of components 
* Having proxy or router component that try to parse messages. If Apivolve does not know about those components, it might pick a version that is too new for them. Either keep those components up-to-date, or make them active participants in the communication (i.e. consumer on one side and server on another).

### Schemas

Having schemas and documentation for data is valuable, whether it is configuration, data or rest apis.

While they may not be 100% matches in features, likely candidates include JSON Schema. Such a schema may be generated along with the API for each version.

### Future applications

The primary use case is communication APIs. For example http or socket based systems with client-server architecture (apps, programs, websites...).

But there are other application to which is might be well-suited in the future, none of which are certain.

#### Structured file storage
 
Store files along with Apivolve version, and read them with a newer version of the software almost automatically.

This would rely largely on being able to consume older versions - the uses of storing older versions are perhaps more limited.

No large structural changes are needed, just utilities for storing data along with version metadata.

#### Configuration files

Similar to structured file storage, Apivolve could be used to read configuration files, like yaml or json.

#### Form generation

The Apivolve schema should contain enough data to generate html forms. These could for example be used in sending manual test requests.

It is doubtful these would replace production-level, customer-facing forms. It seems difficult to add the required flexibility for those. But it may be useful for internal tooling.

This would include sending requests with different older versions.

#### Admin panel

As an extension of the form generation mentioned above, a full admin panel for data management could be generated, provided the data itself is managed by Apivolve.

In this case, the version aspect does not play a big role, any rich enough schema could be used for this.

#### Unit test generation

Parts of unit tests could be generated, such as example test requests.

The validation of functionality would still need to be hand-written, but it could save some setup and mocking.

#### Database migrations

This one is a long shot, and probably won't happen. Apivolve and relational databases have fairly different structure.

Apivolve evolutions are conceptually somewhat similar to SQL migrations: an old version is changes to a newer one, describing what should happen to older data to make it modern.


