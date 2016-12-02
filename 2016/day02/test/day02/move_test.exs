defmodule Day02.MoveTest do
  use ExUnit.Case

  alias Day02.Move

  test "follows a sequence of moves" do
    assert Move.move([:up, :left, :left], {1, 1}) == {0, 0}
  end
end
