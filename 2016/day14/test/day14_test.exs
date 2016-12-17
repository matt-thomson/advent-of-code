defmodule Day14Test do
  use ExUnit.Case

  test "solves part one" do
    assert Day14.part_one("abc") == 22728
  end

  @tag timeout: 300000
  test "solves part two" do
    assert Day14.part_two("abc") == 22551
  end
end
