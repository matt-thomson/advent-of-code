defmodule Day02 do
  alias Day02.Directions
  alias Day02.Move
  alias Day02.Position

  @start {1, 1}

  def part_one(filename) do
    {:ok, input} = filename |> File.read

    {_, visited} = input
      |> Directions.parse
      |> Enum.reduce({@start, []}, &_part_one/2)

    visited
    |> Enum.map(&Position.position/1)
    |> Enum.map(&Integer.to_string/1)
    |> Enum.join
  end

  defp _part_one(directions, {current, visited}) do
    destination = Move.move(directions, current)
    {destination, visited ++ [destination]}
  end
end
