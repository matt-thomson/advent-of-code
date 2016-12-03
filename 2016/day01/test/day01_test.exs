defmodule Day01Test do
  use ExUnit.Case

  test "solves part one" do
    assert Day01.part_one("data/example1.txt") == 12
  end

  test "solves part two" do
    assert Day01.part_two("data/example2.txt") == 4
  end
end
