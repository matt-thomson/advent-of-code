defmodule Day03.Triangle do
  def parse(input) do
    [a, b, c] =
      input
      |> String.split
      |> Enum.map(&parse_side/1)
      |> Enum.sort

    {a, b, c}
  end

  def possible?({a, b, c}), do: a + b > c

  defp parse_side(side) do
    {length, ""} = side |> Integer.parse
    length
  end
end
