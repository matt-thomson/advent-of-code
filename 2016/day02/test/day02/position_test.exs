defmodule Day02.PositionTest do
  use ExUnit.Case

  alias Day02.Position

  test "translates co-ordinates to buttons" do
    assert Position.position({0, 0}) == 1
    assert Position.position({1, 1}) == 5
    assert Position.position({2, 2}) == 9
  end
end
