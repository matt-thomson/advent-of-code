defmodule Day03 do
  alias Day03.Triangle

  def part_one(filename) do
    filename
    |> File.stream!
    |> Enum.map(&Triangle.parse/1)
    |> Enum.filter(&Triangle.possible?/1)
    |> Enum.count
  end
end
