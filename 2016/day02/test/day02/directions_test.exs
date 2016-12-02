defmodule Day02.DirectionsTest do
  use ExUnit.Case

  alias Day02.Directions

  test "parses directions" do
    input = "UDL\nRL"
    output = Directions.parse(input)

    assert output == [
      [:up, :down, :left],
      [:right, :left]
    ]
  end
end
