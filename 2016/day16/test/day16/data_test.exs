defmodule Day16.DataTest do
  use ExUnit.Case

  alias Day16.Data

  test "takes a step correctly" do
    assert "1" |> Data.step == "100"
    assert "0" |> Data.step == "001"
    assert "11111" |> Data.step == "11111000000"
    assert "111100001010" |> Data.step == "1111000010100101011110000"
  end

  test "fills to a given length" do
    assert "10000" |> Data.fill(20) == "10000011110010000111"
  end
end
