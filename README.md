# Base 2048 &emsp; [![pypi_badge]][pypi_url] [![versions]][pypi_url]

[build_badge]: https://github.com/ionite34/base2048/actions/workflows/build.yml/badge.svg
[build_url]: https://github.com/ionite34/base2048/actions/workflows/build.yml
[versions]: https://img.shields.io/pypi/pyversions/base2048
[pypi_badge]: https://badge.fury.io/py/base2048.svg
[pypi_url]: https://pypi.org/project/base2048/
[twitter_count]: https://developer.twitter.com/en/docs/basics/counting-characters
[rs_base]: https://github.com/LLFourn/rust-base2048
[bmp]: https://unicode.org/roadmaps/bmp/

[rtl]: https://wikipedia.org/wiki/Right-to-left_mark

[![build_badge]][build_url]
[![Rust Tests](https://github.com/ionite34/base2048/actions/workflows/rust-test.yml/badge.svg)](https://github.com/ionite34/base2048/actions/workflows/rust-test.yml)
[![codecov](https://codecov.io/gh/ionite34/base2048/branch/main/graph/badge.svg?token=1Qdx8w3zoy)](https://codecov.io/gh/ionite34/base2048)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/ionite34/base2048/main.svg)](https://results.pre-commit.ci/latest/github/ionite34/base2048/main)

### When Base 64 is not enough

Allows up to 11 bits of data per unicode character as counted by
social media and chat platforms such as [Twitter][twitter_count] and Discord.

Uses a limited charset within the [Basic Multilingual Plane][bmp].

Based on, and uses a compatible encoding table with the Rust crate [rust-base2048][rs_base].

### - Charset displayable on most locales and platforms
### - No control sequences, punctuation, quotes, or [RTL][rtl] characters

## Getting Started
```shell
pip install base2048
```

```python
import base2048

base2048.encode(b'Hello!')
>> 'ϓțƘ໐µ'

base2048.decode('ϓțƘ໐µ')
>> b'Hello!'
```

### Up to 2x less counted characters compared to Base 64

```python
import zlib
import base64

import base2048

string = ('🐍 🦀' * 1000 + '🐕' * 1000).encode()
data = zlib.compress(string)

base64.b64encode(b)
>> b'eJztxrEJACAQBLBVHNUFBBvr75zvRvgxBEkRSGqvkbozIiIiIiIiIiIiIiIiIiIiIiJf5wAAAABvNbM+EOk=' # 84 chars

base2048.encode(b)
>> 'ը྿Ԧҩ২ŀΏਬйཬΙāಽႩԷ࿋ႬॴŒǔ०яχσǑňॷβǑňॷβǑňॷβǯၰØØÀձӿօĴ༎' # 46 chars

unpacked = zlib.decompress(base2048.decode('ը྿Ԧҩ২ŀΏਬйཬΙāಽႩԷ࿋ႬॴŒǔ०яχσǑňॷβǑňॷβǑňॷβǯၰØØÀձӿօĴ༎')).decode()
len(unpacked)
>> 4000

unpacked[2000:2002]
>> '🦀🐍'
```

## License
The code in this project is released under the [MIT License](LICENSE).

## Related and prior works
> Javascript - [base2048](https://github.com/qntm/base2048)

> Rust - [rust-base2048][rs_base]
