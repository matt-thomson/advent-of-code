defmodule Day03 do
  alias Day03.Triangle

  def part_one(filename) do
    filename
    |> File.stream!
    |> Enum.map(&Triangle.parse/1)
    |> Enum.filter(&Triangle.possible?/1)
    |> Enum.count
  end

  def part_two(filename) do
    filename
    |> File.stream!
    |> Enum.chunk(3)
    |> Enum.flat_map(&Triangle.parse_vertically/1)
    |> Enum.filter(&Triangle.possible?/1)
    |> Enum.count
  end
end
