defmodule Day16.ChecksumTest do
  use ExUnit.Case

  alias Day16.Checksum

  test "takes a step correctly" do
    assert "110010110100" |> Checksum.step == "110101"
    assert "10000011110010000111" |> Checksum.step == "0111110101"
  end

  test "computes the full checksum" do
    assert "110010110100" |> Checksum.calculate == "100"
    assert "10000011110010000111" |> Checksum.calculate == "01100"    
  end
end
