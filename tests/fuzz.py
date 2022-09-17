import frelatage

import base2048


def fuzz_encode(data: str):
    data: bytes = data.encode("raw_unicode_escape")  # type: ignore
    result = base2048.encode(data)  # type: ignore
    assert base2048.decode(result) == data


if __name__ == "__main__":
    frelatage.Config.FRELATAGE_INPUT_DIR = ""
    frelatage.Config.FRELATAGE_MAX_CYCLES_WITHOUT_NEW_PATHS = 100
    source = frelatage.load_corpus("./dicts", ["dict"])
    assert len(source) > 0
    f = frelatage.Fuzzer(fuzz_encode, [source])
    f.fuzz_all()
