defmodule Day17.DirectionsTest do
  use ExUnit.Case

  alias Day17.Directions

  test "converts directions to strings" do
    input = [:up, :left, :down, :right, :left]
    assert input |> Directions.to_string == "ULDRL"
  end
end
