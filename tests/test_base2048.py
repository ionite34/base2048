import pytest

import base2048


# noinspection SpellCheckingInspection
def test_b2048_encode() -> None:
    assert base2048.encode(b"Hello there!") == "ϓțƘ໐úɡਪϵà"
    assert base2048.decode("ϓțƘ໐úɡਪϵà") == b"Hello there!"


# noinspection SpellCheckingInspection
@pytest.mark.parametrize(
    "data, expected",
    [
        ("ետћζы༑", "Invalid tail character 5: ['༑']"),
        ("ետћζыY", "Invalid termination character 5: ['Y']"),
        (
            "ետћζы༎Z",
            "Unexpected character 6: ['Z'] after termination sequence 5: ['༎']",
        ),
    ],
)
def test_b2048_decode_err(data: str, expected: str) -> None:
    with pytest.raises(base2048.DecodeError) as exc:
        base2048.decode(data)
    assert str(exc.value) == expected
