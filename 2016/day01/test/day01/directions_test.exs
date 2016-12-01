defmodule Day01.DirectionsTest do
  use ExUnit.Case

  test "parses directions" do
    input = "R1, L2, R3\n"
    output = Day01.Directions.parse(input)

    assert output == [
      {:right, 1},
      {:left, 2},
      {:right, 3}
    ]
  end
end
