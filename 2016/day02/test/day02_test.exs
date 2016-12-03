defmodule Day02Test do
  use ExUnit.Case

  test "solves part one" do
    assert Day02.part_one("data/example.txt") == "1985"
  end

  test "solves part two" do
    assert Day02.part_two("data/example.txt") == "5DB3"
  end
end
