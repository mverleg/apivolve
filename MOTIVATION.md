# Why Apivolve?

Apivolve is a tool to create schemas for APIs, in the same category as Protobuf, Avro, XSD, and JSON Schema. Yet it is different.

The core goal is the same as other schema formats: to specifying an API that different programs can use to communicate. Across programming languages, networks and API versions.

But unlike other schemas, the code you write is not the latest schema. Instead, you write what changes compared to the previous version of the schema. Why? For backwards compatibility:

### Why not just the latest schema?

Knowing the latest schema often does not let you parse data from older versions.

The schema does not know that you getting for `balanceEuros: 10000` instead of `balance: 10000` is just a rename, while `debtLimit: 10000` instead of `balance: 10000` is not. And when you move a field to another object, it is worse.
 
You can work around this with a combination of 1) being very careful to only make changes to the api that are compatible, or 2) explicitly adding code to support every older version and maintaining that.

### Why not just keep all the schemas?

What if you just keep all the schemas, for example by attaching it to the data (like Avro)? Unfortunately this is not enough.
 
It sometimes works, but in general it does not. Knowing that `balance` between schema version `X-1` and `X` while `balanceEuros` and `debtLimit` appeared does not tell the computer which one was a rename and which one is new.

That is exactly the information that Apivolve evolutions do describe: `balanceEuros` is a rename, and `debtLimit` is a new field.

All the code to handle that and support `X-1` is generated, while you can focus on the latest and greatest, version `X`.

### It's like SQL (conceptually)

Conceptually Apivolve is like the DDL statements in SQL (create, drop, alter, etc).

In SQL you don't write "the table now has `balanceEuros` and `debtLimit` columns, please figure out how".

Instead, you tell it that you are renaming `balance` to `balanceEuros` so please move the data, and you are adding `debtLimit` and please use `0` as a default, but require a value for all future inserts.

And after that, you can forget the old table layout ever existed.

### Strict validation

One way to make sure data from older schemas can be interpreted by the latest schema, is to just not check it very carefully.

This is a popular approach, for example taken by Protobuf by making everything optional. And it has its uses.

But the Apivolve philosophy is that a schema should describe the data in as much detail as possible. It should describe whether `email: null` is valid, or `email: name+spam@gmail.com`, so that all users are aware of what to send, and what to expect back.

Otherwise it just ends up being your application code that has to do all the validation and handle all the edge cases.
