defmodule Day13.CubicleTest do
  use ExUnit.Case

  alias Day13.Cubicle

  test "computes cell neighbours" do
    assert {1, 2} |> Cubicle.neighbours == [{0, 2}, {2, 2}, {1, 1}, {1, 3}]
  end

  test "handles left wall when computing neighbours" do
    assert {0, 2} |> Cubicle.neighbours == [{1, 2}, {0, 1}, {0, 3}]
  end

  test "handles bottom wall when computing neighbours" do
    assert {1, 0} |> Cubicle.neighbours == [{0, 0}, {2, 0}, {1, 1}]
  end

  test "determines whether a cell is a wall" do
    assert {0, 2} |> Cubicle.wall?(10)
    assert {1, 0} |> Cubicle.wall?(10)
    assert {6, 3} |> Cubicle.wall?(10)

    refute {0, 1} |> Cubicle.wall?(10)
    refute {7, 0} |> Cubicle.wall?(10)
    refute {4, 4} |> Cubicle.wall?(10)
  end
end
