defmodule Day09.DecompressTest do
  use ExUnit.Case

  alias Day09.Decompress

  test "decompresses the examples" do
    assert Decompress.step("ADVENT")   == "ADVENT"
    assert Decompress.step("A(1x5)BC") == "ABBBBBC"
    assert Decompress.step("(3x3)XYZ") == "XYZXYZXYZ"
    assert Decompress.step("(6x1)(1x3)A") == "(1x3)A"
    assert Decompress.step("X(8x2)(3x3)ABCY") == "X(3x3)ABC(3x3)ABCY"
  end

  test "decompresses multiple times within a string" do
    assert Decompress.step("A(2x2)BCD(2x2)EFG") == "ABCBCDEFEFG"
  end

  test "decompresses fully" do
    assert Decompress.full("(3x3)XYZ") == "XYZXYZXYZ"
    assert Decompress.full("X(8x2)(3x3)ABCY") == "XABCABCABCABCABCABCY"
    assert Decompress.full("(27x12)(20x12)(13x14)(7x10)(1x12)A") == String.duplicate("A", 241920)
  end
end
