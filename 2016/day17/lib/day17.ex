defmodule Day17 do
  @moduledoc """
  Entry point for day 17 solver.
  """
  alias Day17.Directions
  alias Day17.Position

  def part_one(input) do
    [input |> Position.new] |> solve_part_one
  end

  defp solve_part_one([position | rest]) do
    if position.coordinates == {3, 3} do
      position.directions |> Directions.to_string
    else
      new_positions =
        position
        |> Position.valid_directions
        |> Enum.map(&Position.step(position, &1))
      solve_part_one(rest ++ new_positions)
    end
  end
end
