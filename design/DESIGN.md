# Design

This library was designed with simplicity in mind for parsing use cases which
are not user facing, due to the lack of error messages.

## Serialization Strategy

### Constant Sized Types

We pack as tightly as we can using a little endian byte ordering.

### Collections

We encode the length as a `u64` and then each element according to its type.
Beware that these collections can be of arbitrary size, and you must be careful
when they are coming from an untrusted source as they are a ripe denial of
service attack vector.

### Custom Types

For structs, we simply encode each field one after another. For enums, we only
permit up to 256 variants and use a `u8` to encode a tag. This tag is not
guaranteed to be the same number as the compiler uses to discriminate the enum,
instead it is guaranteed to be the same as the index of the variant amongst its
peers. For unit structs, we encode them as an empty string.

## Testing Approach

For constant size types, we are generating random values and testing that
serialization is a round trip. For collections, we generate collections of a
random size up to one hundred and test that serialization is a round trip.  We
use Tarpaulin for code coverage and we try to achieve close to total coverage,
however currently we are not testing so much for when things should not parse.
This still leaves us with 90+% code coverage.
