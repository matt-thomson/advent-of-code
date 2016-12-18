defmodule Day18Test do
  use ExUnit.Case

  test "solves the problem" do
    assert Day18.solve("data/example1.txt", "3")  == 6
    assert Day18.solve("data/example2.txt", "10") == 38
  end
end
