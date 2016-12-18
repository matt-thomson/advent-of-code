defmodule Day17 do
  @moduledoc """
  Entry point for day 17 solver.
  """
  alias Day17.Directions
  alias Day17.Position

  def part_one(input) do
    [input |> Position.new] |> solve_part_one
  end

  def part_two(input) do
    [input |> Position.new] |> solve_part_two(0)
  end

  defp solve_part_one([position | rest]) do
    if position.coordinates == {3, 3} do
      position.directions |> Directions.to_string
    else
      solve_part_one(rest ++ (position |> new_positions))
    end
  end

  defp solve_part_two([position | rest], acc) do
    if position.coordinates == {3, 3} do
      new_acc = max(acc, position.directions |> Enum.count)
      solve_part_two(rest, new_acc)
    else
      solve_part_two(rest ++ (position |> new_positions), acc)
    end
  end
  defp solve_part_two([], acc), do: acc

  defp new_positions(position) do
    position
    |> Position.valid_directions
    |> Enum.map(&Position.step(position, &1))
  end
end
