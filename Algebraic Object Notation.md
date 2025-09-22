# Algebraic Object Notation (AON)

This is my attempt at an object notation format with support for union types and
dot notation.

## Values

### Null

No value -> `null`

### Boolean

Either `true` or `false`

### Number

Any real number -> `1`, `1.0`

### String

Any text -> `"Hello World"`

### Struct

A collection of key-value pairs -> `{ key: "value" }`

```
{
    key: "value",
    key2: "value2"
}
```

### Union

A union is a collection where only one value can be present at a time. It is represented as a struct with a special key called `type` that specifies the type of the union.
`type1` is one of the discriminants.

```
#type1 {
    key: "value"
}
```

This syntax allows a normal JSON parser to parse the object, but won't output the correct results without a custom deserializer.

### List

A collection of values -> `[ "value1", "value2" ]`

```
[
    "value1",
    "value2"
]
```

### Features

The syntax allows for trailing commas in structs, arrays and unions.

```
{
    key: "value",
    key2: "value2",
}
```

The syntax allows for dot notation which is unwrapped during deserialization and
automatically wrapped during serialization if only one property of the nested
structure is given. i.e. and object only has one property specified. Should be
possible to disable.

```
{
    key: "value",
    key2: {
        prop: 100
    }
}
```

turns into

```
{
    key: "value",
    key2.prop: 100
}
```

this happens for as many nested objects with only one property as possible

```
{
    a: {
        b: {
            c: {
                value: 0
            }
        }
    }
}
```

turns into

```
{
    a.b.c.value: 0
}
```

### Limitations

AON objects have the following limitations:

- A key (property name) cannot contain any periods since it would interfere with
  the dot notation feature.

### Spec

- A value is parsed at the top level always. arrays, objects and union objects
are considered values.

- A value which is not at the top level is parsed after an open bracket '[', or
after a comma ',' while inside and array enclosed by brackets '[]' OR after a
colon ':' as the value associated with a key.

- A null value is parsed as exactly the word 'null'.
- A boolean value is parsed as exactly the word 'true' or exactly the word
'false'.
- A number value is parsed as any number of digits and the optionally a period
for the decimal point followed by one or more digits. if the decimal point is
omitted, there must be at least one digit to be a valid number. the number can
optionally be prefixed by a dash '-' for negative numbers.
- A string value is enclosed by quotation marks '""'. Within the quotation marks
  can be any character except a quotation mark. To include a
quotation mark use the backslash '\' character to escape is like this: '\"'. to
include a backslash '\' characters, escape it with another backslash like this:
'\\'. Other control characters can be included in the same way by escaping them.
These include:
- New line = '\n'
- Backspace = '\b'
- TODO: Add support for more escapes
- TODO: Add support for unicode code points

- An object always starts an open brace '{' and ends
  with a matching closing brace '}'.

- An array always starts an open bracket '\[' and ends
  with a matching closing bracket '\]'

- A union object always starts with a hash-tag '#'. The variant is then parsed
as any characters until the next open brace '{', but with whitespace trimmed from
both ends. Then the object part comes next in the same fashion as a normal
object.

- A key is parsed after an open brace '{', starts as any non-whitespace, non-period character
and continues until the next period OR until the next ':' OR after a comma ','
while inside an object or union object enclosed by braces '{}'.

### Examples

Here are our models in pseudo code:

```
struct MyStruct {
    nullValue: null,
    boolValue: boolean,
    numberValue: number,
    stringValue: string,
    unionValue: Option<MyStruct>,
}

union Option<T> {
    Some(T),
    None,
}
```

Here are some representations of MyStruct with some sample values:

```aon
{
  nullValue: null,
  boolValue: true,
  numberValue: 14,
  stringValue: "Foo",
  unionValue: #some {
    nullValue: null,
    boolValue: false,
    numberValue: -1.618,
    stringValue: "Bar",
    unionValue: #none {}
  }
}
```

If a union variant has no fields, the braces can be omitted:

```aon
{
  nullValue: null,
  boolValue: false,
  numberValue: -1.618,
  stringValue: "Bar",
  unionValue: #none
}
```
