defmodule Day17.PositionTest do
  use ExUnit.Case

  alias Day17.Position

  test "can step from one position to the next" do
    position = Position.new("hijkl")

    position = position |> Position.step(:down)
    assert position.coordinates == {1, 0}
    assert position.directions  == [:down]

    position = position |> Position.step(:right)
    assert position.coordinates == {1, 1}
    assert position.directions  == [:down, :right]

    position = position |> Position.step(:up)
    assert position.coordinates == {0, 1}
    assert position.directions  == [:down, :right, :up]

    position = position |> Position.step(:left)
    assert position.coordinates == {0, 0}
    assert position.directions  == [:down, :right, :up, :left]
  end

  test "finds valid directions" do
    position = Position.new("hijkl")
    assert position |> Position.valid_directions == [:down]

    position = position |> Position.step(:down)
    assert position |> Position.valid_directions == [:up, :right]

    position = position |> Position.step(:right)
    assert position |> Position.valid_directions == []
  end
end
