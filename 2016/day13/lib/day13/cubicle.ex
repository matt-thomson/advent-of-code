defmodule Day13.Cubicle do
  @moduledoc """
  Functions for working with cubicles.
  """
  def neighbours({x, y}) do
    [{x - 1, y}, {x + 1, y}, {x, y - 1}, {x, y + 1}]
    |> Enum.filter(&inside_building?/1)
  end

  def wall?({x, y}, input) do
    value = x * x + 3 * x + 2 * x * y + y + y * y + input

    value =
      value
      |> Integer.to_string(2)
      |> to_char_list
      |> Enum.count(&(&1 == ?1))

    value |> rem(2) == 1
  end

  defp inside_building?({x, y}), do: x >= 0 && y >= 0
end
