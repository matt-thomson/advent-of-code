defmodule Day06Test do
  use ExUnit.Case

  test "solves part one" do
    assert Day06.part_one("data/example.txt") == "easter"
  end

  test "solves part two" do
    assert Day06.part_two("data/example.txt") == "advent"
  end
end
