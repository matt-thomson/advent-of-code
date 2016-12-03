defmodule Day03.TriangleTest do
  use ExUnit.Case

  alias Day03.Triangle

  test "parses and sorts the sides" do
    input = "  4   5   3\n"
    output = input |> Triangle.parse

    assert output == {3, 4, 5}
  end

  test "decides whether a triangle is possible" do
    assert Triangle.possible?({3, 4, 5})
    refute Triangle.possible?({1, 2, 5})
  end
end
