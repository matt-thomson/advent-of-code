defmodule Day02.Step do
  @moduledoc """
  Computes the co-ordinates after taking a step in a particular direction,
  taking into account which co-ordinates represent buttons.
  """

  def step(direction, current, position) do
    destination = destination(direction, current)

    case position.(destination) do
      nil -> current
      _   -> destination
    end
  end

  defp destination(:up,    {row, col}), do: {row - 1, col}
  defp destination(:down,  {row, col}), do: {row + 1, col}
  defp destination(:left,  {row, col}), do: {row    , col - 1}
  defp destination(:right, {row, col}), do: {row    , col + 1}
end
