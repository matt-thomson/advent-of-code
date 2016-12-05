defmodule Day03 do
  @moduledoc """
  Entry point for day 3 solver.
  """

  alias Day03.Triangle

  def part_one(filename) do
    filename
    |> File.stream!
    |> Stream.map(&Triangle.parse/1)
    |> Stream.filter(&Triangle.possible?/1)
    |> Enum.count
  end

  def part_two(filename) do
    filename
    |> File.stream!
    |> Stream.chunk(3)
    |> Stream.flat_map(&Triangle.parse_vertically/1)
    |> Stream.filter(&Triangle.possible?/1)
    |> Enum.count
  end
end
