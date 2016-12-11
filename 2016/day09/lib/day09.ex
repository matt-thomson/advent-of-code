defmodule Day09 do
  @moduledoc """
  Entry point for day 9 solver.
  """
  alias Day09.Decompress

  def part_one(filename), do: filename |> solve(&Decompress.step/1)
  def part_two(filename), do: filename |> solve(&Decompress.full/1)

  defp solve(filename, decompress) do
    filename
    |> File.stream!
    |> Stream.map(decompress)
    |> Stream.map(&String.length/1)
    |> Enum.sum
  end
end
