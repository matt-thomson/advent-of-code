defmodule Day01.Line do
  @moduledoc """
  Calculates the intersection between two line segments.
  """
  def intersect({{x1, y}, {x2, y}}, {{x, y1}, {x, y2}}) do
    intersect({x1, y1}, {x2, y2}, {x, y})
  end

  def intersect({{x, y1}, {x, y2}}, {{x1, y}, {x2, y}}) do
    intersect({x1, y1}, {x2, y2}, {x, y})
  end

  def intersect(_, _), do: nil

  defp intersect({x1, y1}, {x2, y2}, {x, y}) do
    if x in x1..x2 && y in y1..y2 do
      {x, y}
    end
  end
end
