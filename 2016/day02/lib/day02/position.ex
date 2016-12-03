defmodule Day02.Position do
  @moduledoc """
  Converts co-ordinates into the number of the button that they represent.
  """

  def part_one({row, col}) do
    if row in 0..2 && col in 0..2 do
      (row * 3) + col + 1
    end
  end

  def part_two({0, 2})                   , do: 1
  def part_two({1, col}) when col in 1..3, do: col + 1
  def part_two({2, col}) when col in 0..4, do: col + 5
  def part_two({3, col}) when col in 1..3, do: col + 9
  def part_two({4, 2})                   , do: 13
  def part_two(_)                        , do: nil
end
