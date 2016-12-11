defmodule Day08.DisplayTest do
  use ExUnit.Case

  alias Day08.Display

  @display Display.new(7, 3)

  test "applies a rect instructon" do
    display =
      @display
      |> Display.apply({:rect, 3, 2})

    assert display |> Display.pixels == MapSet.new([
      {0, 0}, {0, 1}, {0, 2},
      {1, 0}, {1, 1}, {1, 2}
    ])
  end

  test "applies a rotate column instructon" do
    display =
      @display
      |> Display.apply({:rect, 3, 2})
      |> Display.apply({:rotate_column, 1, 1})

    assert display |> Display.pixels == MapSet.new([
      {0, 0},         {0, 2},
      {1, 0}, {1, 1}, {1, 2},
              {2, 1}
    ])
  end

  test "applies a rotate row instructon" do
    display =
      @display
      |> Display.apply({:rect, 3, 2})
      |> Display.apply({:rotate_column, 1, 1})
      |> Display.apply({:rotate_row, 0, 4})

    assert display |> Display.pixels == MapSet.new([
                                      {0, 4},         {0, 6},
      {1, 0}, {1, 1}, {1, 2},
              {2, 1}
    ])
  end
end
