defmodule Day03.TriangleTest do
  use ExUnit.Case

  alias Day03.Triangle

  test "parses and sorts the sides" do
    input = "  4   5   3\n"
    output = input |> Triangle.parse

    assert output == [3, 4, 5]
  end

  test "parses triangles vertically" do
    input = [
      "101 202 303",
      "102 203 301",
      "103 201 302"
    ]
    output = input |> Triangle.parse_vertically

    assert output == [
      [101, 102, 103],
      [201, 202, 203],
      [301, 302, 303]
    ]
  end

  test "decides whether a triangle is possible" do
    assert Triangle.possible?([3, 4, 5])
    refute Triangle.possible?([1, 2, 5])
  end
end
