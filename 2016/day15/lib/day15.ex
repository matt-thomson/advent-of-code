defmodule Day15 do
  @moduledoc """
  Entry point for day 15 solver.
  """
  alias Day15.Disc
  alias Day15.Solution

  def part_one(filename) do
    solution =
      filename
      |> File.stream!
      |> Stream.map(&Disc.parse/1)
      |> Enum.reduce(Solution.new, &Solution.step/2)

    solution.push
  end
end
