defmodule Day09.DecompressTest do
  use ExUnit.Case

  alias Day09.Decompress

  test "decompresses the examples" do
    assert Decompress.decompress("ADVENT")   == "ADVENT"
    assert Decompress.decompress("A(1x5)BC") == "ABBBBBC"
    assert Decompress.decompress("(3x3)XYZ") == "XYZXYZXYZ"
    assert Decompress.decompress("(6x1)(1x3)A") == "(1x3)A"
    assert Decompress.decompress("X(8x2)(3x3)ABCY") == "X(3x3)ABC(3x3)ABCY"
  end

  test "decompresses multiple times within a string" do
    assert Decompress.decompress("A(2x2)BCD(2x2)EFG") == "ABCBCDEFEFG"
  end
end
