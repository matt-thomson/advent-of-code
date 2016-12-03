defmodule Day01 do
  @moduledoc """
  Entry point for day 1 solver.
  """

  alias Day01.Directions
  alias Day01.Move

  @start {0, 0, :north}

  def part_one(filename) do
    {x, y, _} = filename |> directions |> Enum.reduce(@start, &Move.move/2)
    abs(x) + abs(y)
  end

  defp directions(filename) do
    {:ok, input} = filename |> File.read
    input |> Directions.parse
  end
end
