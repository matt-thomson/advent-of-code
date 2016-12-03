defmodule Day03Test do
  use ExUnit.Case

  test "solves part one" do
    assert Day03.part_one("data/example.txt") == 4
  end

  test "solves part two" do
    assert Day03.part_two("data/example.txt") == 3
  end
end
