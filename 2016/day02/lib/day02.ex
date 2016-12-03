defmodule Day02 do
  @moduledoc """
  Entry point for day 2 solver.
  """

  alias Day02.Directions
  alias Day02.Position
  alias Day02.Step

  def part_one(filename) do
    filename |> solve({1, 1}, &Position.part_one/1)
  end

  def part_two(filename) do
    filename |> solve({2, 0}, &Position.part_two/1)
  end

  defp solve(filename, start, position) do
    {:ok, input} = filename |> File.read

    {_, visited} = input
      |> Directions.parse
      |> Enum.reduce({start, []}, &move(&1, &2, position))

    visited
    |> Enum.map(position)
    |> Enum.map(&Integer.to_string(&1, 16))
    |> Enum.join
  end

  defp move(directions, {current, visited}, position) do
    destination =
      directions
      |> Enum.reduce(current, &Step.step(&1, &2, position))

    {destination, visited ++ [destination]}
  end
end
