import base2048


# noinspection SpellCheckingInspection
def test_b2048_encode():
    assert base2048.encode(b"Hello there!") == "ϓțƘ໐úɡਪϵà"
    assert base2048.decode("ϓțƘ໐úɡਪϵà") == b"Hello there!"

