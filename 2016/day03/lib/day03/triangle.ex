defmodule Day03.Triangle do
  @moduledoc """
  Functions to parse triangles and check if they're possible.
  """

  def parse(input) do
    input |> to_integers |> Enum.sort
  end

  def parse_vertically(inputs) do
    [[a1, a2, a3], [b1, b2, b3], [c1, c2, c3]] =
      inputs
      |> Enum.map(&to_integers/1)

    [
      [a1, b1, c1] |> Enum.sort,
      [a2, b2, c2] |> Enum.sort,
      [a3, b3, c3] |> Enum.sort
    ]
  end

  def possible?([a, b, c]), do: a + b > c

  defp to_integers(input) do
    input |> String.split |> Enum.map(&parse_side/1)
  end

  defp parse_side(side) do
    {length, ""} = side |> Integer.parse
    length
  end
end
