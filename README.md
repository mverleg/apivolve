# Apivolve

Apivolve is an API evolution tool, it helps keep your APIs backwards compatible yet clean, and generates client/server code in a variety of languages.

## Is it ready for use?

**It is not ready for use**, sorry. It is not finished enough even for experimental use, sorry. Any help is welcome! 

## What does it do?

### Migrations

Instead of having API definition files that you keep changing, you maintain 'migration' files that describe API changes.

By using migrations, Apivolve can not only check backwards compatibility, but often provide it. For example:

* Change a field from number to text, and/or rename it: Apivolve will convert requests with the old field into the new one, and your endpoint only ever sees the newest version.
* Combine first and last name fields into one: Apivolve will automatically convert requests with the old fields into a new one with a single name field, again hiding backwards compatibility from your code.
* You make an optional field required: Apivolve will remind you to either provide a default value, or release a new major version.

This works best if the endpoint server uses the newest code, but clients may use various outdated versions.

### Code generation (DRY)

You can define your API in one place, then generate server and client code in a variety of languages.

This is similar to schemas like XSD, Protobuf, JSON Schema...

But unlike those, Apivolve does not just generate the latest version, it also generates all the code needed to handle clients using older APIs.

Apivolve embraces the idea that APIs are not just en/decoding, but should clearly document what they need/provide. And the best way to do that is by including constraints in the api (as opposed to in the application logic).

## How to use?

**Apivolve is not yet ready for use, sorry.**

### Set up git hook

**todo**

### Show migrations

To show and check the migrations in directories `dir1` and `dir2`:

    apivolve -d=dir1 -d=dir2 check
    apivolve -d=dir1 -d=dir2 list

You can also set the directories using `APIVOLVE_MIGRATION_PATH`.

### Generate code

    apivolve gen

### Create a migration

First create the new file:

    apivolve new

Then edit the newly generated file.

**todo**

## FAQ

#### How are version numbers determined?



#### Which languages are supported?

Apivolve is not tied to any language and could theoretically generate code for any language. However, the code generation is not yet actually complete for any languages. Rust will probably be first.

#### Which protocols are supported?

You can use Apivolve with any protocol (like http, sockets, websockets, ZeroMQ, blockchain...). But you will need to do that part yourself, or with another tool. Apivolve only handles the actual messages, not their transport.

#### What encoding format does Apivolve use?

There are currently two encodings:

* Json for readable, text-based APIs
* MessagePack for compact, binary APIs

Ideally every language must support every encoding, so the number of encodings should be limited.

If there is ever a third encoding, it is probably a binary encoding that uses the fact that the client and server both know the schema to achieve smaller size and/or better performance.

#### Which language do migration files use?

They use a declarative language that is part of Apivolve, with file extension `.apiv`.

#### Is Apivolve secure?

Apivolve is designed to be able to decode untrusted messages safely. It has not been subject to professional security analysis, though.


