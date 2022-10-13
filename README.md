# Abel-on-Rust

```
MIT License

Copyright (c) 2022 wang-xman

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
## Containers, JSON Parsers, and Beyond

Abel-on-Rust is the Rust version of the [Abel library](https://www.github.com/wang-xman/Abel).  

Abel library strives to supply container data types that are capable of storing
*any legitimate type* of data. Besides a root class based on which user-defined
types can be introduced, Abel also provides prefabed *intrinsic types* that are
available to model most data.

An immediate use case of Abel is parsing a `JSON`-formatted file and loading
it as an (Abel) container. The standard data types that `JSON` accepts, i.e.
`Null`, `Bool`, `Double`, and string are part of Abel instrinsic types.
Beyond `JSON`, Abel introduces a more relaxed `JSON`-based format, coined as
`JSON+` (JSON plus). `JSON+` format accepts comments and is capable of parsing
and storing more datatypes. For more details on the design philosophy, please
refer to [Abel library](https://www.github.com/wang-xman/Abel).

Here, I focus on how to use Abel-on-Rust's JSON parser system.

## Abel Standard `JSON` parser

Abel-on-Rust provides a parser `abel::JsonParser` to parse a `JSON`-
formatted file, and a *loader* `abel::JsonLoader` to load (from the
parser) the parsed content into a Abel dictionary. You are strongly encouraged
to walk through the [examples](/examples). The comments
are a bit verbose, but they explain in details the following essential
steps, namely, *parsing*, *loading*, and *getting*.

### Parsing and loading a JSON file

Given an `input_file`, parsing and loading can be combined into two lines,
```
let mut json_loader = JsonLoader::new();
json_loader.load_from_file(input_file);
```
the first line creates a *mutable* `JSON` loader and the second line uses the
loader to load the file. Inside
the loader, the file is parsed by a JSON parser and a global dictionary
is then created. The `global_dictionary` shall look like this
```
{
    "ROOT_KEY_" : [
        content_of_the_file
    ]
}
```
This dictionary has a key `"ROOT_KEY_"` and its value is a `abel::List`. You
shall find the content of the JSON file - be it a dictionary or a list - sitting
in this list as an element.

### Getting the target

Containers in Abel-on-Rust, namely, dictionary and list, come with several
useful getter and setter methods. For example,
```
[container].get_ref::<[TargetType]>([key])
```
is a getter method that returns an immutable reference to the target data. 
Here, two parameters deserve more explanation,

- template parameter `TargetType` must be the type of the target data,

- argument `key` is a `string` type, should the `container` be a `abel::Dict`,
  and an `integer` should the `container` be a `abel::List`.

Abel-on-Rust also provides another method
```
[container].get_mut_ref::<TargetType>([key])
```
to return mutable reference to an object in the container.

Indeed, the turbo fish that appears on the getter methods isn't ideal. But
we must face the reality that Rust is a strong-typed language. It is good to know what you are getting, isn't it?

Examples can be found in [directory](/examples/).

## Abel `JSON+` parser

The so-called `JSON+` data format has the following features. `JSON+` format
is very similar to `JSON` in terms of data structure. However,

- `JSON+` allows comment lines or inline comments. Any text that appears
  after symbol `#` is considered as comment.

- `JSON+` accepts more data types. In addition to the standard `Null`, `Bool`,
  `Double`,`Text` (string), current version of `JSON+` also recognises
  `Integer`, `Complex`, `Binary`, and `Bitstring`.

### Parsing, loading, and getting

The *parsing*, *loading*, and *getting* process is almost identical to the above. The only difference is that you must use a dedicated `JSON+` parser
by calling constructor `new_plus`, i.e.
```
let mut loader = JsonLoader::new_plus();
```

And, the rest remains the same.

Examples can be found in [directory](/examples/).

## Version information

- Version: 0.0.1

- Updated: 13 October 2022.

- Compile:  rustc 1.61.0 on Ubuntu 20.04 and MacOS High Sierra

## Directory

- Source code of the Abel library is [here](/abel/src) and it contains a sub-directory dedicated to [unit tests](/abel/src/unittest)

- [Examples](/examples) are stored as a separate cargo package. 