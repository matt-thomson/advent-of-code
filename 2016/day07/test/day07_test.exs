defmodule Day07Test do
  use ExUnit.Case

  test "solves part one" do
    assert Day07.part_one("data/example1.txt") == 2
  end

  test "solves part two" do
    assert Day07.part_two("data/example2.txt") == 3
  end
end
