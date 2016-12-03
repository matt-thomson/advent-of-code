defmodule Day01 do
  @moduledoc """
  Entry point for day 1 solver.
  """

  alias Day01.Directions
  alias Day01.Line
  alias Day01.Move

  @start {0, 0, :north}

  def part_one(filename) do
    {x, y, _} = filename |> directions |> Enum.reduce(@start, &Move.move/2)
    abs(x) + abs(y)
  end

  def part_two(filename) do
    {x, y} =
      filename
      |> directions
      |> Enum.reduce_while({@start, []}, &find_intersection/2)

    abs(x) + abs(y)
  end

  defp directions(filename) do
    {:ok, input} = filename |> File.read
    input |> Directions.parse
  end

  defp find_intersection(direction, {current = {x1, y1, _}, lines}) do
    destination = Move.move(direction, current)
    {x2, y2, _} = destination
    new_line = {{x1, y1}, {x2, y2}}

    intersection =
      lines
      |> Enum.drop(-1)
      |> Enum.find_value(&Line.intersect(&1, new_line))

    if intersection do
      {:halt, intersection}
    else
      {:cont, {destination, lines ++ [new_line]}}
    end
  end
end
