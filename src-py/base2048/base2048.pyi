"""Unicode Text Compression and Encoding"""


def b2048encode(s: bytes) -> str:
    """
    Encode a bytes-like object s using Base2048 and returns a string.

    Args:
        s: Bytes-like object to encode.

    Returns:
        str: Encoded string.
    """

def b2048decode(s: str) -> bytes:
    """
    Decodes a bytes-like object or ASCII string s using Base2048.

    Args:
        s: Bytes-like or str object to decode.

    Returns:
        bytes: Decoded bytes.
    """

def compress(data: bytes, level: int = 9) -> bytes:
    """
    Compress a bytes-like object data using zstd.

    Args:
        data: Bytes-like object to compress.
        level: Compression level.

    Returns:
        bytes: Compressed bytes.
    """

def decompress(data: bytes) -> bytes:
    """
    Decompress a bytes-like object data using zstd.

    Args:
        data: Bytes-like object to decompress.

    Returns:
        bytes: Decompressed bytes.
    """
