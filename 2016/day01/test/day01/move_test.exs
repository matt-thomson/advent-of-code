defmodule Day01.MoveTest do
  use ExUnit.Case

  test "moves correctly when facing North" do
    position = {1, 2, :north}

    assert Day01.Move.move({:right, 3}, position) == { 4, 2, :east}
    assert Day01.Move.move({:left,  3}, position) == {-2, 2, :west}
  end

  test "moves correctly when facing South" do
    position = {1, 2, :south}

    assert Day01.Move.move({:right, 3}, position) == {-2, 2, :west}
    assert Day01.Move.move({:left,  3}, position) == { 4, 2, :east}
  end

  test "moves correctly when facing East" do
    position = {1, 2, :east}

    assert Day01.Move.move({:right, 3}, position) == {1, -1, :south}
    assert Day01.Move.move({:left,  3}, position) == {1,  5, :north}
  end

  test "moves correctly when facing West" do
    position = {1, 2, :west}

    assert Day01.Move.move({:right, 3}, position) == {1,  5, :north}
    assert Day01.Move.move({:left,  3}, position) == {1, -1, :south}
  end
end
