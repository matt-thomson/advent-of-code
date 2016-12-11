defmodule Day09 do
  @moduledoc """
  Entry point for day 9 solver.
  """
  alias Day09.Decompress

  def part_one(filename) do
    filename
    |> File.stream!
    |> Stream.map(&Decompress.decompress/1)
    |> Stream.map(&String.length/1)
    |> Enum.sum
  end
end
